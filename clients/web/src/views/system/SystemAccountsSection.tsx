import type { IntegrationConnectionData } from '../../types';
import { PanelEmptyRow } from '../../core/PanelChrome';
import {
  SystemDocumentField,
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentMetaRow,
  SystemDocumentSectionLabel,
  SystemDocumentStatsGrid,
  SystemDocumentStatusChip,
} from '../../core/SystemDocument';
import {
  resolveProviderSemantic,
  resolveProviderStatusSemantic,
} from '../../core/Theme/semanticRegistry';
import { systemChildAnchor } from './systemNavigation';
import { formatMaybeTimestamp, ProviderGlyph } from './SystemSupportSections';

function normalizeSemanticLabel(value: string | null | undefined): string {
  return (value ?? '').trim().toLowerCase().replace(/\s+/g, '_');
}

export function resolveConnectionTitle(connection: IntegrationConnectionData): string {
  const providerSemantic = resolveProviderSemantic(connection.provider_key);
  const normalizedDisplayName = normalizeSemanticLabel(connection.display_name);
  const normalizedProviderKey = normalizeSemanticLabel(connection.provider_key);
  if (
    normalizedDisplayName.length === 0
    || normalizedDisplayName === 'provider'
    || normalizedDisplayName === 'account'
    || normalizedDisplayName === normalizedProviderKey
  ) {
    return providerSemantic.label;
  }
  return connection.display_name;
}

export function IntegrationsAccountsDetail({
  connections,
}: {
  connections: IntegrationConnectionData[];
}) {
  const providerCount = new Set(connections.map((connection) => connection.provider_key)).size;
  const connectedCount = connections.filter((connection) => connection.status === 'connected').length;
  const settingsRefCount = connections.reduce((total, connection) => total + connection.setting_refs.length, 0);

  return (
    <SystemDocumentList>
      <SystemDocumentItem
        id={systemChildAnchor('accounts', 'account-summary')}
        title="Connected accounts"
        subtitle="Operator-facing view of account bindings and setting references across providers."
        trailing={<SystemDocumentStatusChip tone="neutral">{`${connections.length} accounts`}</SystemDocumentStatusChip>}
      >
        <SystemDocumentStatsGrid className="gap-x-6">
          <SystemDocumentField label="Providers" value={`${providerCount}`} />
          <SystemDocumentField label="Connected" value={`${connectedCount}`} />
          <SystemDocumentField label="Setting refs" value={`${settingsRefCount}`} />
          <SystemDocumentField label="Disconnected" value={`${Math.max(connections.length - connectedCount, 0)}`} />
        </SystemDocumentStatsGrid>
      </SystemDocumentItem>
      {connections.length === 0 ? (
        <PanelEmptyRow>No integration accounts are connected yet.</PanelEmptyRow>
      ) : connections.map((connection) => {
        const providerSemantic = resolveProviderSemantic(connection.provider_key);
        const statusSemantic = resolveProviderStatusSemantic(connection.status);
        return (
          <SystemDocumentItem
            key={connection.id}
            id={systemChildAnchor('accounts', connection.id)}
            leading={<ProviderGlyph provider={connection.provider_key} />}
            title={resolveConnectionTitle(connection)}
            subtitle={`${providerSemantic.label} · ${connection.family}`}
            trailing={<SystemDocumentStatusChip tone={statusSemantic.tone}>{statusSemantic.label}</SystemDocumentStatusChip>}
          >
            <>
              <SystemDocumentStatsGrid className="gap-x-6">
                <SystemDocumentMetaRow label="Account ref" value={connection.account_ref ?? 'Unavailable'} />
                <SystemDocumentMetaRow label="Updated" value={formatMaybeTimestamp(connection.updated_at)} />
                <SystemDocumentMetaRow label="Setting refs" value={`${connection.setting_refs.length}`} />
                <SystemDocumentMetaRow label="Connection ID" value={connection.id} />
              </SystemDocumentStatsGrid>
              {connection.setting_refs.length > 0 ? (
                <div className="space-y-2 pt-2">
                  <SystemDocumentSectionLabel>Setting references</SystemDocumentSectionLabel>
                  {connection.setting_refs.map((setting) => (
                    <SystemDocumentField key={`${setting.setting_key}-${setting.created_at}`} label={setting.setting_key} value={setting.setting_value} />
                  ))}
                </div>
              ) : (
                <p className="pt-2 text-xs leading-5 text-[var(--vel-color-muted)]">
                  No linked settings are recorded for this account.
                </p>
              )}
            </>
          </SystemDocumentItem>
        );
      })}
    </SystemDocumentList>
  );
}
