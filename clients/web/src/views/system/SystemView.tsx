import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from 'react';
import { loadAgentInspect } from '../../data/agent-grounding';
import {
  buildCoreSetupStatus,
  disconnectGoogleCalendar,
  disconnectTodoist,
  launchOpenAiOauthProxy,
  loadIntegrationConnections,
  loadIntegrations,
  loadLlmProfileHealth,
  runLlmProfileHandshake,
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
  AgentCapabilityEntryData,
  AgentInspectData,
  IntegrationCalendarData,
  IntegrationConnectionData,
  IntegrationsData,
  LocalIntegrationData,
  SettingsData,
} from '../../types';
import { Button } from '../../core/Button';
import { cn } from '../../core/cn';
import { MarkdownMessage } from '../../core/MarkdownMessage';
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
import systemSurfaceDoc from '../../../../../docs/user/system.md?raw';

export type SystemSectionKey = 'core' | 'overview' | 'operations' | 'integrations' | 'control' | 'preferences';
export type SystemSubsectionKey =
  | 'core_settings'
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
  anchor?: string;
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
    key: 'core',
    label: 'Core',
    items: [
      { key: 'core_settings', label: 'Core settings', description: 'Required identity and setup needed before Vel can operate normally.' },
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
  core_settings: { section: 'core', subsection: 'core_settings' },
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

const DEVELOPER_ONLY_BLOCKER_CODES = new Set([
  'writeback_disabled',
  'no_matching_write_grant',
]);

function visibleSectionOrder(developerMode: boolean) {
  return developerMode ? SECTION_ORDER : SECTION_ORDER.filter((section) => section.key !== 'control');
}

function defaultSubsection(section: SystemSectionKey): SystemSubsectionKey {
  return SECTION_ORDER.find((entry) => entry.key === section)?.items[0]?.key ?? 'core_settings';
}

function visibleBlockers(
  blockers: AgentInspectData['blockers'],
  developerMode: boolean,
) {
  return developerMode
    ? blockers
    : blockers.filter((blocker) => !DEVELOPER_ONLY_BLOCKER_CODES.has(blocker.code));
}

function providerNeedsRecovery(provider: IntegrationProviderSummary): boolean {
  const status = provider.status.toLowerCase();
  return status !== 'connected' && status !== 'configured';
}

const CLIENT_LOCATION_LOOKUP_URL = 'https://nominatim.openstreetmap.org/reverse';

type ReverseGeocodeAddress = {
  city?: unknown;
  town?: unknown;
  village?: unknown;
  hamlet?: unknown;
  county?: unknown;
  state?: unknown;
  state_district?: unknown;
  country?: unknown;
};

type ReverseGeocodeResponse = {
  address?: ReverseGeocodeAddress;
  display_name?: unknown;
};

function expectLocationPart(value: unknown): string | null {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : null;
}

function formatClientLocationLabel(response: ReverseGeocodeResponse): string | null {
  const city =
    expectLocationPart(response.address?.city)
    ?? expectLocationPart(response.address?.town)
    ?? expectLocationPart(response.address?.village)
    ?? expectLocationPart(response.address?.hamlet);
  const region =
    expectLocationPart(response.address?.state)
    ?? expectLocationPart(response.address?.state_district)
    ?? expectLocationPart(response.address?.county);
  const country = expectLocationPart(response.address?.country);

  if (city && region) {
    return `${city}, ${region}`;
  }
  if (city && country) {
    return `${city}, ${country}`;
  }
  if (region && country) {
    return `${region}, ${country}`;
  }

  const fallback = expectLocationPart(response.display_name);
  if (!fallback) {
    return null;
  }
  return fallback.split(',').slice(0, 2).join(', ').trim() || fallback;
}

function inferHostNodeDisplayName(): string | null {
  if (typeof window === 'undefined') {
    return null;
  }
  const hostname = window.location.hostname.trim();
  if (!hostname) {
    return null;
  }
  if (hostname === 'localhost' || hostname === '127.0.0.1' || hostname === '[::1]') {
    return 'Local node';
  }
  const normalized = hostname
    .split('.')[0]
    ?.replace(/[-_]+/g, ' ')
    .trim();
  if (!normalized) {
    return null;
  }
  return normalized.replace(/\b\w/g, (part) => part.toUpperCase());
}

function inferHostTimezone(): string | null {
  try {
    const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
    return timezone?.trim() ? timezone.trim() : null;
  } catch {
    return null;
  }
}

function hasMeaningfulText(value: string | null | undefined): boolean {
  return typeof value === 'string' && value.trim().length > 0;
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

function getBrowserCoordinates(): Promise<{ latitude: number; longitude: number }> {
  if (typeof navigator === 'undefined' || !navigator.geolocation) {
    return Promise.reject(new Error('Browser geolocation is unavailable on this device.'));
  }

  return new Promise((resolve, reject) => {
    navigator.geolocation.getCurrentPosition(
      (position) => {
        resolve({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
        });
      },
      (error) => {
        if (error.code === error.PERMISSION_DENIED) {
          reject(new Error('Location permission was denied.'));
          return;
        }
        if (error.code === error.POSITION_UNAVAILABLE) {
          reject(new Error('Browser geolocation could not determine a position.'));
          return;
        }
        if (error.code === error.TIMEOUT) {
          reject(new Error('Browser geolocation timed out.'));
          return;
        }
        reject(new Error('Browser geolocation failed.'));
      },
      {
        enableHighAccuracy: false,
        timeout: 10000,
        maximumAge: 300000,
      },
    );
  });
}

async function lookupClientLocationLabel(): Promise<string> {
  const { latitude, longitude } = await getBrowserCoordinates();
  const url = new URL(CLIENT_LOCATION_LOOKUP_URL);
  url.searchParams.set('format', 'jsonv2');
  url.searchParams.set('lat', latitude.toString());
  url.searchParams.set('lon', longitude.toString());
  url.searchParams.set('zoom', '10');
  url.searchParams.set('addressdetails', '1');

  const response = await fetch(url.toString(), {
    headers: {
      Accept: 'application/json',
    },
  });
  if (!response.ok) {
    throw new Error('Location lookup service could not resolve this device.');
  }

  const body = await response.json() as ReverseGeocodeResponse;
  const label = formatClientLocationLabel(body);
  if (!label) {
    throw new Error('Location lookup returned no usable place label.');
  }
  return label;
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
    : undefined) ?? 'core';

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

type LlmRoutingProfileSummary = {
  id: string;
  provider: string;
  model: string;
  baseUrl: string;
  contextWindow: number | null;
  enabled: boolean;
  editable: boolean;
  hasApiKey: boolean;
  isDefault: boolean;
  isFallback: boolean;
};

type OpenAiCompatProfileDraft = {
  id: string;
  baseUrl: string;
  model: string;
  contextWindow: string;
  enabled: boolean;
};

type OpenAiApiProfileDraft = {
  id: string;
  baseUrl: string;
  model: string;
  contextWindow: string;
  enabled: boolean;
  apiKeyDraft: string;
  hasApiKey: boolean;
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
          label: 'Sync enabled',
          value: `${integrations.google_calendar.calendars.filter((calendar) => calendar.sync_enabled).length}`,
        },
        {
          label: 'Visible',
          value: `${integrations.google_calendar.calendars.filter((calendar) => calendar.display_enabled).length}`,
        },
        {
          label: 'Events synced',
          value: `${integrations.google_calendar.last_item_count ?? 0}`,
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

function llmRoutingProfiles(settings: SettingsData | null | undefined): LlmRoutingProfileSummary[] {
  const llm = settings?.llm;
  if (!llm) {
    return [];
  }
  return llm.profiles.map((profile) => ({
    id: profile.id,
    provider: profile.provider,
    model: profile.model,
    baseUrl: profile.base_url,
    contextWindow: profile.context_window ?? null,
    enabled: profile.enabled,
    editable: profile.editable,
    hasApiKey: profile.has_api_key ?? false,
    isDefault: llm.default_chat_profile_id === profile.id,
    isFallback: llm.fallback_chat_profile_id === profile.id,
  }));
}

function openAiDraftFromProfile(profile: LlmRoutingProfileSummary): OpenAiCompatProfileDraft {
  return {
    id: profile.id,
    baseUrl: profile.baseUrl,
    model: profile.model,
    contextWindow: profile.contextWindow ? String(profile.contextWindow) : '',
    enabled: profile.enabled,
  };
}

function openAiApiDraftFromProfile(profile: LlmRoutingProfileSummary): OpenAiApiProfileDraft {
  return {
    id: profile.id,
    baseUrl: profile.baseUrl,
    model: profile.model,
    contextWindow: profile.contextWindow ? String(profile.contextWindow) : '',
    enabled: profile.enabled,
    apiKeyDraft: '',
    hasApiKey: profile.hasApiKey,
  };
}

function openAiCompatDraftsEqual(left: OpenAiCompatProfileDraft[], right: OpenAiCompatProfileDraft[]): boolean {
  return JSON.stringify(left) === JSON.stringify(right);
}

function openAiApiDraftsEqual(left: OpenAiApiProfileDraft[], right: OpenAiApiProfileDraft[]): boolean {
  return JSON.stringify(left) === JSON.stringify(right);
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
  const [activeChildAnchor, setActiveChildAnchor] = useState<string | null>(null);
  const [optimisticGoogleCalendar, setOptimisticGoogleCalendar] = useState<IntegrationsData['google_calendar'] | null>(null);
  useEffect(() => {
    const resolved = resolveSystemTarget(target);
    setActiveSection(resolved.section);
    setActiveSubsection(resolved.subsection);
  }, [target?.section, target?.subsection]);

  const jumpToTarget = useCallback((nextTarget: SystemNavigationTarget) => {
    const resolved = resolveSystemTarget(nextTarget);
    setActiveSection(resolved.section);
    setActiveSubsection(resolved.subsection);
    setActiveChildAnchor(nextTarget.anchor ?? null);
    if (typeof window === 'undefined') {
      return;
    }
    const scrollTargetId = nextTarget.anchor ?? resolved.subsection;
    window.requestAnimationFrame(() => {
      window.requestAnimationFrame(() => {
        focusSystemNode(scrollTargetId);
      });
    });
  }, []);

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
  const filteredBlockers = visibleBlockers(inspect?.blockers ?? [], developerMode);
  const subsectionChildren = useMemo<Record<SystemSubsectionKey, SystemSidebarChild[]>>(
    () => ({
      core_settings: [
        { id: systemChildAnchor('core_settings', 'required-setup'), label: 'Required setup' },
        { id: systemChildAnchor('core_settings', 'identity'), label: 'Identity' },
        { id: systemChildAnchor('core_settings', 'agent-profile'), label: 'Agent profile' },
        { id: systemChildAnchor('core_settings', 'runtime'), label: 'Runtime identity' },
        { id: systemChildAnchor('core_settings', 'optional-context'), label: 'Optional context' },
        ...(developerMode ? [{ id: systemChildAnchor('core_settings', 'developer-controls'), label: 'Developer controls' }] : []),
      ],
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
      horizon: [
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
          .filter((provider) => providerNeedsRecovery(provider))
          .map((provider) => ({
            id: systemChildAnchor('recovery', provider.key),
            label: provider.label,
          })),
        ...(filteredBlockers.map((blocker) => ({
          id: systemChildAnchor('recovery', blocker.code),
          label: blocker.code,
        }))),
      ],
      providers: [
        ...providers.map((provider) => ({
          id: systemChildAnchor('providers', provider.key),
          label: provider.label,
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
    [capabilityGroups, connections, developerMode, filteredBlockers, inspect?.grounding.people, llmProfiles, projects, providers],
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
  };
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
    <div className="flex-1 bg-transparent">
      <div className="mx-auto max-w-7xl px-4 py-4 pb-32 sm:px-6">
        <div className="grid gap-5 xl:grid-cols-[16rem_minmax(0,1fr)]">
          <aside className="self-start xl:sticky xl:top-[5.25rem] xl:overflow-visible">
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
              {sectionOrder.flatMap((section) => section.items).map((item) => (
                <section key={item.key} id={item.key} className="scroll-mt-24 border-b border-[var(--vel-color-border)] pb-5 last:border-b-0">
                  <div className="mb-2">
                    <SystemDocumentSectionLabel>{item.label}</SystemDocumentSectionLabel>
                  </div>
                  {renderSystemSubsection({
                    subsection: item.key,
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
  integrations,
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
  onUpdateCoreSettings,
  developerMode,
  onUpdateLlmSettings,
  onPatchGoogleCalendar,
  onPatchTodoist,
  onStartGoogleAuth,
  onJumpToTarget,
}: {
  subsection: SystemSubsectionKey;
  inspect: AgentInspectData;
  providers: IntegrationProviderSummary[];
  integrations: IntegrationsData;
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
  onUpdateCoreSettings: (patch: Record<string, unknown>) => Promise<void>;
  developerMode: boolean;
  onUpdateLlmSettings: (patch: Record<string, unknown>) => Promise<void>;
  onPatchGoogleCalendar: (patch: Record<string, unknown>) => Promise<void>;
  onPatchTodoist: (patch: Record<string, unknown>) => Promise<void>;
  onStartGoogleAuth: () => Promise<void>;
  onJumpToTarget: (target: SystemNavigationTarget) => void;
}) {
  if (subsection === 'core_settings') {
    return (
      <CoreSettingsDetail
        settings={settings}
        integrations={integrations}
        onCommitSettingField={onCommitSettingField}
        onUpdateCoreSettings={onUpdateCoreSettings}
        onJumpToTarget={onJumpToTarget}
      />
    );
  }
  if (subsection === 'trust') {
    return <OverviewTrustDetail inspect={inspect} integrations={providers} blockers={blockers} developerMode={developerMode} />;
  }
  if (subsection === 'horizon') {
    return <OverviewHorizonDetail inspect={inspect} />;
  }
  if (subsection === 'activity') {
    return <OperationsActivityDetail providers={providers} connectionCount={connections.length} />;
  }
  if (subsection === 'recovery') {
    return <OperationsRecoveryDetail providers={providers} blockers={blockers} developerMode={developerMode} />;
  }
  if (subsection === 'providers') {
    return (
      <IntegrationsProvidersDetail
        providers={providers}
        settings={settings}
        integrations={integrations}
        pendingAction={pendingAction}
        onRunIntegrationAction={onRunIntegrationAction}
        onUpdateLlmSettings={onUpdateLlmSettings}
        onPatchGoogleCalendar={onPatchGoogleCalendar}
        onPatchTodoist={onPatchTodoist}
        onStartGoogleAuth={onStartGoogleAuth}
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
        developerMode={developerMode}
      />
    );
  }
  if (subsection === 'appearance') {
    return <PreferencesAppearanceDetail preferences={preferences} onToggle={onTogglePreference} />;
  }
  return (
    <PreferencesAccessibilityDetail
      preferences={preferences}
      onToggle={onTogglePreference}
    />
  );
}

function OverviewTrustDetail({
  inspect,
  integrations,
  blockers,
  developerMode,
}: {
  inspect: AgentInspectData;
  integrations: IntegrationProviderSummary[];
  blockers: AgentInspectData['blockers'];
  developerMode: boolean;
}) {
  const degradedProviders = integrations.filter((provider) => provider.status !== 'connected' && provider.configured);
  const filteredBlockers = visibleBlockers(blockers, developerMode);

  return (
    <div className="space-y-5">
      <SystemDocumentStatsGrid className="gap-x-6">
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'current-mode')} label="Current mode" value={inspect.grounding.current_context?.mode ?? 'Unknown'} />
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'persisted-kinds')} label="Persisted kinds" value={inspect.explainability.persisted_record_kinds.join(', ') || 'None'} />
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'grounded-projects')} label={`Grounded projects`} value={`${inspect.grounding.projects.length}`} />
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'degraded-providers')} label="Degraded providers" value={`${degradedProviders.length}`} />
        {filteredBlockers.length === 0 ? (
          <SystemDocumentMetaRow id={systemChildAnchor('trust', 'health')} label="Health" value="Stable" />
        ) : (
          filteredBlockers.map((blocker) => (
            <SystemDocumentMetaRow key={blocker.code} id={systemChildAnchor('trust', blocker.code)} label={blocker.code} value={blocker.message} />
          ))
        )}
      </SystemDocumentStatsGrid>

      <div id="system-docs" className="scroll-mt-24 rounded-[24px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)] px-4 py-4">
        <SystemDocumentSectionLabel>System documentation</SystemDocumentSectionLabel>
        <div className="mt-3 max-w-3xl text-sm leading-6 text-[var(--vel-color-text)]">
          <MarkdownMessage text={systemSurfaceDoc} />
        </div>
      </div>
    </div>
  );
}

function OverviewHorizonDetail({ inspect }: { inspect: AgentInspectData }) {
  const people = inspect.grounding.people;

  return (
    <SystemDocumentList>
      {people.length === 0 ? (
        <PanelEmptyRow>No grounded people are available right now.</PanelEmptyRow>
      ) : (
        people.slice(0, 6).map((person) => (
          <SystemDocumentItem
            key={person.id}
            id={systemChildAnchor('horizon', person.id)}
            title={person.display_name}
            subtitle={person.relationship_context ?? person.id}
            trailing={<SystemDocumentStatusChip tone="neutral">person</SystemDocumentStatusChip>}
          />
        ))
      )}
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
  developerMode,
}: {
  providers: IntegrationProviderSummary[];
  blockers: AgentInspectData['blockers'];
  developerMode: boolean;
}) {
  const filteredBlockers = visibleBlockers(blockers, developerMode);
  const recoveryProviders = providers.filter((provider) => providerNeedsRecovery(provider));

  return (
    <SystemDocumentList>
        {recoveryProviders.map((provider) => (
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
        {recoveryProviders.length === 0 ? (
          <PanelEmptyRow>No recovery actions are pressing right now.</PanelEmptyRow>
        ) : null}
      <div className="py-3">
        {filteredBlockers.length === 0 ? (
          <PanelEmptyRow>No blocker records are active.</PanelEmptyRow>
        ) : (
          filteredBlockers.map((blocker) => <SystemDocumentMetaRow key={blocker.code} id={systemChildAnchor('recovery', blocker.code)} label={blocker.code} value={blocker.message} />)
        )}
      </div>
    </SystemDocumentList>
  );
}

function IntegrationsProvidersDetail({
  providers,
  settings,
  integrations,
  pendingAction,
  onRunIntegrationAction,
  onUpdateLlmSettings,
  onPatchGoogleCalendar,
  onPatchTodoist,
  onStartGoogleAuth,
}: {
  providers: IntegrationProviderSummary[];
  settings: SettingsData | null;
  integrations: IntegrationsData;
  pendingAction: IntegrationActionId | null;
  onRunIntegrationAction: (actionId: IntegrationActionId) => void | Promise<void>;
  onUpdateLlmSettings: (patch: Record<string, unknown>) => Promise<void>;
  onPatchGoogleCalendar: (patch: Record<string, unknown>) => Promise<void>;
  onPatchTodoist: (patch: Record<string, unknown>) => Promise<void>;
  onStartGoogleAuth: () => Promise<void>;
}) {
  const llmProfiles = useMemo(() => llmRoutingProfiles(settings), [settings?.llm]);
  const activeDefaultId = settings?.llm?.default_chat_profile_id ?? null;
  const activeFallbackId = settings?.llm?.fallback_chat_profile_id ?? null;
  const editableOpenAiCompatProfiles = useMemo(
    () => llmProfiles.filter((profile) => profile.provider === 'openai_oauth' && profile.editable),
    [llmProfiles],
  );
  const editableOpenAiApiProfiles = useMemo(
    () => llmProfiles.filter((profile) => profile.provider === 'openai_api' && profile.editable),
    [llmProfiles],
  );
  const [openAiCompatDrafts, setOpenAiCompatDrafts] = useState<OpenAiCompatProfileDraft[]>(() =>
    editableOpenAiCompatProfiles.map(openAiDraftFromProfile),
  );
  const [openAiApiDrafts, setOpenAiApiDrafts] = useState<OpenAiApiProfileDraft[]>(() =>
    editableOpenAiApiProfiles.map(openAiApiDraftFromProfile),
  );
  const [llmHealthMessages, setLlmHealthMessages] = useState<Record<string, { tone: 'done' | 'warn'; message: string }>>({});
  const [launchingProxyProfileId, setLaunchingProxyProfileId] = useState<string | null>(null);

  useEffect(() => {
    const nextDrafts = editableOpenAiCompatProfiles.map(openAiDraftFromProfile);
    setOpenAiCompatDrafts((current) => (openAiCompatDraftsEqual(current, nextDrafts) ? current : nextDrafts));
  }, [editableOpenAiCompatProfiles]);

  useEffect(() => {
    const nextDrafts = editableOpenAiApiProfiles.map(openAiApiDraftFromProfile);
    setOpenAiApiDrafts((current) => (openAiApiDraftsEqual(current, nextDrafts) ? current : nextDrafts));
  }, [editableOpenAiApiProfiles]);

  async function persistLlmSettings(options?: {
    defaultChatProfileId?: string | null;
    fallbackChatProfileId?: string | null;
    openAiCompatProfiles?: OpenAiCompatProfileDraft[];
    openAiApiProfiles?: OpenAiApiProfileDraft[];
  }) {
    const nextDefaultChatProfileId =
      options?.defaultChatProfileId === undefined
        ? activeDefaultId
        : options.defaultChatProfileId;
    const nextFallbackChatProfileId =
      options?.fallbackChatProfileId === undefined
        ? activeFallbackId
        : options.fallbackChatProfileId;
    const nextOpenAiCompatProfiles = options?.openAiCompatProfiles ?? openAiCompatDrafts;
    const nextOpenAiApiProfiles = options?.openAiApiProfiles ?? openAiApiDrafts;

    await onUpdateLlmSettings({
      default_chat_profile_id: nextDefaultChatProfileId,
      fallback_chat_profile_id:
        nextFallbackChatProfileId && nextFallbackChatProfileId !== nextDefaultChatProfileId
          ? nextFallbackChatProfileId
          : null,
      openai_compat_profiles: nextOpenAiCompatProfiles.map((profile) => ({
        id: profile.id.trim(),
        base_url: profile.baseUrl.trim(),
        model: profile.model.trim(),
        context_window: profile.contextWindow.trim()
          ? Number.parseInt(profile.contextWindow.trim(), 10)
          : null,
        enabled: profile.enabled,
      })),
      openai_api_profiles: nextOpenAiApiProfiles.map((profile) => ({
        id: profile.id.trim(),
        base_url: profile.baseUrl.trim(),
        model: profile.model.trim(),
        context_window: profile.contextWindow.trim()
          ? Number.parseInt(profile.contextWindow.trim(), 10)
          : null,
        enabled: profile.enabled,
        ...(profile.apiKeyDraft.trim() ? { api_key: profile.apiKeyDraft.trim() } : {}),
      })),
    });
  }

  function updateOpenAiCompatDraft(
    profileId: string,
    patch: Partial<OpenAiCompatProfileDraft>,
  ) {
    setOpenAiCompatDrafts((current) =>
      current.map((profile) =>
        profile.id === profileId ? { ...profile, ...patch } : profile,
      ),
    );
  }

  function updateOpenAiApiDraft(
    profileId: string,
    patch: Partial<OpenAiApiProfileDraft>,
  ) {
    setOpenAiApiDrafts((current) =>
      current.map((profile) =>
        profile.id === profileId ? { ...profile, ...patch } : profile,
      ),
    );
  }

  function appendOpenAiCompatDraft() {
    const existingIds = new Set(openAiCompatDrafts.map((profile) => profile.id));
    let index = openAiCompatDrafts.length + 1;
    let id = `oauth-openai-${index}`;
    while (existingIds.has(id)) {
      index += 1;
      id = `oauth-openai-${index}`;
    }
    setOpenAiCompatDrafts((current) => [
      ...current,
      {
        id,
        baseUrl: 'http://127.0.0.1:8014/v1',
        model: 'gpt-5.4',
        contextWindow: '32768',
        enabled: true,
      },
    ]);
  }

  async function saveOpenAiCompatDraft(profileId: string) {
    const nextDrafts = openAiCompatDrafts.map((profile) =>
      profile.id === profileId
        ? {
            ...profile,
            id: profile.id.trim(),
            baseUrl: profile.baseUrl.trim(),
            model: profile.model.trim(),
            contextWindow: profile.contextWindow.trim(),
          }
        : profile,
    );
    setOpenAiCompatDrafts(nextDrafts);
    await persistLlmSettings({ openAiCompatProfiles: nextDrafts });
  }

  async function removeOpenAiCompatDraft(profileId: string) {
    const nextDrafts = openAiCompatDrafts.filter((profile) => profile.id !== profileId);
    setOpenAiCompatDrafts(nextDrafts);
    await persistLlmSettings({
      openAiCompatProfiles: nextDrafts,
      defaultChatProfileId: activeDefaultId === profileId ? null : activeDefaultId,
      fallbackChatProfileId: activeFallbackId === profileId ? null : activeFallbackId,
    });
  }

  function appendOpenAiApiDraft() {
    const existingIds = new Set(openAiApiDrafts.map((profile) => profile.id));
    let index = openAiApiDrafts.length + 1;
    let id = `openai-api-${index}`;
    while (existingIds.has(id)) {
      index += 1;
      id = `openai-api-${index}`;
    }
    setOpenAiApiDrafts((current) => [
      ...current,
      {
        id,
        baseUrl: 'https://api.openai.com/v1',
        model: 'gpt-5.4',
        contextWindow: '32768',
        enabled: true,
        apiKeyDraft: '',
        hasApiKey: false,
      },
    ]);
  }

  async function saveOpenAiApiDraft(profileId: string) {
    const nextDrafts = openAiApiDrafts.map((profile) =>
      profile.id === profileId
        ? {
            ...profile,
            id: profile.id.trim(),
            baseUrl: profile.baseUrl.trim(),
            model: profile.model.trim(),
            contextWindow: profile.contextWindow.trim(),
            hasApiKey: profile.hasApiKey || profile.apiKeyDraft.trim().length > 0,
          }
        : profile,
    );
    setOpenAiApiDrafts(nextDrafts);
    await persistLlmSettings({ openAiApiProfiles: nextDrafts });
    setOpenAiApiDrafts((current) =>
      current.map((profile) =>
        profile.id === profileId ? { ...profile, apiKeyDraft: '', hasApiKey: true } : profile,
      ),
    );
  }

  async function removeOpenAiApiDraft(profileId: string) {
    const nextDrafts = openAiApiDrafts.filter((profile) => profile.id !== profileId);
    setOpenAiApiDrafts(nextDrafts);
    await persistLlmSettings({
      openAiApiProfiles: nextDrafts,
      defaultChatProfileId: activeDefaultId === profileId ? null : activeDefaultId,
      fallbackChatProfileId: activeFallbackId === profileId ? null : activeFallbackId,
    });
  }

  async function runLlmHandshake(profile: LlmRoutingProfileSummary) {
    try {
      let response;
      if (profile.provider === 'openai_oauth') {
        const draft = openAiCompatDrafts.find((candidate) => candidate.id === profile.id);
        response = await runLlmProfileHandshake({
          profile_id: profile.id,
          provider: profile.provider,
          base_url: draft?.baseUrl.trim() ?? profile.baseUrl,
          model: draft?.model.trim() ?? profile.model,
          context_window: draft?.contextWindow.trim()
            ? Number.parseInt(draft.contextWindow.trim(), 10)
            : profile.contextWindow,
        });
      } else if (profile.provider === 'openai_api') {
        const draft = openAiApiDrafts.find((candidate) => candidate.id === profile.id);
        const pendingApiKey = draft?.apiKeyDraft.trim();
        response = pendingApiKey
          ? await runLlmProfileHandshake({
              profile_id: profile.id,
              provider: profile.provider,
              base_url: draft?.baseUrl.trim() ?? profile.baseUrl,
              model: draft?.model.trim() ?? profile.model,
              context_window: draft?.contextWindow.trim()
                ? Number.parseInt(draft.contextWindow.trim(), 10)
                : profile.contextWindow,
              api_key: pendingApiKey,
            })
          : await loadLlmProfileHealth(profile.id);
      } else {
        response = await loadLlmProfileHealth(profile.id);
      }
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Provider handshake failed.');
      }
      const health = response.data;
      setLlmHealthMessages((current) => ({
        ...current,
        [profile.id]: {
          tone: health.healthy ? 'done' : 'warn',
          message: health.message,
        },
      }));
    } catch (error) {
      setLlmHealthMessages((current) => ({
        ...current,
        [profile.id]: {
          tone: 'warn',
          message: error instanceof Error ? error.message : 'Provider handshake failed.',
        },
      }));
    }
  }

  async function runDraftOpenAiCompatHandshake(draft: OpenAiCompatProfileDraft) {
    const profileId = draft.id.trim() || 'draft-openai-proxy';
    try {
      const response = await runLlmProfileHandshake({
        profile_id: profileId,
        provider: 'openai_oauth',
        base_url: draft.baseUrl.trim(),
        model: draft.model.trim(),
        context_window: draft.contextWindow.trim()
          ? Number.parseInt(draft.contextWindow.trim(), 10)
          : null,
      });
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Provider handshake failed.');
      }
      const health = response.data;
      setLlmHealthMessages((current) => ({
        ...current,
        [profileId]: {
          tone: health.healthy ? 'done' : 'warn',
          message: health.message,
        },
      }));
    } catch (error) {
      setLlmHealthMessages((current) => ({
        ...current,
        [profileId]: {
          tone: 'warn',
          message: error instanceof Error ? error.message : 'Provider handshake failed.',
        },
      }));
    }
  }

  async function runDraftOpenAiApiHandshake(draft: OpenAiApiProfileDraft) {
    const profileId = draft.id.trim() || 'draft-openai-api';
    try {
      const pendingApiKey = draft.apiKeyDraft.trim();
      if (!pendingApiKey) {
        throw new Error('Enter an OpenAI API key before running handshake.');
      }
      const response = await runLlmProfileHandshake({
        profile_id: profileId,
        provider: 'openai_api',
        base_url: draft.baseUrl.trim(),
        model: draft.model.trim(),
        context_window: draft.contextWindow.trim()
          ? Number.parseInt(draft.contextWindow.trim(), 10)
          : null,
        api_key: pendingApiKey,
      });
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Provider handshake failed.');
      }
      const health = response.data;
      setLlmHealthMessages((current) => ({
        ...current,
        [profileId]: {
          tone: health.healthy ? 'done' : 'warn',
          message: health.message,
        },
      }));
    } catch (error) {
      setLlmHealthMessages((current) => ({
        ...current,
        [profileId]: {
          tone: 'warn',
          message: error instanceof Error ? error.message : 'Provider handshake failed.',
        },
      }));
    }
  }

  async function launchOpenAiProxy(profileId: string, baseUrl: string) {
    setLaunchingProxyProfileId(profileId);
    try {
      const response = await launchOpenAiOauthProxy({
        profile_id: profileId,
        base_url: baseUrl.trim(),
      });
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'OpenAI OAuth proxy launch failed.');
      }
      const health = response.data;
      setLlmHealthMessages((current) => ({
        ...current,
        [profileId]: {
          tone: health.healthy ? 'done' : 'warn',
          message: health.message,
        },
      }));
    } catch (error) {
      setLlmHealthMessages((current) => ({
        ...current,
        [profileId]: {
          tone: 'warn',
          message: error instanceof Error ? error.message : 'OpenAI OAuth proxy launch failed.',
        },
      }));
    } finally {
      setLaunchingProxyProfileId(null);
    }
  }

  return (
    <SystemDocumentList>
      {providers.map((provider) => {
        const collapseUnavailable = provider.key !== 'google_calendar' && provider.key !== 'todoist' && !provider.configured;
        const providerFields = (
          <>
            {provider.meta.map((item) => <SystemDocumentField key={`${provider.key}-${item.label}`} label={item.label} value={item.value} />)}
            {provider.key === 'google_calendar' ? (
              <>
                <SystemDocumentField
                  label="Replace Google client ID"
                  value=""
                  placeholder={integrations.google_calendar.has_client_id ? 'Saved. Enter a new client ID to replace it.' : 'Enter Google client ID'}
                  onCommit={async (nextValue) => {
                    const trimmed = nextValue.trim();
                    if (!trimmed) {
                      return;
                    }
                    await onPatchGoogleCalendar({ client_id: trimmed });
                  }}
                />
                <SystemDocumentField
                  label="Replace Google client secret"
                  value=""
                  placeholder={integrations.google_calendar.has_client_secret ? 'Saved. Enter a new client secret to replace it.' : 'Enter Google client secret'}
                  onCommit={async (nextValue) => {
                    const trimmed = nextValue.trim();
                    if (!trimmed) {
                      return;
                    }
                    await onPatchGoogleCalendar({ client_secret: trimmed });
                  }}
                />
                <div className="flex flex-wrap justify-end gap-2 pt-1">
                  <Button
                    variant="secondary"
                    size="sm"
                    disabled={!integrations.google_calendar.has_client_id || !integrations.google_calendar.has_client_secret}
                    onClick={() => void onStartGoogleAuth()}
                  >
                    {integrations.google_calendar.connected ? 'Reconnect Google' : 'Connect Google'}
                  </Button>
                  <Button
                    variant="secondary"
                    size="sm"
                    loading={pendingAction === 'google-refresh'}
                    onClick={() => void onRunIntegrationAction('google-refresh')}
                  >
                    Refresh
                  </Button>
                  {provider.connected ? (
                    <Button
                      variant="danger"
                      size="sm"
                      loading={pendingAction === 'google-disconnect'}
                      onClick={() => void onRunIntegrationAction('google-disconnect')}
                    >
                      Disconnect
                    </Button>
                  ) : null}
                </div>
                {integrations.google_calendar.calendars.length > 0 ? (
                  <div className="space-y-3 pt-2">
                    <SystemDocumentSectionLabel>Calendar scope</SystemDocumentSectionLabel>
                    <div className="overflow-hidden rounded-2xl border border-[var(--vel-color-border)]">
                      <table className="w-full table-fixed border-collapse text-left">
                        <thead className="text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]">
                          <tr className="border-b border-[var(--vel-color-border)]">
                            <th className="px-3 py-2 font-medium">Calendar</th>
                            <th className="w-20 px-3 py-2 text-center font-medium">Sync</th>
                            <th className="w-20 px-3 py-2 text-center font-medium">Visible</th>
                          </tr>
                        </thead>
                        <tbody>
                          {integrations.google_calendar.calendars.map((calendar) => (
                            <tr key={calendar.id} className="border-t border-[var(--vel-color-border-subtle)]">
                              <td className="px-3 py-2.5">
                                <div className="min-w-0">
                                  <p className="truncate text-sm font-medium text-[var(--vel-color-text)]">{calendar.summary}</p>
                                  <p className="text-[11px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]">
                                    {calendar.primary ? 'Primary' : 'Available'}
                                  </p>
                                </div>
                              </td>
                              <td className="px-3 py-2.5 text-center">
                                <label className="inline-flex cursor-pointer items-center justify-center rounded-md px-2 py-1">
                                  <input
                                    type="checkbox"
                                    aria-label={`${calendar.summary} sync`}
                                    className="h-4 w-4 cursor-pointer accent-[var(--vel-color-accent-strong)]"
                                    checked={calendar.sync_enabled}
                                    onChange={() => void onPatchGoogleCalendar({
                                      calendar_settings: [
                                        {
                                          id: calendar.id,
                                          sync_enabled: !calendar.sync_enabled,
                                          display_enabled: calendar.sync_enabled ? false : calendar.display_enabled,
                                        },
                                      ],
                                    })}
                                  />
                                </label>
                              </td>
                              <td className="px-3 py-2.5 text-center">
                                <label
                                  className={cn(
                                    'inline-flex items-center justify-center rounded-md px-2 py-1',
                                    calendar.sync_enabled ? 'cursor-pointer' : 'cursor-not-allowed opacity-45',
                                  )}
                                >
                                  <input
                                    type="checkbox"
                                    aria-label={`${calendar.summary} visible`}
                                    className="h-4 w-4 cursor-pointer accent-[var(--vel-color-accent-strong)] disabled:cursor-not-allowed"
                                    checked={calendar.display_enabled}
                                    disabled={!calendar.sync_enabled}
                                    onChange={() => void onPatchGoogleCalendar({
                                      calendar_settings: [
                                        {
                                          id: calendar.id,
                                          display_enabled: !calendar.display_enabled,
                                        },
                                      ],
                                    })}
                                  />
                                </label>
                              </td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </div>
                ) : null}
              </>
            ) : null}
            {provider.key === 'todoist' ? (
              <>
                <SystemDocumentField
                  label="Replace Todoist API token"
                  value=""
                  placeholder={integrations.todoist.has_api_token ? 'Saved. Enter a new API token to replace it.' : 'Enter Todoist API token'}
                  onCommit={async (nextValue) => {
                    const trimmed = nextValue.trim();
                    if (!trimmed) {
                      return;
                    }
                    await onPatchTodoist({ api_token: trimmed });
                  }}
                />
                <div className="space-y-3 pt-2">
                  <SystemDocumentSectionLabel>Todoist writeback capabilities</SystemDocumentSectionLabel>
                  <SystemDocumentToggleRow
                    title="Completion status"
                    detail="Allow Vel to complete and reopen Todoist-backed tasks upstream."
                    value={integrations.todoist.write_capabilities.completion_status}
                    onToggle={() => void onPatchTodoist({
                      write_capabilities: {
                        completion_status: !integrations.todoist.write_capabilities.completion_status,
                      },
                    })}
                  />
                  <SystemDocumentToggleRow
                    title="Due date"
                    detail="Allow Vel to push due-date changes back to Todoist."
                    value={integrations.todoist.write_capabilities.due_date}
                    onToggle={() => void onPatchTodoist({
                      write_capabilities: {
                        due_date: !integrations.todoist.write_capabilities.due_date,
                      },
                    })}
                  />
                  <SystemDocumentToggleRow
                    title="Tags"
                    detail="Allow Vel to write Todoist labels from canonical task tags."
                    value={integrations.todoist.write_capabilities.tags}
                    onToggle={() => void onPatchTodoist({
                      write_capabilities: {
                        tags: !integrations.todoist.write_capabilities.tags,
                      },
                    })}
                  />
                </div>
                <div className="flex flex-wrap justify-end gap-2 pt-1">
                  <Button
                    variant="secondary"
                    size="sm"
                    loading={pendingAction === 'todoist-refresh'}
                    onClick={() => void onRunIntegrationAction('todoist-refresh')}
                  >
                    Refresh
                  </Button>
                  {provider.connected ? (
                    <Button
                      variant="danger"
                      size="sm"
                      loading={pendingAction === 'todoist-disconnect'}
                      onClick={() => void onRunIntegrationAction('todoist-disconnect')}
                    >
                      Disconnect
                    </Button>
                  ) : null}
                </div>
              </>
            ) : null}
          </>
        );

        return (
          <SystemDocumentItem
            key={provider.key}
            id={systemChildAnchor('providers', provider.key)}
            leading={<ProviderGlyph provider={provider.key} />}
            title={provider.label}
            subtitle={provider.guidance}
            trailing={<SystemDocumentStatusChip tone={providerStatusTone(provider.status)}>{provider.status}</SystemDocumentStatusChip>}
          >
            {collapseUnavailable ? (
              <details className="rounded-[18px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)]/55 px-3 py-2">
                <summary className="cursor-pointer list-none text-[11px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">
                  Unavailable on this system
                </summary>
                <div className="pt-2">
                  {providerFields}
                </div>
              </details>
            ) : providerFields}
          </SystemDocumentItem>
        );
      })}
      {settings?.llm ? (
        <SystemDocumentItem
          id={systemChildAnchor('providers', 'llm-routing')}
          title="LLM routing"
          subtitle="Choose the default local/remote chat profile and optional fallback from the discovered model registry."
          trailing={<SystemDocumentStatusChip tone={llmProfiles.some((profile) => profile.enabled) ? 'done' : 'offline'}>{activeDefaultId ? 'configured' : 'not configured'}</SystemDocumentStatusChip>}
        >
          <>
            <p className="text-xs leading-5 text-[var(--vel-color-muted)]">
              Handshake checks use the provider metadata endpoint only. They confirm auth and reachability without sending a completion request.
            </p>
            <SystemDocumentStatsGrid className="gap-x-6">
              <SystemDocumentMetaRow label="Models directory" value={settings.llm.models_dir} />
              <SystemDocumentMetaRow label="Default chat profile" value={activeDefaultId ?? 'Unset'} />
              <SystemDocumentMetaRow label="Fallback profile" value={activeFallbackId ?? 'Unset'} />
            </SystemDocumentStatsGrid>
            {llmProfiles.length === 0 ? (
              <PanelEmptyRow>No model profiles are available yet.</PanelEmptyRow>
            ) : (
              llmProfiles.map((profile) => (
                <SystemDocumentItem
                  key={profile.id}
                  id={systemChildAnchor('providers', `llm-${profile.id}`)}
                  className="ml-3"
                  title={profile.id}
                  subtitle={`${profile.provider} · ${profile.model}`}
                  trailing={<SystemDocumentStatusChip tone={profile.enabled ? 'done' : 'offline'}>{profile.enabled ? 'enabled' : 'disabled'}</SystemDocumentStatusChip>}
                >
                  <>
                    <SystemDocumentStatsGrid className="gap-x-6">
                      <SystemDocumentMetaRow label="Base URL" value={profile.baseUrl} />
                      <SystemDocumentMetaRow label="Editable" value={profile.editable ? 'Yes' : 'No'} />
                      {profile.provider === 'openai_api' ? (
                        <SystemDocumentMetaRow label="API key" value={profile.hasApiKey ? 'Saved' : 'Missing'} />
                      ) : null}
                    </SystemDocumentStatsGrid>
                    {profile.provider === 'openai_oauth' ? (
                      <>
                        {openAiCompatDrafts
                          .filter((draft) => draft.id === profile.id)
                          .map((draft) => (
                            <div key={`${profile.id}-editor`} className="space-y-1 rounded-[18px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)]/40 px-3 py-2">
                              <SystemDocumentField
                                label="Profile ID"
                                value={draft.id}
                                onChange={(value) => updateOpenAiCompatDraft(profile.id, { id: value })}
                              />
                              <SystemDocumentField
                                label="OpenAI-compatible base URL"
                                value={draft.baseUrl}
                                onChange={(value) => updateOpenAiCompatDraft(profile.id, { baseUrl: value })}
                              />
                              <SystemDocumentField
                                label="Model"
                                value={draft.model}
                                onChange={(value) => updateOpenAiCompatDraft(profile.id, { model: value })}
                              />
                              <SystemDocumentField
                                label="Context window"
                                value={draft.contextWindow}
                                onChange={(value) => updateOpenAiCompatDraft(profile.id, { contextWindow: value })}
                              />
                              <SystemDocumentToggleRow
                                title="Enabled"
                                detail="Keep this localhost OpenAI-compatible profile available for routing."
                                value={draft.enabled}
                                onToggle={() => updateOpenAiCompatDraft(profile.id, { enabled: !draft.enabled })}
                              />
                              <div className="flex flex-wrap justify-end gap-2 pt-1">
                                <Button
                                  variant="outline"
                                  size="sm"
                                  loading={launchingProxyProfileId === profile.id}
                                  onClick={() => void launchOpenAiProxy(profile.id, draft.baseUrl)}
                                >
                                  {`Launch ${profile.id}`}
                                </Button>
                                <Button
                                  variant="secondary"
                                  size="sm"
                                  onClick={() => void saveOpenAiCompatDraft(profile.id)}
                                >
                                  {`Save ${profile.id} proxy`}
                                </Button>
                                <Button
                                  variant="danger"
                                  size="sm"
                                  onClick={() => void removeOpenAiCompatDraft(profile.id)}
                                >
                                  Remove
                                </Button>
                              </div>
                              {llmHealthMessages[profile.id] ? (
                                <p className={cn('text-xs leading-5', llmHealthMessages[profile.id]?.tone === 'done' ? 'text-[var(--vel-color-muted)]' : 'text-amber-200')}>
                                  {llmHealthMessages[profile.id]?.message}
                                </p>
                              ) : null}
                            </div>
                          ))}
                      </>
                    ) : null}
                    {profile.provider === 'openai_api' ? (
                      <>
                        {openAiApiDrafts
                          .filter((draft) => draft.id === profile.id)
                          .map((draft) => (
                            <div key={`${profile.id}-editor`} className="space-y-1 rounded-[18px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)]/40 px-3 py-2">
                              <SystemDocumentField
                                label="Profile ID"
                                value={draft.id}
                                onChange={(value) => updateOpenAiApiDraft(profile.id, { id: value })}
                              />
                              <SystemDocumentField
                                label="OpenAI base URL"
                                value={draft.baseUrl}
                                onChange={(value) => updateOpenAiApiDraft(profile.id, { baseUrl: value })}
                              />
                              <SystemDocumentField
                                label="Model"
                                value={draft.model}
                                onChange={(value) => updateOpenAiApiDraft(profile.id, { model: value })}
                              />
                              <SystemDocumentField
                                label="Context window"
                                value={draft.contextWindow}
                                onChange={(value) => updateOpenAiApiDraft(profile.id, { contextWindow: value })}
                              />
                              <SystemDocumentField
                                label="Replace OpenAI API key"
                                value={draft.apiKeyDraft}
                                placeholder={draft.hasApiKey ? 'Saved. Enter a new API key to replace it.' : 'Enter OpenAI API key'}
                                onChange={(value) => updateOpenAiApiDraft(profile.id, { apiKeyDraft: value })}
                              />
                              <SystemDocumentToggleRow
                                title="Enabled"
                                detail="Keep this direct OpenAI profile available for routing."
                                value={draft.enabled}
                                onToggle={() => updateOpenAiApiDraft(profile.id, { enabled: !draft.enabled })}
                              />
                              <div className="flex flex-wrap justify-end gap-2 pt-1">
                                <Button
                                  variant="secondary"
                                  size="sm"
                                  onClick={() => void saveOpenAiApiDraft(profile.id)}
                                >
                                  {`Save ${profile.id} API`}
                                </Button>
                                <Button
                                  variant="danger"
                                  size="sm"
                                  onClick={() => void removeOpenAiApiDraft(profile.id)}
                                >
                                  Remove
                                </Button>
                              </div>
                              {llmHealthMessages[profile.id] ? (
                                <p className={cn('text-xs leading-5', llmHealthMessages[profile.id]?.tone === 'done' ? 'text-[var(--vel-color-muted)]' : 'text-amber-200')}>
                                  {llmHealthMessages[profile.id]?.message}
                                </p>
                              ) : null}
                            </div>
                          ))}
                      </>
                    ) : null}
                    <div className="flex flex-wrap justify-end gap-2 pt-1">
                      {profile.provider === 'openai_oauth' ? (
                        <Button
                          variant="outline"
                          size="sm"
                          loading={launchingProxyProfileId === profile.id}
                          onClick={() => {
                            const draft = openAiCompatDrafts.find((candidate) => candidate.id === profile.id);
                            void launchOpenAiProxy(profile.id, draft?.baseUrl.trim() ?? profile.baseUrl);
                          }}
                        >
                          {`Launch ${profile.id}`}
                        </Button>
                      ) : null}
                      <Button
                        variant="outline"
                        size="sm"
                        disabled={profile.provider === 'openai_api' && !profile.hasApiKey && !openAiApiDrafts.some((draft) => draft.id === profile.id && draft.apiKeyDraft.trim())}
                        onClick={() => void runLlmHandshake(profile)}
                      >
                        {`Handshake ${profile.id}`}
                      </Button>
                      <Button
                        variant={profile.isDefault ? 'success' : 'secondary'}
                        size="sm"
                        disabled={!profile.enabled || profile.isDefault}
                        onClick={() => void persistLlmSettings({
                          defaultChatProfileId: profile.id,
                          fallbackChatProfileId: activeFallbackId === profile.id ? null : activeFallbackId,
                        })}
                      >
                        {profile.isDefault ? 'Default chat' : 'Set default'}
                      </Button>
                      <Button
                        variant={profile.isFallback ? 'success' : 'outline'}
                        size="sm"
                        disabled={!profile.enabled || profile.isFallback || profile.id === activeDefaultId}
                        onClick={() => void persistLlmSettings({ fallbackChatProfileId: profile.id })}
                      >
                        {profile.isFallback ? 'Fallback' : 'Set fallback'}
                      </Button>
                    </div>
                    {!profile.editable && llmHealthMessages[profile.id] ? (
                      <p className={cn('text-xs leading-5', llmHealthMessages[profile.id]?.tone === 'done' ? 'text-[var(--vel-color-muted)]' : 'text-amber-200')}>
                        {llmHealthMessages[profile.id]?.message}
                      </p>
                    ) : null}
                  </>
                </SystemDocumentItem>
              ))
            )}
            {openAiCompatDrafts
              .filter((draft) => !llmProfiles.some((profile) => profile.id === draft.id))
              .map((draft) => (
                <SystemDocumentItem
                  key={`draft-${draft.id}`}
                  className="ml-3"
                  title={draft.id || 'New OpenAI profile'}
                  subtitle="openai_oauth · localhost OpenAI-compatible proxy"
                  trailing={<SystemDocumentStatusChip tone={draft.enabled ? 'done' : 'offline'}>{draft.enabled ? 'enabled' : 'disabled'}</SystemDocumentStatusChip>}
                >
                  <div className="space-y-1 rounded-[18px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)]/40 px-3 py-2">
                    <SystemDocumentField
                      label="Profile ID"
                      value={draft.id}
                      onChange={(value) => updateOpenAiCompatDraft(draft.id, { id: value })}
                    />
                    <SystemDocumentField
                      label="OpenAI-compatible base URL"
                      value={draft.baseUrl}
                      onChange={(value) => updateOpenAiCompatDraft(draft.id, { baseUrl: value })}
                    />
                    <SystemDocumentField
                      label="Model"
                      value={draft.model}
                      onChange={(value) => updateOpenAiCompatDraft(draft.id, { model: value })}
                    />
                    <SystemDocumentField
                      label="Context window"
                      value={draft.contextWindow}
                      onChange={(value) => updateOpenAiCompatDraft(draft.id, { contextWindow: value })}
                    />
                    <SystemDocumentToggleRow
                      title="Enabled"
                      detail="Keep this localhost OpenAI-compatible profile available for routing."
                      value={draft.enabled}
                      onToggle={() => updateOpenAiCompatDraft(draft.id, { enabled: !draft.enabled })}
                    />
                    <div className="flex flex-wrap justify-end gap-2 pt-1">
                      <Button
                        variant="outline"
                        size="sm"
                        loading={launchingProxyProfileId === draft.id}
                        onClick={() => void launchOpenAiProxy(draft.id, draft.baseUrl)}
                      >
                        {`Launch ${draft.id}`}
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => void runDraftOpenAiCompatHandshake(draft)}
                      >
                        {`Handshake ${draft.id}`}
                      </Button>
                      <Button
                        variant="secondary"
                        size="sm"
                        onClick={() => void saveOpenAiCompatDraft(draft.id)}
                      >
                        {`Save ${draft.id} proxy`}
                      </Button>
                      <Button
                        variant="danger"
                        size="sm"
                        onClick={() => void removeOpenAiCompatDraft(draft.id)}
                      >
                        Remove
                      </Button>
                    </div>
                  </div>
                </SystemDocumentItem>
              ))}
            {openAiApiDrafts
              .filter((draft) => !llmProfiles.some((profile) => profile.id === draft.id))
              .map((draft) => (
                <SystemDocumentItem
                  key={`draft-${draft.id}`}
                  className="ml-3"
                  title={draft.id || 'New OpenAI API profile'}
                  subtitle="openai_api · direct OpenAI API"
                  trailing={<SystemDocumentStatusChip tone={draft.enabled ? 'done' : 'offline'}>{draft.enabled ? 'enabled' : 'disabled'}</SystemDocumentStatusChip>}
                >
                  <div className="space-y-1 rounded-[18px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)]/40 px-3 py-2">
                    <SystemDocumentField
                      label="Profile ID"
                      value={draft.id}
                      onChange={(value) => updateOpenAiApiDraft(draft.id, { id: value })}
                    />
                    <SystemDocumentField
                      label="OpenAI base URL"
                      value={draft.baseUrl}
                      onChange={(value) => updateOpenAiApiDraft(draft.id, { baseUrl: value })}
                    />
                    <SystemDocumentField
                      label="Model"
                      value={draft.model}
                      onChange={(value) => updateOpenAiApiDraft(draft.id, { model: value })}
                    />
                    <SystemDocumentField
                      label="Context window"
                      value={draft.contextWindow}
                      onChange={(value) => updateOpenAiApiDraft(draft.id, { contextWindow: value })}
                    />
                    <SystemDocumentField
                      label="Replace OpenAI API key"
                      value={draft.apiKeyDraft}
                      placeholder="Enter OpenAI API key"
                      onChange={(value) => updateOpenAiApiDraft(draft.id, { apiKeyDraft: value })}
                    />
                    <SystemDocumentToggleRow
                      title="Enabled"
                      detail="Keep this direct OpenAI profile available for routing."
                      value={draft.enabled}
                      onToggle={() => updateOpenAiApiDraft(draft.id, { enabled: !draft.enabled })}
                    />
                    <div className="flex flex-wrap justify-end gap-2 pt-1">
                      <Button
                        variant="outline"
                        size="sm"
                        disabled={!draft.apiKeyDraft.trim()}
                        onClick={() => void runDraftOpenAiApiHandshake(draft)}
                      >
                        {`Handshake ${draft.id}`}
                      </Button>
                      <Button
                        variant="secondary"
                        size="sm"
                        onClick={() => void saveOpenAiApiDraft(draft.id)}
                      >
                        {`Save ${draft.id} API`}
                      </Button>
                      <Button
                        variant="danger"
                        size="sm"
                        onClick={() => void removeOpenAiApiDraft(draft.id)}
                      >
                        Remove
                      </Button>
                    </div>
                    {llmHealthMessages[draft.id] ? (
                      <p className={cn('text-xs leading-5', llmHealthMessages[draft.id]?.tone === 'done' ? 'text-[var(--vel-color-muted)]' : 'text-amber-200')}>
                        {llmHealthMessages[draft.id]?.message}
                      </p>
                    ) : null}
                  </div>
                </SystemDocumentItem>
              ))}
            <div className="flex flex-wrap justify-end gap-2 pt-2">
              <Button
                variant="outline"
                size="sm"
                onClick={appendOpenAiCompatDraft}
              >
                Add OpenAI proxy
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={appendOpenAiApiDraft}
              >
                Add OpenAI API
              </Button>
            </div>
          </>
        </SystemDocumentItem>
      ) : null}
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
  developerMode,
}: {
  capabilityGroups: AgentInspectData['capabilities']['groups'];
  blockers: AgentInspectData['blockers'];
  developerMode: boolean;
}) {
  const filteredBlockers = visibleBlockers(blockers, developerMode);

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
        {filteredBlockers.length === 0 ? (
          <PanelEmptyRow>No blocking scope failures are active.</PanelEmptyRow>
        ) : (
          filteredBlockers.map((blocker) => <SystemDocumentMetaRow key={blocker.code} label={blocker.code} value={blocker.message} />)
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

function RequiredSetupRow({
  label,
  detail,
  ready,
  action,
}: {
  label: string;
  detail: string;
  ready: boolean;
  action?: ReactNode;
}) {
  return (
    <div className="flex flex-wrap items-start justify-between gap-3 border-b border-[var(--vel-color-border)] py-2 last:border-b-0">
      <div className="min-w-0 flex-1">
        <p className="text-sm font-medium leading-5 text-[var(--vel-color-text)]">{label}</p>
        <p className="text-xs leading-5 text-[var(--vel-color-muted)]">{detail}</p>
      </div>
      <div className="flex shrink-0 items-center gap-2">
        <SystemDocumentStatusChip tone={ready ? 'done' : 'warning'}>
          {ready ? 'ready' : 'required'}
        </SystemDocumentStatusChip>
        {action}
      </div>
    </div>
  );
}

function CoreSettingsDetail({
  settings,
  integrations,
  onCommitSettingField,
  onUpdateCoreSettings,
  onJumpToTarget,
}: {
  settings: SettingsData | null;
  integrations: IntegrationsData;
  onCommitSettingField: (key: 'node_display_name' | 'timezone' | 'tailscale_base_url' | 'lan_base_url', value: string) => Promise<void>;
  onUpdateCoreSettings: (patch: Record<string, unknown>) => Promise<void>;
  onJumpToTarget: (target: SystemNavigationTarget) => void;
}) {
  const coreSettings = settings?.core_settings;
  const developerMode = coreSettings?.developer_mode ?? false;
  const [autoLocationState, setAutoLocationState] = useState<'idle' | 'saving' | 'done' | 'error'>('idle');
  const [autoLocationMessage, setAutoLocationMessage] = useState<string | null>(null);
  const inferredNodeName = useMemo(() => inferHostNodeDisplayName(), []);
  const inferredTimezone = useMemo(() => inferHostTimezone(), []);
  const nodeInferenceCommitted = useRef(false);
  const timezoneInferenceCommitted = useRef(false);
  const coreSetupStatus = useMemo(
    () => buildCoreSetupStatus(settings, integrations),
    [integrations, settings],
  );
  const hasConfiguredLlm = Boolean(
    settings?.llm?.default_chat_profile_id
    && settings.llm.profiles.some(
      (profile) => profile.enabled && profile.id === settings.llm?.default_chat_profile_id,
    ),
  );
  const hasSyncedProvider = Boolean(
    integrations.google_calendar.configured || integrations.todoist.configured,
  );
  const hasAgentProfile = Boolean(
    hasMeaningfulText(coreSettings?.agent_profile?.role)
    || hasMeaningfulText(coreSettings?.agent_profile?.preferences)
    || hasMeaningfulText(coreSettings?.agent_profile?.constraints)
    || hasMeaningfulText(coreSettings?.agent_profile?.freeform),
  );

  useEffect(() => {
    if (hasMeaningfulText(settings?.node_display_name) || !inferredNodeName || nodeInferenceCommitted.current) {
      return;
    }
    nodeInferenceCommitted.current = true;
    void onCommitSettingField('node_display_name', inferredNodeName);
  }, [inferredNodeName, onCommitSettingField, settings?.node_display_name]);

  useEffect(() => {
    if (hasMeaningfulText(settings?.timezone) || !inferredTimezone || timezoneInferenceCommitted.current) {
      return;
    }
    timezoneInferenceCommitted.current = true;
    void onCommitSettingField('timezone', inferredTimezone);
  }, [inferredTimezone, onCommitSettingField, settings?.timezone]);

  async function autoSetClientLocation() {
    setAutoLocationState('saving');
    setAutoLocationMessage('Resolving browser location…');
    try {
      const label = await lookupClientLocationLabel();
      await onUpdateCoreSettings({ client_location_label: label });
      setAutoLocationState('done');
      setAutoLocationMessage(`Updated from browser location: ${label}`);
    } catch (error) {
      setAutoLocationState('error');
      setAutoLocationMessage(error instanceof Error ? error.message : 'Location lookup failed.');
    }
  }

  return (
    <div className="space-y-4">
      <div className="space-y-3">
        <SystemDocumentSectionLabel>Core settings</SystemDocumentSectionLabel>
        <SystemDocumentList>
          <SystemDocumentItem
            id={systemChildAnchor('core_settings', 'required-setup')}
            title="Required setup"
            subtitle="Vel stays partially disabled until every required Core item is saved."
            trailing={<SystemDocumentStatusChip tone={coreSetupStatus.ready ? 'done' : 'warning'}>{coreSetupStatus.ready ? 'ready' : 'blocked'}</SystemDocumentStatusChip>}
          >
            <>
              <p className="rounded-[18px] border border-amber-500/30 bg-amber-950/30 px-3 py-2 text-sm leading-6 text-amber-100">
                Vel will not be fully functional until required Core settings are submitted. Required items are marked below, and host details are auto-inferred when possible.
              </p>
              <div className="space-y-0.5 rounded-[18px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)]/35 px-3 py-1.5">
                <RequiredSetupRow
                  label="Your name"
                  detail="Used in operator-facing setup, nudges, and proof flows."
                  ready={hasMeaningfulText(coreSettings?.user_display_name)}
                />
                <RequiredSetupRow
                  label="Node name"
                  detail={inferredNodeName ? `Auto-inferred from this host as ${inferredNodeName}.` : 'Required so Vel can identify this authority node clearly.'}
                  ready={hasMeaningfulText(settings?.node_display_name)}
                />
                <RequiredSetupRow
                  label="Agent profile"
                  detail="At least one role, preference, constraint, or freeform note is required."
                  ready={hasAgentProfile}
                />
                <RequiredSetupRow
                  label="LLM integration"
                  detail="A default enabled chat profile is required before the composer can work."
                  ready={hasConfiguredLlm}
                  action={(
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => onJumpToTarget({ section: 'integrations', subsection: 'providers', anchor: systemChildAnchor('providers', 'llm-routing') })}
                    >
                      Open LLM routing
                    </Button>
                  )}
                />
                <RequiredSetupRow
                  label="Synced provider"
                  detail="Connect at least Google Calendar or Todoist so Now has grounded external truth."
                  ready={hasSyncedProvider}
                  action={(
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => onJumpToTarget({ section: 'integrations', subsection: 'providers', anchor: systemChildAnchor('providers', 'google_calendar') })}
                    >
                      Open integrations
                    </Button>
                  )}
                />
              </div>
            </>
          </SystemDocumentItem>

          <SystemDocumentItem
            id={systemChildAnchor('core_settings', 'identity')}
            title="Identity"
            subtitle="Required identity fields for a usable single-node Vel."
          >
            <>
              <SystemDocumentField
                label="Your name *"
                fieldId="core-settings-user-display-name"
                value={coreSettings?.user_display_name ?? ''}
                placeholder="Required before Vel can operate normally"
                onCommit={(value) => onUpdateCoreSettings({ user_display_name: value })}
              />
              <SystemDocumentField
                label="Node name *"
                fieldId="core-settings-node-display-name"
                value={settings?.node_display_name ?? ''}
                placeholder={inferredNodeName ?? 'Required host label'}
                onCommit={(value) => onCommitSettingField('node_display_name', value)}
              />
              <SystemDocumentField
                label="Timezone"
                fieldId="core-settings-timezone"
                value={settings?.timezone ?? ''}
                placeholder={inferredTimezone ?? 'Auto-inferred from host when available'}
                onCommit={(value) => onCommitSettingField('timezone', value)}
              />
            </>
          </SystemDocumentItem>

          <SystemDocumentItem
            id={systemChildAnchor('core_settings', 'agent-profile')}
            title="Agent profile"
            subtitle="Required. Fill at least one field so Vel knows how to work with you."
          >
            <>
              <SystemDocumentField
                label="Agent role"
                fieldId="core-settings-agent-profile-role"
                value={coreSettings?.agent_profile?.role ?? ''}
                placeholder="Required somewhere in this section"
                onCommit={(value) => onUpdateCoreSettings({ agent_profile: { role: value } })}
              />
              <SystemDocumentField
                label="Working preferences"
                value={coreSettings?.agent_profile?.preferences ?? ''}
                onCommit={(value) => onUpdateCoreSettings({ agent_profile: { preferences: value } })}
              />
              <SystemDocumentField
                label="Constraints"
                value={coreSettings?.agent_profile?.constraints ?? ''}
                onCommit={(value) => onUpdateCoreSettings({ agent_profile: { constraints: value } })}
              />
              <SystemDocumentField
                label="What Vel should know about you *"
                fieldId="core-settings-agent-profile-freeform"
                value={coreSettings?.agent_profile?.freeform ?? ''}
                multiline
                placeholder="What should every provider know about you by default?"
                onCommit={(value) => onUpdateCoreSettings({ agent_profile: { freeform: value } })}
              />
            </>
          </SystemDocumentItem>

          <SystemDocumentItem
            id={systemChildAnchor('core_settings', 'optional-context')}
            title="Optional host context"
            subtitle="Helpful setup Vel can infer or enrich from this device."
          >
            <>
              <SystemDocumentField
                label="Client location"
                value={coreSettings?.client_location_label ?? ''}
                onCommit={(value) => onUpdateCoreSettings({ client_location_label: value })}
              />
              <div className="flex flex-wrap items-center gap-2 border-b border-[var(--vel-color-border)] py-1.5">
                <Button
                  variant="outline"
                  size="sm"
                  loading={autoLocationState === 'saving'}
                  aria-label="Auto-set client location"
                  onClick={() => {
                    void autoSetClientLocation();
                  }}
                >
                  Auto-set
                </Button>
                <span className="text-xs leading-5 text-[var(--vel-color-muted)]">
                  Use browser permission and OpenStreetMap reverse geocoding to fill this field.
                </span>
                {autoLocationMessage ? (
                  <p
                    className={cn(
                      'w-full text-xs leading-5',
                      autoLocationState === 'error' ? 'text-amber-200' : 'text-[var(--vel-color-muted)]',
                    )}
                  >
                    {autoLocationMessage}
                  </p>
                ) : null}
              </div>
            </>
          </SystemDocumentItem>

          <SystemDocumentItem
            id={systemChildAnchor('core_settings', 'runtime')}
            title="Runtime identity"
            subtitle="Authority transport preferences and host routing."
          >
            <>
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

          <SystemDocumentItem
            id={systemChildAnchor('core_settings', 'developer-controls')}
            title="Developer controls"
            subtitle="Only needed when you want to inspect or override MVP setup behavior."
          >
            <>
              <SystemDocumentToggleRow
                title="Developer mode"
                detail="Reveal deeper runtime controls and setup overrides that are not needed for normal MVP operation."
                value={developerMode}
                onToggle={() => void onUpdateCoreSettings({ developer_mode: !developerMode })}
              />
              {developerMode ? (
                <SystemDocumentToggleRow
                  title="Bypass setup gate"
                  detail="Allow the composer and task bar before minimum Core setup is complete."
                  value={coreSettings?.bypass_setup_gate ?? false}
                  onToggle={() => void onUpdateCoreSettings({ bypass_setup_gate: !(coreSettings?.bypass_setup_gate ?? false) })}
                />
              ) : null}
            </>
          </SystemDocumentItem>
        </SystemDocumentList>
      </div>
    </div>
  );
}

function PreferencesAccessibilityDetail({
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
