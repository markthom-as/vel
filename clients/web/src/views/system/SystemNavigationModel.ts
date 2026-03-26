import type { AgentInspectData, IntegrationConnectionData } from '../../types';
import { resolveProviderSemantic } from '../../core/Theme/semanticRegistry';
import type { IntegrationProviderSummary } from './SystemProvidersSection';
import { resolveConnectionTitle } from './SystemAccountsSection';
import {
  integrationPrimitiveDescriptor,
  normalizeIntegrationPrimitiveValue,
  SYSTEM_INTEGRATION_PRIMITIVES,
  type IntegrationPrimitiveKey,
} from './SystemIntegrationTaxonomy';
import {
  groupForSystemSubsection as groupForSubsection,
  SYSTEM_DOCUMENTATION_ANCHOR,
  SYSTEM_GROUP_ORDER,
  SYSTEM_SECTION_ORDER,
  systemChildAnchor,
  type SystemGroupKey,
  type SystemSubsectionKey,
} from './systemNavigation';

export type SystemSidebarChild = {
  id: string;
  label: string;
};

function normalizeIntegrationKey(value: string): string {
  return normalizeIntegrationPrimitiveValue(value);
}

function sourceLabelsForPrimitive({
  connections,
  primitiveKey,
}: {
  connections: IntegrationConnectionData[];
  primitiveKey: IntegrationPrimitiveKey;
}) {
  const primitive = integrationPrimitiveDescriptor(primitiveKey);
  const { families, providerKeys } = primitive;
  const familySet = new Set(families.map(normalizeIntegrationKey));
  const providerSet = new Set(providerKeys.map(normalizeIntegrationKey));
  const labels = new Map<string, string>();

  connections.forEach((connection) => {
    const normalizedFamily = normalizeIntegrationKey(connection.family);
    const normalizedProvider = normalizeIntegrationKey(connection.provider_key);
    if (!familySet.has(normalizedFamily) && !providerSet.has(normalizedProvider)) {
      return;
    }
    if (!labels.has(normalizedProvider)) {
      labels.set(normalizedProvider, resolveProviderSemantic(connection.provider_key).label);
    }
  });

  if (labels.size === 0) {
    providerKeys.forEach((providerKey) => {
      labels.set(providerKey, resolveProviderSemantic(providerKey).label);
    });
  }

  return Array.from(labels.entries()).map(([key, label]) => ({ id: key, label }));
}

export function visibleSectionOrder(developerMode: boolean) {
  return developerMode ? SYSTEM_SECTION_ORDER : SYSTEM_SECTION_ORDER.filter((section) => section.key !== 'control');
}

export function resolveNavigableSystemAnchor(
  subsection: SystemSubsectionKey,
  requestedAnchor: string | null | undefined,
  children: Record<SystemSubsectionKey, SystemSidebarChild[]>,
): string | null {
  const subsectionItems = children[subsection] ?? [];
  if (requestedAnchor && subsectionItems.some((child) => child.id === requestedAnchor)) {
    return requestedAnchor;
  }
  return subsectionItems[0]?.id ?? null;
}

