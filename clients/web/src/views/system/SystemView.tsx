import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { loadAgentInspect } from '../../data/agent-grounding';
import {
  disconnectGoogleCalendar,
  disconnectTodoist,
  loadIntegrationConnections,
  loadIntegrations,
  loadSettings,
  operatorQueryKeys,
  startGoogleCalendarAuth,
  syncSource,
  updateGoogleCalendarIntegration,
  updateSettings,
  updateTodoistIntegration,
  updateWebSettings,
} from '../../data/operator';
import { contextQueryKeys } from '../../data/context';
import { invalidateQuery, setQueryData, useQuery } from '../../data/query';
import type {
  AgentInspectData,
  IntegrationCalendarData,
  IntegrationConnectionData,
  IntegrationsData,
  SemanticAliasOverridesData,
  SettingsData,
} from '../../types';
import { SettingsIcon } from '../../core/Icons';
import {
  SystemDocumentSectionLabel,
} from '../../core/SystemDocument';
import { SurfaceState } from '../../core/SurfaceState';
import { uiFonts } from '../../core/Theme';
import {
  buildGroupedSystemNav,
  buildSystemGroupSummaries,
  buildSystemSubsectionChildren,
  resolveNavigableSystemAnchor,
  type SystemSidebarChild,
} from './SystemNavigationModel';
import {
  SystemDocumentationDetail,
} from './SystemOverviewSections';
import { renderSystemSubsection } from './SystemSectionContent';
import { SystemSidebarNav } from './SystemSidebarNav';
import {
  SystemAnchorStrip,
  SystemGroupCard,
  SystemSubsectionHero,
} from './SystemSurfaceChrome';
import { SystemSurfaceLayout } from './SystemSurfaceLayout';
import {
  llmRoutingProfiles,
  providerSummaries,
  type IntegrationActionId,
  type IntegrationProviderSummary,
} from './SystemProvidersSection';
import {
  defaultSubsectionForSystemSection as defaultSubsection,
  groupForSystemSubsection as groupForSubsection,
  resolveSystemTarget,
  SYSTEM_DOCUMENTATION_ANCHOR,
  SYSTEM_GROUP_ORDER,
  SYSTEM_SECTION_BY_SUBSECTION as SECTION_BY_SUBSECTION,
  SYSTEM_SECTION_ORDER as SECTION_ORDER,
  type SystemGroupKey,
  type SystemNavigationTarget,
  type SystemSectionKey,
  type SystemSubsectionKey,
} from './systemNavigation';
import systemSurfaceDoc from '../../../../../docs/user/system.md?raw';

interface SystemViewProps {
  target?: SystemNavigationTarget;
}

const DEVELOPER_ONLY_BLOCKER_CODES = new Set([
  'writeback_disabled',
  'no_matching_write_grant',
]);

function visibleBlockers(
  blockers: AgentInspectData['blockers'],
  developerMode: boolean,
) {
  return developerMode
    ? blockers
    : blockers.filter((blocker) => !DEVELOPER_ONLY_BLOCKER_CODES.has(blocker.code));
}

function focusSystemNode(anchor: string) {
  const node = document.getElementById(anchor);
  if (!(node instanceof HTMLElement)) {
    return;
  }
  node.scrollIntoView({ block: 'start', behavior: 'smooth' });
  const focusTarget =
    node instanceof HTMLInputElement
    || node instanceof HTMLTextAreaElement
    || node instanceof HTMLButtonElement
      ? node
      : node.querySelector<HTMLElement>('input, textarea, button, [tabindex]:not([tabindex="-1"])');
  focusTarget?.focus({ preventScroll: true });
}

function scheduleFocusSystemNode(anchor: string) {
  if (typeof window === 'undefined') {
    return;
  }
  window.requestAnimationFrame(() => {
    window.requestAnimationFrame(() => {
      focusSystemNode(anchor);
    });
  });
}

function applyReturnedSettings(settingsKey: readonly (string | number | boolean | null | undefined)[], response: {
  ok: boolean;
  data?: SettingsData | null;
}) {
  if (response.ok && response.data) {
    setQueryData<SettingsData | null>(settingsKey, response.data);
    return;
  }
  invalidateQuery(settingsKey, { refetch: true });
}

type GoogleCalendarSettingsPatch = {
  id: string;
  sync_enabled?: boolean;
  display_enabled?: boolean;
};

type GoogleCalendarPatchRequest = {
  calendar_settings?: GoogleCalendarSettingsPatch[];
};

