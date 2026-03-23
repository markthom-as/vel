import { useEffect, useMemo, useState } from 'react';
import { loadAgentInspect } from '../../data/agent-grounding';
import {
  disconnectGoogleCalendar,
  disconnectTodoist,
  loadIntegrationConnections,
  loadIntegrations,
  operatorQueryKeys,
  syncSource,
} from '../../data/operator';
import { invalidateQuery, useQuery } from '../../data/query';
import { contextQueryKeys } from '../../data/context';
import type {
  AgentCapabilityEntryData,
  AgentInspectData,
  IntegrationConnectionData,
  IntegrationsData,
  LocalIntegrationData,
} from '../../types';
import { Button } from '../../core/Button';
import {
  PanelDenseRow,
  PanelEyebrow,
  PanelEmptyRow,
  PanelInsetCard,
  PanelIntroStrip,
  PanelKeyValueRow,
  PanelPageSection,
  PanelSectionHeader,
} from '../../core/PanelChrome';
import { PanelMetaPill } from '../../core/PanelItem';
import { SurfaceState } from '../../core/SurfaceState';

export type SystemSectionKey = 'domain' | 'capabilities' | 'configuration';
export type SystemSubsectionKey =
  | 'people'
  | 'calendar'
  | 'knowledge'
  | 'tools'
  | 'workflows'
  | 'templates'
  | 'modules'
  | 'integrations'
  | 'accounts'
  | 'scopes';

export interface SystemNavigationTarget {
  section?: SystemSectionKey;
  subsection?: SystemSubsectionKey;
}

interface SystemViewProps {
  target?: SystemNavigationTarget;
}

type IntegrationActionId = 'google-disconnect' | 'google-refresh' | 'todoist-disconnect' | 'todoist-refresh';

const SECTION_ORDER: Array<{
  key: SystemSectionKey;
  label: string;
  items: Array<{ key: SystemSubsectionKey; label: string; description: string }>;
}> = [
  {
    key: 'domain',
    label: 'Domain',
    items: [
      { key: 'people', label: 'People', description: 'Canonical people grounded right now.' },
      { key: 'calendar', label: 'Calendar', description: 'Thin event truth without merged priority.' },
      { key: 'knowledge', label: 'Knowledge', description: 'Context and explainability paths only.' },
    ],
  },
  {
    key: 'capabilities',
    label: 'Capabilities',
    items: [
      { key: 'tools', label: 'Tools', description: 'Read-only capability exposure.' },
      { key: 'workflows', label: 'Workflows', description: 'Workflow posture without product widening.' },
      { key: 'templates', label: 'Templates', description: 'List-only until canonical apply exists.' },
    ],
  },
  {
    key: 'configuration',
    label: 'Configuration',
    items: [
      { key: 'modules', label: 'Modules', description: 'Module registry posture, read-first.' },
      { key: 'integrations', label: 'Integrations', description: 'Named canonical actions only.' },
      { key: 'accounts', label: 'Accounts', description: 'Canonical account and connection truth.' },
      { key: 'scopes', label: 'Scopes', description: 'Read-only scope posture unless canonical actions exist.' },
    ],
  },
];

const SECTION_BY_SUBSECTION = new Map<SystemSubsectionKey, SystemSectionKey>(
  SECTION_ORDER.flatMap((section) => section.items.map((item) => [item.key, section.key] as const)),
);

function defaultSubsection(section: SystemSectionKey): SystemSubsectionKey {
  return SECTION_ORDER.find((entry) => entry.key === section)?.items[0]?.key ?? 'people';
}

function resolveSystemTarget(target?: SystemNavigationTarget): {
  section: SystemSectionKey;
  subsection: SystemSubsectionKey;
} {
  const fallbackSection = target?.section ?? (target?.subsection ? SECTION_BY_SUBSECTION.get(target.subsection) : undefined) ?? 'domain';
  const fallbackSubsection = target?.subsection ?? defaultSubsection(fallbackSection);
  return {
    section: fallbackSection,
    subsection: fallbackSubsection,
  };
}

