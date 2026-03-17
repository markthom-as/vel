import { apiGet, apiPatch, apiPost } from '../api/client';
import {
  decodeApiResponse,
  decodeArray,
  decodeCommitmentData,
  decodeContextExplainData,
  decodeCurrentContextData,
  decodeDriftExplainData,
  decodeNowData,
  decodeNullable,
  decodeSuggestionData,
  decodeUncertaintyData,
  type ApiResponse,
  type CommitmentData,
  type ContextExplainData,
  type CurrentContextData,
  type DriftExplainData,
  type NowData,
  type SuggestionData,
  type UncertaintyData,
} from '../types';

export const contextQueryKeys = {
  suggestions: (state: string) => ['suggestions', state] as const,
  suggestion: (suggestionId: string | null) => ['suggestions', suggestionId] as const,
  uncertainty: (status: string) => ['uncertainty', status] as const,
  now: () => ['now'] as const,
  currentContext: () => ['context', 'current'] as const,
  contextExplain: () => ['context', 'explain'] as const,
  driftExplain: () => ['context', 'drift-explain'] as const,
  commitments: (limit: number) => ['commitments', limit] as const,
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
  return apiGet<ApiResponse<NowData>>(
    '/v1/now',
    (value) => decodeApiResponse(value, decodeNowData),
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
