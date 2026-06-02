export type HealthResponse = {
  status: string;
  version: string;
};

export type DependencyCheck = {
  name: string;
  ok: boolean;
  message: string;
};

export type ReadyResponse = {
  status: string;
  checks: DependencyCheck[];
};

export type ApiInfo = {
  name: string;
  version: string;
  docs: string;
  health: string;
};
