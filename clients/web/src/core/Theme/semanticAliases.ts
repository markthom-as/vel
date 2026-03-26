export type SemanticAliasOverrides = Partial<{
  project: Record<string, string>;
  calendar: Record<string, string>;
  nudge: Record<string, string>;
  alert: Record<string, string>;
  mode: Record<string, string>;
  provider: Record<string, string>;
}>;

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
    git: 'Git',
    activity: 'Activity',
    health: 'Health',
  },
} as const;

let runtimeSemanticAliasOverrides: SemanticAliasOverrides = {};

export function setSemanticAliasRuntimeOverrides(overrides: SemanticAliasOverrides | null | undefined) {
  runtimeSemanticAliasOverrides = overrides ?? {};
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
