use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use rand_core::{OsRng, RngCore};
use totp_rs::{Algorithm, TOTP};
use uuid::Uuid;

use crate::config::MfaEncryptionKeyRing;
use crate::error::AppError;

pub const TOTP_SECRET_BYTES: usize = 20;
pub const TOTP_DIGITS: usize = 6;
pub const TOTP_PERIOD_SECONDS: u64 = 30;
pub const TOTP_CLOCK_SKEW_STEPS: i64 = 1;
const MFA_ENCRYPTION_VERSION: i16 = 1;
const MFA_NONCE_BYTES: usize = 12;

#[derive(Clone)]
pub struct TotpMaterial {
    pub secret: [u8; TOTP_SECRET_BYTES],
    pub manual_secret: String,
    pub provisioning_uri: String,
    pub qr_code_base64: String,
}

impl TotpMaterial {
    pub fn generate_at(&self, unix_time_seconds: u64) -> Result<String, AppError> {
        totp(&self.secret, None, "account", 0).map(|totp| totp.generate(unix_time_seconds))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncryptedTotpSecret {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub kid: String,
    pub version: i16,
}

pub fn generate_totp_material(issuer: &str, account_label: &str) -> Result<TotpMaterial, AppError> {
    let mut secret = [0_u8; TOTP_SECRET_BYTES];
    OsRng.fill_bytes(&mut secret);
    let totp = totp(
        &secret,
        Some(issuer),
        account_label,
        TOTP_CLOCK_SKEW_STEPS as u8,
    )?;
    let provisioning_uri = totp.get_url();
    let manual_secret = totp.get_secret_base32();
    let qr_code_base64 = totp
        .get_qr_base64()
        .map_err(|_| AppError::Internal("unable to generate MFA QR code".to_owned()))?;
    Ok(TotpMaterial {
        secret,
        manual_secret,
        provisioning_uri,
        qr_code_base64,
    })
}

pub fn verify_totp_at(
    secret: &[u8],
    code: &str,
    unix_time_seconds: i64,
) -> Result<Option<i64>, AppError> {
    if unix_time_seconds < 0
        || code.len() != TOTP_DIGITS
        || !code.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Ok(None);
    }
    let verifier = totp(secret, None, "account", 0)?;
    let current_step = unix_time_seconds.div_euclid(TOTP_PERIOD_SECONDS as i64);
    for offset in [0_i64, -1, 1] {
        let Some(step) = current_step.checked_add(offset) else {
            continue;
        };
        if step < 0 {
            continue;
        }
        let Some(timestamp) = step.checked_mul(TOTP_PERIOD_SECONDS as i64) else {
            continue;
        };
        if verifier.check(code, timestamp as u64) {
            return Ok(Some(step));
        }
    }
    Ok(None)
}

pub fn encrypt_totp_secret(
    key_ring: &MfaEncryptionKeyRing,
    user_id: Uuid,
    enrollment_id: Uuid,
    plaintext: &[u8],
) -> Result<EncryptedTotpSecret, AppError> {
    if plaintext.len() != TOTP_SECRET_BYTES {
        return Err(AppError::Internal(
            "MFA secret length is invalid".to_owned(),
        ));
    }
    let active = key_ring.active();
    let cipher = Aes256Gcm::new_from_slice(&active.key)
        .map_err(|_| AppError::Internal("MFA encryption configuration is invalid".to_owned()))?;
    let mut nonce = [0_u8; MFA_NONCE_BYTES];
    OsRng.fill_bytes(&mut nonce);
    let aad = encryption_aad(user_id, enrollment_id, MFA_ENCRYPTION_VERSION);
    let cipher_nonce = Nonce::try_from(nonce.as_slice())
        .map_err(|_| AppError::Internal("MFA encryption nonce is invalid".to_owned()))?;
    let ciphertext = cipher
        .encrypt(
            &cipher_nonce,
            Payload {
                msg: plaintext,
                aad: &aad,
            },
        )
        .map_err(|_| AppError::Internal("unable to encrypt MFA secret".to_owned()))?;
    Ok(EncryptedTotpSecret {
        ciphertext,
        nonce: nonce.to_vec(),
        kid: active.kid.clone(),
        version: MFA_ENCRYPTION_VERSION,
    })
}

pub fn decrypt_totp_secret(
    key_ring: &MfaEncryptionKeyRing,
    user_id: Uuid,
    enrollment_id: Uuid,
    encrypted: &EncryptedTotpSecret,
    now: i64,
) -> Result<Vec<u8>, AppError> {
    if encrypted.version != MFA_ENCRYPTION_VERSION
        || encrypted.nonce.len() != MFA_NONCE_BYTES
        || encrypted.ciphertext.len() != TOTP_SECRET_BYTES + 16
    {
        return Err(AppError::Internal(
            "MFA secret record is invalid".to_owned(),
        ));
    }
    let key = key_ring
        .decryption_key(&encrypted.kid, now)
        .ok_or_else(|| AppError::Internal("MFA encryption key is unavailable".to_owned()))?;
    let cipher = Aes256Gcm::new_from_slice(&key.key)
        .map_err(|_| AppError::Internal("MFA encryption configuration is invalid".to_owned()))?;
    let aad = encryption_aad(user_id, enrollment_id, encrypted.version);
    let cipher_nonce = Nonce::try_from(encrypted.nonce.as_slice())
        .map_err(|_| AppError::Internal("MFA encryption nonce is invalid".to_owned()))?;
    cipher
        .decrypt(
            &cipher_nonce,
            Payload {
                msg: &encrypted.ciphertext,
                aad: &aad,
            },
        )
        .map_err(|_| AppError::Internal("MFA secret authentication failed".to_owned()))
}

fn totp(
    secret: &[u8],
    issuer: Option<&str>,
    account_label: &str,
    skew: u8,
) -> Result<TOTP, AppError> {
    if secret.len() < TOTP_SECRET_BYTES {
        return Err(AppError::Internal("MFA secret is invalid".to_owned()));
    }
    TOTP::new(
        Algorithm::SHA1,
        TOTP_DIGITS,
        skew,
        TOTP_PERIOD_SECONDS,
        secret.to_vec(),
        issuer.map(str::to_owned),
        account_label.to_owned(),
    )
    .map_err(|_| AppError::Internal("MFA TOTP configuration is invalid".to_owned()))
}

fn encryption_aad(user_id: Uuid, enrollment_id: Uuid, version: i16) -> Vec<u8> {
    format!("zinhar:mfa:v{version}:{user_id}:{enrollment_id}").into_bytes()
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::{
        EncryptedTotpSecret, decrypt_totp_secret, encrypt_totp_secret, generate_totp_material,
        verify_totp_at,
    };
    use crate::config::{Config, MfaEncryptionKeyConfig, MfaEncryptionKeyStatus};

    #[test]
    fn totp_uses_standard_parameters_and_deterministic_time() {
        let material = generate_totp_material("ZinharCMS Test", "account").unwrap();
        assert_eq!(material.secret.len(), 20);
        assert!(material.provisioning_uri.starts_with("otpauth://totp/"));
        let different = generate_totp_material("ZinharCMS Test", "account").unwrap();
        assert_ne!(material.secret, different.secret);

        let now = 1_800_000_000_u64;
        let code = material.generate_at(now).unwrap();
        assert_eq!(
            verify_totp_at(&material.secret, &code, now as i64).unwrap(),
            Some((now / 30) as i64)
        );
        assert_eq!(
            verify_totp_at(&material.secret, "not-a-code", now as i64).unwrap(),
            None
        );
        assert_eq!(
            verify_totp_at(&material.secret, "12345", now as i64).unwrap(),
            None
        );
        assert_eq!(
            verify_totp_at(&material.secret, "1234567", now as i64).unwrap(),
            None
        );
    }

    #[test]
    fn totp_vectors_leading_zero_and_clock_skew_follow_policy() {
        let rfc_secret = *b"12345678901234567890";
        assert_eq!(verify_totp_at(&rfc_secret, "287082", 59).unwrap(), Some(1));
        assert_eq!(verify_totp_at(&rfc_secret, "287083", 59).unwrap(), None);
        assert_eq!(verify_totp_at(&rfc_secret, "287082", 89).unwrap(), Some(1));
        assert_eq!(verify_totp_at(&rfc_secret, "287082", 119).unwrap(), None);

        let mut leading_zero = None;
        for step in 1_u64..=100_000 {
            let timestamp = step * 30;
            let code = super::totp(&rfc_secret, None, "account", 0)
                .unwrap()
                .generate(timestamp);
            if code.starts_with('0') {
                leading_zero = Some((timestamp, code));
                break;
            }
        }
        let (timestamp, code) = leading_zero.expect("a leading-zero vector must be found");
        assert_eq!(code.len(), 6);
        assert_eq!(
            verify_totp_at(&rfc_secret, &code, timestamp as i64).unwrap(),
            Some((timestamp / 30) as i64)
        );
    }

    #[test]
    fn encrypted_totp_secrets_are_authenticated_and_record_bound() {
        let config = Config::test_with_stripe_secret("test-webhook-secret");
        let user_id = Uuid::now_v7();
        let enrollment_id = Uuid::now_v7();
        let first = encrypt_totp_secret(
            &config.mfa_encryption_key_ring,
            user_id,
            enrollment_id,
            &[9_u8; 20],
        )
        .unwrap();
        let second = encrypt_totp_secret(
            &config.mfa_encryption_key_ring,
            user_id,
            enrollment_id,
            &[9_u8; 20],
        )
        .unwrap();

        assert_ne!(first.nonce, second.nonce);
        assert_ne!(first.ciphertext, [9_u8; 20]);
        assert_eq!(
            decrypt_totp_secret(
                &config.mfa_encryption_key_ring,
                user_id,
                enrollment_id,
                &first,
                chrono::Utc::now().timestamp(),
            )
            .unwrap(),
            [9_u8; 20]
        );
        assert!(
            decrypt_totp_secret(
                &config.mfa_encryption_key_ring,
                Uuid::now_v7(),
                enrollment_id,
                &first,
                chrono::Utc::now().timestamp(),
            )
            .is_err()
        );

        let tampered_ciphertext = EncryptedTotpSecret {
            ciphertext: {
                let mut value = first.ciphertext.clone();
                value[0] ^= 1;
                value
            },
            nonce: first.nonce.clone(),
            kid: first.kid.clone(),
            version: first.version,
        };
        assert!(
            decrypt_totp_secret(
                &config.mfa_encryption_key_ring,
                user_id,
                enrollment_id,
                &tampered_ciphertext,
                chrono::Utc::now().timestamp(),
            )
            .is_err()
        );
        let tampered_nonce = EncryptedTotpSecret {
            ciphertext: first.ciphertext.clone(),
            nonce: {
                let mut value = first.nonce.clone();
                value[0] ^= 1;
                value
            },
            kid: first.kid.clone(),
            version: first.version,
        };
        assert!(
            decrypt_totp_secret(
                &config.mfa_encryption_key_ring,
                user_id,
                enrollment_id,
                &tampered_nonce,
                chrono::Utc::now().timestamp(),
            )
            .is_err()
        );
        let unknown_kid = EncryptedTotpSecret {
            ciphertext: first.ciphertext.clone(),
            nonce: first.nonce.clone(),
            kid: "unknown-mfa-key".to_owned(),
            version: first.version,
        };
        assert!(
            decrypt_totp_secret(
                &config.mfa_encryption_key_ring,
                user_id,
                enrollment_id,
                &unknown_kid,
                chrono::Utc::now().timestamp(),
            )
            .is_err()
        );
        let wrong_config = Config::test_with_mfa_keys(vec![MfaEncryptionKeyConfig::test(
            &first.kid,
            MfaEncryptionKeyStatus::Active,
            [77_u8; 32],
            None,
        )]);
        assert!(
            decrypt_totp_secret(
                &wrong_config.mfa_encryption_key_ring,
                user_id,
                enrollment_id,
                &first,
                chrono::Utc::now().timestamp(),
            )
            .is_err()
        );
    }
}
