import { useEffect } from "react";
import { Navigate, useNavigate, useParams } from "react-router-dom";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n } from "../i18n";
import { api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";

export function WorkspaceRedirectPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { slug } = useParams();
  const organizations = useAppStore((state) => state.organizations);
  const setActiveOrganization = useAppStore((state) => state.setActiveOrganization);
  const setOrganizations = useAppStore((state) => state.setOrganizations);

  useEffect(() => {
    if (!slug) return;
    let cancelled = false;

    async function selectWorkspace() {
      const localMatch = organizations.find((organization) => organization.slug === slug);
      if (localMatch) {
        setActiveOrganization(localMatch.id);
        navigate("/organization", { replace: true });
        return;
      }

      const nextOrganizations = await api.organizations.list();
      if (cancelled) return;
      const remoteMatch = nextOrganizations.find((organization) => organization.slug === slug);
      setOrganizations(nextOrganizations, remoteMatch?.id ?? null);
      navigate("/organization", { replace: true });
    }

    void selectWorkspace();

    return () => {
      cancelled = true;
    };
  }, [navigate, organizations, setActiveOrganization, setOrganizations, slug]);

  if (!slug) return <Navigate to="/organization" replace />;

  return (
    <div className="empty-state padded">
      <StatusBadge label={t("common.loading")} tone="neutral" />
    </div>
  );
}
