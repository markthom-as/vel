import {
  calendarFilterChipAppearance,
  nudgeKindTagAppearance,
  projectTagAppearance,
  type SystemStatusTone,
} from './semanticAppearance';
import { resolveSemanticAliasOverride } from './semanticAliases';

export type SemanticFamily = 'project' | 'calendar' | 'nudge' | 'alert' | 'mode' | 'provider';
export type SemanticIconKey =
  | 'calendar'
  | 'folder'
  | 'layers'
  | 'open_thread'
  | 'server'
  | 'settings'
  | 'spark'
  | 'sync'
  | 'tag'
  | 'threads'
  | 'warning';

export type SemanticEntry = {
  icon: SemanticIconKey;
  label: string;
};

function normalizeSemanticKey(value: string): string {
  return value.trim().toLowerCase().replace(/\s+/g, '_');
}

function resolveSemanticAlias(family: SemanticFamily, key: string, fallback: string): string {
  return resolveSemanticAliasOverride(family, key) ?? fallback;
}

const alertSemanticEntries: Record<string, SemanticEntry> = {
  default: { icon: 'spark', label: 'Alert' },
  urgent: { icon: 'warning', label: 'Urgent' },
  warning: { icon: 'warning', label: 'Warning' },
};

const modeSemanticEntries: Record<string, SemanticEntry> = {
  default: { icon: 'layers', label: 'Mode' },
  state: { icon: 'layers', label: 'State' },
  why: { icon: 'spark', label: 'Why' },
  debug: { icon: 'server', label: 'Debug' },
  focus: { icon: 'layers', label: 'Focus' },
  review: { icon: 'layers', label: 'Review' },
  degraded: { icon: 'server', label: 'Degraded' },
};

const nudgeSemanticEntries: Record<string, SemanticEntry> = {
  default: { icon: 'spark', label: 'Nudge' },
  system: { icon: 'settings', label: 'System' },
  system_settings: { icon: 'settings', label: 'System' },
  warning: { icon: 'warning', label: 'Warning' },
  trust_warning: { icon: 'warning', label: 'Trust warning' },
  freshness_warning: { icon: 'warning', label: 'Freshness warning' },
  review_request: { icon: 'open_thread', label: 'Review request' },
  reflow_proposal: { icon: 'spark', label: 'Reflow proposal' },
  needs_input: { icon: 'threads', label: 'Needs input' },
  thread_continuation: { icon: 'threads', label: 'Thread' },
  nudge: { icon: 'spark', label: 'Nudge' },
};

const providerSemanticEntries: Record<string, SemanticEntry & { glyphClassName: string }> = {
  default: { icon: 'server', label: 'Provider', glyphClassName: 'bg-zinc-700 text-zinc-200' },
  google_calendar: { icon: 'calendar', label: 'Google Calendar', glyphClassName: 'bg-[#b96e3a] text-[#ffd7bf]' },
  todoist: { icon: 'tag', label: 'Todoist', glyphClassName: 'bg-[#8d4a35] text-[#ffd8c9]' },
  github: { icon: 'folder', label: 'GitHub', glyphClassName: 'bg-[#535d72] text-[#e0e8ff]' },
  git: { icon: 'folder', label: 'Git', glyphClassName: 'bg-[#73553a] text-[#f7d0af]' },
  activity: { icon: 'warning', label: 'Activity', glyphClassName: 'bg-zinc-700 text-zinc-200' },
  health: { icon: 'warning', label: 'Health', glyphClassName: 'bg-zinc-700 text-zinc-200' },
  email: { icon: 'threads', label: 'Email', glyphClassName: 'bg-[#5a5368] text-[#f2e6ff]' },
  messaging: { icon: 'threads', label: 'Messaging', glyphClassName: 'bg-[#5b556f] text-[#e7deff]' },
  reminders: { icon: 'tag', label: 'Reminders', glyphClassName: 'bg-[#5e643d] text-[#eef6bd]' },
  notes: { icon: 'folder', label: 'Notes', glyphClassName: 'bg-[#64513d] text-[#ffe0bf]' },
  transcripts: { icon: 'open_thread', label: 'Transcripts', glyphClassName: 'bg-[#4a5b69] text-[#d7ebff]' },
};

export function resolveProjectSemantic(label: string): SemanticEntry & { tagClassName: string } {
  return {
    icon: 'folder',
    label: resolveSemanticAlias('project', label, label),
    tagClassName: projectTagAppearance(label),
  };
}

export function resolveCalendarSemantic(label: string, active = false): SemanticEntry & { chipClassName: string } {
  return {
    icon: 'calendar',
    label: resolveSemanticAlias('calendar', label, label),
    chipClassName: calendarFilterChipAppearance(active),
  };
}

