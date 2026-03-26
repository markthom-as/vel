import { useEffect, useMemo, useState } from 'react';
import {
  launchOpenAiOauthProxy,
  loadLlmProfileHealth,
  runLlmProfileHandshake,
} from '../../data/operator';
import type {
  IntegrationsData,
  LocalIntegrationData,
  SettingsData,
} from '../../types';
import { Button } from '../../core/Button';
import { cn } from '../../core/cn';
import { PanelEmptyRow } from '../../core/PanelChrome';
import {
  SystemDocumentField,
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentMetaRow,
  SystemDocumentSectionLabel,
  SystemDocumentStatsGrid,
  SystemDocumentStatusChip,
  SystemDocumentToggleRow,
} from '../../core/SystemDocument';
import {
  resolveProviderSemantic,
  resolveProviderStatusSemantic,
} from '../../core/Theme/semanticRegistry';
import { formatMaybeTimestamp, ProviderGlyph } from './SystemSupportSections';
import { systemChildAnchor } from './systemNavigation';

export type IntegrationActionId = 'google-disconnect' | 'google-refresh' | 'todoist-disconnect' | 'todoist-refresh';

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

export type IntegrationProviderSummary = {
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

export function providerSummaries(integrations: IntegrationsData): IntegrationProviderSummary[] {
  const locals: Array<{ key: Exclude<IntegrationProviderKey, 'google_calendar' | 'todoist'>; data: LocalIntegrationData }> = [
    { key: 'activity', data: integrations.activity },
    { key: 'health', data: integrations.health },
    { key: 'git', data: integrations.git },
    { key: 'messaging', data: integrations.messaging },
    { key: 'reminders', data: integrations.reminders },
    { key: 'notes', data: integrations.notes },
    { key: 'transcripts', data: integrations.transcripts },
  ];

  return [
    {
      key: 'google_calendar',
      label: resolveProviderSemantic('google_calendar').label,
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
      label: resolveProviderSemantic('todoist').label,
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
    ...locals.map(({ key, data }) => ({
      key,
      label: resolveProviderSemantic(key).label,
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

export function llmRoutingProfiles(settings: SettingsData | null | undefined): LlmRoutingProfileSummary[] {
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

function ProviderSnapshot({
  provider,
}: {
  provider: IntegrationProviderSummary;
}) {
  return (
    <SystemDocumentStatsGrid className="gap-x-6">
      <SystemDocumentMetaRow label="Status" value={provider.status} />
      <SystemDocumentMetaRow label="Configured" value={provider.configured ? 'Yes' : 'No'} />
      <SystemDocumentMetaRow label="Last sync" value={formatMaybeTimestamp(provider.lastSyncAt)} />
      <SystemDocumentMetaRow label="Details" value={provider.meta[0]?.value ?? 'Unavailable'} />
    </SystemDocumentStatsGrid>
  );
}

export function IntegrationsProvidersDetail({
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
  const connectedProviders = useMemo(
    () => providers.filter((provider) => provider.status === 'connected').length,
    [providers],
  );
  const configuredProviders = useMemo(
    () => providers.filter((provider) => provider.configured).length,
    [providers],
  );
  const localOnlyProviders = useMemo(
    () => providers.filter((provider) => provider.key !== 'google_calendar' && provider.key !== 'todoist').length,
    [providers],
  );
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
      <SystemDocumentItem
        id={systemChildAnchor('providers', 'provider-summary')}
        title="Integration posture"
        subtitle="Scan configured providers, live connections, and routing inventory before editing individual records."
        trailing={<SystemDocumentStatusChip tone="neutral">{`${providers.length} providers`}</SystemDocumentStatusChip>}
      >
        <SystemDocumentStatsGrid className="gap-x-6">
          <SystemDocumentMetaRow label="Configured" value={`${configuredProviders}`} />
          <SystemDocumentMetaRow label="Connected" value={`${connectedProviders}`} />
          <SystemDocumentMetaRow label="Local sources" value={`${localOnlyProviders}`} />
          <SystemDocumentMetaRow label="LLM profiles" value={`${llmProfiles.length}`} />
        </SystemDocumentStatsGrid>
      </SystemDocumentItem>
      {providers.map((provider) => {
        const collapseUnavailable = provider.key !== 'google_calendar' && provider.key !== 'todoist' && !provider.configured;
        const providerFields = (
          <>
            <ProviderSnapshot provider={provider} />
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
            title={resolveProviderSemantic(provider.key).label}
            subtitle={`${provider.guidance}${provider.lastSyncAt ? ` · Last sync ${formatMaybeTimestamp(provider.lastSyncAt)}` : ''}`}
            trailing={<SystemDocumentStatusChip tone={resolveProviderStatusSemantic(provider.status).tone}>{resolveProviderStatusSemantic(provider.status).label}</SystemDocumentStatusChip>}
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
          trailing={<SystemDocumentStatusChip tone={resolveProviderStatusSemantic(activeDefaultId ? 'configured' : 'not configured').tone}>{resolveProviderStatusSemantic(activeDefaultId ? 'configured' : 'not configured').label}</SystemDocumentStatusChip>}
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
                  trailing={<SystemDocumentStatusChip tone={resolveProviderStatusSemantic(profile.enabled ? 'enabled' : 'disabled').tone}>{resolveProviderStatusSemantic(profile.enabled ? 'enabled' : 'disabled').label}</SystemDocumentStatusChip>}
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
                  trailing={<SystemDocumentStatusChip tone={resolveProviderStatusSemantic(draft.enabled ? 'enabled' : 'disabled').tone}>{resolveProviderStatusSemantic(draft.enabled ? 'enabled' : 'disabled').label}</SystemDocumentStatusChip>}
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
                    {llmHealthMessages[draft.id] ? (
                      <p className={cn('text-xs leading-5', llmHealthMessages[draft.id]?.tone === 'done' ? 'text-[var(--vel-color-muted)]' : 'text-amber-200')}>
                        {llmHealthMessages[draft.id]?.message}
                      </p>
                    ) : null}
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
                  trailing={<SystemDocumentStatusChip tone={resolveProviderStatusSemantic(draft.enabled ? 'enabled' : 'disabled').tone}>{resolveProviderStatusSemantic(draft.enabled ? 'enabled' : 'disabled').label}</SystemDocumentStatusChip>}
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