export function SystemView({ target }: SystemViewProps) {
  const inspectKey = useMemo(() => operatorQueryKeys.agentInspect(), []);
  const integrationsKey = useMemo(() => operatorQueryKeys.integrations(), []);
  const connectionsKey = useMemo(() => operatorQueryKeys.integrationConnections(), []);
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

  const initialTarget = resolveSystemTarget(target);
  const [activeSection, setActiveSection] = useState<SystemSectionKey>(initialTarget.section);
  const [activeSubsection, setActiveSubsection] = useState<SystemSubsectionKey>(initialTarget.subsection);
  const [pendingAction, setPendingAction] = useState<IntegrationActionId | null>(null);
  const [actionMessage, setActionMessage] = useState<string | null>(null);

  useEffect(() => {
    const resolved = resolveSystemTarget(target);
    setActiveSection(resolved.section);
    setActiveSubsection(resolved.subsection);
  }, [target?.section, target?.subsection]);

  if (inspectLoading || integrationsLoading || connectionsLoading) {
    return <SurfaceState message="Loading canonical system state…" layout="centered" />;
  }

  const error = inspectError ?? integrationsError ?? connectionsError;
  if (error) {
    return <SurfaceState message={error} layout="centered" tone="warning" />;
  }

  if (!inspect || !integrations) {
    return <SurfaceState message="No canonical system state is available yet." layout="centered" />;
  }

  const people = inspect.grounding.people;
  const projects = inspect.grounding.projects;
  const upcomingEvents = inspect.grounding.now.schedule?.upcoming_events ?? [];
  const capabilityGroups = inspect.capabilities.groups;
  const activeSectionConfig = SECTION_ORDER.find((section) => section.key === activeSection) ?? SECTION_ORDER[0];
  const activeSubsectionConfig = activeSectionConfig.items.find((item) => item.key === activeSubsection) ?? activeSectionConfig.items[0];

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
    <div className="flex-1 overflow-y-auto bg-zinc-950">
      <div className="mx-auto max-w-7xl px-6 py-8 pb-36">
        <header className="mb-8">
          <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">System</p>
          <h1 className="mt-2 text-3xl font-semibold text-zinc-100">Canonical object and capability truth</h1>
          <p className="mt-2 max-w-3xl text-sm leading-6 text-zinc-400">
            Browse structural state directly from canonical backend outputs. This surface stays read-heavy by default
            and only exposes named backend actions where the contract is already stable.
          </p>
        </header>

        <PanelIntroStrip>
          <div className="flex flex-wrap items-center gap-2">
            <PanelMetaPill tone="kind">Single surface</PanelMetaPill>
            <PanelMetaPill tone="state">Backend-owned truth</PanelMetaPill>
            <PanelMetaPill tone="state">No client simulation</PanelMetaPill>
          </div>
          <p className="mt-3 text-sm leading-6 text-zinc-300">
            `/system` remains one route with stable internal sections. Browse broadly, invoke narrowly, and do not
            infer capabilities from UI mood.
          </p>
        </PanelIntroStrip>

        {actionMessage ? (
          <div className="mb-4 rounded-xl border border-zinc-800 bg-zinc-900/70 px-4 py-3 text-sm text-zinc-300">
            {actionMessage}
          </div>
        ) : null}

        <div className="grid gap-6 xl:grid-cols-[18rem_minmax(0,1fr)]">
          <PanelPageSection className="h-fit xl:sticky xl:top-0">
            <PanelSectionHeader
              title="Structure"
              description="Browse one subsection at a time so detail stays legible instead of flattening into admin soup."
            />
            <div className="mt-5 space-y-5">
              {SECTION_ORDER.map((section) => (
                <div key={section.key}>
                  <PanelEyebrow tracking="wide">{section.label}</PanelEyebrow>
                  <div className="mt-2 space-y-1.5">
                    {section.items.map((item) => {
                      const active = activeSubsection === item.key;
                      return (
                        <button
                          key={item.key}
                          type="button"
                          onClick={() => {
                            setActiveSection(section.key);
                            setActiveSubsection(item.key);
                          }}
                          className={`w-full rounded-2xl border px-3 py-3 text-left transition ${
                            active
                              ? 'border-[#ff6b00]/60 bg-[#2d1608] text-[#ffd4b8]'
                              : 'border-zinc-800 bg-zinc-950/70 text-zinc-300 hover:border-zinc-700 hover:text-zinc-100'
                          }`}
                          aria-pressed={active}
                        >
                          <div className="flex items-start justify-between gap-3">
                            <div className="min-w-0">
                              <p className="text-sm font-medium">{item.label}</p>
                              <p className={`mt-1 text-xs leading-5 ${active ? 'text-[#ffb784]' : 'text-zinc-500'}`}>
                                {item.description}
                              </p>
                            </div>
                            {active ? <PanelMetaPill tone="state">active</PanelMetaPill> : null}
                          </div>
                        </button>
                      );
                    })}
                  </div>
                </div>
              ))}
            </div>
          </PanelPageSection>

          <PanelPageSection className="min-w-0">
            <div className="border-b border-zinc-900 pb-4">
              <PanelEyebrow tracking="wide">{activeSectionConfig.label}</PanelEyebrow>
              <div className="mt-3 flex flex-wrap items-start justify-between gap-3">
                <div className="min-w-0">
                  <h2 className="text-2xl font-semibold tracking-tight text-zinc-100">{activeSubsectionConfig.label}</h2>
                  <p className="mt-2 max-w-3xl text-sm leading-6 text-zinc-400">{activeSubsectionConfig.description}</p>
                </div>
                <PanelMetaPill tone="kind">read-first</PanelMetaPill>
              </div>
            </div>

            <div className="mt-5">
              {activeSubsection === 'people' ? (
                <div className="space-y-2">
                  {people.length === 0 ? (
                    <PanelEmptyRow>No canonical people are grounded yet.</PanelEmptyRow>
                  ) : (
                    people.slice(0, 8).map((person) => (
                      <PanelDenseRow key={person.id}>
                        <div className="flex items-center justify-between gap-3">
                          <div>
                            <p className="text-sm font-medium text-zinc-100">{person.display_name}</p>
                            <p className="mt-1 text-xs text-zinc-500">{person.relationship_context ?? person.id}</p>
                          </div>
                          <PanelMetaPill tone="state">{person.aliases.length} aliases</PanelMetaPill>
                        </div>
                      </PanelDenseRow>
                    ))
                  )}
                </div>
              ) : null}

              {activeSubsection === 'calendar' ? (
                <div className="space-y-2">
                  {upcomingEvents.length === 0 ? (
                    <PanelEmptyRow>No canonical upcoming events are grounded right now.</PanelEmptyRow>
                  ) : (
                    upcomingEvents.slice(0, 6).map((event, index) => (
                      <PanelDenseRow key={`${event.title}-${event.start_ts}-${index}`}>
                        <div className="flex items-center justify-between gap-3">
                          <div>
                            <p className="text-sm font-medium text-zinc-100">{event.title}</p>
                            <p className="mt-1 text-xs text-zinc-500">
                              {formatEventTiming(event.start_ts, event.end_ts, inspect.grounding.now.timezone)}
                            </p>
                          </div>
                          <PanelMetaPill tone="kind">event</PanelMetaPill>
                        </div>
                      </PanelDenseRow>
                    ))
                  )}
                </div>
              ) : null}

              {activeSubsection === 'knowledge' ? (
                <div className="grid gap-4 lg:grid-cols-[1fr_1fr]">
                  <PanelInsetCard>
                    <PanelEyebrow tracking="wide">Context</PanelEyebrow>
                    <div className="mt-3 space-y-2">
                      <PanelKeyValueRow
                        label="Current context"
                        value={inspect.grounding.current_context?.current_context_path ?? 'Unavailable'}
                      />
                      <PanelKeyValueRow
                        label="Explain context"
                        value={inspect.grounding.current_context?.explain_context_path ?? 'Unavailable'}
                      />
                      <PanelKeyValueRow
                        label="Explain drift"
                        value={inspect.grounding.current_context?.explain_drift_path ?? 'Unavailable'}
                      />
                    </div>
                  </PanelInsetCard>
                  <PanelInsetCard>
                    <PanelEyebrow tracking="wide">Support Paths</PanelEyebrow>
                    <div className="mt-3 space-y-2 text-xs text-zinc-400">
                      {inspect.explainability.supporting_paths.length === 0 ? (
                        <PanelEmptyRow>No supporting paths were exposed in the inspect payload.</PanelEmptyRow>
                      ) : (
                        inspect.explainability.supporting_paths.slice(0, 8).map((path) => (
                          <p key={path} className="break-all rounded-lg border border-zinc-800 bg-zinc-950/70 px-3 py-2">
                            {path}
                          </p>
                        ))
                      )}
                      <p className="pt-2 text-zinc-500">{projects.length} grounded projects remain available through canonical context.</p>
                    </div>
                  </PanelInsetCard>
                </div>
              ) : null}

              {activeSubsection === 'tools' ? (
                <div className="space-y-3">
                  {capabilityGroups.length === 0 ? (
                    <PanelEmptyRow>No capability groups are exposed yet.</PanelEmptyRow>
                  ) : (
                    capabilityGroups.map((group) => (
                      <PanelInsetCard key={group.kind}>
                        <PanelEyebrow tracking="wide">{group.label}</PanelEyebrow>
                        <div className="mt-3 space-y-2">
                          {group.entries.map((entry) => (
                            <CapabilityRow key={entry.key} entry={entry} />
                          ))}
                        </div>
                      </PanelInsetCard>
                    ))
                  )}
                </div>
              ) : null}

              {activeSubsection === 'workflows' ? (
                <PanelEmptyRow>
                  No standalone workflow catalog or builder is exposed here in `v0.5.2`. Invocation stays narrow and backend-owned.
                </PanelEmptyRow>
              ) : null}

              {activeSubsection === 'templates' ? (
                <PanelEmptyRow>
                  No canonical template apply surface is exposed yet, so templates remain read-only in this milestone.
                </PanelEmptyRow>
              ) : null}

              {activeSubsection === 'modules' ? (
                <PanelEmptyRow>
                  Activation and registration stay backend-governed. No additional client-side module actions are inferred.
                </PanelEmptyRow>
              ) : null}

              {activeSubsection === 'integrations' ? (
                <div className="space-y-3">
                  <IntegrationCard
                    title="Google Calendar"
                    status={integrations.google_calendar.connected ? 'connected' : integrations.google_calendar.configured ? 'configured' : 'not configured'}
                    guidance={integrations.google_calendar.guidance?.detail ?? integrations.google_calendar.last_error}
                    meta={[
                      { label: 'Selected calendars', value: `${integrations.google_calendar.calendars.filter((calendar) => calendar.selected).length}` },
                      { label: 'Last sync', value: formatMaybeTimestamp(integrations.google_calendar.last_sync_at) },
                    ]}
                    actions={[
                      {
                        label: 'Refresh',
                        visible: true,
                        pending: pendingAction === 'google-refresh',
                        onClick: () => void runIntegrationAction('google-refresh'),
                      },
                      {
                        label: 'Disconnect',
                        visible: integrations.google_calendar.connected,
                        pending: pendingAction === 'google-disconnect',
                        onClick: () => void runIntegrationAction('google-disconnect'),
                      },
                    ]}
                  />
                  <IntegrationCard
                    title="Todoist"
                    status={integrations.todoist.connected ? 'connected' : integrations.todoist.configured ? 'configured' : 'not configured'}
                    guidance={integrations.todoist.guidance?.detail ?? integrations.todoist.last_error}
                    meta={[
                      { label: 'Last item count', value: `${integrations.todoist.last_item_count ?? 0}` },
                      { label: 'Last sync', value: formatMaybeTimestamp(integrations.todoist.last_sync_at) },
                    ]}
                    actions={[
                      {
                        label: 'Refresh',
                        visible: true,
                        pending: pendingAction === 'todoist-refresh',
                        onClick: () => void runIntegrationAction('todoist-refresh'),
                      },
                      {
                        label: 'Disconnect',
                        visible: integrations.todoist.connected,
                        pending: pendingAction === 'todoist-disconnect',
                        onClick: () => void runIntegrationAction('todoist-disconnect'),
                      },
                    ]}
                  />
                  {localIntegrationCards(integrations).map((integration) => (
                    <IntegrationCard
                      key={integration.title}
                      title={integration.title}
                      status={integration.status}
                      guidance={integration.guidance}
                      meta={integration.meta}
                      actions={[]}
                    />
                  ))}
                </div>
              ) : null}

              {activeSubsection === 'accounts' ? (
                <div className="space-y-2">
                  {connections.length === 0 ? (
                    <PanelEmptyRow>No integration accounts are connected yet.</PanelEmptyRow>
                  ) : (
                    connections.map((connection) => (
                      <PanelDenseRow key={connection.id}>
                        <div className="flex items-center justify-between gap-3">
                          <div className="min-w-0">
                            <p className="truncate text-sm font-medium text-zinc-100">{connection.display_name}</p>
                            <p className="mt-1 text-xs text-zinc-500">
                              {connection.family} · {connection.provider_key} · {connection.account_ref ?? connection.id}
                            </p>
                          </div>
                          <PanelMetaPill tone="state">{connection.status}</PanelMetaPill>
                        </div>
                      </PanelDenseRow>
                    ))
                  )}
                </div>
              ) : null}

              {activeSubsection === 'scopes' ? (
                <div className="space-y-3">
                  <PanelInsetCard>
                    <PanelEyebrow tracking="wide">Capability Scope</PanelEyebrow>
                    <div className="mt-3 space-y-2">
                      {inspect.blockers.length === 0 ? (
                        <PanelEmptyRow>No blocking scope failures are active.</PanelEmptyRow>
                      ) : (
                        inspect.blockers.map((blocker) => (
                          <PanelKeyValueRow key={blocker.code} label={blocker.code} value={blocker.message} />
                        ))
                      )}
                    </div>
                  </PanelInsetCard>
                </div>
              ) : null}
            </div>
          </PanelPageSection>
        </div>
      </div>
    </div>
  );
}

