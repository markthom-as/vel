import { useEffect, useMemo, useState } from 'react';
import { loadAgentInspect } from '../../data/agent-grounding';
import {
  disconnectGoogleCalendar,
  disconnectTodoist,
  loadIntegrationConnections,
  loadIntegrations,
  loadSettings,
  operatorQueryKeys,
  syncSource,
  updateSettings,
  updateWebSettings,
} from '../../data/operator';
import { contextQueryKeys } from '../../data/context';
import { invalidateQuery, useQuery } from '../../data/query';
import type {
  AgentCapabilityEntryData,
  AgentInspectData,
  IntegrationConnectionData,
  IntegrationsData,
  LocalIntegrationData,
  SettingsData,
} from '../../types';
import { Button } from '../../core/Button';
import { cn } from '../../core/cn';
import { SettingsIcon, SyncIcon, WarningIcon } from '../../core/Icons';
import {
  PanelEmptyRow,
} from '../../core/PanelChrome';
import {
  SystemDocumentField,
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentMetaRow,
  SystemDocumentStatsGrid,
  SystemDocumentSectionLabel,
  SystemDocumentStatusChip,
  SystemDocumentToggleRow,
} from '../../core/SystemDocument';
import { SurfaceState } from '../../core/SurfaceState';
import { uiFonts } from '../../core/Theme';
import { SearchField } from '../../core/SearchField/SearchField';

export type SystemSectionKey = 'overview' | 'operations' | 'integrations' | 'control' | 'preferences';
export type SystemSubsectionKey =
  | 'trust'
  | 'horizon'
  | 'activity'
  | 'recovery'
  | 'providers'
  | 'accounts'
  | 'projects'
  | 'capabilities'
  | 'appearance'
  | 'accessibility';

export interface SystemNavigationTarget {
  section?: SystemSectionKey;
  subsection?: SystemSubsectionKey;
}

interface SystemViewProps {
  target?: SystemNavigationTarget;
}

type IntegrationActionId = 'google-disconnect' | 'google-refresh' | 'todoist-disconnect' | 'todoist-refresh';
type IntegrationProviderKey =
  | 'google_calendar'
  | 'todoist'
  | 'activity'
  | 'health'
  | 'git'
  | 'messaging'
  | 'reminders'
  | 'notes'
  | 'transcripts';

const SECTION_ORDER: Array<{
  key: SystemSectionKey;
  label: string;
  items: Array<{ key: SystemSubsectionKey; label: string; description: string }>;
}> = [
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
    items: [
      { key: 'providers', label: 'Providers', description: 'Connection health and provider detail.' },
      { key: 'accounts', label: 'Accounts', description: 'Connected accounts and scoped identity.' },
    ],
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
    ],
  },
];

const SECTION_BY_SUBSECTION = new Map<SystemSubsectionKey, SystemSectionKey>(
  SECTION_ORDER.flatMap((section) => section.items.map((item) => [item.key, section.key] as const)),
);

const LEGACY_SUBSECTION_MAP: Record<string, { section: SystemSectionKey; subsection: SystemSubsectionKey }> = {
  people: { section: 'control', subsection: 'projects' },
  calendar: { section: 'overview', subsection: 'horizon' },
  knowledge: { section: 'overview', subsection: 'trust' },
  tools: { section: 'control', subsection: 'capabilities' },
  workflows: { section: 'operations', subsection: 'recovery' },
  templates: { section: 'control', subsection: 'capabilities' },
  modules: { section: 'control', subsection: 'projects' },
  integrations: { section: 'integrations', subsection: 'providers' },
  accounts: { section: 'integrations', subsection: 'accounts' },
  scopes: { section: 'control', subsection: 'capabilities' },
};

const LEGACY_SECTION_MAP: Record<string, SystemSectionKey> = {
  domain: 'overview',
  capabilities: 'control',
  configuration: 'integrations',
};

function defaultSubsection(section: SystemSectionKey): SystemSubsectionKey {
  return SECTION_ORDER.find((entry) => entry.key === section)?.items[0]?.key ?? 'trust';
}

function resolveSystemTarget(target?: SystemNavigationTarget | { section?: string; subsection?: string }) {
  if (target?.subsection && LEGACY_SUBSECTION_MAP[target.subsection]) {
    return LEGACY_SUBSECTION_MAP[target.subsection];
  }

  const normalizedSection = target?.section
    ? (LEGACY_SECTION_MAP[target.section] ?? target.section)
    : undefined;
  const fallbackSection = (normalizedSection && SECTION_ORDER.some((entry) => entry.key === normalizedSection)
    ? normalizedSection
    : undefined) ?? 'overview';

  const subsection = target?.subsection && SECTION_BY_SUBSECTION.has(target.subsection as SystemSubsectionKey)
    ? (target.subsection as SystemSubsectionKey)
    : defaultSubsection(fallbackSection);

  return {
    section: fallbackSection,
    subsection,
  };
}

function providerTintClass(provider: IntegrationProviderKey) {
  switch (provider) {
    case 'google_calendar':
      return 'bg-[#b96e3a] text-[#ffd7bf]';
    case 'todoist':
      return 'bg-[#8d4a35] text-[#ffd8c9]';
    case 'git':
      return 'bg-[#73553a] text-[#f7d0af]';
    default:
      return 'bg-zinc-700 text-zinc-200';
  }
}

function stateToneFromStatus(status: string | null | undefined): 'active' | 'warning' | 'degraded' | 'offline' | 'done' | 'neutral' {
  const normalized = status?.toLowerCase() ?? '';
  if (normalized.includes('error') || normalized.includes('blocked')) {
    return 'warning';
  }
  if (normalized.includes('degraded') || normalized.includes('stale')) {
    return 'degraded';
  }
  if (normalized.includes('connected') || normalized.includes('configured') || normalized.includes('ready') || normalized.includes('available')) {
    return 'active';
  }
  if (normalized.includes('offline') || normalized.includes('never') || normalized.includes('not')) {
    return 'offline';
  }
  if (normalized.includes('ok')) {
    return 'done';
  }
  return 'neutral';
}

