import { PanelEmptyRow } from '../../core/PanelChrome';
import { normalizeSemanticLabelValue } from '../../data/embeddedBridgeAdapter';
import type {
  IntegrationConnectionData,
  IntegrationsData,
  SettingsData,
} from '../../types';
import { IntegrationsAccountsDetail } from './SystemAccountsSection';
import {
  IntegrationsProvidersDetail,
  type IntegrationActionId,
  type IntegrationProviderSummary,
} from './SystemProvidersSection';

type IntegrationPrimitiveSubsection =
  | 'calendar'
  | 'tasks'
  | 'messages'
  | 'notes'
  | 'reminders'
  | 'transcripts'
  | 'git';

function normalizeIntegrationKey(value: string): string {
  return normalizeSemanticLabelValue(value).normalized;
}

function primitiveProviderKeys(subsection: IntegrationPrimitiveSubsection): string[] {
  switch (subsection) {
    case 'calendar':
      return ['google_calendar'];
    case 'tasks':
      return ['todoist'];
    case 'messages':
      return ['messaging'];
    case 'notes':
      return ['notes'];
    case 'reminders':
      return ['reminders'];
    case 'transcripts':
      return ['transcripts'];
    case 'git':
      return ['git'];
  }
}

function primitiveFamilies(subsection: IntegrationPrimitiveSubsection): string[] {
  switch (subsection) {
    case 'calendar':
      return ['calendar'];
    case 'tasks':
      return ['tasks', 'todoist'];
    case 'messages':
      return ['messages', 'messaging'];
    case 'notes':
      return ['notes'];
    case 'reminders':
      return ['reminders'];
    case 'transcripts':
      return ['transcripts'];
    case 'git':
      return ['git'];
  }
}

function primitiveLabel(subsection: IntegrationPrimitiveSubsection): string {
  switch (subsection) {
    case 'calendar':
      return 'Calendar';
    case 'tasks':
      return 'Tasks';
    case 'messages':
      return 'Messages';
    case 'notes':
      return 'Notes';
    case 'reminders':
      return 'Reminders';
    case 'transcripts':
      return 'Transcripts';
    case 'git':
      return 'Git';
  }
}

export function IntegrationPrimitiveDetail({
  subsection,
  providers,
  settings,
  integrations,
  connections,
  pendingAction,
  onRunIntegrationAction,
  onUpdateLlmSettings,
  onPatchGoogleCalendar,
  onPatchTodoist,
  onStartGoogleAuth,
}: {
  subsection: IntegrationPrimitiveSubsection;
  providers: IntegrationProviderSummary[];
  settings: SettingsData | null;
  integrations: IntegrationsData;
  connections: IntegrationConnectionData[];
  pendingAction: IntegrationActionId | null;
  onRunIntegrationAction: (actionId: IntegrationActionId) => void | Promise<void>;
  onUpdateLlmSettings: (patch: Record<string, unknown>) => Promise<void>;
  onPatchGoogleCalendar: (patch: Record<string, unknown>) => Promise<void>;
  onPatchTodoist: (patch: Record<string, unknown>) => Promise<void>;
  onStartGoogleAuth: () => Promise<void>;
}) {
  const allowedProviderKeys = new Set(primitiveProviderKeys(subsection).map(normalizeIntegrationKey));
  const allowedFamilies = new Set(primitiveFamilies(subsection).map(normalizeIntegrationKey));
  const filteredProviders = providers.filter((provider) => allowedProviderKeys.has(normalizeIntegrationKey(provider.key)));
  const filteredConnections = connections.filter((connection) => {
    const family = normalizeIntegrationKey(connection.family);
    const providerKey = normalizeIntegrationKey(connection.provider_key);
    return allowedFamilies.has(family) || allowedProviderKeys.has(providerKey);
  });

  if (filteredProviders.length === 0 && filteredConnections.length === 0) {
    return (
      <PanelEmptyRow>
        No {primitiveLabel(subsection).toLowerCase()} sources are configured yet.
      </PanelEmptyRow>
    );
  }

  return (
    <div className="space-y-4">
      {filteredProviders.length > 0 ? (
        <IntegrationsProvidersDetail
          subsectionKey={subsection}
          includeLlmRouting={false}
          providers={filteredProviders}
          settings={settings}
          integrations={integrations}
          pendingAction={pendingAction}
          onRunIntegrationAction={onRunIntegrationAction}
          onUpdateLlmSettings={onUpdateLlmSettings}
          onPatchGoogleCalendar={onPatchGoogleCalendar}
          onPatchTodoist={onPatchTodoist}
          onStartGoogleAuth={onStartGoogleAuth}
        />
      ) : null}
      {filteredConnections.length > 0 ? (
        <IntegrationsAccountsDetail
          subsectionKey={subsection}
          summaryTitle={`${primitiveLabel(subsection)} sources`}
          summarySubtitle={`Linked source accounts currently feeding ${primitiveLabel(subsection).toLowerCase()} into Vel.`}
          connections={filteredConnections}
        />
      ) : null}
    </div>
  );
}
