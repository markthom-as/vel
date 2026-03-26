import {
  SYSTEM_INTEGRATION_PRIMITIVES,
  type IntegrationPrimitiveKey,
} from './SystemIntegrationTaxonomy';

export type SystemSectionKey = 'core' | 'overview' | 'operations' | 'integrations' | 'control' | 'preferences';

export type SystemSubsectionKey =
  | 'core_settings'
  | 'pairing'
  | 'trust'
  | 'horizon'
  | 'activity'
  | 'recovery'
  | IntegrationPrimitiveKey
  | 'providers'
  | 'accounts'
  | 'projects'
  | 'capabilities'
  | 'appearance'
  | 'accessibility'
  | 'documentation';

export type SystemGroupKey = 'domain' | 'capabilities' | 'configuration';

export interface SystemNavigationTarget {
  section?: SystemSectionKey;
  subsection?: SystemSubsectionKey;
  anchor?: string;
}

export const SYSTEM_DOCUMENTATION_ANCHOR = 'system-documentation';

export const SYSTEM_CORE_SETTING_ANCHORS = {
  requiredSetup: 'core-settings-required-setup',
  userDisplayName: 'core-settings-user-display-name',
  nodeDisplayName: 'core-settings-node-display-name',
  agentProfileFreeform: 'core-settings-agent-profile-freeform',
} as const;

export const SYSTEM_PROVIDER_ANCHORS = {
  llmRouting: 'providers-llm-routing',
  googleCalendar: 'providers-google-calendar',
} as const;

export function systemChildAnchor(subsection: SystemSubsectionKey, child: string): string {
  return `${subsection.replaceAll('_', '-')}-${child}`;
}

export const SYSTEM_SECTION_ORDER: Array<{
  key: SystemSectionKey;
  label: string;
  items: Array<{ key: SystemSubsectionKey; label: string; description: string }>;
}> = [
  {
    key: 'core',
    label: 'Core',
    items: [
      { key: 'core_settings', label: 'Core settings', description: 'Required identity and setup needed before Vel can operate normally.' },
      { key: 'pairing', label: 'Node pairing', description: 'Issue, redeem, and inspect node trust links for companion devices.' },
    ],
  },
  {
    key: 'overview',
    label: 'Overview',
    items: [
      { key: 'trust', label: 'Status', description: 'What is healthy, degraded, and active right now.' },
      { key: 'horizon', label: 'Activity', description: 'Near-horizon signals and recent grounded activity.' },
    ],
  },
  {
    key: 'operations',
    label: 'Operations',
    items: [
      { key: 'activity', label: 'Activity', description: 'Sync and runtime summaries before raw logs.' },
      { key: 'recovery', label: 'Backup & Recovery', description: 'Named operational recovery paths only.' },
    ],
  },
  {
    key: 'integrations',
    label: 'Integrations',
    items: SYSTEM_INTEGRATION_PRIMITIVES.map((primitive) => ({
      key: primitive.key as SystemSubsectionKey,
      label: primitive.label,
      description: primitive.description,
    })),
  },
  {
    key: 'control',
    label: 'Control',
    items: [
      { key: 'projects', label: 'Projects', description: 'Project registry and structural mappings.' },
      { key: 'capabilities', label: 'Capabilities', description: 'Capability groups and guarded scope.' },
    ],
  },
  {
    key: 'preferences',
    label: 'Preferences',
    items: [
      { key: 'appearance', label: 'Appearance', description: 'Presentation, density, and number rendering.' },
      { key: 'accessibility', label: 'Accessibility', description: 'Motion, focus, and touch ergonomics.' },
      { key: 'documentation', label: 'Documentation', description: 'Shipped system-surface guidance and reference notes.' },
    ],
  },
];

export const SYSTEM_SECTION_BY_SUBSECTION = new Map<SystemSubsectionKey, SystemSectionKey>(
  SYSTEM_SECTION_ORDER.flatMap((section) => section.items.map((item) => [item.key, section.key] as const)),
);

const LEGACY_SUBSECTION_MAP: Record<string, { section: SystemSectionKey; subsection: SystemSubsectionKey }> = {
  core_settings: { section: 'core', subsection: 'core_settings' },
  people: { section: 'control', subsection: 'projects' },
  calendar: { section: 'overview', subsection: 'horizon' },
  knowledge: { section: 'overview', subsection: 'trust' },
  tools: { section: 'control', subsection: 'capabilities' },
  workflows: { section: 'operations', subsection: 'recovery' },
  templates: { section: 'control', subsection: 'capabilities' },
  modules: { section: 'control', subsection: 'projects' },
  integrations: { section: 'integrations', subsection: 'calendar' },
  accounts: { section: 'integrations', subsection: 'sources' },
  scopes: { section: 'control', subsection: 'capabilities' },
};

const LEGACY_SECTION_MAP: Record<string, SystemSectionKey> = {
  domain: 'core',
  capabilities: 'integrations',
  configuration: 'preferences',
};

const INTEGRATION_GROUP_BY_SUBSECTION = Object.fromEntries(
  SYSTEM_INTEGRATION_PRIMITIVES.map((primitive) => [primitive.key, 'capabilities']),
) as Record<IntegrationPrimitiveKey, SystemGroupKey>;