function providerStatusTone(status: string | null | undefined): 'active' | 'warning' | 'degraded' | 'offline' | 'done' | 'neutral' {
  const normalized = status?.toLowerCase() ?? '';
  if (normalized.includes('connected') || normalized.includes('configured') || normalized.includes('ready') || normalized.includes('available') || normalized.includes('ok')) {
    return 'done';
  }
  if (normalized.includes('error') || normalized.includes('blocked')) {
    return 'warning';
  }
  if (normalized.includes('degraded') || normalized.includes('stale')) {
    return 'degraded';
  }
  if (normalized.includes('offline') || normalized.includes('never') || normalized.includes('not')) {
    return 'offline';
  }
  return 'neutral';
}

type IntegrationProviderSummary = {
  key: IntegrationProviderKey;
  label: string;
  guidance: string;
  status: string;
  configured: boolean;
  connected?: boolean;
  lastSyncAt: number | null;
  meta: Array<{ label: string; value: string }>;
};

type SystemSidebarChild = {
  id: string;
  label: string;
};

function systemChildAnchor(subsection: SystemSubsectionKey, key: string): string {
  return `${subsection}-${key.replace(/[^a-z0-9]+/gi, '-').replace(/^-+|-+$/g, '').toLowerCase()}`;
}

function providerSummaries(integrations: IntegrationsData): IntegrationProviderSummary[] {
  const locals: Array<{ key: Exclude<IntegrationProviderKey, 'google_calendar' | 'todoist'>; label: string; data: LocalIntegrationData }> = [
    { key: 'activity', label: 'Activity', data: integrations.activity },
    { key: 'health', label: 'Health', data: integrations.health },
    { key: 'git', label: 'Git', data: integrations.git },
    { key: 'messaging', label: 'Messaging', data: integrations.messaging },
    { key: 'reminders', label: 'Reminders', data: integrations.reminders },
    { key: 'notes', label: 'Notes', data: integrations.notes },
    { key: 'transcripts', label: 'Transcripts', data: integrations.transcripts },
  ];

  return [
    {
      key: 'google_calendar',
      label: 'Google Calendar',
      guidance: integrations.google_calendar.guidance?.detail ?? integrations.google_calendar.last_error ?? 'Calendar connection is healthy.',
      status: integrations.google_calendar.connected ? 'connected' : integrations.google_calendar.configured ? 'configured' : 'not configured',
      configured: integrations.google_calendar.configured,
      connected: integrations.google_calendar.connected,
      lastSyncAt: integrations.google_calendar.last_sync_at,
      meta: [
        {
          label: 'Selected calendars',
          value: `${integrations.google_calendar.calendars.filter((calendar) => calendar.selected).length}`,
        },
        { label: 'Last sync', value: formatMaybeTimestamp(integrations.google_calendar.last_sync_at) },
      ],
    },
    {
      key: 'todoist',
      label: 'Todoist',
      guidance: integrations.todoist.guidance?.detail ?? integrations.todoist.last_error ?? 'Todoist connection is healthy.',
      status: integrations.todoist.connected ? 'connected' : integrations.todoist.configured ? 'configured' : 'not configured',
      configured: integrations.todoist.configured,
      connected: integrations.todoist.connected,
      lastSyncAt: integrations.todoist.last_sync_at,
      meta: [
        { label: 'Last item count', value: `${integrations.todoist.last_item_count ?? 0}` },
        { label: 'Last sync', value: formatMaybeTimestamp(integrations.todoist.last_sync_at) },
      ],
    },
    ...locals.map(({ key, label, data }) => ({
      key,
      label,
      guidance: data.guidance?.detail ?? data.last_error ?? 'No additional guidance recorded.',
      status: data.configured ? 'configured' : 'not configured',
      configured: data.configured,
      lastSyncAt: data.last_sync_at,
      meta: [
        { label: 'Source', value: data.source_path ?? 'Unset' },
        { label: 'Last sync', value: formatMaybeTimestamp(data.last_sync_at) },
      ],
    })),
  ];
}

