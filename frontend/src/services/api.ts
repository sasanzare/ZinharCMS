import type { ApiInfo, HealthResponse, ReadyResponse } from "../types/api";

const API_BASE_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";

async function request<T>(path: string): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, {
    headers: { "Content-Type": "application/json" },
  });

  if (!response.ok) {
    throw new Error(`${response.status} ${response.statusText}`);
  }

  return response.json() as Promise<T>;
}

export const api = {
  baseUrl: API_BASE_URL,
  info: () => request<ApiInfo>("/"),
  health: () => request<HealthResponse>("/health"),
  readiness: () => request<ReadyResponse>("/ready"),
};