export const SYSTEM_GROUP_ORDER: Array<{
  key: SystemGroupKey;
  label: string;
  description: string;
}> = [
  {
    key: 'domain',
    label: 'Domain',
    description: 'Identity, trust posture, relationship horizon, and grounded project structure.',
  },
  {
    key: 'capabilities',
    label: 'Capabilities',
    description: 'Connections, runtime affordances, recovery lanes, and inspectable account state.',
  },
  {
    key: 'configuration',
    label: 'Configuration',
    description: 'Operator-facing web preferences, accessibility posture, and semantic UI controls.',
  },
];

const GROUP_BY_SUBSECTION: Record<SystemSubsectionKey, SystemGroupKey> = {
  core_settings: 'domain',
  pairing: 'domain',
  trust: 'domain',
  horizon: 'domain',
  projects: 'domain',
  activity: 'capabilities',
  recovery: 'capabilities',
  ...INTEGRATION_GROUP_BY_SUBSECTION,
  providers: 'capabilities',
  accounts: 'capabilities',
  capabilities: 'capabilities',
  appearance: 'configuration',
  accessibility: 'configuration',
  documentation: 'configuration',
};

export function defaultSubsectionForSystemSection(section: SystemSectionKey): SystemSubsectionKey {
  return SYSTEM_SECTION_ORDER.find((entry) => entry.key === section)?.items[0]?.key ?? 'core_settings';
}

export function groupForSystemSubsection(subsection: SystemSubsectionKey): SystemGroupKey {
  return GROUP_BY_SUBSECTION[subsection];
}

export function resolveSystemTarget(
  target?: SystemNavigationTarget | { section?: string; subsection?: string },
): { section: SystemSectionKey; subsection: SystemSubsectionKey } {
  if (target?.subsection && LEGACY_SUBSECTION_MAP[target.subsection]) {
    return LEGACY_SUBSECTION_MAP[target.subsection];
  }

  if (target?.subsection === 'accounts') {
    return {
      section: 'integrations',
      subsection: 'sources',
    };
  }

  if (target?.subsection === 'providers') {
    const targetAnchor = target && 'anchor' in target ? target.anchor : undefined;
    return {
      section: 'integrations',
      subsection:
        targetAnchor === SYSTEM_PROVIDER_ANCHORS.llmRouting
          ? 'models'
          : 'calendar',
    };
  }

  const normalizedSection = target?.section
    ? (LEGACY_SECTION_MAP[target.section] ?? target.section)
    : undefined;
  const fallbackSection = (normalizedSection && SYSTEM_SECTION_ORDER.some((entry) => entry.key === normalizedSection)
    ? normalizedSection
    : undefined) ?? 'core';

  const subsection = target?.subsection && SYSTEM_SECTION_BY_SUBSECTION.has(target.subsection as SystemSubsectionKey)
    ? (target.subsection as SystemSubsectionKey)
    : defaultSubsectionForSystemSection(fallbackSection);

  return {
    section: fallbackSection,
    subsection,
  };
}

export function systemTrustStatusTarget(): SystemNavigationTarget {
  return { section: 'overview', subsection: 'trust' };
}

export function systemDocumentationDeepLink() {
  return {
    view: 'system' as const,
    systemTarget: {
      section: 'preferences' as const,
      subsection: 'documentation' as const,
      anchor: SYSTEM_DOCUMENTATION_ANCHOR,
    },
    anchor: SYSTEM_DOCUMENTATION_ANCHOR,
  };
}

export function systemSurfaceDeepLink(target: SystemNavigationTarget, anchor?: string) {
  return {
    view: 'system' as const,
    systemTarget: target,
    anchor: anchor ?? target.anchor,
  };
}

export function systemTargetForCoreSetting(
  key: 'user_display_name' | 'node_display_name' | 'agent_profile' | 'required_setup',
): SystemNavigationTarget {
  switch (key) {
    case 'user_display_name':
      return { section: 'core', subsection: 'core_settings', anchor: SYSTEM_CORE_SETTING_ANCHORS.userDisplayName };
    case 'node_display_name':
      return { section: 'core', subsection: 'core_settings', anchor: SYSTEM_CORE_SETTING_ANCHORS.nodeDisplayName };
    case 'agent_profile':
      return { section: 'core', subsection: 'core_settings', anchor: SYSTEM_CORE_SETTING_ANCHORS.agentProfileFreeform };
    case 'required_setup':
    default:
      return { section: 'core', subsection: 'core_settings', anchor: SYSTEM_CORE_SETTING_ANCHORS.requiredSetup };
  }
}

export function systemTargetForProvider(
  key: 'llm_routing' | 'google_calendar' | 'accounts' | 'recovery',
): SystemNavigationTarget {
  switch (key) {
    case 'llm_routing':
      return { section: 'integrations', subsection: 'models', anchor: systemChildAnchor('models', 'llm-routing') };
    case 'google_calendar':
      return { section: 'integrations', subsection: 'calendar', anchor: systemChildAnchor('calendar', 'google_calendar') };
    case 'accounts':
      return { section: 'integrations', subsection: 'sources', anchor: systemChildAnchor('sources', 'account-summary') };
    case 'recovery':
      return { section: 'operations', subsection: 'recovery' };
    default:
      return { section: 'integrations', subsection: 'models' };
  }
}