export function SystemView({ target }: SystemViewProps) {
  const inspectKey = useMemo(() => operatorQueryKeys.agentInspect(), []);
  const integrationsKey = useMemo(() => operatorQueryKeys.integrations(), []);
  const connectionsKey = useMemo(() => operatorQueryKeys.integrationConnections(), []);
  const settingsKey = useMemo(() => operatorQueryKeys.settings(), []);
  const nowKey = useMemo(() => contextQueryKeys.now(), []);

  const {
    data: inspect,
    loading: inspectLoading,
    error: inspectError,
  } = useQuery<AgentInspectData | null>(
    inspectKey,
    async () => {
      const response = await loadAgentInspect();
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to load system grounding');
      }
      return response.data;
    },
  );
  const {
    data: integrations,
    loading: integrationsLoading,
    error: integrationsError,
  } = useQuery<IntegrationsData | null>(
    integrationsKey,
    async () => {
      const response = await loadIntegrations();
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to load integration state');
      }
      return response.data;
    },
  );
  const {
    data: connections = [],
    loading: connectionsLoading,
    error: connectionsError,
  } = useQuery<IntegrationConnectionData[]>(
    connectionsKey,
    async () => {
      const response = await loadIntegrationConnections({ includeDisabled: true });
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to load account connections');
      }
      return response.data;
    },
  );
  const {
    data: settings,
    loading: settingsLoading,
    error: settingsError,
  } = useQuery<SettingsData | null>(
    settingsKey,
    async () => {
      const response = await loadSettings();
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to load system settings');
      }
      return response.data;
    },
  );

  const initialTarget = resolveSystemTarget(target);
  const [activeSection, setActiveSection] = useState<SystemSectionKey>(initialTarget.section);
  const [activeSubsection, setActiveSubsection] = useState<SystemSubsectionKey>(initialTarget.subsection);
  const [pendingAction, setPendingAction] = useState<IntegrationActionId | null>(null);
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [sidebarFilter, setSidebarFilter] = useState('');
  const [activeChildAnchor, setActiveChildAnchor] = useState<string | null>(null);
  useEffect(() => {
    const resolved = resolveSystemTarget(target);
    setActiveSection(resolved.section);
    setActiveSubsection(resolved.subsection);
  }, [target?.section, target?.subsection]);

  useEffect(() => {
    if (typeof window === 'undefined' || typeof IntersectionObserver === 'undefined') return;
    const sections = SECTION_ORDER.flatMap((section) => section.items)
      .map((item) => document.getElementById(item.key))
      .filter((node): node is HTMLElement => Boolean(node));
    if (sections.length === 0) return;
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((entry) => entry.isIntersecting)
          .sort((a, b) => b.intersectionRatio - a.intersectionRatio)[0];
        if (!visible) return;
        const key = visible.target.id as SystemSubsectionKey;
        setActiveSubsection(key);
        setActiveSection(SECTION_BY_SUBSECTION.get(key) ?? 'overview');
      },
      { rootMargin: '-18% 0px -60% 0px', threshold: [0.1, 0.4, 0.7] },
    );
    sections.forEach((node) => observer.observe(node));
    return () => observer.disconnect();
  }, [inspect, integrations, connections, sidebarFilter]);

  const providers = integrations ? providerSummaries(integrations) : [];
  const projects = inspect?.grounding.projects ?? [];
  const capabilityGroups = inspect?.capabilities.groups ?? [];
  const subsectionChildren = useMemo<Record<SystemSubsectionKey, SystemSidebarChild[]>>(
    () => ({
      trust: [
        { id: systemChildAnchor('trust', 'current-mode'), label: 'Current mode' },
        { id: systemChildAnchor('trust', 'persisted-kinds'), label: 'Persisted kinds' },
        { id: systemChildAnchor('trust', 'grounded-projects'), label: 'Grounded projects' },
        { id: systemChildAnchor('trust', 'degraded-providers'), label: 'Degraded providers' },
        ...((inspect?.blockers ?? []).length === 0
          ? [{ id: systemChildAnchor('trust', 'health'), label: 'Health' }]
          : (inspect?.blockers ?? []).map((blocker) => ({
              id: systemChildAnchor('trust', blocker.code),
              label: blocker.code,
            }))),
      ],
      horizon: [
        ...((inspect?.grounding.now.schedule?.upcoming_events ?? []).slice(0, 6).map((event, index) => ({
          id: systemChildAnchor('horizon', `${event.title}-${index}`),
          label: event.title,
        }))),
        ...((inspect?.grounding.people ?? []).slice(0, 6).map((person) => ({
          id: systemChildAnchor('horizon', person.id),
          label: person.display_name,
        }))),
      ],
      activity: providers.map((provider) => ({
        id: systemChildAnchor('activity', provider.key),
        label: provider.label,
      })),
      recovery: [
        ...providers
          .filter((provider) => provider.status !== 'connected')
          .map((provider) => ({
            id: systemChildAnchor('recovery', provider.key),
            label: provider.label,
          })),
        ...((inspect?.blockers ?? []).map((blocker) => ({
          id: systemChildAnchor('recovery', blocker.code),
          label: blocker.code,
        }))),
      ],
      providers: providers.map((provider) => ({
        id: systemChildAnchor('providers', provider.key),
        label: provider.label,
      })),
      accounts: connections.map((connection) => ({
        id: systemChildAnchor('accounts', connection.id),
        label: connection.display_name,
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
    }),
    [capabilityGroups, connections, inspect?.blockers, inspect?.grounding.now.schedule?.upcoming_events, inspect?.grounding.people, projects, providers],
  );

  useEffect(() => {
    if (typeof window === 'undefined' || typeof IntersectionObserver === 'undefined') return;
    const childNodes = Object.values(subsectionChildren)
      .flat()
      .map((item) => document.getElementById(item.id))
      .filter((node): node is HTMLElement => Boolean(node));
    if (childNodes.length === 0) return;
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((entry) => entry.isIntersecting)
          .sort((a, b) => b.intersectionRatio - a.intersectionRatio)[0];
        if (!visible) return;
        setActiveChildAnchor(visible.target.id);
      },
      { rootMargin: '-20% 0px -66% 0px', threshold: [0.1, 0.5, 0.8] },
    );
    childNodes.forEach((node) => observer.observe(node));
    return () => observer.disconnect();
  }, [subsectionChildren]);

  if (inspectLoading || integrationsLoading || connectionsLoading || settingsLoading) {
    return <SurfaceState message="Loading canonical system state…" layout="centered" />;
  }

  const error = inspectError ?? integrationsError ?? connectionsError ?? settingsError;
  if (error) {
    return <SurfaceState message={error} layout="centered" tone="warning" />;
  }

  if (!inspect || !integrations) {
    return <SurfaceState message="No canonical system state is available yet." layout="centered" />;
  }
  const preferences = {
    denseRows: settings?.web_settings?.dense_rows ?? true,
    tabularNumbers: settings?.web_settings?.tabular_numbers ?? true,
    reducedMotion: settings?.web_settings?.reduced_motion ?? false,
    strongFocus: settings?.web_settings?.strong_focus ?? true,
    dockedActionBar: settings?.web_settings?.docked_action_bar ?? true,
  };
  const sectionFilterQuery = sidebarFilter.trim().toLowerCase();
  const filteredSectionOrder = SECTION_ORDER.map((section) => ({
    ...section,
    items: section.items.filter((item) => {
      if (!sectionFilterQuery) return true;
      const childLabels = (subsectionChildren[item.key] ?? []).map((child) => child.label).join(' ');
      const haystack = `${section.label} ${item.label} ${item.description} ${childLabels}`.toLowerCase();
      return haystack.includes(sectionFilterQuery);
    }),
  })).filter((section) => section.items.length > 0);
  async function runIntegrationAction(actionId: IntegrationActionId) {
    setPendingAction(actionId);
    setActionMessage(null);
    try {
      if (actionId === 'google-disconnect') {
        const response = await disconnectGoogleCalendar();
        if (!response.ok) {
          throw new Error(response.error?.message ?? 'Failed to disconnect Google Calendar');
        }
      } else if (actionId === 'google-refresh') {
        const response = await syncSource('calendar');
        if (!response.ok) {
          throw new Error(response.error?.message ?? 'Failed to refresh Google Calendar');
        }
      } else if (actionId === 'todoist-disconnect') {
        const response = await disconnectTodoist();
        if (!response.ok) {
          throw new Error(response.error?.message ?? 'Failed to disconnect Todoist');
        }
      } else if (actionId === 'todoist-refresh') {
        const response = await syncSource('todoist');
        if (!response.ok) {
          throw new Error(response.error?.message ?? 'Failed to refresh Todoist');
        }
      }

      invalidateQuery(integrationsKey, { refetch: true });
      invalidateQuery(connectionsKey, { refetch: true });
      invalidateQuery(nowKey, { refetch: true });
      setActionMessage('Canonical action completed.');
    } catch (actionError) {
      setActionMessage(actionError instanceof Error ? actionError.message : String(actionError));
    } finally {
      setPendingAction(null);
    }
  }

  return (
    <div className="flex-1 overflow-y-auto bg-transparent">
      <div className="mx-auto max-w-7xl px-4 py-4 pb-32 sm:px-6">
        <div className="grid gap-5 xl:grid-cols-[16rem_minmax(0,1fr)]">
          <aside className="self-start xl:sticky xl:top-4 xl:max-h-[calc(100vh-2rem)] xl:overflow-y-auto">
            <div className="border-b border-[var(--vel-color-border)] pb-3">
              <p className={`${uiFonts.display} inline-flex items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-muted)]`}>
                <SettingsIcon size={12} />
                <span>SYSTEM</span>
              </p>
            </div>
            <SearchField
              className="mt-3"
              aria-label="Filter system sections"
              value={sidebarFilter}
              onChange={(event) => setSidebarFilter(event.target.value)}
              placeholder="Filter system"
            />
            <nav className="mt-3 space-y-2" aria-label="System sections">
              {filteredSectionOrder.map((section) => {
                const sectionActive = section.key === activeSection;
                return (
                  <div key={section.key} className="border-b border-[var(--vel-color-border)] pb-2">
                    <button
                      type="button"
                      onClick={() => {
                        setActiveSection(section.key);
                        setActiveSubsection(defaultSubsection(section.key));
                      }}
                      aria-pressed={sectionActive}
                      className={cn(
                        'w-full text-left text-sm font-medium transition',
                        sectionActive ? 'text-[var(--vel-color-accent-strong)]' : 'text-[var(--vel-color-text)] hover:text-[var(--vel-color-accent-soft)]',
                      )}
                      >
                        {section.label}
                      </button>
                    {sectionActive ? (
                      <div className="mt-1.5 space-y-0.5 pl-3">
                        {section.items.map((item) => {
                          const itemActive = activeSubsection === item.key;
                          const children = subsectionChildren[item.key] ?? [];
                          const showChildren = itemActive || children.some((child) => child.id === activeChildAnchor);
                          return (
                            <div key={item.key} className="space-y-1">
                              <button
                                type="button"
                                onClick={() => {
                                  setActiveSection(section.key);
                                  setActiveSubsection(item.key);
                                  if (typeof document !== 'undefined') {
                                    window.requestAnimationFrame(() => {
                                      const targetEl = document.getElementById(item.key);
                                      if (typeof targetEl?.scrollIntoView === 'function') {
                                        targetEl.scrollIntoView({ block: 'start', behavior: 'smooth' });
                                      }
                                    });
                                  }
                                }}
                                aria-pressed={itemActive}
                                className={cn(
                                  'block w-full border-l pl-3 text-left text-[13px] leading-5 transition',
                                  itemActive
                                    ? 'border-[var(--vel-color-accent-strong)] text-[var(--vel-color-accent-soft)]'
                                    : 'border-[var(--vel-color-border)] text-[var(--vel-color-muted)] hover:text-[var(--vel-color-text)]',
                                )}
                              >
                                {item.label}
                              </button>
                              {showChildren && children.length > 0 ? (
                                <div className="space-y-1 pl-5">
                                  {children.map((child) => {
                                    const childActive = child.id === activeChildAnchor;
                                    return (
                                      <button
                                        key={child.id}
                                        type="button"
                                        onClick={() => {
                                          setActiveSection(section.key);
                                          setActiveSubsection(item.key);
                                          setActiveChildAnchor(child.id);
                                          if (typeof document !== 'undefined') {
                                            window.requestAnimationFrame(() => {
                                              const targetEl = document.getElementById(child.id);
                                              if (typeof targetEl?.scrollIntoView === 'function') {
                                                targetEl.scrollIntoView({ block: 'start', behavior: 'smooth' });
                                              }
                                            });
                                          }
                                        }}
                                        aria-pressed={childActive}
                                        className={cn(
                                          'block w-full border-l pl-3 text-left text-[11px] uppercase tracking-[0.14em] transition',
                                          childActive
                                            ? 'border-[var(--vel-color-accent-border)] text-[var(--vel-color-accent-soft)]'
                                            : 'border-[var(--vel-color-border)] text-[var(--vel-color-dim)] hover:text-[var(--vel-color-text)]',
                                        )}
                                      >
                                        {child.label}
                                      </button>
                                    );
                                  })}
                                </div>
                              ) : null}
                            </div>
                          );
                        })}
                      </div>
                    ) : null}
                  </div>
                );
              })}
            </nav>

          </aside>

          <section className="space-y-5">
            {actionMessage ? (
              <div className="border-b border-[var(--vel-color-border)] pb-2 text-[13px] leading-5 text-[var(--vel-color-text)]">
                {actionMessage}
              </div>
            ) : null}

            <div className="min-w-0 space-y-6">
              {SECTION_ORDER.flatMap((section) => section.items).map((item) => (
                <section key={item.key} id={item.key} className="scroll-mt-24 border-b border-[var(--vel-color-border)] pb-5 last:border-b-0">
                  <div className="mb-2">
                    <SystemDocumentSectionLabel>{item.label}</SystemDocumentSectionLabel>
                  </div>
                  {renderSystemSubsection({
                    subsection: item.key,
                    inspect,
                    providers,
                    connections,
                    projects,
                    capabilityGroups,
                    settings: settings ?? null,
                    pendingAction,
                    onRunIntegrationAction: runIntegrationAction,
                    blockers: inspect.blockers,
                    preferences,
                    onTogglePreference: async (key) => {
                      const patchMap = {
                        denseRows: 'dense_rows',
                        tabularNumbers: 'tabular_numbers',
                        reducedMotion: 'reduced_motion',
                        strongFocus: 'strong_focus',
                        dockedActionBar: 'docked_action_bar',
                      } as const;
                      const response = await updateWebSettings({
                        [patchMap[key]]: !preferences[key],
                      });
                      if (!response.ok) {
                        throw new Error(response.error?.message ?? 'Failed to update web settings');
                      }
                      invalidateQuery(settingsKey, { refetch: true });
                    },
                    onCommitSettingField: async (key, value) => {
                      const response = await updateSettings({ [key]: value });
                      if (!response.ok) {
                        throw new Error(response.error?.message ?? 'Failed to update settings');
                      }
                      invalidateQuery(settingsKey, { refetch: true });
                    },
                    onToggleSetting: async (key, value) => {
                      const response = await updateSettings({ [key]: value });
                      if (!response.ok) {
                        throw new Error(response.error?.message ?? 'Failed to update settings');
                      }
                      invalidateQuery(settingsKey, { refetch: true });
                    },
                  })}
                </section>
              ))}
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}

function renderSystemSubsection({
  subsection,
  inspect,
  providers,
  connections,
  projects,
  capabilityGroups,
  settings,
  pendingAction,
  onRunIntegrationAction,
  blockers,
  preferences,
  onTogglePreference,
  onCommitSettingField,
  onToggleSetting,
}: {
  subsection: SystemSubsectionKey;
  inspect: AgentInspectData;
  providers: IntegrationProviderSummary[];
  connections: IntegrationConnectionData[];
  projects: AgentInspectData['grounding']['projects'];
  capabilityGroups: AgentInspectData['capabilities']['groups'];
  settings: SettingsData | null;
  pendingAction: IntegrationActionId | null;
  onRunIntegrationAction: (actionId: IntegrationActionId) => void | Promise<void>;
  blockers: AgentInspectData['blockers'];
  preferences: {
    denseRows: boolean;
    tabularNumbers: boolean;
    reducedMotion: boolean;
    strongFocus: boolean;
    dockedActionBar: boolean;
  };
  onTogglePreference: (key: 'denseRows' | 'tabularNumbers' | 'reducedMotion' | 'strongFocus' | 'dockedActionBar') => void;
  onCommitSettingField: (key: 'node_display_name' | 'timezone' | 'tailscale_base_url' | 'lan_base_url', value: string) => Promise<void>;
  onToggleSetting: (key: 'writeback_enabled' | 'tailscale_preferred', value: boolean) => Promise<void>;
}) {
  if (subsection === 'trust') {
    return <OverviewTrustDetail inspect={inspect} integrations={providers} />;
  }
  if (subsection === 'horizon') {
    return <OverviewHorizonDetail inspect={inspect} />;
  }
  if (subsection === 'activity') {
    return <OperationsActivityDetail providers={providers} connectionCount={connections.length} />;
  }
  if (subsection === 'recovery') {
    return <OperationsRecoveryDetail providers={providers} blockers={blockers} />;
  }
  if (subsection === 'providers') {
    return (
      <IntegrationsProvidersDetail
        providers={providers}
        pendingAction={pendingAction}
        onRunIntegrationAction={onRunIntegrationAction}
      />
    );
  }
  if (subsection === 'accounts') {
    return (
      <IntegrationsAccountsDetail
        connections={connections}
      />
    );
  }
  if (subsection === 'projects') {
    return (
      <ControlProjectsDetail
        projects={projects}
      />
    );
  }
  if (subsection === 'capabilities') {
    return (
      <ControlCapabilitiesDetail
        capabilityGroups={capabilityGroups}
        blockers={blockers}
      />
    );
  }
  if (subsection === 'appearance') {
    return <PreferencesAppearanceDetail preferences={preferences} onToggle={onTogglePreference} />;
  }
  return (
    <PreferencesAccessibilityDetail
      preferences={preferences}
      settings={settings}
      onToggle={onTogglePreference}
      onCommitSettingField={onCommitSettingField}
      onToggleSetting={onToggleSetting}
    />
  );
}

function OverviewTrustDetail({
  inspect,
  integrations,
}: {
  inspect: AgentInspectData;
  integrations: IntegrationProviderSummary[];
}) {
  const degradedProviders = integrations.filter((provider) => provider.status !== 'connected' && provider.configured);

  return (
    <SystemDocumentStatsGrid className="gap-x-6">
      <SystemDocumentMetaRow id={systemChildAnchor('trust', 'current-mode')} label="Current mode" value={inspect.grounding.current_context?.mode ?? 'Unknown'} />
      <SystemDocumentMetaRow id={systemChildAnchor('trust', 'persisted-kinds')} label="Persisted kinds" value={inspect.explainability.persisted_record_kinds.join(', ') || 'None'} />
      <SystemDocumentMetaRow id={systemChildAnchor('trust', 'grounded-projects')} label="Grounded projects" value={`${inspect.grounding.projects.length}`} />
      <SystemDocumentMetaRow id={systemChildAnchor('trust', 'degraded-providers')} label="Degraded providers" value={`${degradedProviders.length}`} />
      {inspect.blockers.length === 0 ? (
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'health')} label="Health" value="Stable" />
      ) : (
        inspect.blockers.map((blocker) => (
          <SystemDocumentMetaRow key={blocker.code} id={systemChildAnchor('trust', blocker.code)} label={blocker.code} value={blocker.message} />
        ))
      )}
    </SystemDocumentStatsGrid>
  );
}

