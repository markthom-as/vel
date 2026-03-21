import type { ActionItemData, NowData, NowTaskData, RoutineBlockData } from '../../types';

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
  return items.filter((item) => {
    const dedupeKey = [
      item.kind,
      item.title.trim().toLowerCase(),
      item.summary.trim().toLowerCase(),
      item.project_label?.trim().toLowerCase() ?? '',
      item.thread_route?.thread_id ?? '',
      item.thread_route?.label.trim().toLowerCase() ?? '',
    ].join('::');
    if (seen.has(dedupeKey)) {
      return false;
    }
    seen.add(dedupeKey);
    return true;
  });
}

export function findActiveEvent(events: NowData['schedule']['upcoming_events'], nowTs: number) {
  return (
    events.find((event) => {
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

export function scoreNudge(bar: { urgent: boolean; kind: string }): number {
  let score = 0;
  if (bar.urgent) {
    score += 10;
  }
  if (bar.kind === 'trust_warning') {
    score += 8;
  }
  if (bar.kind === 'needs_input') {
    score += 6;
  }
  return score;
}

export function formatNowBarKind(kind: string): string {
  return kind.replaceAll('_', ' ');
}

export function findBarProjectTags(bar: NowData['nudge_bars'][number], items: ActionItemData[]): string[] {
  const matching = items.filter((item) => item.id === bar.id || item.title === bar.title);
  const labels = matching.map((item) => item.project_label).filter((value): value is string => Boolean(value));
  return [...new Set(labels)];
}

export function projectTagTone(label: string): string {
  const tones = [
    'bg-sky-950/40 text-sky-200',
    'bg-emerald-950/40 text-emerald-200',
    'bg-fuchsia-950/40 text-fuchsia-200',
    'bg-cyan-950/40 text-cyan-200',
    'bg-indigo-950/40 text-indigo-200',
  ];
  const seed = Array.from(label).reduce((sum, char) => sum + char.charCodeAt(0), 0);
  return tones[seed % tones.length];
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
