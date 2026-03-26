import { canonicalPatchMutation, canonicalPostMutation, canonicalQuery } from './canonicalTransport';
import { normalizeTaskDisplayBatchValue } from './embeddedBridgeAdapter';
import { normalizeTaskDisplayBatchInWorker } from './embeddedBridgeWorker';
import {
  decodeApiResponse,
  decodeArray,
  decodeCommitmentData,
  decodeContextExplainData,
  decodeCurrentReflowActionResponseData,
  decodeCurrentContextData,
  decodeDailyLoopSessionData,
  decodeDriftExplainData,
  decodeNowData,
  decodeNullable,
  decodeDailyLoopCheckInSkipResponseData,
  decodeDailyLoopCheckInEventData,
  decodeSyncBootstrapData,
  type ApiResponse,
  type CommitmentData,
  type ContextExplainData,
  type CurrentReflowActionResponseData,
  type CurrentContextData,
  type DailyLoopPhaseData,
  type DailyLoopCheckInSkipRequestData,
  type DailyLoopCheckInSkipResponseData,
  type DailyLoopCheckInEventData,
  type DailyLoopCheckInEventsQueryData,
  type DailyLoopSessionData,
  type DailyLoopStartRequestData,
  type DailyLoopTurnActionData,
  type DriftExplainData,
  type NowData,
  type NowTaskData,
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

async function normalizeNowTasks(tasks: NowTaskData[]): Promise<NowTaskData[]> {
  const entries = tasks.map((task) => ({
    tags: task.tags ?? null,
    project: task.project ?? null,
  }));
  const normalizedDisplay = await normalizeTaskDisplayBatchInWorker(entries).catch(
    () => normalizeTaskDisplayBatchValue(entries),
  );

  return tasks.map((task, index) => {
    const display = normalizedDisplay[index] ?? { tags: task.tags ?? [], project: task.project ?? null };
    return {
      ...task,
      tags: display.tags,
      project: display.project,
    };
  });
}

async function normalizeNowDataPayload(data: NowData): Promise<NowData> {
  const taskLane = data.task_lane;
  if (!taskLane) {
    return data;
  }

  const [
    normalizedActiveList,
    normalizedActiveItems,
    normalizedNextUp,
    normalizedPending,
    normalizedInbox,
    normalizedLater,
    normalizedCompleted,
    normalizedRecentCompleted,
    normalizedNextUpTasks,
  ] = await Promise.all([
    taskLane.active ? normalizeNowTasks([taskLane.active]) : Promise.resolve([]),
    normalizeNowTasks(taskLane.active_items ?? []),
    normalizeNowTasks(taskLane.next_up ?? []),
    normalizeNowTasks(taskLane.pending ?? []),
    normalizeNowTasks(taskLane.inbox ?? []),
    normalizeNowTasks(taskLane.if_time_allows ?? []),
    normalizeNowTasks(taskLane.completed ?? []),
    normalizeNowTasks(taskLane.recent_completed ?? []),
    normalizeNowTasks(
      (data.next_up_items ?? [])
        .map((item) => item.task)
        .filter((task): task is NowTaskData => task != null),
    ),
  ]);

  let nextUpTaskIndex = 0;
  const normalizedNextUpItems = (data.next_up_items ?? []).map((item) => ({
    ...item,
    task: item.task
      ? (normalizedNextUpTasks[nextUpTaskIndex++] ?? item.task)
      : item.task,
  }));

  return {
    ...data,
    task_lane: {
      ...taskLane,
      active: normalizedActiveList[0] ?? taskLane.active,
      active_items: normalizedActiveItems,
      next_up: normalizedNextUp,
      pending: normalizedPending,
      inbox: normalizedInbox,
      if_time_allows: normalizedLater,
      completed: normalizedCompleted,
      recent_completed: normalizedRecentCompleted,
    },
    next_up_items: normalizedNextUpItems,
  };
}

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
  ).then(async (response) => {
    if (!response.ok || response.data == null) {
      return response;
    }
    return {
      ...response,
      data: await normalizeNowDataPayload(response.data),
    };
  });
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

export function applyCurrentReflow(): Promise<ApiResponse<CurrentReflowActionResponseData>> {
  return canonicalPostMutation<CurrentReflowActionResponseData>(
    '/v1/now/reflow/apply',
    {},
    (value) => decodeApiResponse(value, decodeCurrentReflowActionResponseData),
  );
}

export function editCurrentReflow(): Promise<ApiResponse<CurrentReflowActionResponseData>> {
  return canonicalPostMutation<CurrentReflowActionResponseData>(
    '/v1/now/reflow/edit',
    {},
    (value) => decodeApiResponse(value, decodeCurrentReflowActionResponseData),
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

export function listSessionDailyLoopCheckIns(
  sessionId: string,
  query?: DailyLoopCheckInEventsQueryData,
): Promise<ApiResponse<DailyLoopCheckInEventData[]>> {
  const params = new URLSearchParams();
  if (query?.check_in_type) {
    params.append('check_in_type', query.check_in_type);
  }
  if (query?.session_phase) {
    params.append('session_phase', query.session_phase);
  }
  if (query?.include_skipped) {
    params.append('include_skipped', 'true');
  }
  if (query?.limit !== undefined) {
    params.append('limit', query.limit.toString());
  }
  const qs = params.toString();
  return canonicalQuery<DailyLoopCheckInEventData[]>(
    `/v1/daily-loop/sessions/${encodeURIComponent(sessionId.trim())}/check-ins${
      qs ? `?${qs}` : ''
    }`,
    (value) =>
      decodeApiResponse(value, (data) => decodeArray(data, decodeDailyLoopCheckInEventData)),
  );
}

export function skipDailyLoopCheckIn(
  checkInEventId: string,
  request: DailyLoopCheckInSkipRequestData,
): Promise<ApiResponse<DailyLoopCheckInSkipResponseData>> {
  return canonicalPostMutation<DailyLoopCheckInSkipResponseData>(
    `/v1/daily-loop/check-ins/${encodeURIComponent(checkInEventId.trim())}/skip`,
    request,
    (value) => decodeApiResponse(value, decodeDailyLoopCheckInSkipResponseData),
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