export function buildSystemSubsectionChildren({
  coreSetupReady,
  developerMode,
  filteredBlockers,
  people,
  providers,
  llmProfiles,
  connections,
  projects,
  capabilityGroups,
}: {
  coreSetupReady: boolean;
  developerMode: boolean;
  filteredBlockers: AgentInspectData['blockers'];
  people: AgentInspectData['grounding']['people'];
  providers: IntegrationProviderSummary[];
  llmProfiles: Array<{ id: string }>;
  connections: IntegrationConnectionData[];
  projects: AgentInspectData['grounding']['projects'];
  capabilityGroups: AgentInspectData['capabilities']['groups'];
}): Record<SystemSubsectionKey, SystemSidebarChild[]> {
  const primitiveChildren = Object.fromEntries(
    SYSTEM_INTEGRATION_PRIMITIVES.map((primitive) => {
      if (primitive.key === 'models') {
        return [
          primitive.key,
          [
            { id: systemChildAnchor('models', 'llm-routing'), label: 'LLM routing' },
            ...llmProfiles.map((profile) => ({
              id: systemChildAnchor('models', `llm-${profile.id}`),
              label: profile.id,
            })),
          ],
        ] as const;
      }
      if (primitive.key === 'sources') {
        return [
          primitive.key,
          [
            { id: systemChildAnchor('sources', 'account-summary'), label: 'All sources' },
            ...connections.map((connection) => ({
              id: systemChildAnchor('sources', connection.id),
              label: resolveConnectionTitle(connection),
            })),
          ],
        ] as const;
      }
      return [
        primitive.key,
        sourceLabelsForPrimitive({
          connections,
          primitiveKey: primitive.key,
        }).map((source) => ({
          id: systemChildAnchor(primitive.key, source.id),
          label: source.label,
        })),
      ] as const;
    }),
  ) as Partial<Record<SystemSubsectionKey, SystemSidebarChild[]>>;

  return {
    core_settings: [
      ...(!coreSetupReady ? [{ id: systemChildAnchor('core_settings', 'required-setup'), label: 'Required setup' }] : []),
      { id: systemChildAnchor('core_settings', 'user-info'), label: 'User info' },
      { id: systemChildAnchor('core_settings', 'node-client'), label: 'Node / client' },
      { id: systemChildAnchor('core_settings', 'about-agent'), label: 'About agent' },
      ...(developerMode ? [{ id: systemChildAnchor('core_settings', 'developer-controls'), label: 'Developer controls' }] : []),
    ],
    pairing: [
      { id: systemChildAnchor('pairing', 'guide'), label: 'Guide' },
      { id: systemChildAnchor('pairing', 'scope'), label: 'Trust scope' },
      { id: systemChildAnchor('pairing', 'issue'), label: 'Issue token' },
      { id: systemChildAnchor('pairing', 'redeem'), label: 'Redeem token' },
      { id: systemChildAnchor('pairing', 'linked'), label: 'Linked nodes' },
    ],
    ...primitiveChildren,
    trust: [
      { id: systemChildAnchor('trust', 'current-mode'), label: 'Current mode' },
      { id: systemChildAnchor('trust', 'persisted-kinds'), label: 'Persisted kinds' },
      { id: systemChildAnchor('trust', 'grounded-projects'), label: 'Grounded projects' },
      { id: systemChildAnchor('trust', 'degraded-providers'), label: 'Degraded providers' },
      ...(filteredBlockers.length === 0
        ? [{ id: systemChildAnchor('trust', 'health'), label: 'Health' }]
        : filteredBlockers.map((blocker) => ({
            id: systemChildAnchor('trust', blocker.code),
            label: blocker.code,
          }))),
    ],
    horizon: people.slice(0, 6).map((person) => ({
      id: systemChildAnchor('horizon', person.id),
      label: person.display_name,
    })),
    activity: providers.map((provider) => ({
      id: systemChildAnchor('activity', provider.key),
      label: provider.label,
    })),
    recovery: [
      ...providers
        .filter((provider) => {
          const status = provider.status.toLowerCase();
          return status !== 'connected' && status !== 'configured';
        })
        .map((provider) => ({
          id: systemChildAnchor('recovery', provider.key),
          label: provider.label,
        })),
      ...filteredBlockers.map((blocker) => ({
        id: systemChildAnchor('recovery', blocker.code),
        label: blocker.code,
      })),
    ],
    providers: [
      ...providers.map((provider) => ({
        id: systemChildAnchor('providers', provider.key),
        label: resolveProviderSemantic(provider.key).label,
      })),
      ...(llmProfiles.length > 0
        ? [
            { id: systemChildAnchor('providers', 'llm-routing'), label: 'LLM routing' },
            ...llmProfiles.map((profile) => ({
              id: systemChildAnchor('providers', `llm-${profile.id}`),
              label: profile.id,
            })),
          ]
        : []),
    ],
    accounts: connections.map((connection) => ({
      id: systemChildAnchor('accounts', connection.id),
      label: resolveConnectionTitle(connection),
    })),
    projects: projects.map((project) => ({
      id: systemChildAnchor('projects', project.id),
      label: project.name,
    })),
    capabilities: capabilityGroups.map((group) => ({
      id: systemChildAnchor('capabilities', group.kind),
      label: group.label,
    })),
    appearance: [
      { id: systemChildAnchor('appearance', 'dense-rows'), label: 'Dense rows' },
      { id: systemChildAnchor('appearance', 'tabular-numerals'), label: 'Tabular numerals' },
      { id: systemChildAnchor('appearance', 'preview-posture'), label: 'Preview posture' },
    ],
    accessibility: [
      { id: systemChildAnchor('accessibility', 'reduced-motion'), label: 'Reduced motion' },
      { id: systemChildAnchor('accessibility', 'strong-focus-states'), label: 'Strong focus states' },
      { id: systemChildAnchor('accessibility', 'docked-action-bar'), label: 'Docked action bar' },
      { id: systemChildAnchor('accessibility', 'accessibility-law'), label: 'Accessibility law' },
    ],
    documentation: [
      { id: SYSTEM_DOCUMENTATION_ANCHOR, label: 'System documentation' },
    ],
  } as Record<SystemSubsectionKey, SystemSidebarChild[]>;
}

export function buildSystemGroupSummaries({
  filteredBlockers,
  projectsCount,
  providersCount,
  connectionsCount,
}: {
  filteredBlockers: AgentInspectData['blockers'];
  projectsCount: number;
  providersCount: number;
  connectionsCount: number;
}): Record<SystemGroupKey, string> {
  return {
    domain: filteredBlockers.length > 0
      ? `${filteredBlockers.length} active blockers`
      : `${projectsCount} grounded projects`,
    capabilities: `${providersCount} providers · ${connectionsCount} accounts`,
    configuration: 'Appearance, accessibility, and shared aliases',
  };
}

export function buildGroupedSystemNav({
  developerMode,
  sidebarFilter,
  subsectionChildren,
}: {
  developerMode: boolean;
  sidebarFilter: string;
  subsectionChildren: Record<SystemSubsectionKey, SystemSidebarChild[]>;
}) {
  const sectionOrder = visibleSectionOrder(developerMode);
  const sectionFilterQuery = sidebarFilter.trim().toLowerCase();
  const filteredSectionOrder = sectionOrder.map((section) => ({
    ...section,
    items: section.items.filter((item) => {
      if (!sectionFilterQuery) return true;
      const childLabels = (subsectionChildren[item.key] ?? []).map((child) => child.label).join(' ');
      const haystack = `${section.label} ${item.label} ${item.description} ${childLabels}`.toLowerCase();
      return haystack.includes(sectionFilterQuery);
    }),
  })).filter((section) => section.items.length > 0);

  const groupedNav = SYSTEM_GROUP_ORDER.map((group) => ({
    ...group,
    items: filteredSectionOrder
      .flatMap((section) => section.items)
      .filter((item) => groupForSubsection(item.key) === group.key),
  })).filter((group) => group.items.length > 0);

  return {
    sectionOrder,
    groupedNav,
    filteredSectionOrder,
  };
}
