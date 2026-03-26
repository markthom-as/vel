import type {
  ActionItemData,
  ClusterBootstrapData,
  NowData,
  NowNudgeActionData,
  NowTaskData,
  RoutineBlockData,
  WorkerPresenceData,
} from '../../types';
import {
  actionItemDedupeKeysValue,
  shortClientKindLabelValue,
} from '../../data/embeddedBridgeAdapter';
import {
  systemTargetForCoreSetting,
  systemTargetForProvider,
  type SystemNavigationTarget,
} from '../system';

function coreSetupChecklistTarget(actionKind: string): SystemNavigationTarget | null {
  const parts = actionKind.split(':');
  if (parts[0] !== 'open_settings' || parts[1] !== 'core_settings') {
    return null;
  }
  const item = parts[2] ?? null;
  switch (item) {
    case 'user_display_name':
      return systemTargetForCoreSetting('user_display_name');
    case 'node_display_name':
      return systemTargetForCoreSetting('node_display_name');
    case 'agent_profile':
      return systemTargetForCoreSetting('agent_profile');
    case 'llm_provider':
      return systemTargetForProvider('llm_routing');
    case 'synced_provider':
      return systemTargetForProvider('google_calendar');
    default:
      return systemTargetForCoreSetting('required_setup');
  }
}

/** Maps legacy `open_settings` nudges onto the canonical `/system` surface. */
export function nudgeOpenSystemTarget(
  bar: { id: string },
  action?: Pick<NowNudgeActionData, 'kind'> | null,
): SystemNavigationTarget {
  const actionKind = action?.kind ?? 'open_settings';
  const directTarget = coreSetupChecklistTarget(actionKind);
  if (directTarget) {
    return directTarget;
  }
  if (bar.id === 'core_setup_required') {
    return coreSetupChecklistTarget(actionKind)
      ?? systemTargetForCoreSetting('required_setup');
  }
  if (bar.id === 'backup_trust_warning') {
    return systemTargetForProvider('recovery');
  }
  if (bar.id === 'mesh_summary_warning') {
    return systemTargetForProvider('accounts');
  }
  return systemTargetForCoreSetting('required_setup');
}

export function dedupeTasks(tasks: Array<NowTaskData | null | undefined>): NowTaskData[] {
  const seen = new Set<string>();
  return tasks.filter((task): task is NowTaskData => {
    if (!task || seen.has(task.id)) {
      return false;
    }
    seen.add(task.id);
    return true;
  });
}

export function dedupeActionItems(items: ActionItemData[]): ActionItemData[] {
  const seen = new Set<string>();
  const dedupeKeys = actionItemDedupeKeysValue(
    items.map((item) => ({
      kind: item.kind,
      title: item.title,
      summary: item.summary,
      projectLabel: item.project_label ?? null,
      threadId: item.thread_route?.thread_id ?? null,
      threadLabel: item.thread_route?.label ?? null,
    })),
  );
  return items.filter((item, index) => {
    const dedupeKey = dedupeKeys[index] ?? item.id;
    if (seen.has(dedupeKey)) {
      return false;
    }
    seen.add(dedupeKey);
    return true;
  });
}

export function findActiveEvent(events: NowData['schedule']['upcoming_events'], nowTs: number) {
  const activeEvents = events.filter((event) => {
    const endTs = event.end_ts ?? event.start_ts;
    return event.start_ts <= nowTs && endTs >= nowTs;
  });
  return (
    activeEvents.find((event) => !event.all_day)
    ?? activeEvents.find((event) => {
      const endTs = event.end_ts ?? event.start_ts;
      return event.start_ts <= nowTs && endTs >= nowTs;
    }) ?? null
  );
}

export function findNextEvent(events: NowData['schedule']['upcoming_events'], nowTs: number) {
  return events.find((event) => event.start_ts > nowTs) ?? null;
}

export function findActiveRoutineBlock(dayPlan: NowData['day_plan'], nowTs: number) {
  return dayPlan?.routine_blocks.find((block) => block.start_ts <= nowTs && block.end_ts >= nowTs) ?? null;
}

function sourceSummaryLines(summary: unknown, keys: string[]): string[] {
  if (!summary || typeof summary !== 'object') {
    return [];
  }
  const record = summary as Record<string, unknown>;
  return keys
    .map((key) => {
      const value = record[key];
      if (typeof value === 'string' && value.length > 0) {
        return `${key.replaceAll('_', ' ')}: ${value}`;
      }
      if (typeof value === 'number' || typeof value === 'boolean') {
        return `${key.replaceAll('_', ' ')}: ${value}`;
      }
      return null;
    })
    .filter((value): value is string => value !== null);
}

function deriveInferredActivity(data: NowData): { title: string; detail: string } | null {
  if (data.sources.git_activity) {
    return {
      title: 'Likely working from recent activity',
      detail: sourceSummaryLines(data.sources.git_activity.summary, ['repo', 'operation'])[0] ?? 'Git activity is the strongest recent signal.',
    };
  }
  if (data.sources.note_document) {
    return {
      title: 'Likely in note work',
      detail: sourceSummaryLines(data.sources.note_document.summary, ['title', 'path'])[0] ?? 'A recent note is the strongest signal.',
    };
  }
  return null;
}

export function formatTimeUntil(targetTs: number, nowTs: number): string {
  const diffSeconds = targetTs - nowTs;
  if (diffSeconds <= 0) return 'now';
  const diffMinutes = Math.floor(diffSeconds / 60);
  if (diffMinutes < 60) return `in ${diffMinutes}m`;
  const hours = Math.floor(diffMinutes / 60);
  const mins = diffMinutes % 60;
  if (hours < 24) return mins > 0 ? `in ${hours}h ${mins}m` : `in ${hours}h`;
  return `in ${Math.floor(hours / 24)}d`;
}

