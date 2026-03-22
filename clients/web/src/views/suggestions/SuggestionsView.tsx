import { useMemo, useState } from 'react';
import type { JsonValue, SuggestionData, UncertaintyData } from '../../types';
import {
  contextQueryKeys,
  loadSuggestion,
  loadSuggestions,
  loadUncertainty,
  resolveUncertainty,
  updateSuggestion,
} from '../../data/context';
import { invalidateQuery, useQuery } from '../../data/query';
import { Button } from '../../core/Button';
import {
  PanelCallout,
  PanelDenseRow,
  PanelDetailShell,
  PanelInsetCard,
  PanelJsonPre,
  PanelSelectableListButton,
} from '../../core/PanelChrome';
import { PanelItemTitle, PanelMetaPill } from '../../core/PanelItem';
import { SurfaceState } from '../../core/SurfaceState';

export function SuggestionsView() {
  const listKey = useMemo(() => contextQueryKeys.suggestions('pending'), []);
  const uncertaintyKey = useMemo(() => contextQueryKeys.uncertainty('open'), []);
  const [selectedSuggestionId, setSelectedSuggestionId] = useState<string | null>(null);
  const [selectedUncertaintyId, setSelectedUncertaintyId] = useState<string | null>(null);
  const [pendingActionId, setPendingActionId] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  const {
    data: suggestions = [],
    loading,
    error: suggestionsError,
    refetch: refetchSuggestions,
  } = useQuery<SuggestionData[]>(
    listKey,
    async () => {
      const response = await loadSuggestions('pending');
      return response.ok ? response.data ?? [] : [];
    },
  );
  const {
    data: uncertainty = [],
    loading: uncertaintyLoading,
    error: uncertaintyError,
    refetch: refetchUncertainty,
  } = useQuery<UncertaintyData[]>(
    uncertaintyKey,
    async () => {
      const response = await loadUncertainty('open');
      return response.ok ? response.data ?? [] : [];
    },
  );

  const activeSuggestionId =
    selectedUncertaintyId === null ? selectedSuggestionId ?? suggestions?.[0]?.id ?? null : null;
  const activeUncertainty =
    selectedUncertaintyId !== null
      ? uncertainty.find((item) => item.id === selectedUncertaintyId) ?? null
      : activeSuggestionId === null
        ? uncertainty[0] ?? null
        : null;
  const detailKey = useMemo(() => contextQueryKeys.suggestion(activeSuggestionId), [activeSuggestionId]);
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
        invalidateQuery(contextQueryKeys.suggestion(id), { refetch: true });
      }
      invalidateQuery(listKey, { refetch: true });
      invalidateQuery(contextQueryKeys.now(), { refetch: true });
      invalidateQuery(contextQueryKeys.contextExplain(), { refetch: true });
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

  async function handleResolveUncertainty(id: string) {
    setPendingActionId(id);
    setActionError(null);
    try {
      const response = await resolveUncertainty(id);
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to resolve uncertainty');
      }
      invalidateQuery(uncertaintyKey, { refetch: true });
      await refetchUncertainty();
      if (selectedUncertaintyId === id) {
        setSelectedUncertaintyId(null);
      }
    } catch (nextError) {
      setActionError(nextError instanceof Error ? nextError.message : String(nextError));
    } finally {
      setPendingActionId(null);
    }
  }

  if (loading || uncertaintyLoading) {
    return <SurfaceState message="Loading suggestions…" layout="centered" />;
  }

  const combinedError = suggestionsError ?? uncertaintyError;
  if (combinedError) {
    return <SurfaceState message={combinedError} layout="centered" tone="warning" />;
  }

  return (
    <div className="flex-1 overflow-hidden bg-zinc-950">
      <div className="grid h-full grid-cols-1 xl:grid-cols-[0.95fr_1.05fr]">
        <section className="border-b border-zinc-800 xl:border-b-0 xl:border-r">
          <header className="border-b border-zinc-800 px-6 py-5">
            <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">Suggestions</p>
            <h1 className="mt-2 text-2xl font-semibold text-zinc-100">Reviewable suggestion detail</h1>
            <p className="mt-2 text-sm text-zinc-400">
              Pending adjustments and deferred low-confidence decisions stay available here as a
              supporting detail surface, not a first-contact destination.
            </p>
          </header>
          <div className="h-[calc(100%-112px)] overflow-y-auto px-4 py-4">
            {suggestions.length > 0 || uncertainty.length > 0 ? (
              <div className="space-y-6">
                {suggestions.length > 0 ? (
                  <section>
                    <div className="mb-3 flex items-center justify-between gap-3">
                      <h2 className="text-sm font-medium text-zinc-100">Pending suggestions</h2>
                      <span className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                        {suggestions.length}
                      </span>
                    </div>
                    <div className="space-y-3">
                      {suggestions.map((suggestion) => (
                        <PanelSelectableListButton
                          key={suggestion.id}
                          selected={activeSuggestionId === suggestion.id}
                          selectionAccent="emerald"
                          onClick={() => {
                            setSelectedSuggestionId(suggestion.id);
                            setSelectedUncertaintyId(null);
                          }}
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
                            <PanelMetaPill tone="state">p{suggestion.priority}</PanelMetaPill>
                          </div>
                          <div className="mt-3 flex flex-wrap gap-2 text-xs text-zinc-400">
                            <span>{suggestion.suggestion_type}</span>
                            <span>{suggestion.confidence ?? 'unscored'} confidence</span>
                            <span>{suggestion.evidence_count} evidence</span>
                          </div>
                        </PanelSelectableListButton>
                      ))}
                    </div>
                  </section>
                ) : null}

                {uncertainty.length > 0 ? (
                  <section>
                    <div className="mb-3 flex items-center justify-between gap-3">
                      <h2 className="text-sm font-medium text-zinc-100">Deferred uncertainty</h2>
                      <span className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                        {uncertainty.length}
                      </span>
                    </div>
                    <div className="space-y-3">
                      {uncertainty.map((record) => (
                        <PanelSelectableListButton
                          key={record.id}
                          selected={activeUncertainty?.id === record.id}
                          selectionAccent="amber"
                          onClick={() => {
                            setSelectedUncertaintyId(record.id);
                            setSelectedSuggestionId(null);
                          }}
                        >
                          <div className="flex items-start justify-between gap-3">
                            <div>
                              <p className="text-base font-medium text-zinc-100">
                                {record.decision_kind.replace(/_/g, ' ')}
                              </p>
                              <p className="mt-1 text-sm text-zinc-400">
                                {record.subject_id ?? record.subject_type}
                              </p>
                            </div>
                            <PanelMetaPill tone="state">{record.confidence_band}</PanelMetaPill>
                          </div>
                          <div className="mt-3 flex flex-wrap gap-2 text-xs text-zinc-400">
                            <span>{record.resolution_mode.replace(/_/g, ' ')}</span>
                            <span>{record.status}</span>
                          </div>
                        </PanelSelectableListButton>
                      ))}
                    </div>
                  </section>
                ) : null}
              </div>
            ) : (
              <SurfaceState message="No pending suggestions or open uncertainty right now." />
            )}
          </div>
        </section>

        <section className="overflow-y-auto px-6 py-5">
          {actionError ? (
            <PanelCallout tone="danger" className="mb-4">
              {actionError}
            </PanelCallout>
          ) : null}
          {!activeSuggestionId ? (
            activeUncertainty ? (
              <UncertaintyDetailCard
                record={activeUncertainty}
                pending={pendingActionId === activeUncertainty.id}
                onResolve={() => void handleResolveUncertainty(activeUncertainty.id)}
              />
            ) : (
              <SurfaceState message="Pick a suggestion or uncertainty record to inspect it." />
            )
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

function UncertaintyDetailCard({
  record,
  pending,
  onResolve,
}: {
  record: UncertaintyData;
  pending: boolean;
  onResolve: () => void;
}) {
  return (
    <PanelDetailShell>
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">
            Deferred uncertainty
          </p>
          <h2 className="mt-2 text-2xl font-semibold text-zinc-100">
            {record.decision_kind.replace(/_/g, ' ')}
          </h2>
          <p className="mt-2 text-sm text-zinc-300">
            Subject: {record.subject_id ?? record.subject_type}
          </p>
        </div>
        <div className="text-right text-xs text-zinc-400">
          <p>{record.confidence_band} confidence band</p>
          <p className="mt-1">{record.resolution_mode.replace(/_/g, ' ')}</p>
        </div>
      </div>

      <div className="mt-5 flex flex-wrap gap-2 text-xs text-zinc-400">
        <PanelMetaPill tone="state">{record.status}</PanelMetaPill>
        {record.confidence_score != null ? (
          <PanelMetaPill tone="state">score {record.confidence_score.toFixed(2)}</PanelMetaPill>
        ) : null}
      </div>

      <section className="mt-6">
        <PanelItemTitle as="h3" size="sm">
          Reasons
        </PanelItemTitle>
        <JsonBlock value={record.reasons} />
      </section>

      {record.missing_evidence ? (
        <section className="mt-6">
          <PanelItemTitle as="h3" size="sm">
            Missing evidence
          </PanelItemTitle>
          <JsonBlock value={record.missing_evidence} />
        </section>
      ) : null}

      <div className="mt-6 flex gap-3">
        <Button variant="secondary" size="md" disabled={pending} loading={pending} onClick={onResolve}>
          {pending ? 'Resolving…' : 'Mark resolved'}
        </Button>
      </div>
    </PanelDetailShell>
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
    <PanelDetailShell>
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
        <PanelMetaPill tone="state">{suggestion.evidence_count} evidence</PanelMetaPill>
        {suggestion.decision_context_summary ? (
          <PanelMetaPill tone="state">{suggestion.decision_context_summary}</PanelMetaPill>
        ) : null}
      </div>

      <section className="mt-6">
        <PanelItemTitle as="h3" size="sm">
          Payload
        </PanelItemTitle>
        <JsonBlock value={suggestion.payload} />
      </section>

      {suggestion.decision_context ? (
        <section className="mt-6">
          <PanelItemTitle as="h3" size="sm">
            Decision context
          </PanelItemTitle>
          <JsonBlock value={suggestion.decision_context} />
        </section>
      ) : null}

      {suggestion.adaptive_policy ? (
        <section className="mt-6">
          <PanelItemTitle as="h3" size="sm">
            Adaptive policy provenance
          </PanelItemTitle>
          <PanelInsetCard className="mt-3 text-sm text-zinc-200">
            <p>Policy: {suggestion.adaptive_policy.policy_key}</p>
            <p>Suggested minutes: {suggestion.adaptive_policy.suggested_minutes}</p>
            {suggestion.adaptive_policy.current_minutes != null ? (
              <p>Current minutes: {suggestion.adaptive_policy.current_minutes}</p>
            ) : null}
            {suggestion.adaptive_policy.active_override ? (
              <>
                <p>Active override: {suggestion.adaptive_policy.active_override.value_minutes} min</p>
                {suggestion.adaptive_policy.active_override.source_title
                  || suggestion.adaptive_policy.active_override.source_suggestion_id ? (
                    <p>
                      Source:{' '}
                      {suggestion.adaptive_policy.active_override.source_title
                        ?? suggestion.adaptive_policy.active_override.source_suggestion_id}
                    </p>
                  ) : null}
                {suggestion.adaptive_policy.is_active_source ? (
                  <p className="text-emerald-300">This suggestion is the active policy source.</p>
                ) : null}
              </>
            ) : (
              <p>No active override is currently applied for this policy.</p>
            )}
          </PanelInsetCard>
        </section>
      ) : null}

      <section className="mt-6">
        <PanelItemTitle as="h3" size="sm">
          Evidence
        </PanelItemTitle>
        {suggestion.evidence && suggestion.evidence.length > 0 ? (
          <div className="mt-3 space-y-3">
            {suggestion.evidence.map((item) => (
              <PanelDenseRow key={item.id}>
                <div className="flex items-center justify-between gap-3">
                  <p className="text-sm text-zinc-100">
                    {item.evidence_type} · {item.ref_id}
                  </p>
                  <p className="text-xs text-zinc-500">
                    {item.weight != null ? `weight ${item.weight}` : 'unweighted'}
                  </p>
                </div>
                {item.evidence ? <JsonBlock value={item.evidence} compact /> : null}
              </PanelDenseRow>
            ))}
          </div>
        ) : (
          <SurfaceState message="No evidence rows attached." />
        )}
      </section>

      <div className="mt-6 flex gap-3">
        <Button variant="primary" size="md" disabled={pending} loading={pending} onClick={onAccept}>
          {pending ? 'Applying…' : 'Accept'}
        </Button>
        <Button variant="outline" size="md" disabled={pending} onClick={onReject}>
          Reject
        </Button>
      </div>
    </PanelDetailShell>
  );
}

function JsonBlock({ value, compact = false }: { value: JsonValue; compact?: boolean }) {
  return (
    <PanelJsonPre compact={compact}>{JSON.stringify(value, null, 2)}</PanelJsonPre>
  );
}
