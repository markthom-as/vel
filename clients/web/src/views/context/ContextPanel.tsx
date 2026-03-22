import { useMemo, useState } from 'react';
import type { ContextExplainData, DriftExplainData, JsonObject, JsonValue, SignalExplainSummary } from '../../types';
import { contextQueryKeys, loadContextExplain, loadDriftExplain } from '../../data/context';
import { useQuery } from '../../data/query';
import { Button } from '../../core/Button';
import {
  PanelDebugBlock,
  PanelDenseRow,
  PanelKeyValueRow,
  PanelListBullet,
  PanelListBulletMuted,
  PanelMutedInset,
  PanelStatTile,
} from '../../core/PanelChrome';
import { PanelItemSectionLabel } from '../../core/PanelItem';
import { SurfaceState } from '../../core/SurfaceState';

type ContextMode = 'state' | 'why' | 'debug';

export function ContextPanel() {
  const [mode, setMode] = useState<ContextMode>('state');
  const contextExplainKey = useMemo(() => contextQueryKeys.contextExplain(), []);
  const driftExplainKey = useMemo(() => contextQueryKeys.driftExplain(), []);
  const {
    data: context,
    loading: contextLoading,
    error: contextError,
  } = useQuery<ContextExplainData | null>(
    contextExplainKey,
    async () => {
      const response = await loadContextExplain();
      return response.ok ? response.data ?? null : null;
    },
  );
  const {
    data: drift,
    loading: driftLoading,
    error: driftError,
  } = useQuery<DriftExplainData | null>(
    driftExplainKey,
    async () => {
      const response = await loadDriftExplain();
      return response.ok ? response.data ?? null : null;
    },
  );

  const loading = contextLoading || driftLoading;
  const error = contextError ?? driftError;
  const stateEntries = context ? summarizeContext(context.context) : [];
  const signalSummaries = context ? mergeSignalSummaries(
    context?.signal_summaries ?? [],
    drift?.signal_summaries ?? [],
  ) : [];
  const sourceSummaries = context ? summarizeSourceSummaries(context.source_summaries) : [];

  if (loading) return <SurfaceState message="Loading context…" title="Context" />;
  if (error) return <SurfaceState message={error} title="Context" tone="warning" />;
  if (!context) {
    return <SurfaceState message="No context data. Run evaluate or start the engine." title="Context" />;
  }

  return (
    <div className="space-y-4 overflow-y-auto p-4 text-sm">
      <div className="space-y-3">
        <div>
          <h3 className="mb-2 font-medium text-zinc-400">Context</h3>
          <p className="text-xs text-zinc-500">
            computed at {new Date(context.computed_at * 1000).toLocaleString()}
          </p>
        </div>
        <div className="flex gap-2">
          <ModeButton
            label="State"
            active={mode === 'state'}
            onClick={() => setMode('state')}
          />
          <ModeButton
            label="Why"
            active={mode === 'why'}
            onClick={() => setMode('why')}
          />
          <ModeButton
            label="Debug"
            active={mode === 'debug'}
            onClick={() => setMode('debug')}
          />
        </div>
      </div>

      {mode === 'state' ? (
        <>
          <section className="grid gap-3">
            <div className="grid grid-cols-2 gap-3">
              <PanelStatTile density="compact" label="Mode" value={context.mode ?? 'unknown'} />
              <PanelStatTile density="compact" label="Morning state" value={context.morning_state ?? 'unknown'} />
            </div>
            {drift && hasDriftData(drift) && (
              <PanelMutedInset>
                <PanelItemSectionLabel>Attention</PanelItemSectionLabel>
                <div className="mt-2 grid grid-cols-2 gap-3">
                  <PanelStatTile density="compact" label="State" value={drift.attention_state ?? 'unknown'} />
                  <PanelStatTile density="compact" label="Drift" value={drift.drift_type ?? 'none'} />
                  <PanelStatTile density="compact" label="Severity" value={drift.drift_severity ?? 'n/a'} />
                  <PanelStatTile
                    density="compact"
                    label="Confidence"
                    value={drift.confidence == null ? 'n/a' : `${Math.round(drift.confidence * 100)}%`}
                  />
                </div>
              </PanelMutedInset>
            )}
          </section>

          {stateEntries.length > 0 ? (
            <section>
              <SectionHeading title="Current state" />
              <div className="mt-2 space-y-2">
                {stateEntries.map(([label, value]) => (
                  <PanelKeyValueRow key={label} label={label} value={value} />
                ))}
              </div>
            </section>
          ) : null}
        </>
      ) : null}

      {mode === 'why' ? (
        <>
          {(context.reasons.length > 0 || (drift?.reasons.length ?? 0) > 0) ? (
            <section>
              <SectionHeading title="Why this context" />
              <ul className="mt-2 list-none space-y-2 p-0">
                {context.reasons.map((reason) => (
                  <PanelListBullet key={reason}>{reason}</PanelListBullet>
                ))}
                {drift?.reasons.map((reason) => (
                  <PanelListBulletMuted key={`drift-${reason}`}>{reason}</PanelListBulletMuted>
                ))}
              </ul>
            </section>
          ) : null}

          {sourceSummaries.length > 0 ? (
            <section>
              <SectionHeading title="Source summaries" />
              <div className="mt-2 space-y-2">
                {sourceSummaries.map((source) => (
                  <PanelDenseRow key={source.label}>
                    <div className="flex items-start justify-between gap-3">
                      <p className="text-zinc-100">{source.label}</p>
                      <p className="text-xs text-zinc-500">
                        {new Date(source.timestamp * 1000).toLocaleString()}
                      </p>
                    </div>
                    <p className="mt-2 whitespace-pre-wrap break-words text-xs text-zinc-300">
                      {formatSummary(source.summary)}
                    </p>
                  </PanelDenseRow>
                ))}
              </div>
            </section>
          ) : null}

          {context.adaptive_policy_overrides.length > 0 ? (
            <section>
              <SectionHeading title="Adaptive policy overrides" />
              <div className="mt-2 space-y-2">
                {context.adaptive_policy_overrides.map((override) => (
                  <PanelDenseRow key={override.policy_key}>
                    <div className="flex items-start justify-between gap-3">
                      <p className="text-zinc-100">{override.policy_key}</p>
                      <p className="text-xs text-zinc-500">{override.value_minutes} min</p>
                    </div>
                    {override.source_title || override.source_suggestion_id ? (
                      <p className="mt-2 text-xs text-zinc-300">
                        Source:{' '}
                        {override.source_title ?? override.source_suggestion_id}
                      </p>
                    ) : null}
                  </PanelDenseRow>
                ))}
              </div>
            </section>
          ) : null}

          {signalSummaries.length > 0 ? (
            <section>
              <SectionHeading title="Signals used" />
              <div className="mt-2 space-y-2">
                {signalSummaries.map((signal) => (
                  <PanelDenseRow key={signal.signal_id}>
                    <div className="flex items-start justify-between gap-3">
                      <div>
                        <p className="text-zinc-100">{signal.signal_type}</p>
                        <p className="text-xs text-zinc-500">{signal.source}</p>
                      </div>
                      <p className="text-xs text-zinc-500">
                        {new Date(signal.timestamp * 1000).toLocaleString()}
                      </p>
                    </div>
                    <p className="mt-2 whitespace-pre-wrap break-words text-xs text-zinc-300">
                      {formatSummary(signal.summary)}
                    </p>
                  </PanelDenseRow>
                ))}
              </div>
            </section>
          ) : null}
        </>
      ) : null}

      {mode === 'debug' ? (
        <section className="space-y-3">
          <SectionHeading title="Debug payloads" />
          <DebugBlock title="Context JSON" value={context.context} />
          <DebugBlock title="Drift summary" value={drift ?? null} />
          <DebugBlock title="Source summaries JSON" value={context.source_summaries} />
          <DebugBlock title="Signal summaries JSON" value={signalSummaries} />
          <DebugBlock title="Signals used IDs" value={signalSummaries.map((signal) => signal.signal_id)} />
          <DebugBlock title="Commitments used IDs" value={context.commitments_used} />
          <DebugBlock title="Risk used IDs" value={context.risk_used} />
        </section>
      ) : null}
    </div>
  );
}