function OverviewHorizonDetail({ inspect }: { inspect: AgentInspectData }) {
  const upcomingEvents = inspect.grounding.now.schedule?.upcoming_events ?? [];
  const people = inspect.grounding.people;

  return (
    <SystemDocumentList>
      {upcomingEvents.length === 0 ? (
        <PanelEmptyRow>No grounded upcoming events are available right now.</PanelEmptyRow>
      ) : (
        upcomingEvents.slice(0, 6).map((event, index) => (
          <SystemDocumentItem
            key={`${event.title}-${event.start_ts}-${index}`}
            id={systemChildAnchor('horizon', `${event.title}-${index}`)}
            title={event.title}
            subtitle={formatEventTiming(event.start_ts, event.end_ts, inspect.grounding.now.timezone)}
            trailing={<SystemDocumentStatusChip tone="neutral">event</SystemDocumentStatusChip>}
          />
        ))
      )}
      {people.slice(0, 6).map((person) => (
        <SystemDocumentItem
          key={person.id}
          id={systemChildAnchor('horizon', person.id)}
          title={person.display_name}
          subtitle={person.relationship_context ?? person.id}
          trailing={<SystemDocumentStatusChip tone="neutral">person</SystemDocumentStatusChip>}
        />
      ))}
    </SystemDocumentList>
  );
}

