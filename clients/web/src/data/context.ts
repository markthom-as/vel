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
  decodeSuggestionData,
  decodeUncertaintyData,
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
  type SuggestionData,
  type UncertaintyData,
} from '../types';

export const contextQueryKeys = {
  suggestions: (state: string) => ['suggestions', state] as const,
  suggestion: (suggestionId: string | null) => ['suggestions', suggestionId] as const,
  uncertainty: (status: string) => ['uncertainty', status] as const,
  now: () => ['now'] as const,
  currentContext: () => ['context', 'current'] as const,
  syncBootstrap: () => ['sync', 'bootstrap'] as const,
  contextExplain: () => ['context', 'explain'] as const,
  driftExplain: () => ['context', 'drift-explain'] as const,
  commitments: (limit: number) => ['commitments', limit] as const,
  dailyLoopActive: (sessionDate: string, phase: DailyLoopPhaseData) =>
    ['daily-loop', 'active', sessionDate, phase] as const,
};

export function loadSuggestions(state = 'pending'): Promise<ApiResponse<SuggestionData[]>> {
  return apiGet<ApiResponse<SuggestionData[]>>(
    `/v1/suggestions?state=${encodeURIComponent(state)}&limit=50`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeSuggestionData)),
  );
}

export function loadSuggestion(suggestionId: string): Promise<ApiResponse<SuggestionData>> {
  return apiGet<ApiResponse<SuggestionData>>(
    `/v1/suggestions/${encodeURIComponent(suggestionId.trim())}`,
    (value) => decodeApiResponse(value, decodeSuggestionData),
  );
}

export function updateSuggestion(
  suggestionId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<SuggestionData>> {
  return apiPatch<ApiResponse<SuggestionData>>(
    `/v1/suggestions/${encodeURIComponent(suggestionId.trim())}`,
    patch,
    (value) => decodeApiResponse(value, decodeSuggestionData),
  );
}

export function loadUncertainty(status = 'open'): Promise<ApiResponse<UncertaintyData[]>> {
  return apiGet<ApiResponse<UncertaintyData[]>>(
    `/v1/uncertainty?status=${encodeURIComponent(status)}&limit=50`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeUncertaintyData)),
  );
}

export function resolveUncertainty(uncertaintyId: string): Promise<ApiResponse<UncertaintyData>> {
  return apiPost<ApiResponse<UncertaintyData>>(
    `/v1/uncertainty/${encodeURIComponent(uncertaintyId.trim())}/resolve`,
    {},
    (value) => decodeApiResponse(value, decodeUncertaintyData),
  );
}

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
