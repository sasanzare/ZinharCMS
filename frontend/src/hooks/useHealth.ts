import { useEffect, useState } from "react";

import { api } from "../services/api";
import type { HealthResponse, ReadyResponse } from "../types/api";

type HealthState = {
  health: HealthResponse | null;
  readiness: ReadyResponse | null;
  loading: boolean;
  error: string | null;
};

export function useHealth(pollMs = 15_000): HealthState {
  const [state, setState] = useState<HealthState>({
    health: null,
    readiness: null,
    loading: true,
    error: null,
  });

  useEffect(() => {
    let active = true;

    async function load() {
      try {
        const [health, readiness] = await Promise.all([api.health(), api.readiness()]);
        if (!active) return;
        setState({ health, readiness, loading: false, error: null });
      } catch (error) {
        if (!active) return;
        setState((previous) => ({
          ...previous,
          loading: false,
          error: error instanceof Error ? error.message : "Unknown error",
        }));
      }
    }

    void load();
    const interval = window.setInterval(load, pollMs);

    return () => {
      active = false;
      window.clearInterval(interval);
    };
  }, [pollMs]);

  return state;
}
