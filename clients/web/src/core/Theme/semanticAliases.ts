export type SemanticAliasOverrides = Partial<{
  project: Record<string, string>;
  calendar: Record<string, string>;
  nudge: Record<string, string>;
  alert: Record<string, string>;
  mode: Record<string, string>;
  provider: Record<string, string>;
}>;

export type SemanticAliasFamily = keyof SemanticAliasOverrides;

export const semanticAliasDefaults = {
  project: {
    active: 'Active',
    inactive: 'Inactive',
  },
  calendar: {},
  nudge: {
    nudge: 'Nudge',
    system_settings: 'System',
    trust_warning: 'Trust warning',
    freshness_warning: 'Freshness warning',
    review_request: 'Review request',
    reflow_proposal: 'Reflow proposal',
    needs_input: 'Needs input',
    thread_continuation: 'Thread',
  },
  alert: {
    urgent: 'Urgent',
    warning: 'Warning',
  },
  mode: {
    state: 'State',
    why: 'Why',
    debug: 'Debug',
    focus: 'Focus',
    review: 'Review',
    degraded: 'Degraded',
  },
  provider: {
    google_calendar: 'Google Calendar',
    todoist: 'Todoist',
    github: 'GitHub',
    git: 'Git',
    activity: 'Activity',
    health: 'Health',
    email: 'Email',
    messaging: 'Messaging',
    reminders: 'Reminders',
    notes: 'Notes',
    transcripts: 'Transcripts',
  },
} as const;

export const semanticAliasFamilyOrder: SemanticAliasFamily[] = [
  'provider',
  'project',
  'calendar',
  'mode',
  'nudge',
  'alert',
];

export const semanticAliasFamilyLabels: Record<SemanticAliasFamily, string> = {
  provider: 'Providers',
  project: 'Projects',
  calendar: 'Calendars',
  mode: 'Modes',
  nudge: 'Nudges',
  alert: 'Alerts',
};

let runtimeSemanticAliasOverrides: SemanticAliasOverrides = {};

export function normalizeSemanticAliasOverrides(
  overrides: SemanticAliasOverrides | null | undefined,
): SemanticAliasOverrides {
  if (!overrides) {
    return {};
  }
  const normalized: SemanticAliasOverrides = {};
  for (const family of semanticAliasFamilyOrder) {
    const entries = overrides[family];
    if (!entries) {
      continue;
    }
    const cleanedEntries = Object.entries(entries).reduce<Record<string, string>>((acc, [rawKey, rawValue]) => {
      const key = rawKey.trim().toLowerCase().replace(/\s+/g, '_');
      const value = rawValue.trim();
      if (key.length > 0 && value.length > 0) {
        acc[key] = value;
      }
      return acc;
    }, {});
    if (Object.keys(cleanedEntries).length > 0) {
      normalized[family] = cleanedEntries;
    }
  }
  return normalized;
}

export function setSemanticAliasRuntimeOverrides(overrides: SemanticAliasOverrides | null | undefined) {
  runtimeSemanticAliasOverrides = normalizeSemanticAliasOverrides(overrides);
}

export function resetSemanticAliasRuntimeOverrides() {
  runtimeSemanticAliasOverrides = {};
}

export function resolveSemanticAliasOverride(
  family: keyof typeof semanticAliasDefaults,
  key: string,
): string | null {
  const normalizedKey = key.trim().toLowerCase().replace(/\s+/g, '_');
  return runtimeSemanticAliasOverrides[family]?.[normalizedKey]
    ?? semanticAliasDefaults[family]?.[normalizedKey]
    ?? null;
}