function OperationsActivityDetail({
  providers,
  connectionCount,
}: {
  providers: IntegrationProviderSummary[];
  connectionCount: number;
}) {
  return (
    <div className="space-y-5">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <SystemDocumentSectionLabel>Status and activity</SystemDocumentSectionLabel>
        <SystemDocumentStatusChip tone="neutral">{`${connectionCount} accounts`}</SystemDocumentStatusChip>
      </div>
      <SystemDocumentList>
        {providers.map((provider) => (
          <SystemDocumentItem
            key={provider.key}
            id={systemChildAnchor('activity', provider.key)}
            leading={<ProviderGlyph provider={provider.key} />}
            title={provider.label}
            subtitle={provider.guidance}
            trailing={<SystemDocumentStatusChip tone={stateToneFromStatus(provider.status)}>{provider.status}</SystemDocumentStatusChip>}
          >
            <SystemDocumentStatsGrid className="gap-x-6">
              {provider.meta.map((item) => <SystemDocumentMetaRow key={`${provider.key}-${item.label}`} label={item.label} value={item.value} />)}
            </SystemDocumentStatsGrid>
          </SystemDocumentItem>
        ))}
      </SystemDocumentList>
    </div>
  );
}

function OperationsRecoveryDetail({
  providers,
  blockers,
}: {
  providers: IntegrationProviderSummary[];
  blockers: AgentInspectData['blockers'];
}) {
  return (
    <SystemDocumentList>
        {providers
          .filter((provider) => provider.status !== 'connected')
          .map((provider) => (
            <SystemDocumentItem
              key={provider.key}
              id={systemChildAnchor('recovery', provider.key)}
              leading={<ProviderGlyph provider={provider.key} />}
              title={provider.label}
              subtitle={provider.guidance}
              trailing={<SystemDocumentStatusChip tone={stateToneFromStatus(provider.status)}>{provider.status}</SystemDocumentStatusChip>}
            >
              <>
                {provider.meta.map((item) => <SystemDocumentMetaRow key={`${provider.key}-${item.label}`} label={item.label} value={item.value} />)}
              </>
            </SystemDocumentItem>
          ))}
        {providers.every((provider) => provider.status === 'connected') ? (
          <PanelEmptyRow>No recovery actions are pressing right now.</PanelEmptyRow>
        ) : null}
      <div className="py-3">
        {blockers.length === 0 ? (
          <PanelEmptyRow>No blocker records are active.</PanelEmptyRow>
        ) : (
          blockers.map((blocker) => <SystemDocumentMetaRow key={blocker.code} id={systemChildAnchor('recovery', blocker.code)} label={blocker.code} value={blocker.message} />)
        )}
      </div>
    </SystemDocumentList>
  );
}