function applyGoogleCalendarSettingsPatch(
  calendars: IntegrationCalendarData[],
  patches: GoogleCalendarSettingsPatch[],
): IntegrationCalendarData[] {
  const patchesById = new Map(patches.map((patch) => [patch.id, patch]));
  return calendars.map((calendar) => {
    const patch = patchesById.get(calendar.id);
    if (!patch) {
      return calendar;
    }
    const syncEnabled = patch.sync_enabled ?? calendar.sync_enabled;
    const displayEnabled = syncEnabled && (patch.display_enabled ?? calendar.display_enabled);
    return {
      ...calendar,
      sync_enabled: syncEnabled,
      display_enabled: displayEnabled,
    };
  });
}

function applyOptimisticGoogleCalendarPatch(
  current: IntegrationsData | null | undefined,
  patch: GoogleCalendarPatchRequest,
): IntegrationsData | null | undefined {
  if (!current || !patch.calendar_settings) {
    return current;
  }
  const calendars = applyGoogleCalendarSettingsPatch(
    current.google_calendar.calendars,
    patch.calendar_settings,
  );
  return {
    ...current,
    google_calendar: {
      ...current.google_calendar,
      calendars,
      all_calendars_selected: calendars.every((calendar) => calendar.sync_enabled),
    },
  };
}

