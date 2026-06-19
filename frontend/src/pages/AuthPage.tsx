import { FormEvent, useState } from "react";
import { LockKeyhole, LogIn, UserPlus } from "lucide-react";
import { Navigate, useNavigate } from "react-router-dom";

import { StatusBadge } from "../components/StatusBadge";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";

type AuthMode = "login" | "register";

export function AuthPage() {
  const navigate = useNavigate();
  const accessToken = useAppStore((state) => state.accessToken);
  const setSession = useAppStore((state) => state.setSession);
  const [mode, setMode] = useState<AuthMode>("login");
  const [email, setEmail] = useState("admin@example.com");
  const [name, setName] = useState("Admin User");
  const [password, setPassword] = useState("password123");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (accessToken) return <Navigate to="/" replace />;

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);

    try {
      const response =
        mode === "login" ? await api.auth.login(email, password) : await api.auth.register(email, password, name);
      setSession({
        accessToken: response.access_token,
        refreshToken: response.refresh_token,
        user: response.user,
      });
      navigate("/", { replace: true });
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Authentication failed");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <main className="auth-screen">
      <section className="auth-panel">
        <div className="auth-brand">
          <div className="brand-mark">Z</div>
          <div>
            <h1>ZinharCMS</h1>
            <p>Admin workspace</p>
          </div>
        </div>

        <div className="segmented-control" role="tablist" aria-label="Authentication mode">
          <button className={mode === "login" ? "is-active" : ""} type="button" onClick={() => setMode("login")}>
            <LogIn size={16} aria-hidden="true" />
            Login
          </button>
          <button className={mode === "register" ? "is-active" : ""} type="button" onClick={() => setMode("register")}>
            <UserPlus size={16} aria-hidden="true" />
            Register
          </button>
        </div>

        <form className="form-grid" onSubmit={handleSubmit}>
          {mode === "register" && (
            <label>
              Name
              <input value={name} onChange={(event) => setName(event.target.value)} required />
            </label>
          )}
          <label>
            Email
            <input type="email" value={email} onChange={(event) => setEmail(event.target.value)} required />
          </label>
          <label>
            Password
            <input
              type="password"
              minLength={8}
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              required
            />
          </label>

          {error && <StatusBadge label={error} tone="danger" />}

          <button className="primary-button auth-submit" type="submit" disabled={submitting}>
            <LockKeyhole size={16} aria-hidden="true" />
            {submitting ? "Working..." : mode === "login" ? "Enter admin" : "Create first user"}
          </button>
        </form>
      </section>
    </main>
  );
}