function IntegrationsProvidersDetail({
  providers,
  pendingAction,
  onRunIntegrationAction,
}: {
  providers: IntegrationProviderSummary[];
  pendingAction: IntegrationActionId | null;
  onRunIntegrationAction: (actionId: IntegrationActionId) => void | Promise<void>;
}) {
  return (
    <SystemDocumentList>
      {providers.map((provider) => (
        <SystemDocumentItem
          key={provider.key}
          id={systemChildAnchor('providers', provider.key)}
          leading={<ProviderGlyph provider={provider.key} />}
          title={provider.label}
          subtitle={provider.guidance}
          trailing={<SystemDocumentStatusChip tone={providerStatusTone(provider.status)}>{provider.status}</SystemDocumentStatusChip>}
        >
          <>
            {provider.meta.map((item) => <SystemDocumentField key={`${provider.key}-${item.label}`} label={item.label} value={item.value} />)}
            {provider.key === 'google_calendar' || provider.key === 'todoist' ? (
              <div className="flex flex-wrap justify-end gap-2 pt-1">
                <Button
                  variant="secondary"
                  size="sm"
                  loading={pendingAction === (provider.key === 'google_calendar' ? 'google-refresh' : 'todoist-refresh')}
                  onClick={() => void onRunIntegrationAction(provider.key === 'google_calendar' ? 'google-refresh' : 'todoist-refresh')}
                >
                  Refresh
                </Button>
                {provider.connected ? (
                  <Button
                    variant="danger"
                    size="sm"
                    loading={pendingAction === (provider.key === 'google_calendar' ? 'google-disconnect' : 'todoist-disconnect')}
                    onClick={() => void onRunIntegrationAction(provider.key === 'google_calendar' ? 'google-disconnect' : 'todoist-disconnect')}
                  >
                    Disconnect
                  </Button>
                ) : null}
              </div>
            ) : null}
          </>
        </SystemDocumentItem>
      ))}
    </SystemDocumentList>
  );
}

