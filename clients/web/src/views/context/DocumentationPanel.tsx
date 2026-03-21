import {
  CORE_DOCUMENTATION_ENTRIES,
  USER_DOCUMENTATION_ENTRIES,
  type DocumentationTuple,
} from '../../data/documentationCatalog.generated';

interface DocumentationPanelProps {
  compact?: boolean;
  currentView?: string;
}

export function DocumentationPanel({ compact = false, currentView }: DocumentationPanelProps) {
  return (
    <div className={compact ? 'space-y-4 p-3' : 'space-y-5 p-4'}>
      <div>
        <p className="text-[10px] uppercase tracking-[0.24em] text-zinc-500">Documentation</p>
        <p className="mt-2 text-xs text-zinc-400">
          {currentView
            ? `Contextual notes for ${currentView} can live here. Placeholder content is active for this phase.`
            : 'Core and operator docs stay top-level here instead of buried in Settings.'}
        </p>
      </div>
      {currentView ? (
        <section className="rounded-2xl border border-zinc-800 bg-zinc-900/50 px-3 py-3">
          <p className="text-[10px] uppercase tracking-[0.24em] text-zinc-500">Current View</p>
          <p className="mt-2 text-sm font-medium text-zinc-100">{currentView}</p>
          <p className="mt-2 text-xs leading-5 text-zinc-400">
            Keep this panel focused on contextual docs, definitions, and trust notes for the active surface.
          </p>
        </section>
      ) : null}
      <DocumentationSection title="Core" docs={CORE_DOCUMENTATION_ENTRIES} compact={compact} />
      <DocumentationSection title="Operator" docs={USER_DOCUMENTATION_ENTRIES} compact={compact} />
    </div>
  );
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
      <p className="text-[10px] uppercase tracking-[0.24em] text-zinc-500">{title}</p>
      <div className="space-y-2">
        {docs.map(([label, path, hint]) => (
          <div key={path} className="rounded-xl border border-zinc-800 bg-zinc-900/50 px-3 py-2">
            <p className="text-xs font-medium uppercase tracking-[0.18em] text-zinc-200">{label}</p>
            {!compact ? (
              <p className="mt-1 text-xs leading-5 text-zinc-400">{hint}</p>
            ) : null}
            <p className="mt-1 break-all text-[11px] text-zinc-500">{path}</p>
          </div>
        ))}
      </div>
    </section>
  );
}
