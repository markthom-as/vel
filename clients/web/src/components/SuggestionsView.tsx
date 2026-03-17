import { useMemo, useState } from 'react';
import type { JsonValue, SuggestionData } from '../types';
import { invalidateQuery, useQuery } from '../data/query';
import { loadSuggestion, loadSuggestions, queryKeys, updateSuggestion } from '../data/resources';
import { SurfaceState } from './SurfaceState';

export function SuggestionsView() {
  const listKey = useMemo(() => queryKeys.suggestions('pending'), []);
  const [selectedSuggestionId, setSelectedSuggestionId] = useState<string | null>(null);
  const [pendingActionId, setPendingActionId] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  const {
    data: suggestions,
    loading,
    error,
    refetch: refetchSuggestions,
  } = useQuery<SuggestionData[]>(
    listKey,
    async () => {
      const response = await loadSuggestions('pending');
      return response.ok ? response.data ?? [] : [];
    },
  );

  const activeSuggestionId = selectedSuggestionId ?? suggestions?.[0]?.id ?? null;
  const detailKey = useMemo(() => queryKeys.suggestion(activeSuggestionId), [activeSuggestionId]);
  const {
    data: selectedSuggestion,
    loading: detailLoading,
  } = useQuery<SuggestionData | null>(
    detailKey,
    async () => {
      if (!activeSuggestionId) {
        return null;
      }
      const response = await loadSuggestion(activeSuggestionId);
      return response.ok ? response.data ?? null : null;
    },
    { enabled: Boolean(activeSuggestionId) },
  );

  async function applySuggestionAction(id: string, state: 'accepted' | 'rejected') {
    setPendingActionId(id);
    setActionError(null);
    try {
      const response = await updateSuggestion(id, { state });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update suggestion');
      }
      if (activeSuggestionId === id) {
        invalidateQuery(queryKeys.suggestion(id), { refetch: true });
      }
      invalidateQuery(listKey, { refetch: true });
      invalidateQuery(queryKeys.now(), { refetch: true });
      invalidateQuery(queryKeys.contextExplain(), { refetch: true });
      await refetchSuggestions();
      if (activeSuggestionId === id) {
        setSelectedSuggestionId(null);
      }
    } catch (nextError) {
      setActionError(nextError instanceof Error ? nextError.message : String(nextError));
    } finally {
      setPendingActionId(null);
    }
  }

  if (loading) {
    return <SurfaceState message="Loading suggestions…" layout="centered" />;
  }

  if (error) {
    return <SurfaceState message={error} layout="centered" tone="warning" />;
  }

  return (
    <div className="flex-1 overflow-hidden bg-zinc-950">
      <div className="grid h-full grid-cols-1 xl:grid-cols-[0.95fr_1.05fr]">
        <section className="border-b border-zinc-800 xl:border-b-0 xl:border-r">
          <header className="border-b border-zinc-800 px-6 py-5">
            <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">Suggestions</p>
            <h1 className="mt-2 text-2xl font-semibold text-zinc-100">Steering proposals</h1>
            <p className="mt-2 text-sm text-zinc-400">
              Pending adjustments Vel wants you to explicitly accept or reject.
            </p>
          </header>
          <div className="h-[calc(100%-112px)] overflow-y-auto px-4 py-4">
            {suggestions && suggestions.length > 0 ? (
              <div className="space-y-3">
                {suggestions.map((suggestion) => (
                  <button
                    key={suggestion.id}
                    type="button"
                    onClick={() => setSelectedSuggestionId(suggestion.id)}
                    className={`w-full rounded-2xl border p-4 text-left transition ${
                      activeSuggestionId === suggestion.id
                        ? 'border-emerald-500 bg-emerald-500/10'
                        : 'border-zinc-800 bg-zinc-900/70 hover:border-zinc-700'
                    }`}
                  >
                    <div className="flex items-start justify-between gap-3">
                      <div>
                        <p className="text-base font-medium text-zinc-100">
                          {suggestion.title ?? suggestion.suggestion_type}
                        </p>
                        <p className="mt-1 text-sm text-zinc-400">
                          {suggestion.summary ?? suggestion.suggestion_type}
                        </p>
                      </div>
                      <span className="rounded-full border border-zinc-700 px-2 py-0.5 text-[11px] uppercase tracking-wide text-zinc-300">
                        p{suggestion.priority}
                      </span>
                    </div>
                    <div className="mt-3 flex flex-wrap gap-2 text-xs text-zinc-400">
                      <span>{suggestion.suggestion_type}</span>
                      <span>{suggestion.confidence ?? 'unscored'} confidence</span>
                      <span>{suggestion.evidence_count} evidence</span>
                    </div>
                  </button>
                ))}
              </div>
            ) : (
              <SurfaceState message="No pending suggestions right now." />
            )}
          </div>
        </section>

        <section className="overflow-y-auto px-6 py-5">
          {actionError ? (
            <div className="mb-4 rounded-xl border border-rose-500/40 bg-rose-500/10 px-4 py-3 text-sm text-rose-200">
              {actionError}
            </div>
          ) : null}
          {!activeSuggestionId ? (
            <SurfaceState message="Pick a suggestion to inspect its evidence." />
          ) : detailLoading && !selectedSuggestion ? (
            <SurfaceState message="Loading suggestion detail…" />
          ) : selectedSuggestion ? (
            <SuggestionDetailCard
              suggestion={selectedSuggestion}
              pending={pendingActionId === selectedSuggestion.id}
              onAccept={() => void applySuggestionAction(selectedSuggestion.id, 'accepted')}
              onReject={() => void applySuggestionAction(selectedSuggestion.id, 'rejected')}
            />
          ) : (
            <SurfaceState message="Suggestion detail unavailable." tone="warning" />
          )}
        </section>
      </div>
    </div>
  );
}

