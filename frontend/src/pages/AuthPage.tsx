import { FormEvent, useEffect, useState } from "react";
import { LockKeyhole, LogIn, UserPlus } from "lucide-react";
import { Navigate, useLocation, useNavigate } from "react-router";

import { StatusBadge } from "../components/StatusBadge";
import { LanguageSelect, useI18n } from "../i18n";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type { MfaProofKind } from "../types/api";

type AuthMode = "login" | "register";

export function AuthPage() {
  const navigate = useNavigate();
  const location = useLocation();
  const { t } = useI18n();
  const authStatus = useAppStore((state) => state.authStatus);
  const setSession = useAppStore((state) => state.setSession);
  const [mode, setMode] = useState<AuthMode>("login");
  const [email, setEmail] = useState("");
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [preAuthToken, setPreAuthToken] = useState<string | null>(null);
  const [preAuthExpiresAt, setPreAuthExpiresAt] = useState<number | null>(null);
  const [mfaCode, setMfaCode] = useState("");
  const [mfaProofKind, setMfaProofKind] = useState<MfaProofKind>("totp");

  useEffect(() => {
    if (!preAuthToken || preAuthExpiresAt === null) return;
    const remainingMs = Math.max(0, preAuthExpiresAt - Date.now());
    const timeout = window.setTimeout(() => {
      setPreAuthToken(null);
      setPreAuthExpiresAt(null);
      setMfaCode("");
      setError("The MFA challenge expired. Sign in again.");
    }, remainingMs);
    return () => window.clearTimeout(timeout);
  }, [preAuthExpiresAt, preAuthToken]);

  const returnTo =
    typeof location.state === "object" &&
    location.state !== null &&
    "from" in location.state &&
    typeof location.state.from === "string"
      ? location.state.from
      : "/";

  if (authStatus === "unknown" || authStatus === "refreshing") {
    return <main className="auth-screen" aria-busy="true">Restoring session…</main>;
  }
  if (authStatus === "authenticated") return <Navigate to={returnTo} replace />;

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);

    try {
      const response = preAuthToken
        ? await api.auth.verifyMfa(preAuthToken, mfaProofKind, mfaCode)
        : mode === "login"
          ? await api.auth.login(email, password)
          : await api.auth.register(email, password, name);
      if ("mfa_required" in response) {
        setPreAuthToken(response.pre_auth_token);
        setPreAuthExpiresAt(Date.now() + response.expires_in * 1_000);
        setPassword("");
        setMfaCode("");
        return;
      }
      setSession({
        accessToken: response.access_token,
        user: response.user,
        organizations: response.organizations,
        defaultOrganizationId: response.default_organization_id,
      });
      navigate(returnTo, { replace: true });
    } catch (caught) {
      if (preAuthToken && mfaProofKind === "recovery") {
        setMfaCode("");
      }
      setError(caught instanceof ApiError ? caught.message : t("auth.error.failed"));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <main className="auth-screen">
      <section className="auth-panel">
        <div className="auth-toolbar">
          <div className="auth-brand">
            <div className="brand-mark">Z</div>
            <div>
              <h1>ZinharCMS</h1>
              <p>{t("auth.workspace")}</p>
            </div>
          </div>
          <LanguageSelect compact />
        </div>

        <div className="segmented-control" role="tablist" aria-label={t("auth.mode.aria")}>
          <button className={mode === "login" ? "is-active" : ""} type="button" onClick={() => setMode("login")} disabled={Boolean(preAuthToken)}>
            <LogIn size={16} aria-hidden="true" />
            {t("auth.login")}
          </button>
          <button className={mode === "register" ? "is-active" : ""} type="button" onClick={() => setMode("register")} disabled={Boolean(preAuthToken)}>
            <UserPlus size={16} aria-hidden="true" />
            {t("auth.register")}
          </button>
        </div>

        <form className="form-grid" onSubmit={handleSubmit}>
          {preAuthToken ? (
            <>
              <p>Enter the code from your authenticator app or use one recovery code.</p>
              <label>
                Verification method
                <select
                  value={mfaProofKind}
                  onChange={(event) => {
                    setMfaProofKind(event.target.value as MfaProofKind);
                    setMfaCode("");
                  }}
                >
                  <option value="totp">Authenticator code</option>
                  <option value="recovery">Recovery code</option>
                </select>
              </label>
              <label>
                {mfaProofKind === "totp" ? "Six-digit code" : "Recovery code"}
                <input
                  autoComplete="one-time-code"
                  inputMode={mfaProofKind === "totp" ? "numeric" : "text"}
                  value={mfaCode}
                  onChange={(event) => setMfaCode(event.target.value)}
                  required
                />
              </label>
            </>
          ) : (
            <>
          {mode === "register" && (
            <label>
              {t("auth.name")}
              <input value={name} onChange={(event) => setName(event.target.value)} required />
            </label>
          )}
          <label>
            {t("auth.email")}
            <input type="email" value={email} onChange={(event) => setEmail(event.target.value)} required />
          </label>
          <label>
            {t("auth.password")}
            <input
              type="password"
              minLength={8}
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              required
            />
          </label>
            </>
          )}

          {error && <StatusBadge label={error} tone="danger" />}

          <button className="primary-button auth-submit" type="submit" disabled={submitting}>
            <LockKeyhole size={16} aria-hidden="true" />
            {submitting
              ? t("auth.submit.working")
              : preAuthToken
                ? "Verify and sign in"
                : mode === "login"
                  ? t("auth.submit.login")
                  : t("auth.submit.register")}
          </button>
          {preAuthToken && (
            <button
              className="secondary-button"
              type="button"
                onClick={() => {
                  setPreAuthToken(null);
                  setPreAuthExpiresAt(null);
                  setMfaCode("");
                  setError(null);
              }}
            >
              Start over
            </button>
          )}
        </form>
      </section>
    </main>
  );
}