export function formatTime(timestamp: number, timezone: string): string {
  return new Date(timestamp * 1000).toLocaleTimeString(undefined, {
    timeZone: timezone,
    hour: 'numeric',
    minute: '2-digit',
  });
}

export function formatSessionDate(timestamp: number, timezone: string): string {
  return new Intl.DateTimeFormat('en-CA', {
    timeZone: timezone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  }).format(new Date(timestamp * 1000));
}

export function formatRelativeMinutes(unixSeconds: number): string {
  const deltaMinutes = Math.max(1, Math.floor((Date.now() / 1000 - unixSeconds) / 60));
  return `${deltaMinutes} min ago`;
}

export function formatTaskDate(value: string): string {
  try {
    return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(new Date(value));
  } catch {
    return value;
  }
}

export function formatNowBarKind(kind: string): string {
  if (kind === 'trust_warning') {
    return 'VEL CONFIG';
  }
  return kind.replaceAll('_', ' ');
}

export function findBarProjectTags(bar: NowData['nudge_bars'][number], items: ActionItemData[]): string[] {
  const matching = items.filter((item) => item.id === bar.id || item.title === bar.title);
  const labels = matching.map((item) => item.project_label).filter((value): value is string => Boolean(value));
  return [...new Set(labels)];
}

/** Same primary line the nav previously showed: active task, else context line, else a placeholder. */
export function nowNavContextSummary(data: NowData): string {
  return data.task_lane?.active?.text ?? data.context_line?.text ?? 'No active task';
}

export function nowLocationLabel(
  data: NowData,
  activeEvent: NowData['schedule']['upcoming_events'][number] | null,
): string {
  const loc = activeEvent?.location?.trim();
  if (loc) {
    return loc;
  }
  try {
    const parts = new Intl.DateTimeFormat('en-US', {
      timeZone: data.timezone,
      timeZoneName: 'long',
    }).formatToParts(new Date(data.computed_at * 1000));
    return parts.find((part) => part.type === 'timeZoneName')?.value ?? data.timezone;
  } catch {
    return data.timezone;
  }
}

export function shortClientKindLabel(clientKind: string | null | undefined): string | null {
  return shortClientKindLabelValue(clientKind ?? null).shortLabel;
}

export function formatNowClientCaption(
  bootstrap: ClusterBootstrapData | null | undefined,
  localWorker: WorkerPresenceData | null | undefined,
): string {
  const name = bootstrap?.node_display_name?.trim() || 'Unknown host';
  const kind = shortClientKindLabel(localWorker?.client_kind);
  return kind ? `${name} · ${kind}` : name;
}

export function buildCurrentStatus(
  data: NowData,
  activeEvent: NowData['schedule']['upcoming_events'][number] | null,
  activeRoutineBlock: RoutineBlockData | null,
  currentCommitment: NowTaskData | null,
  nextEvent: NowData['schedule']['upcoming_events'][number] | null,
) {
  if (activeEvent) {
    return {
      kind: 'Calendar',
      title: activeEvent.title,
      detail: activeEvent.location ?? formatTime(activeEvent.start_ts, data.timezone),
      subtitle: 'What is happening now takes precedence over everything else.',
      summary: `You are in ${activeEvent.title}${activeEvent.location ? ` at ${activeEvent.location}` : ''}.`,
      fallbackEventMessage: null,
    };
  }
  if (currentCommitment) {
    return {
      kind: 'Commitment',
      title: currentCommitment.text,
      detail: currentCommitment.project ?? 'No project',
      subtitle: 'No calendar event is active, so the current commitment becomes the execution anchor.',
      summary: `Current commitment: ${currentCommitment.text}.`,
      fallbackEventMessage: null,
    };
  }
  if (activeRoutineBlock) {
    return {
      kind: 'Routine',
      title: activeRoutineBlock.label,
      detail: activeRoutineBlock.source.replaceAll('_', ' '),
      subtitle: 'Routine stays visible when it is active, but it does not replace calendar truth.',
      summary: `Routine block in progress: ${activeRoutineBlock.label}.`,
      fallbackEventMessage: null,
    };
  }
  const inferred = deriveInferredActivity(data);
  if (inferred) {
    return {
      kind: 'Inference',
      title: inferred.title,
      detail: inferred.detail,
      subtitle: 'Inference only shows when no stronger declared structure is active.',
      summary: inferred.detail,
      fallbackEventMessage: null,
    };
  }
  if (nextEvent) {
    return {
      kind: 'Free',
      title: `Free until ${formatTime(nextEvent.start_ts, data.timezone)}`,
      detail: nextEvent.title,
      subtitle: 'Nothing explicit is active right now, so the next event sets the edge of free time.',
      summary: `Free until ${nextEvent.title} at ${formatTime(nextEvent.start_ts, data.timezone)}.`,
      fallbackEventMessage: `Free until ${nextEvent.title} at ${formatTime(nextEvent.start_ts, data.timezone)}.`,
    };
  }
  return {
    kind: 'Between blocks',
    title: 'Between blocks',
    detail: 'No event, commitment, or strong routine signal is active.',
    subtitle: 'When Vel has no stronger current-day structure, it should say so plainly.',
    summary: 'Nothing stronger is active right now.',
    fallbackEventMessage: 'No more calendar events are scheduled right now.',
  };
}
