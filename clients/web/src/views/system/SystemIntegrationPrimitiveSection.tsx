import { PanelEmptyRow } from '../../core/PanelChrome';
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
import {
  integrationPrimitiveDescriptor,
  normalizeIntegrationPrimitiveValue,
  type IntegrationPrimitiveKey,
} from './SystemIntegrationTaxonomy';

function normalizeIntegrationKey(value: string): string {
  return normalizeIntegrationPrimitiveValue(value);
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
  subsection: Exclude<IntegrationPrimitiveKey, 'models' | 'sources'>;
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
  const primitive = integrationPrimitiveDescriptor(subsection);
  const allowedProviderKeys = new Set(primitive.providerKeys.map(normalizeIntegrationKey));
  const allowedFamilies = new Set(primitive.families.map(normalizeIntegrationKey));
  const filteredProviders = providers.filter((provider) => allowedProviderKeys.has(normalizeIntegrationKey(provider.key)));
  const filteredConnections = connections.filter((connection) => {
    const family = normalizeIntegrationKey(connection.family);
    const providerKey = normalizeIntegrationKey(connection.provider_key);
    return allowedFamilies.has(family) || allowedProviderKeys.has(providerKey);
  });

  if (filteredProviders.length === 0 && filteredConnections.length === 0) {
    return (
      <PanelEmptyRow>
        No {primitive.label.toLowerCase()} sources are configured yet.
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
          summaryTitle={`${primitive.label} sources`}
          summarySubtitle={`Linked source accounts currently feeding ${primitive.label.toLowerCase()} into Vel.`}
          connections={filteredConnections}
        />
      ) : null}
    </div>
  );
}
