import {
  CORE_DOCUMENTATION_ENTRIES,
  USER_DOCUMENTATION_ENTRIES,
  type DocumentationTuple,
} from '../../data/documentationCatalog.generated';
import { PanelDenseRow, PanelEyebrow, PanelPageSection } from '../../core/PanelChrome';

interface DocumentationPanelProps {
  compact?: boolean;
  currentView?: string;
}

export function DocumentationPanel({ compact = false, currentView }: DocumentationPanelProps) {
  const currentViewHint = currentView ? contextualDocHint(currentView) : null;

  return (
    <div className={compact ? 'space-y-4 p-3' : 'space-y-5 p-4'}>
      <div>
        <PanelEyebrow tracking="wide">Documentation</PanelEyebrow>
        <p className="mt-2 text-xs text-zinc-400">
          {currentView
            ? `Contextual notes for ${currentView} can live here. Placeholder content is active for this phase.`
            : 'Core and operator docs stay top-level here instead of buried in Settings.'}
        </p>
      </div>
      {currentView ? (
        <PanelPageSection className="!p-3">
          <PanelEyebrow tracking="wide">Current View</PanelEyebrow>
          <p className="mt-2 text-sm font-medium text-zinc-100">{currentView}</p>
          <p className="mt-2 text-xs leading-5 text-zinc-400">
            {currentViewHint}
          </p>
        </PanelPageSection>
      ) : null}
      <DocumentationSection title="Core" docs={CORE_DOCUMENTATION_ENTRIES} compact={compact} />
      <DocumentationSection title="Operator" docs={USER_DOCUMENTATION_ENTRIES} compact={compact} />
    </div>
  );
}

function contextualDocHint(currentView: string): string {
  switch (currentView.toLowerCase()) {
    case 'now':
      return 'Keep this panel focused on current-day truth, nudge meaning, and the operator rules behind queue ordering and trust warnings.';
    case 'threads':
      return 'Keep this panel focused on continuation intent, intervention meaning, and why a thread is asking for attention.';
    case 'settings':
      return 'Keep this panel focused on practical control surfaces, trust posture, and what each settings section actually governs.';
    case 'inbox':
      return 'Keep this panel focused on queue meaning, object provenance, and how Inbox stays aligned to the same actionable truth as Now.';
    default:
      return 'Keep this panel focused on contextual docs, definitions, and trust notes for the active surface.';
  }
}

function DocumentationSection({
  title,
  docs,
  compact,
}: {
  title: string;
  docs: DocumentationTuple[];
  compact: boolean;
}) {
  return (
    <section className="space-y-2">
      <PanelEyebrow tracking="wide">{title}</PanelEyebrow>
      <div className="space-y-2">
        {docs.map(([label, path, hint]) => (
          <PanelDenseRow key={path}>
            <p className="text-xs font-medium uppercase tracking-[0.18em] text-zinc-200">{label}</p>
            {!compact ? (
              <p className="mt-1 text-xs leading-5 text-zinc-400">{hint}</p>
            ) : null}
            <p className="mt-1 break-all text-[11px] text-zinc-500">{path}</p>
          </PanelDenseRow>
        ))}
      </div>
    </section>
  );
}