function CapabilityRow({ entry }: { entry: AgentCapabilityEntryData }) {
  return (
    <PanelDenseRow>
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <p className="text-sm font-medium text-zinc-100">{entry.label}</p>
          <p className="mt-1 text-xs leading-5 text-zinc-500">{entry.summary}</p>
        </div>
        <PanelMetaPill tone={entry.available ? 'state' : 'kind'}>
          {entry.available ? 'available' : 'blocked'}
        </PanelMetaPill>
      </div>
      {entry.blocked_reason ? (
        <p className="mt-2 text-xs text-amber-200">{entry.blocked_reason.message}</p>
      ) : null}
    </PanelDenseRow>
  );
}

function IntegrationCard({
  title,
  status,
  guidance,
  meta,
  actions,
}: {
  title: string;
  status: string;
  guidance: string | null | undefined;
  meta: Array<{ label: string; value: string }>;
  actions: Array<{ label: string; visible: boolean; pending: boolean; onClick: () => void }>;
}) {
  return (
    <PanelInsetCard>
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <p className="text-sm font-medium text-zinc-100">{title}</p>
          <p className="mt-1 text-xs text-zinc-500">{guidance ?? 'No additional guidance recorded.'}</p>
        </div>
        <PanelMetaPill tone="state">{status}</PanelMetaPill>
      </div>
      <div className="mt-3 grid gap-2 md:grid-cols-2">
        {meta.map((item) => (
          <PanelKeyValueRow key={item.label} label={item.label} value={item.value} />
        ))}
      </div>
      {actions.some((action) => action.visible) ? (
        <div className="mt-4 flex flex-wrap gap-2">
          {actions.filter((action) => action.visible).map((action) => (
            <Button key={action.label} variant="secondary" size="sm" disabled={action.pending} onClick={action.onClick}>
              {action.pending ? `${action.label}…` : action.label}
            </Button>
          ))}
        </div>
      ) : null}
    </PanelInsetCard>
  );
}

function localIntegrationCards(integrations: IntegrationsData) {
  const locals: Array<{ title: string; data: LocalIntegrationData }> = [
    { title: 'Activity', data: integrations.activity },
    { title: 'Health', data: integrations.health },
    { title: 'Git', data: integrations.git },
    { title: 'Messaging', data: integrations.messaging },
    { title: 'Reminders', data: integrations.reminders },
    { title: 'Notes', data: integrations.notes },
    { title: 'Transcripts', data: integrations.transcripts },
  ];

  return locals.map(({ title, data }) => ({
    title,
    status: data.configured ? 'configured' : 'not configured',
    guidance: data.guidance?.detail ?? data.last_error,
    meta: [
      { label: 'Source', value: data.source_path ?? 'Unset' },
      { label: 'Last sync', value: formatMaybeTimestamp(data.last_sync_at) },
    ],
  }));
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