function SuggestionDetailCard({
  suggestion,
  pending,
  onAccept,
  onReject,
}: {
  suggestion: SuggestionData;
  pending: boolean;
  onAccept: () => void;
  onReject: () => void;
}) {
  return (
    <div className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-5">
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">
            {suggestion.suggestion_type}
          </p>
          <h2 className="mt-2 text-2xl font-semibold text-zinc-100">
            {suggestion.title ?? suggestion.suggestion_type}
          </h2>
          {suggestion.summary ? (
            <p className="mt-2 text-sm text-zinc-300">{suggestion.summary}</p>
          ) : null}
        </div>
        <div className="text-right text-xs text-zinc-400">
          <p>priority p{suggestion.priority}</p>
          <p className="mt-1">{suggestion.confidence ?? 'unscored'} confidence</p>
        </div>
      </div>

      <div className="mt-5 flex flex-wrap gap-2 text-xs text-zinc-400">
        <span className="rounded-full border border-zinc-700 px-2 py-1">
          {suggestion.evidence_count} evidence
        </span>
        {suggestion.decision_context_summary ? (
          <span className="rounded-full border border-zinc-700 px-2 py-1">
            {suggestion.decision_context_summary}
          </span>
        ) : null}
      </div>

      <section className="mt-6">
        <h3 className="text-sm font-medium text-zinc-100">Payload</h3>
        <JsonBlock value={suggestion.payload} />
      </section>

      {suggestion.decision_context ? (
        <section className="mt-6">
          <h3 className="text-sm font-medium text-zinc-100">Decision context</h3>
          <JsonBlock value={suggestion.decision_context} />
        </section>
      ) : null}

      <section className="mt-6">
        <h3 className="text-sm font-medium text-zinc-100">Evidence</h3>
        {suggestion.evidence && suggestion.evidence.length > 0 ? (
          <div className="mt-3 space-y-3">
            {suggestion.evidence.map((item) => (
              <div
                key={item.id}
                className="rounded-xl border border-zinc-800 bg-zinc-950/70 px-4 py-3"
              >
                <div className="flex items-center justify-between gap-3">
                  <p className="text-sm text-zinc-100">
                    {item.evidence_type} · {item.ref_id}
                  </p>
                  <p className="text-xs text-zinc-500">
                    {item.weight != null ? `weight ${item.weight}` : 'unweighted'}
                  </p>
                </div>
                {item.evidence ? <JsonBlock value={item.evidence} compact /> : null}
              </div>
            ))}
          </div>
        ) : (
          <SurfaceState message="No evidence rows attached." />
        )}
      </section>

      <div className="mt-6 flex gap-3">
        <button
          type="button"
          disabled={pending}
          onClick={onAccept}
          className="rounded-xl bg-emerald-500 px-4 py-2 text-sm font-medium text-zinc-950 transition hover:bg-emerald-400 disabled:cursor-not-allowed disabled:opacity-60"
        >
          {pending ? 'Applying…' : 'Accept'}
        </button>
        <button
          type="button"
          disabled={pending}
          onClick={onReject}
          className="rounded-xl border border-zinc-700 px-4 py-2 text-sm font-medium text-zinc-200 transition hover:border-zinc-500 disabled:cursor-not-allowed disabled:opacity-60"
        >
          Reject
        </button>
      </div>
    </div>
  );
}

function JsonBlock({ value, compact = false }: { value: JsonValue; compact?: boolean }) {
  return (
    <pre
      className={`mt-3 overflow-x-auto rounded-xl border border-zinc-800 bg-zinc-950/80 px-4 py-3 text-xs text-zinc-300 ${
        compact ? '' : 'whitespace-pre-wrap'
      }`}
    >
      {JSON.stringify(value, null, 2)}
    </pre>
  );
}