function IntegrationsAccountsDetail({
  connections,
}: {
  connections: IntegrationConnectionData[];
}) {
  return (
    <SystemDocumentList>
      {connections.length === 0 ? (
        <PanelEmptyRow>No integration accounts are connected yet.</PanelEmptyRow>
      ) : connections.map((connection) => (
        <SystemDocumentItem
          key={connection.id}
          id={systemChildAnchor('accounts', connection.id)}
          title={connection.display_name}
          subtitle={connection.provider_key}
          trailing={<SystemDocumentStatusChip tone={providerStatusTone(connection.status)}>{connection.status}</SystemDocumentStatusChip>}
        >
          <>
            <SystemDocumentStatsGrid className="gap-x-6">
              <SystemDocumentField label="Family" value={connection.family} />
              <SystemDocumentField label="Provider" value={connection.provider_key} />
              <SystemDocumentField label="Account ref" value={connection.account_ref ?? 'Unavailable'} />
              <SystemDocumentField label="Updated" value={formatMaybeTimestamp(connection.updated_at)} />
            </SystemDocumentStatsGrid>
            {connection.setting_refs.map((setting) => (
              <SystemDocumentField key={`${setting.setting_key}-${setting.created_at}`} label={setting.setting_key} value={setting.setting_value} />
            ))}
          </>
        </SystemDocumentItem>
      ))}
    </SystemDocumentList>
  );
}

function ControlProjectsDetail({
  projects,
}: {
  projects: AgentInspectData['grounding']['projects'];
}) {
  return (
    <SystemDocumentList>
      {projects.length === 0 ? (
        <PanelEmptyRow>No grounded projects are available.</PanelEmptyRow>
      ) : projects.map((project) => (
        <SystemDocumentItem
          key={project.id}
          id={systemChildAnchor('projects', project.id)}
          title={project.name}
          subtitle={project.slug}
          trailing={<SystemDocumentStatusChip tone={project.status === 'active' ? 'active' : 'neutral'}>{project.status}</SystemDocumentStatusChip>}
        >
          <>
            <SystemDocumentStatsGrid className="gap-x-6">
              <SystemDocumentField label="Slug" value={project.slug} />
              <SystemDocumentField label="Family" value={project.family} />
              <SystemDocumentField label="Primary repo" value={project.primary_repo?.path ?? 'Unavailable'} />
              <SystemDocumentField label="Primary notes" value={project.primary_notes_root?.path ?? 'Unavailable'} />
            </SystemDocumentStatsGrid>
          </>
        </SystemDocumentItem>
      ))}
    </SystemDocumentList>
  );
}

function ControlCapabilitiesDetail({
  capabilityGroups,
  blockers,
}: {
  capabilityGroups: AgentInspectData['capabilities']['groups'];
  blockers: AgentInspectData['blockers'];
}) {
  return (
    <div className="space-y-4">
      <SystemDocumentList>
        {capabilityGroups.length === 0 ? (
          <PanelEmptyRow>No capability groups are exposed yet.</PanelEmptyRow>
        ) : capabilityGroups.map((group) => (
        <SystemDocumentItem
          key={group.kind}
          id={systemChildAnchor('capabilities', group.kind)}
          title={group.label}
          subtitle={group.kind}
          trailing={<SystemDocumentStatusChip tone="neutral">{`${group.entries.length} entries`}</SystemDocumentStatusChip>}
        >
          <SystemDocumentStatsGrid className="gap-x-6">
            {group.entries.map((entry) => (
              <CapabilityRow key={entry.key} entry={entry} />
            ))}
          </SystemDocumentStatsGrid>
        </SystemDocumentItem>
      ))}
      </SystemDocumentList>
      <div className="space-y-2">
        <SystemDocumentSectionLabel>Scope blockers</SystemDocumentSectionLabel>
        {blockers.length === 0 ? (
          <PanelEmptyRow>No blocking scope failures are active.</PanelEmptyRow>
        ) : (
          blockers.map((blocker) => <SystemDocumentMetaRow key={blocker.code} label={blocker.code} value={blocker.message} />)
        )}
      </div>
    </div>
  );
}

function PreferencesAppearanceDetail({
  preferences,
  onToggle,
}: {
  preferences: {
    denseRows: boolean;
    tabularNumbers: boolean;
    reducedMotion: boolean;
    strongFocus: boolean;
    dockedActionBar: boolean;
  };
  onToggle: (key: 'denseRows' | 'tabularNumbers' | 'reducedMotion' | 'strongFocus' | 'dockedActionBar') => void;
}) {
  return (
    <div className="space-y-4">
      <div className="space-y-3">
        <SystemDocumentSectionLabel>Visual and interaction preferences</SystemDocumentSectionLabel>
        <SystemDocumentToggleRow
          id={systemChildAnchor('appearance', 'dense-rows')}
          title="Dense rows"
          detail="Keep rows slightly denser while preserving readability."
          value={preferences.denseRows}
          onToggle={() => onToggle('denseRows')}
        />
        <SystemDocumentToggleRow
          id={systemChildAnchor('appearance', 'tabular-numerals')}
          title="Tabular numerals"
          detail="Use stable numeric alignment for timestamps, counts, durations, and metrics."
          value={preferences.tabularNumbers}
          onToggle={() => onToggle('tabularNumbers')}
        />
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Preview posture</SystemDocumentSectionLabel>
        <SystemDocumentStatsGrid id={systemChildAnchor('appearance', 'preview-posture')} className="gap-x-6">
          <SystemDocumentMetaRow label="Theme temperament" value="warmer industrial" />
          <SystemDocumentMetaRow label="Action bar" value={preferences.dockedActionBar ? 'Docked' : 'Undocked'} />
          <SystemDocumentMetaRow label="Typography" value="Geist / Inter / JetBrains Mono" />
        </SystemDocumentStatsGrid>
      </div>
    </div>
  );
}