function ModeButton({
  label,
  active,
  onClick,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <Button
      variant={active ? 'secondary' : 'outline'}
      size="sm"
      onClick={onClick}
      aria-pressed={active}
      className={active ? '' : 'border-zinc-800 bg-zinc-900/55 text-zinc-400 hover:text-zinc-200'}
    >
      {label}
    </Button>
  );
}

function DebugBlock({ title, value }: { title: string; value: unknown }) {
  return (
    <PanelDebugBlock title={title}>
      <pre className="mt-2 whitespace-pre-wrap break-words font-mono text-xs text-zinc-200">
        {JSON.stringify(value, null, 2)}
      </pre>
    </PanelDebugBlock>
  );
}

function SectionHeading({ title }: { title: string }) {
  return <h4 className="font-medium text-zinc-400">{title}</h4>;
}

function hasDriftData(drift: DriftExplainData): boolean {
  return drift.attention_state !== null
    || drift.drift_type !== null
    || drift.drift_severity !== null
    || drift.confidence !== null
    || drift.reasons.length > 0;
}

function summarizeContext(context: JsonValue): Array<[string, string]> {
  if (!context || typeof context !== 'object' || Array.isArray(context)) {
    return [['value', formatSummary(context)]];
  }

  const record = context as JsonObject;
  const keys = [
    'next_commitment_id',
    'meds_status',
    'prep_window_active',
    'commute_window_active',
    'global_risk_level',
    'global_risk_score',
    'git_activity_summary',
    'note_document_summary',
    'assistant_message_summary',
    'message_waiting_on_me_count',
    'message_waiting_on_others_count',
    'message_scheduling_thread_count',
    'message_urgent_thread_count',
    'message_summary',
  ];

  return keys
    .filter((key) => record[key] !== undefined && record[key] !== null)
    .map((key) => [key.replace(/_/g, ' '), formatSummary(record[key])]);
}

