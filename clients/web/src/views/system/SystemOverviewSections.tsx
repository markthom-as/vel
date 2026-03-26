import type { AgentInspectData } from '../../types';
import { PanelEmptyRow } from '../../core/PanelChrome';
import { MarkdownMessage } from '../../core/MarkdownMessage';
import {
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentMetaRow,
  SystemDocumentSectionLabel,
  SystemDocumentStatsGrid,
  SystemDocumentStatusChip,
} from '../../core/SystemDocument';
import { systemChildAnchor } from './systemNavigation';

export function OverviewTrustDetail({
  inspect,
  degradedProviderCount,
  filteredBlockers,
}: {
  inspect: AgentInspectData;
  degradedProviderCount: number;
  filteredBlockers: AgentInspectData['blockers'];
}) {
  return (
    <div className="space-y-5">
      <SystemDocumentStatsGrid className="gap-x-6">
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'current-mode')} label="Current mode" value={inspect.grounding.current_context?.mode ?? 'Unknown'} />
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'persisted-kinds')} label="Persisted kinds" value={inspect.explainability.persisted_record_kinds.join(', ') || 'None'} />
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'grounded-projects')} label="Grounded projects" value={`${inspect.grounding.projects.length}`} />
        <SystemDocumentMetaRow id={systemChildAnchor('trust', 'degraded-providers')} label="Degraded providers" value={`${degradedProviderCount}`} />
        {filteredBlockers.length === 0 ? (
          <SystemDocumentMetaRow id={systemChildAnchor('trust', 'health')} label="Health" value="Stable" />
        ) : (
          filteredBlockers.map((blocker) => (
            <SystemDocumentMetaRow key={blocker.code} id={systemChildAnchor('trust', blocker.code)} label={blocker.code} value={blocker.message} />
          ))
        )}
      </SystemDocumentStatsGrid>
    </div>
  );
}

export function SystemDocumentationDetail({ doc }: { doc: string }) {
  return (
    <div id="system-docs" className="scroll-mt-24 rounded-[24px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)] px-4 py-4">
      <SystemDocumentSectionLabel>System documentation</SystemDocumentSectionLabel>
      <div className="mt-3 max-w-3xl text-sm leading-6 text-[var(--vel-color-text)]">
        <MarkdownMessage text={doc} />
      </div>
    </div>
  );
}

export function OverviewHorizonDetail({ inspect }: { inspect: AgentInspectData }) {
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
