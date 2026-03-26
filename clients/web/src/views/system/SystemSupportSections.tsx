import type { AgentCapabilityEntryData, AgentInspectData } from '../../types';
import { PanelEmptyRow } from '../../core/PanelChrome';
import { SemanticIcon } from '../../core/Icons/SemanticIcon';
import { cn } from '../../core/cn';
import {
  SystemDocumentItem,
  SystemDocumentList,
  SystemDocumentMetaRow,
  SystemDocumentSectionLabel,
  SystemDocumentStatsGrid,
  SystemDocumentStatusChip,
  SystemDocumentToggleRow,
} from '../../core/SystemDocument';
import {
  resolveProjectStatusSemantic,
  resolveProviderSemantic,
  resolveStateStatusSemantic,
} from '../../core/Theme/semanticRegistry';
import { systemChildAnchor } from './systemNavigation';

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

function CapabilityRow({ entry }: { entry: AgentCapabilityEntryData }) {
  return (
    <SystemDocumentItem
      title={entry.label}
      subtitle={entry.summary}
      trailing={<SystemDocumentStatusChip tone={resolveStateStatusSemantic(entry.available ? 'available' : 'blocked').tone}>{resolveStateStatusSemantic(entry.available ? 'available' : 'blocked').label}</SystemDocumentStatusChip>}
      className="py-2 first:pt-0 last:pb-0"
    >
      {entry.blocked_reason ? (
        <SystemDocumentMetaRow label="Reason" value={entry.blocked_reason.message} className="border-b-0 py-1" />
      ) : null}
    </SystemDocumentItem>
  );
}

export function ProviderGlyph({ provider }: { provider: string }) {
  const semantic = resolveProviderSemantic(provider);

  return (
    <div
      className={cn(
        'flex h-8 w-8 shrink-0 items-center justify-center rounded-full text-[10px] font-semibold uppercase tracking-[0.16em]',
        semantic.glyphClassName,
      )}
      aria-hidden
    >
      <SemanticIcon icon={semantic.icon} size={12} />
    </div>
  );
}

export function formatMaybeTimestamp(timestamp: number | null): string {
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

export function ControlProjectsDetail({
  projects,
}: {
  projects: AgentInspectData['grounding']['projects'];
}) {
  return (
    <SystemDocumentList>
      {projects.length === 0 ? (
        <PanelEmptyRow>No grounded projects are available.</PanelEmptyRow>
      ) : projects.map((project) => {
        const statusSemantic = resolveProjectStatusSemantic(project.status);
        return (
        <SystemDocumentItem
          key={project.id}
          id={systemChildAnchor('projects', project.id)}
          title={project.name}
          subtitle={project.slug}
          trailing={<SystemDocumentStatusChip tone={statusSemantic.tone}>{statusSemantic.label}</SystemDocumentStatusChip>}
        >
          <>
            <SystemDocumentStatsGrid className="gap-x-6">
              <SystemDocumentMetaRow label="Slug" value={project.slug} />
              <SystemDocumentMetaRow label="Family" value={project.family} />
              <SystemDocumentMetaRow label="Primary repo" value={project.primary_repo?.path ?? 'Unavailable'} />
              <SystemDocumentMetaRow label="Primary notes" value={project.primary_notes_root?.path ?? 'Unavailable'} />
            </SystemDocumentStatsGrid>
          </>
        </SystemDocumentItem>
        );
      })}
    </SystemDocumentList>
  );
}

export function ControlCapabilitiesDetail({
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

export function PreferencesAccessibilityDetail({
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
