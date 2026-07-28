use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Algorithm, Argon2, Params, Version};
use rand_core::OsRng;

use crate::error::AppError;

pub const MAX_PASSWORD_BYTES: usize = 1024;
const ARGON2_MEMORY_KIB: u32 = 19_456;
const ARGON2_ITERATIONS: u32 = 2;
const ARGON2_PARALLELISM: u32 = 1;
const ARGON2_OUTPUT_LENGTH: usize = 32;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    validate_password_input(password)?;
    let salt = SaltString::generate(&mut OsRng);
    password_hasher()?
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| AppError::Internal(error.to_string()))
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    validate_password_input(password)?;
    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|error| AppError::Internal(error.to_string()))?;

    Ok(password_hasher()?
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

fn password_hasher() -> Result<Argon2<'static>, AppError> {
    let params = Params::new(
        ARGON2_MEMORY_KIB,
        ARGON2_ITERATIONS,
        ARGON2_PARALLELISM,
        Some(ARGON2_OUTPUT_LENGTH),
    )
    .map_err(|error| AppError::Internal(error.to_string()))?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

fn validate_password_input(password: &str) -> Result<(), AppError> {
    if password.len() > MAX_PASSWORD_BYTES || password.contains('\0') {
        return Err(AppError::Validation("password input is invalid".to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use argon2::password_hash::PasswordHash;

    use super::{hash_password, verify_password};

    #[test]
    fn password_hashes_use_explicit_argon2id_policy_and_unique_salts() {
        let first = hash_password("correct horse battery staple").unwrap();
        let second = hash_password("correct horse battery staple").unwrap();
        assert_ne!(first, second);
        let parsed = PasswordHash::new(&first).unwrap();
        assert_eq!(parsed.algorithm.as_str(), "argon2id");
        assert_eq!(parsed.version, Some(19));
        assert_eq!(parsed.params.get("m").unwrap().decimal().unwrap(), 19_456);
        assert_eq!(parsed.params.get("t").unwrap().decimal().unwrap(), 2);
        assert_eq!(parsed.params.get("p").unwrap().decimal().unwrap(), 1);
        assert!(verify_password("correct horse battery staple", &first).unwrap());
        assert!(!verify_password("incorrect", &first).unwrap());
    }

    #[test]
    fn password_hashing_and_verification_reject_oversized_inputs() {
        let oversized = "a".repeat(1025);
        assert!(hash_password(&oversized).is_err());
        let hash = hash_password("valid local test password").unwrap();
        assert!(verify_password(&oversized, &hash).is_err());
    }
}