export function resolveNudgeKindSemantic(urgent: boolean): SemanticEntry & { tagClassName: string } {
  const entry = urgent ? alertSemanticEntries.urgent : nudgeSemanticEntries.default;
  return {
    ...entry,
    tagClassName: nudgeKindTagAppearance(urgent),
  };
}

export function resolveAlertSemantic(kind: string): SemanticEntry {
  const entry = alertSemanticEntries[normalizeSemanticKey(kind)] ?? alertSemanticEntries.default;
  return {
    ...entry,
    label: resolveSemanticAlias('alert', kind, entry.label),
  };
}

export function resolveModeSemantic(kind: string): SemanticEntry {
  const entry = modeSemanticEntries[normalizeSemanticKey(kind)] ?? modeSemanticEntries.default;
  return {
    ...entry,
    label: resolveSemanticAlias('mode', kind, entry.label),
  };
}

export function resolveNudgeSemantic(kind: string): SemanticEntry {
  const entry = nudgeSemanticEntries[normalizeSemanticKey(kind)] ?? nudgeSemanticEntries.default;
  return {
    ...entry,
    label: resolveSemanticAlias('nudge', kind, entry.label),
  };
}

export function resolveProviderSemantic(provider: string): SemanticEntry & { glyphClassName: string } {
  const key = normalizeSemanticKey(provider);
  const entry = providerSemanticEntries[key] ?? providerSemanticEntries.default;
  return {
    ...entry,
    label: resolveSemanticAlias('provider', provider, entry.label),
  };
}

export function resolveStateStatusSemantic(status: string | null | undefined): SemanticEntry & { tone: SystemStatusTone } {
  const normalized = status?.toLowerCase() ?? '';
  let tone: SystemStatusTone = 'neutral';
  let icon: SemanticIconKey = 'spark';
  if (normalized.includes('error') || normalized.includes('blocked') || normalized.includes('required')) {
    tone = 'warning';
    icon = 'warning';
  } else if (normalized.includes('degraded') || normalized.includes('stale')) {
    tone = 'degraded';
    icon = 'warning';
  } else if (normalized.includes('connected') || normalized.includes('configured') || normalized.includes('ready') || normalized.includes('available')) {
    tone = 'active';
    icon = 'server';
  } else if (normalized.includes('offline') || normalized.includes('never') || normalized.includes('not')) {
    tone = 'offline';
    icon = 'server';
  } else if (normalized.includes('enabled')) {
    tone = 'done';
    icon = 'server';
  } else if (normalized.includes('disabled')) {
    tone = 'offline';
    icon = 'server';
  } else if (normalized.includes('ok')) {
    tone = 'done';
    icon = 'server';
  }
  const fallbackLabel = status?.trim() || 'unknown';
  return {
    icon,
    label: resolveSemanticAlias('alert', normalized || fallbackLabel, fallbackLabel),
    tone,
  };
}

export function resolveProviderStatusSemantic(status: string | null | undefined): SemanticEntry & { tone: SystemStatusTone } {
  const normalized = status?.toLowerCase() ?? '';
  let tone: SystemStatusTone = 'neutral';
  let icon: SemanticIconKey = 'server';
  if (normalized.includes('connected') || normalized.includes('configured') || normalized.includes('ready') || normalized.includes('available') || normalized.includes('ok')) {
    tone = 'done';
  } else if (normalized.includes('enabled')) {
    tone = 'done';
  } else if (normalized.includes('error') || normalized.includes('blocked')) {
    tone = 'warning';
    icon = 'warning';
  } else if (normalized.includes('degraded') || normalized.includes('stale')) {
    tone = 'degraded';
    icon = 'warning';
  } else if (normalized.includes('offline') || normalized.includes('never') || normalized.includes('not') || normalized.includes('disabled')) {
    tone = 'offline';
  }
  const fallbackLabel = status?.trim() || 'unknown';
  return {
    icon,
    label: resolveSemanticAlias('alert', normalized || fallbackLabel, fallbackLabel),
    tone,
  };
}

export function resolveProjectStatusSemantic(status: string | null | undefined): SemanticEntry & { tone: SystemStatusTone } {
  const normalized = normalizeSemanticKey(status ?? 'unknown');
  const fallbackLabel = status?.trim() || 'unknown';
  return {
    icon: normalized === 'active' ? 'folder' : 'tag',
    label: resolveSemanticAlias('project', normalized || fallbackLabel, fallbackLabel),
    tone: normalized === 'active' ? 'active' : 'neutral',
  };
}