function PreferencesAccessibilityDetail({
  preferences,
  settings,
  onToggle,
  onCommitSettingField,
  onToggleSetting,
}: {
  preferences: {
    denseRows: boolean;
    tabularNumbers: boolean;
    reducedMotion: boolean;
    strongFocus: boolean;
    dockedActionBar: boolean;
  };
  settings: SettingsData | null;
  onToggle: (key: 'denseRows' | 'tabularNumbers' | 'reducedMotion' | 'strongFocus' | 'dockedActionBar') => void;
  onCommitSettingField: (key: 'node_display_name' | 'timezone' | 'tailscale_base_url' | 'lan_base_url', value: string) => Promise<void>;
  onToggleSetting: (key: 'writeback_enabled' | 'tailscale_preferred', value: boolean) => Promise<void>;
}) {
  return (
    <div className="space-y-4">
      <div className="space-y-3">
        <SystemDocumentSectionLabel>Accessibility and operator ergonomics</SystemDocumentSectionLabel>
        <SystemDocumentToggleRow
          id={systemChildAnchor('accessibility', 'reduced-motion')}
          title="Reduced motion"
          detail="Suppress non-essential motion while keeping functional transitions."
          value={preferences.reducedMotion}
          onToggle={() => onToggle('reducedMotion')}
        />
        <SystemDocumentToggleRow
          id={systemChildAnchor('accessibility', 'strong-focus-states')}
          title="Strong focus states"
          detail="Keep visible focus treatment high-contrast and persistent."
          value={preferences.strongFocus}
          onToggle={() => onToggle('strongFocus')}
        />
        <SystemDocumentToggleRow
          id={systemChildAnchor('accessibility', 'docked-action-bar')}
          title="Docked action bar"
          detail="Preserve a stable bottom action bar across the surface shell."
          value={preferences.dockedActionBar}
          onToggle={() => onToggle('dockedActionBar')}
        />
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Accessibility law</SystemDocumentSectionLabel>
        <SystemDocumentStatsGrid id={systemChildAnchor('accessibility', 'accessibility-law')} className="gap-x-6">
          <SystemDocumentMetaRow label="Color" value="Never stands alone" />
          <SystemDocumentMetaRow label="Touch targets" value="Minimum enforced" />
          <SystemDocumentMetaRow label="Keyboard" value="First-class navigation" />
        </SystemDocumentStatsGrid>
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Operator settings</SystemDocumentSectionLabel>
        <SystemDocumentList>
          <SystemDocumentItem title="Runtime identity" subtitle="Persisted authority identity and time settings.">
            <>
              <SystemDocumentField
                label="Node display name"
                value={settings?.node_display_name ?? ''}
                onCommit={(value) => onCommitSettingField('node_display_name', value)}
              />
              <SystemDocumentField
                label="Timezone"
                value={settings?.timezone ?? ''}
                onCommit={(value) => onCommitSettingField('timezone', value)}
              />
            </>
          </SystemDocumentItem>
          <SystemDocumentItem title="Network posture" subtitle="Persisted writeback and authority transport preferences.">
            <>
              <SystemDocumentToggleRow
                title="Writeback enabled"
                detail="Allow durable writeback flows where the authority supports them."
                value={settings?.writeback_enabled ?? false}
                onToggle={() => void onToggleSetting('writeback_enabled', !(settings?.writeback_enabled ?? false))}
              />
              <SystemDocumentToggleRow
                title="Prefer Tailscale"
                detail="Prefer the tailnet route when both local and tailnet paths are available."
                value={settings?.tailscale_preferred ?? false}
                onToggle={() => void onToggleSetting('tailscale_preferred', !(settings?.tailscale_preferred ?? false))}
              />
              <SystemDocumentField
                label="Tailscale base URL"
                value={settings?.tailscale_base_url ?? ''}
                onCommit={(value) => onCommitSettingField('tailscale_base_url', value)}
              />
              <SystemDocumentField
                label="LAN base URL"
                value={settings?.lan_base_url ?? ''}
                onCommit={(value) => onCommitSettingField('lan_base_url', value)}
              />
            </>
          </SystemDocumentItem>
        </SystemDocumentList>
      </div>
    </div>
  );
}

function CapabilityRow({ entry }: { entry: AgentCapabilityEntryData }) {
  return (
    <SystemDocumentItem
      title={entry.label}
      subtitle={entry.summary}
      trailing={<SystemDocumentStatusChip tone={entry.available ? 'active' : 'warning'}>{entry.available ? 'available' : 'blocked'}</SystemDocumentStatusChip>}
      className="py-2 first:pt-0 last:pb-0"
    >
      {entry.blocked_reason ? (
        <SystemDocumentMetaRow label="Reason" value={entry.blocked_reason.message} className="border-b-0 py-1" />
      ) : null}
    </SystemDocumentItem>
  );
}

function ProviderGlyph({ provider }: { provider: IntegrationProviderKey }) {
  const statusIcon =
    provider === 'activity' || provider === 'health'
      ? <WarningIcon size={12} />
      : provider === 'google_calendar' || provider === 'todoist'
        ? <SyncIcon size={12} />
        : null;

  return (
    <div
      className={cn(
        'flex h-8 w-8 shrink-0 items-center justify-center rounded-full text-[10px] font-semibold uppercase tracking-[0.16em]',
        providerTintClass(provider),
      )}
      aria-hidden
    >
      {statusIcon ?? (provider === 'google_calendar'
        ? 'G'
        : provider === 'todoist'
          ? 'T'
          : provider.slice(0, 1))}
    </div>
  );
}

function formatMaybeTimestamp(timestamp: number | null): string {
  if (!timestamp) {
    return 'Never';
  }
  try {
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
    }).format(new Date(timestamp * 1000));
  } catch {
    return String(timestamp);
  }
}

function formatEventTiming(startTs: number, endTs: number | null, timezone: string): string {
  try {
    const start = new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
      timeZone: timezone,
    }).format(new Date(startTs * 1000));
    if (!endTs) {
      return start;
    }
    const end = new Intl.DateTimeFormat('en-US', {
      hour: 'numeric',
      minute: '2-digit',
      timeZone: timezone,
    }).format(new Date(endTs * 1000));
    return `${start} → ${end}`;
  } catch {
    return `${startTs}`;
  }
}
