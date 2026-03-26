import { useEffect, useMemo, useRef, useState, type ReactNode } from 'react';
import { buildCoreSetupStatus } from '../../data/operator';
import type { IntegrationsData, SettingsData } from '../../types';
import { Button } from '../../core/Button';
import { cn } from '../../core/cn';
import {
  SystemDocumentField,
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentStatusChip,
  SystemDocumentToggleRow,
} from '../../core/SystemDocument';
import { resolveStateStatusSemantic } from '../../core/Theme/semanticRegistry';
import {
  SYSTEM_CORE_SETTING_ANCHORS,
  systemChildAnchor,
  systemTargetForProvider,
  type SystemNavigationTarget,
} from './systemNavigation';

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
  const statusSemantic = resolveStateStatusSemantic(ready ? 'ready' : 'required');

  return (
    <div className="flex flex-wrap items-start justify-between gap-3 border-b border-[var(--vel-color-border)] py-2 last:border-b-0">
      <div className="min-w-0 flex-1">
        <h4 className="text-sm font-medium leading-5 text-[var(--vel-color-text)]">{label}</h4>
        <p className="text-xs leading-5 text-[var(--vel-color-muted)]">{detail}</p>
      </div>
      <div className="flex shrink-0 items-center gap-2">
        <SystemDocumentStatusChip tone={statusSemantic.tone}>
          {statusSemantic.label}
        </SystemDocumentStatusChip>
        {action}
      </div>
    </div>
  );
}

export function CoreSettingsDetail({
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
  const requiredSetupSemantic = resolveStateStatusSemantic(coreSetupStatus.ready ? 'ready' : 'blocked');
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
      <SystemDocumentList>
        {!coreSetupStatus.ready && (
          <SystemDocumentItem
            id={systemChildAnchor('core_settings', 'required-setup')}
            title="Required setup"
            subtitle="Vel stays partially disabled until every required Core item is saved."
            trailing={<SystemDocumentStatusChip tone={requiredSetupSemantic.tone}>{requiredSetupSemantic.label}</SystemDocumentStatusChip>}
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
                      onClick={() => onJumpToTarget(systemTargetForProvider('llm_routing'))}
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
                      onClick={() => onJumpToTarget(systemTargetForProvider('google_calendar'))}
                    >
                      Open integrations
                    </Button>
                  )}
                />
              </div>
            </>
          </SystemDocumentItem>
        )}

        <SystemDocumentItem
          id={systemChildAnchor('core_settings', 'identity')}
          title="Identity"
          subtitle="Required identity fields for a usable single-node Vel."
        >
          <>
            <SystemDocumentField
              label="Your name *"
              fieldId={SYSTEM_CORE_SETTING_ANCHORS.userDisplayName}
              value={coreSettings?.user_display_name ?? ''}
              placeholder="Required before Vel can operate normally"
              onCommit={(value) => onUpdateCoreSettings({ user_display_name: value })}
            />
            <SystemDocumentField
              label="Node name *"
              fieldId={SYSTEM_CORE_SETTING_ANCHORS.nodeDisplayName}
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
              fieldId={SYSTEM_CORE_SETTING_ANCHORS.agentProfileFreeform}
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
  );
}
