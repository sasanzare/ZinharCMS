import { FormEvent, useEffect, useState } from "react";
import { ShieldCheck, X } from "lucide-react";

import { StatusBadge } from "./StatusBadge";
import { ApiError, api } from "../services/api";
import type { MfaProofKind, StepUpScope } from "../types/api";

type StepUpDialogProps = {
  open: boolean;
  scope: StepUpScope;
  title: string;
  onGranted: (stepUpToken: string) => Promise<void>;
  onCancel: () => void;
};

export function StepUpDialog({
  open,
  scope,
  title,
  onGranted,
  onCancel,
}: StepUpDialogProps) {
  const [challenge, setChallenge] = useState<string | null>(null);
  const [proofKind, setProofKind] = useState<MfaProofKind>("totp");
  const [code, setCode] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [working, setWorking] = useState(false);

  useEffect(() => {
    if (!open) return;
    let active = true;
    setChallenge(null);
    setCode("");
    setError(null);
    setWorking(true);
    void api.auth
      .createStepUp(scope)
      .then((response) => {
        if (active) setChallenge(response.challenge);
      })
      .catch((caught) => {
        if (active) {
          setError(
            caught instanceof ApiError
              ? caught.message
              : "Unable to start step-up verification.",
          );
        }
      })
      .finally(() => {
        if (active) setWorking(false);
      });
    return () => {
      active = false;
    };
  }, [open, scope]);

  if (!open) return null;

  async function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!challenge || working) return;
    setWorking(true);
    setError(null);
    try {
      const grant = await api.auth.verifyStepUp(challenge, proofKind, code);
      await onGranted(grant.step_up_token);
      setCode("");
    } catch (caught) {
      setError(
        caught instanceof ApiError
          ? caught.message
          : "Step-up verification failed.",
      );
    } finally {
      setWorking(false);
    }
  }

  return (
    <div className="marketplace-dialog-backdrop" role="presentation">
      <section
        className="marketplace-install-dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby="step-up-title"
      >
        <div className="panel-header">
          <div>
            <h2 id="step-up-title">{title}</h2>
            <span>Verify this sensitive action with MFA.</span>
          </div>
          <button
            className="icon-button"
            type="button"
            aria-label="Cancel step-up verification"
            onClick={onCancel}
            disabled={working}
          >
            <X size={16} aria-hidden="true" />
          </button>
        </div>
        <form className="form-grid padded" onSubmit={submit}>
          <label>
            Verification method
            <select
              value={proofKind}
              onChange={(event) =>
                setProofKind(event.target.value as MfaProofKind)
              }
            >
              <option value="totp">Authenticator code</option>
              <option value="recovery">Recovery code</option>
            </select>
          </label>
          <label>
            {proofKind === "totp" ? "Six-digit code" : "Recovery code"}
            <input
              autoFocus
              autoComplete="one-time-code"
              inputMode={proofKind === "totp" ? "numeric" : "text"}
              value={code}
              onChange={(event) => setCode(event.target.value)}
              required
            />
          </label>
          {error && <StatusBadge label={error} tone="danger" />}
          <button
            className="primary-button"
            type="submit"
            disabled={working || !challenge}
          >
            <ShieldCheck size={16} aria-hidden="true" />
            {working ? "Verifying..." : "Verify and continue"}
          </button>
        </form>
      </section>
    </div>
  );
}