function mergeSignalSummaries(
  left: SignalExplainSummary[],
  right: SignalExplainSummary[],
): SignalExplainSummary[] {
  const byId = new Map<string, SignalExplainSummary>();
  for (const signal of [...left, ...right]) {
    byId.set(signal.signal_id, signal);
  }
  return [...byId.values()];
}

function summarizeSourceSummaries(sourceSummaries: ContextExplainData['source_summaries']) {
  const entries = [
    sourceSummaries.git_activity
      ? { label: 'Git activity', ...sourceSummaries.git_activity }
      : null,
    sourceSummaries.health
      ? { label: 'Health', ...sourceSummaries.health }
      : null,
    sourceSummaries.note_document
      ? { label: 'Recent note', ...sourceSummaries.note_document }
      : null,
    sourceSummaries.assistant_message
      ? { label: 'Recent transcript', ...sourceSummaries.assistant_message }
      : null,
  ];
  return entries.filter(
    (
      value,
    ): value is { label: string; timestamp: number; summary: JsonValue } => value !== null,
  );
}

function formatSummary(value: JsonValue): string {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean' || value === null) {
    return String(value);
  }
  if (Array.isArray(value)) {
    return value.map((item) => formatSummary(item)).join(', ');
  }
  return Object.entries(value as JsonObject)
    .filter(([, next]) => next !== null && next !== undefined && next !== '')
    .map(([key, next]) => `${key.replace(/_/g, ' ')}: ${formatSummary(next)}`)
    .join(' · ');
}
