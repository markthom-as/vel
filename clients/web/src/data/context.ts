import { canonicalPatchMutation, canonicalPostMutation, canonicalQuery } from './canonicalTransport';
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
  return canonicalQuery<CurrentContextData | null>(
    '/v1/context/current',
    (value) => decodeApiResponse(value, (data) => decodeNullable(data, decodeCurrentContextData)),
  );
}

export function loadNow(): Promise<ApiResponse<NowData>> {
  // Phase 05 decode path includes Now review_snapshot plus ranked action_items.
  return canonicalQuery<NowData>(
    '/v1/now',
    (value) => decodeApiResponse(value, decodeNowData),
  );
}

export function loadSyncBootstrap(): Promise<ApiResponse<SyncBootstrapData>> {
  // Phase 05 sync bootstrap carries linked_nodes, projects, and action_items for thin clients.
  return canonicalQuery<SyncBootstrapData>(
    '/v1/sync/bootstrap',
    (value) => decodeApiResponse(value, decodeSyncBootstrapData),
  );
}

export function loadContextExplain(): Promise<ApiResponse<ContextExplainData>> {
  return canonicalQuery<ContextExplainData>(
    '/v1/explain/context',
    (value) => decodeApiResponse(value, decodeContextExplainData),
  );
}

export function loadDriftExplain(): Promise<ApiResponse<DriftExplainData>> {
  return canonicalQuery<DriftExplainData>(
    '/v1/explain/drift',
    (value) => decodeApiResponse(value, decodeDriftExplainData),
  );
}

export function loadCommitments(limit: number): Promise<ApiResponse<CommitmentData[]>> {
  return canonicalQuery<CommitmentData[]>(
    `/v1/commitments?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeCommitmentData)),
  );
}

export function updateCommitment(
  commitmentId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<CommitmentData>> {
  return canonicalPatchMutation<CommitmentData>(
    `/v1/commitments/${encodeURIComponent(commitmentId.trim())}`,
    patch,
    (value) => decodeApiResponse(value, decodeCommitmentData),
  );
}

export function updateNowTaskLane(
  commitmentId: string,
  lane: 'active' | 'next_up' | 'if_time_allows' | 'completed',
  position?: number,
): Promise<ApiResponse<NowData>> {
  return canonicalPatchMutation<NowData>(
    '/v1/now/task-lane',
    {
      commitment_id: commitmentId,
      lane,
      position: position ?? null,
    },
    (value) => decodeApiResponse(value, decodeNowData),
  );
}

export function rescheduleNowTasksToToday(
  commitmentIds: string[],
): Promise<ApiResponse<NowData>> {
  return canonicalPostMutation<NowData>(
    '/v1/now/tasks/reschedule-today',
    {
      commitment_ids: commitmentIds,
    },
    (value) => decodeApiResponse(value, decodeNowData),
  );
}

export function rescheduleNowCalendarEvent(
  payload: {
    event_id: string;
    calendar_id?: string | null;
    start_ts: number;
    end_ts?: number | null;
  },
): Promise<ApiResponse<NowData>> {
  return canonicalPostMutation<NowData>(
    '/v1/now/calendar-events/reschedule',
    {
      event_id: payload.event_id,
      calendar_id: payload.calendar_id ?? null,
      start_ts: payload.start_ts,
      end_ts: payload.end_ts ?? null,
    },
    (value) => decodeApiResponse(value, decodeNowData),
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
  return canonicalQuery<DailyLoopSessionData | null>(
    `/v1/daily-loop/sessions/active?${params.toString()}`,
    (value) => decodeApiResponse(value, (data) => decodeNullable(data, decodeDailyLoopSessionData)),
  );
}

export function startDailyLoopSession(
  request: DailyLoopStartRequestData,
): Promise<ApiResponse<DailyLoopSessionData>> {
  return canonicalPostMutation<DailyLoopSessionData>(
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
  return canonicalPostMutation<DailyLoopSessionData>(
    `/v1/daily-loop/sessions/${encodeURIComponent(sessionId.trim())}/turn`,
    {
      session_id: sessionId,
      action,
      response_text: responseText ?? null,
    },
    (value) => decodeApiResponse(value, decodeDailyLoopSessionData),
  );
}
