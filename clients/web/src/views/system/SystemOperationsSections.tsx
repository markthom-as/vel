import type { AgentInspectData } from '../../types';
import { PanelEmptyRow } from '../../core/PanelChrome';
import {
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentMetaRow,
  SystemDocumentSectionLabel,
  SystemDocumentStatsGrid,
  SystemDocumentStatusChip,
} from '../../core/SystemDocument';
import { resolveStateStatusSemantic } from '../../core/Theme/semanticRegistry';
import type { IntegrationProviderSummary } from './SystemProvidersSection';
import { ProviderGlyph } from './SystemSupportSections';
import { systemChildAnchor } from './systemNavigation';

export function OperationsActivityDetail({
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
        {providers.map((provider) => {
          const statusSemantic = resolveStateStatusSemantic(provider.status);
          return (
            <SystemDocumentItem
              key={provider.key}
              id={systemChildAnchor('activity', provider.key)}
              leading={<ProviderGlyph provider={provider.key} />}
              title={provider.label}
              subtitle={provider.guidance}
              trailing={<SystemDocumentStatusChip tone={statusSemantic.tone}>{statusSemantic.label}</SystemDocumentStatusChip>}
            >
              <SystemDocumentStatsGrid className="gap-x-6">
                {provider.meta.map((item) => <SystemDocumentMetaRow key={`${provider.key}-${item.label}`} label={item.label} value={item.value} />)}
              </SystemDocumentStatsGrid>
            </SystemDocumentItem>
          );
        })}
      </SystemDocumentList>
    </div>
  );
}

export function OperationsRecoveryDetail({
  recoveryProviders,
  filteredBlockers,
}: {
  recoveryProviders: IntegrationProviderSummary[];
  filteredBlockers: AgentInspectData['blockers'];
}) {
  return (
    <SystemDocumentList>
      {recoveryProviders.map((provider) => {
        const statusSemantic = resolveStateStatusSemantic(provider.status);
        return (
          <SystemDocumentItem
            key={provider.key}
            id={systemChildAnchor('recovery', provider.key)}
            leading={<ProviderGlyph provider={provider.key} />}
            title={provider.label}
            subtitle={provider.guidance}
            trailing={<SystemDocumentStatusChip tone={statusSemantic.tone}>{statusSemantic.label}</SystemDocumentStatusChip>}
          >
            <>
              {provider.meta.map((item) => <SystemDocumentMetaRow key={`${provider.key}-${item.label}`} label={item.label} value={item.value} />)}
            </>
          </SystemDocumentItem>
        );
      })}
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