function googleCalendarPatchSatisfied(
  current: IntegrationsData['google_calendar'] | null | undefined,
  patch: GoogleCalendarPatchRequest,
): boolean {
  if (!current || !patch.calendar_settings) {
    return true;
  }
  const calendarsById = new Map(current.calendars.map((calendar) => [calendar.id, calendar]));
  return patch.calendar_settings.every((calendarPatch) => {
    const calendar = calendarsById.get(calendarPatch.id);
    if (!calendar) {
      return false;
    }
    if (calendarPatch.sync_enabled !== undefined && calendar.sync_enabled !== calendarPatch.sync_enabled) {
      return false;
    }
    if (
      calendarPatch.display_enabled !== undefined
      && calendar.display_enabled !== (calendar.sync_enabled && calendarPatch.display_enabled)
    ) {
      return false;
    }
    return true;
  });
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
  const [activeChildAnchor, setActiveChildAnchor] = useState<string | null>(target?.anchor ?? null);
  const [optimisticGoogleCalendar, setOptimisticGoogleCalendar] = useState<IntegrationsData['google_calendar'] | null>(null);

  const renderedIntegrations = useMemo(() => {
    if (!integrations || !optimisticGoogleCalendar) {
      return integrations;
    }
    return {
      ...integrations,
      google_calendar: optimisticGoogleCalendar,
    };
  }, [integrations, optimisticGoogleCalendar]);

  useEffect(() => {
    if (!optimisticGoogleCalendar || !integrations) {
      return;
    }
    if (
      integrations.google_calendar.calendars === optimisticGoogleCalendar.calendars
      && integrations.google_calendar.all_calendars_selected === optimisticGoogleCalendar.all_calendars_selected
    ) {
      setOptimisticGoogleCalendar(null);
    }
  }, [integrations, optimisticGoogleCalendar]);

  const providers = renderedIntegrations ? providerSummaries(renderedIntegrations) : [];
  const llmProfiles = llmRoutingProfiles(settings);
  const projects = inspect?.grounding.projects ?? [];
  const capabilityGroups = inspect?.capabilities.groups ?? [];
  const developerMode = settings?.core_settings?.developer_mode ?? false;
  const coreSetupStatus = buildCoreSetupStatus(settings ?? null, renderedIntegrations);
  const filteredBlockers = visibleBlockers(inspect?.blockers ?? [], developerMode);
  const subsectionChildren = useMemo<Record<SystemSubsectionKey, SystemSidebarChild[]>>(
    () => buildSystemSubsectionChildren({
      coreSetupReady: coreSetupStatus.ready,
      developerMode,
      filteredBlockers,
      people: inspect?.grounding.people ?? [],
      providers,
      llmProfiles,
      connections,
      projects,
      capabilityGroups,
    }),
    [capabilityGroups, connections, coreSetupStatus.ready, developerMode, filteredBlockers, inspect?.grounding.people, llmProfiles, projects, providers],
  );

  useEffect(() => {
    const resolved = resolveSystemTarget(target);
    setActiveSection(resolved.section);
    setActiveSubsection(resolved.subsection);
    setActiveChildAnchor(target?.anchor ?? null);
    scheduleFocusSystemNode(target?.anchor ?? resolved.subsection);
  }, [target?.anchor, target?.section, target?.subsection]);

  const jumpToTarget = useCallback((nextTarget: SystemNavigationTarget) => {
    const resolved = resolveSystemTarget(nextTarget);
    setActiveSection(resolved.section);
    setActiveSubsection(resolved.subsection);
    setActiveChildAnchor(nextTarget.anchor ?? null);
    scheduleFocusSystemNode(nextTarget.anchor ?? resolved.subsection);
  }, []);

  useEffect(() => {
    const resolvedAnchor = resolveNavigableSystemAnchor(
      activeSubsection,
      activeChildAnchor,
      subsectionChildren,
    );
    if (resolvedAnchor === activeChildAnchor) {
      return;
    }
    setActiveChildAnchor(resolvedAnchor);
    if (resolvedAnchor) {
      scheduleFocusSystemNode(resolvedAnchor);
    }
  }, [activeChildAnchor, activeSubsection, subsectionChildren]);

  useEffect(() => {
    if (typeof window === 'undefined' || typeof IntersectionObserver === 'undefined') return;
    const childNodes = (subsectionChildren[activeSubsection] ?? [])
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
  }, [activeSubsection, subsectionChildren]);

  useEffect(() => {
    if (developerMode || activeSection !== 'control') {
      return;
    }
    setActiveSection('overview');
    setActiveSubsection(defaultSubsection('overview'));
    setActiveChildAnchor(null);
  }, [activeSection, developerMode]);

  if (inspectLoading || integrationsLoading || connectionsLoading || settingsLoading) {
    return <SurfaceState message="Loading canonical system state…" layout="centered" />;
  }

  const error = inspectError ?? integrationsError ?? connectionsError ?? settingsError;
  if (error) {
    return <SurfaceState message={error} layout="centered" tone="warning" />;
  }

  if (!inspect || !renderedIntegrations) {
    return <SurfaceState message="No canonical system state is available yet." layout="centered" />;
  }
  const preferences = {
    denseRows: settings?.web_settings?.dense_rows ?? true,
    tabularNumbers: settings?.web_settings?.tabular_numbers ?? true,
    reducedMotion: settings?.web_settings?.reduced_motion ?? false,
    strongFocus: settings?.web_settings?.strong_focus ?? true,
    dockedActionBar: settings?.web_settings?.docked_action_bar ?? true,
    semanticAliases: settings?.web_settings?.semantic_aliases ?? {},
  };
  const { sectionOrder, groupedNav } = buildGroupedSystemNav({
    developerMode,
    sidebarFilter,
    subsectionChildren,
  });
  const subsectionMeta = sectionOrder.flatMap((section) => section.items).find((item) => item.key === activeSubsection)
    ?? sectionOrder[0]?.items[0]
    ?? SECTION_ORDER[0].items[0];
  const activeGroup = groupForSubsection(activeSubsection);
  const activeChildren = subsectionChildren[activeSubsection] ?? [];
  const groupSummaries: Record<SystemGroupKey, string> = buildSystemGroupSummaries({
    filteredBlockers,
    projectsCount: projects.length,
    providersCount: providers.length,
    connectionsCount: connections.length,
  });
  const activeSectionKey = SECTION_BY_SUBSECTION.get(subsectionMeta.key) ?? activeSection;
  const activeSectionMeta = sectionOrder.find((section) => section.key === activeSectionKey)
    ?? sectionOrder[0]
    ?? SECTION_ORDER[0];
  const activeGroupMeta = SYSTEM_GROUP_ORDER.find((group) => group.key === activeGroup) ?? SYSTEM_GROUP_ORDER[0];
  const activeSubsectionContent = renderSystemSubsection({
    subsection: subsectionMeta.key,
    inspect,
    providers,
    integrations: renderedIntegrations,
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
      applyReturnedSettings(settingsKey, response);
    },
    onUpdateSemanticAliases: async (aliases) => {
      const response = await updateWebSettings({
        semantic_aliases: aliases,
      });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update semantic alias settings');
      }
      applyReturnedSettings(settingsKey, response);
    },
    onCommitSettingField: async (key, value) => {
      const response = await updateSettings({ [key]: value });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update settings');
      }
      applyReturnedSettings(settingsKey, response);
    },
    onUpdateCoreSettings: async (patch) => {
      const response = await updateSettings({ core_settings: patch });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update core settings');
      }
      applyReturnedSettings(settingsKey, response);
    },
    developerMode,
    onUpdateLlmSettings: async (patch) => {
      const response = await updateSettings({ llm: patch });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update LLM settings');
      }
      applyReturnedSettings(settingsKey, response);
    },
    onPatchGoogleCalendar: async (patch) => {
      const previousIntegrations = integrations;
      const optimisticIntegrations = applyOptimisticGoogleCalendarPatch(
        renderedIntegrations,
        patch as GoogleCalendarPatchRequest,
      );
      setOptimisticGoogleCalendar(optimisticIntegrations?.google_calendar ?? null);
      setQueryData<IntegrationsData | null>(
        integrationsKey,
        (current) => applyOptimisticGoogleCalendarPatch(current, patch as GoogleCalendarPatchRequest) ?? current,
      );
      try {
        const response = await updateGoogleCalendarIntegration(patch);
        if (!response.ok) {
          throw new Error(response.error?.message ?? 'Failed to update Google Calendar settings');
        }
        const reconciledIntegrations = applyOptimisticGoogleCalendarPatch(
          response.data ?? null,
          patch as GoogleCalendarPatchRequest,
        ) ?? response.data ?? null;
        setOptimisticGoogleCalendar(
          googleCalendarPatchSatisfied(response.data?.google_calendar, patch as GoogleCalendarPatchRequest)
            ? null
            : reconciledIntegrations?.google_calendar ?? null,
        );
        setQueryData(integrationsKey, reconciledIntegrations);
        invalidateQuery(connectionsKey, { refetch: true });
        invalidateQuery(nowKey, { refetch: true });
      } catch (error) {
        setOptimisticGoogleCalendar(null);
        if (previousIntegrations) {
          setQueryData(integrationsKey, previousIntegrations);
        } else {
          invalidateQuery(integrationsKey, { refetch: true });
        }
        throw error;
      }
    },
    onPatchTodoist: async (patch) => {
      const response = await updateTodoistIntegration(patch);
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update Todoist settings');
      }
      invalidateQuery(integrationsKey, { refetch: true });
      invalidateQuery(connectionsKey, { refetch: true });
    },
    onStartGoogleAuth: async () => {
      const response = await startGoogleCalendarAuth();
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to start Google Calendar auth');
      }
      if (typeof window !== 'undefined' && typeof window.open === 'function') {
        window.open(response.data.auth_url, '_blank', 'noopener,noreferrer');
      }
      setActionMessage('Google Calendar auth opened in a new window.');
    },
    onJumpToTarget: jumpToTarget,
  });

  function selectSubsection(subsection: SystemSubsectionKey, anchor?: string | null) {
    const nextSection = SECTION_BY_SUBSECTION.get(subsection) ?? activeSection;
    const resolvedAnchor = resolveNavigableSystemAnchor(subsection, anchor ?? null, subsectionChildren);
    setActiveSection(nextSection);
    setActiveSubsection(subsection);
    setActiveChildAnchor(resolvedAnchor);
    scheduleFocusSystemNode(resolvedAnchor ?? subsection);
  }

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
    <SystemSurfaceLayout
      header={(
        <div className="border-b border-[var(--vel-color-border)] pb-3">
          <p className={`${uiFonts.display} inline-flex items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-muted)]`}>
            <SettingsIcon size={12} />
            <span>SYSTEM</span>
          </p>
        </div>
      )}
      groupCards={(
        <div className="grid gap-3 lg:grid-cols-3">
          {SYSTEM_GROUP_ORDER.map((group) => {
            const firstItem = groupedNav.find((entry) => entry.key === group.key)?.items[0];
            return (
              <SystemGroupCard
                key={group.key}
                title={group.label}
                description={group.description}
                summary={groupSummaries[group.key]}
                tone={group.key}
                active={group.key === activeGroup}
                onClick={firstItem ? () => selectSubsection(firstItem.key) : undefined}
              />
            );
          })}
        </div>
      )}
      sidebar={(
        <SystemSidebarNav
          sidebarFilter={sidebarFilter}
          onSidebarFilterChange={setSidebarFilter}
          groupedNav={groupedNav}
          activeSubsection={activeSubsection}
          activeChildAnchor={activeChildAnchor}
          subsectionChildren={subsectionChildren}
          onSelectSubsection={selectSubsection}
        />
      )}
      content={(
        <>
          <SystemSubsectionHero
            eyebrow={`${activeGroupMeta.label} / ${activeSectionMeta.label}`}
            title={subsectionMeta.label}
            description={subsectionMeta.description}
          />
          <SystemAnchorStrip
            items={activeChildren}
            activeId={activeChildAnchor}
            onSelect={(anchor) => selectSubsection(subsectionMeta.key, anchor)}
          />
          {actionMessage ? (
            <div className="rounded-[18px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.025)] px-4 py-3 text-[13px] leading-5 text-[var(--vel-color-text)]">
              {actionMessage}
            </div>
          ) : null}

          <section
            id={subsectionMeta.key}
            className="scroll-mt-24 rounded-[24px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-5 py-5"
          >
            {activeSubsectionContent}
          </section>
          <section id={SYSTEM_DOCUMENTATION_ANCHOR} className="scroll-mt-24 rounded-[24px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-5 py-5">
            <div className="mb-3">
              <SystemDocumentSectionLabel>Documentation</SystemDocumentSectionLabel>
            </div>
            <SystemDocumentationDetail doc={systemSurfaceDoc} />
          </section>
        </>
      )}
    />
  );
}
