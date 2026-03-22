import { apiGet, apiPatch, apiPost } from '../api/client';
import {
  decodeApiResponse,
  decodeArray,
  decodeCommitmentData,
  decodeContextExplainData,
  decodeCurrentContextData,
  decodeDailyLoopSessionData,
  decodeDriftExplainData,
  decodeNowData,
  decodeNullable,
  decodeSyncBootstrapData,
  type ApiResponse,
  type CommitmentData,
  type ContextExplainData,
  type CurrentContextData,
  type DailyLoopPhaseData,
  type DailyLoopSessionData,
  type DailyLoopStartRequestData,
  type DailyLoopTurnActionData,
  type DriftExplainData,
  type NowData,
  type SyncBootstrapData,
} from '../types';

export const contextQueryKeys = {
  now: () => ['now'] as const,
  currentContext: () => ['context', 'current'] as const,
  syncBootstrap: () => ['sync', 'bootstrap'] as const,
  contextExplain: () => ['context', 'explain'] as const,
  driftExplain: () => ['context', 'drift-explain'] as const,
  commitments: (limit: number) => ['commitments', limit] as const,
  dailyLoopActive: (sessionDate: string, phase: DailyLoopPhaseData) =>
    ['daily-loop', 'active', sessionDate, phase] as const,
};

export function loadCurrentContext(): Promise<ApiResponse<CurrentContextData | null>> {
  return apiGet<ApiResponse<CurrentContextData | null>>(
    '/v1/context/current',
    (value) => decodeApiResponse(value, (data) => decodeNullable(data, decodeCurrentContextData)),
  );
}

export function loadNow(): Promise<ApiResponse<NowData>> {
  // Phase 05 decode path includes Now review_snapshot plus ranked action_items.
  return apiGet<ApiResponse<NowData>>(
    '/v1/now',
    (value) => decodeApiResponse(value, decodeNowData),
  );
}

export function loadSyncBootstrap(): Promise<ApiResponse<SyncBootstrapData>> {
  // Phase 05 sync bootstrap carries linked_nodes, projects, and action_items for thin clients.
  return apiGet<ApiResponse<SyncBootstrapData>>(
    '/v1/sync/bootstrap',
    (value) => decodeApiResponse(value, decodeSyncBootstrapData),
  );
}

export function loadContextExplain(): Promise<ApiResponse<ContextExplainData>> {
  return apiGet<ApiResponse<ContextExplainData>>(
    '/v1/explain/context',
    (value) => decodeApiResponse(value, decodeContextExplainData),
  );
}

export function loadDriftExplain(): Promise<ApiResponse<DriftExplainData>> {
  return apiGet<ApiResponse<DriftExplainData>>(
    '/v1/explain/drift',
    (value) => decodeApiResponse(value, decodeDriftExplainData),
  );
}

export function loadCommitments(limit: number): Promise<ApiResponse<CommitmentData[]>> {
  return apiGet<ApiResponse<CommitmentData[]>>(
    `/v1/commitments?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeCommitmentData)),
  );
}

export function updateCommitment(
  commitmentId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<CommitmentData>> {
  return apiPatch<ApiResponse<CommitmentData>>(
    `/v1/commitments/${encodeURIComponent(commitmentId.trim())}`,
    patch,
    (value) => decodeApiResponse(value, decodeCommitmentData),
  );
}

export function loadActiveDailyLoopSession(
  sessionDate: string,
  phase: DailyLoopPhaseData,
): Promise<ApiResponse<DailyLoopSessionData | null>> {
  const params = new URLSearchParams({
    session_date: sessionDate,
    phase,
  });
  return apiGet<ApiResponse<DailyLoopSessionData | null>>(
    `/v1/daily-loop/sessions/active?${params.toString()}`,
    (value) => decodeApiResponse(value, (data) => decodeNullable(data, decodeDailyLoopSessionData)),
  );
}

export function startDailyLoopSession(
  request: DailyLoopStartRequestData,
): Promise<ApiResponse<DailyLoopSessionData>> {
  return apiPost<ApiResponse<DailyLoopSessionData>>(
    '/v1/daily-loop/sessions',
    request,
    (value) => decodeApiResponse(value, decodeDailyLoopSessionData),
  );
}

export function submitDailyLoopTurn(
  sessionId: string,
  action: DailyLoopTurnActionData,
  responseText?: string | null,
): Promise<ApiResponse<DailyLoopSessionData>> {
  return apiPost<ApiResponse<DailyLoopSessionData>>(
    `/v1/daily-loop/sessions/${encodeURIComponent(sessionId.trim())}/turn`,
    {
      session_id: sessionId,
      action,
      response_text: responseText ?? null,
    },
    (value) => decodeApiResponse(value, decodeDailyLoopSessionData),
  );
}
