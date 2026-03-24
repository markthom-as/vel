import { useEffect, useMemo, useState } from 'react';
import { contextQueryKeys, loadNow, rescheduleNowTasksToToday } from '../../data/context';
import { invalidateQuery, useQuery } from '../../data/query';
import type { MainView } from '../../data/operatorSurfaces';
import type { NowData, NowNudgeBarData } from '../../types';
import { acknowledgeInboxItem, invalidateInboxQueries, snoozeInboxItem } from '../../data/chat';
import {
  ClockIcon,
  OpenThreadIcon,
  WarningIcon,
} from '../../core/Icons';
import { uiFonts } from '../../core/Theme';
import { nudgeOpenSystemTarget } from '../../views/now/nowModel';
import {
  NudgeActionIcon,
  NudgeLeadOrb,
  nudgeActionAriaLabel,
  nudgeActionButtonLabel,
} from '../../views/now/nowNudgePresentation';
import type { SystemNavigationTarget } from '../../views/system';
import { cn } from '../../core/cn';
import { ActionChipButton } from '../../core/FilterToggleTag';
import { FloatingPill } from '../../core/FloatingPill';
import { SurfaceSpinner } from '../../core/SurfaceState';

interface NudgeZoneProps {
  activeView: MainView;
  extraNudges?: NowNudgeBarData[];
  highlightedNudgeId?: string | null;
  highlightedNudgeNonce?: number | null;
  onOpenThread?: (conversationId: string) => void;
  onOpenSystem?: (target?: SystemNavigationTarget) => void;
}

function nudgeTone(bar: NowNudgeBarData) {
  switch (bar.kind) {
    case 'trust_warning':
      return {
        shell: 'border-amber-500/45 bg-amber-950/30 text-amber-100',
        activeOutline: 'ring-1 ring-amber-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(251,191,36,0.3),0_0_24px_rgba(245,158,11,0.18)]',
        warmSurface: true,
      };
    case 'freshness_warning':
      return {
        shell: 'border-sky-500/35 bg-sky-950/25 text-sky-100',
        activeOutline: 'ring-1 ring-sky-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(56,189,248,0.28),0_0_24px_rgba(14,165,233,0.16)]',
        warmSurface: false,
      };
    case 'needs_input':
      return {
        shell:
          'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel)]/82 text-[var(--vel-color-text)]',
        activeOutline: 'ring-1 ring-[var(--vel-color-accent-strong)]/80 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(255,107,0,0.34),0_0_24px_rgba(255,107,0,0.18)]',
        warmSurface: false,
      };
    case 'review_request':
      return {
        shell: 'border-emerald-500/30 bg-emerald-950/20 text-emerald-100',
        activeOutline: 'ring-1 ring-emerald-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(52,211,153,0.28),0_0_24px_rgba(16,185,129,0.16)]',
        warmSurface: false,
      };
    case 'reflow_proposal':
      return {
        shell: 'border-orange-500/35 bg-orange-950/20 text-orange-100',
        activeOutline: 'ring-1 ring-orange-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(251,146,60,0.28),0_0_24px_rgba(249,115,22,0.16)]',
        warmSurface: true,
      };
    default:
      return {
        shell:
          'border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/78 text-[var(--vel-color-text)]',
        activeOutline: 'ring-1 ring-[var(--vel-color-accent-border)]/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(255,107,0,0.22),0_0_20px_rgba(255,107,0,0.12)]',
        warmSurface: false,
      };
  }
}

const actionChipClass =
  '!min-h-[1.1rem] !gap-1.5 !rounded-full !px-2 !py-[0.2rem] !text-[9px] !tracking-[0.1em] opacity-90';

type CoreChecklistItem = {
  id: string;
  label: string;
  state: 'required' | 'ready';
  value: string | null;
};

function parseCoreSetupChecklistItem(action: NowNudgeBarData['actions'][number]): CoreChecklistItem | null {
  const parts = action.kind.split(':');
  if (parts[0] !== 'open_settings' || parts[1] !== 'core_settings' || !parts[2] || !parts[3]) {
    return null;
  }
  return {
    id: parts[2],
    label: action.label,
    state: parts[3] === 'ready' ? 'ready' : 'required',
    value: parts[4] ? decodeURIComponent(parts.slice(4).join(':')) : null,
  };
}

function formatNudgeAge(timestamp: number | null | undefined): string | null {
  if (!timestamp) return null;
  const diffMinutes = Math.max(0, Math.floor((Date.now() / 1000 - timestamp) / 60));
  if (diffMinutes < 1) return 'NOW';
  if (diffMinutes < 60) return `${diffMinutes} MIN AGO`;
  const diffHours = Math.floor(diffMinutes / 60);
  if (diffHours < 24) return `${diffHours} H AGO`;
  return `${Math.floor(diffHours / 24)} D AGO`;
}

function interventionIdForBar(bar: NowNudgeBarData, data: NowData | null): string | null {
  if (bar.id.startsWith('intv_')) {
    return bar.id;
  }
  const actionItem = data?.action_items?.find((item) => item.id === bar.id);
  const fromEvidence = actionItem?.evidence.find(
    (evidence) => evidence.source_kind === 'intervention' || evidence.source_kind === 'assistant_proposal',
  );
  if (fromEvidence?.source_id) {
    return fromEvidence.source_id;
  }
  const prefix = 'act_intervention_';
  return bar.id.startsWith(prefix) ? bar.id.slice(prefix.length) : null;
}

export function NudgeZone({
  activeView,
  extraNudges = [],
  highlightedNudgeId = null,
  highlightedNudgeNonce = null,
  onOpenThread,
  onOpenSystem,
}: NudgeZoneProps) {
  const [expandedNudgeId, setExpandedNudgeId] = useState<string | null>(null);
  const [flashingNudgeId, setFlashingNudgeId] = useState<string | null>(null);
  const [pendingActionKey, setPendingActionKey] = useState<string | null>(null);
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const { data, loading, error } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );

  const nudgeBars = [...extraNudges, ...(data?.nudge_bars ?? [])];
  const orderedNudges = nudgeBars
    .filter((bar, index) => nudgeBars.findIndex((item) => item.id === bar.id) === index)
    .sort((a, b) => Number(b.urgent) - Number(a.urgent));
  const deferredCount = (data?.header?.buckets ?? []).find((bucket) => bucket.kind === 'snoozed')?.count ?? 0;
  const orderedNudgeIdsKey = orderedNudges.map((bar) => bar.id).join('|');

  function expandNudge(nudgeId: string) {
    setExpandedNudgeId(nudgeId);
  }

  useEffect(() => {
    if (!highlightedNudgeId || highlightedNudgeNonce == null) {
      return;
    }
    const highlightedIndex = orderedNudges.findIndex((bar) => bar.id === highlightedNudgeId);
    if (highlightedIndex === -1) {
      return;
    }
    setExpandedNudgeId(highlightedNudgeId);
    setFlashingNudgeId(highlightedNudgeId);
    const timeoutId = window.setTimeout(() => {
      setFlashingNudgeId((current) => (current === highlightedNudgeId ? null : current));
    }, 1600);
    return () => window.clearTimeout(timeoutId);
  }, [highlightedNudgeId, highlightedNudgeNonce, orderedNudgeIdsKey]);

  async function runNudgeMutation(
    actionKey: string,
    callback: () => Promise<unknown>,
  ) {
    setPendingActionKey(actionKey);
    try {
      await callback();
      invalidateInboxQueries();
      invalidateQuery(nowKey, { refetch: true });
    } finally {
      setPendingActionKey(null);
    }
  }

  function parseRescheduleCommitmentIds(kind: string): string[] {
    const [prefix, encodedIds] = kind.split(':', 2);
    if (prefix !== 'reschedule_today' || !encodedIds) {
      return [];
    }
    return encodedIds
      .split(',')
      .map((id) => id.trim())
      .filter((id) => id.length > 0);
  }

  function parseJumpAnchor(kind: string): string | null {
    const [prefix, anchor] = kind.split(':', 2);
    if (prefix !== 'jump_backlog' || !anchor?.trim()) {
      return null;
    }
    return anchor.trim();
  }

  return (
    <aside id="nudges-section" aria-label="Nudges" className="flex flex-col gap-2 overflow-visible pl-6 pr-3">
      <div className="flex items-center justify-between gap-3 px-2">
        <p className={`${uiFonts.display} inline-flex items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
          <WarningIcon size={11} />
          NUDGES ({orderedNudges.length})
          {deferredCount > 0 ? <span className="ml-2 text-[var(--vel-color-muted)]">| DEFERRED ({deferredCount})</span> : null}
        </p>
      </div>

      {loading ? (
        <div className="px-2 py-1 text-sm text-[var(--vel-color-muted)]">
          <SurfaceSpinner className="mb-1 h-4 w-4" />
          <p>Loading signals…</p>
        </div>
      ) : error ? (
        <p className="px-2 text-sm text-[var(--vel-color-error)]">{error}</p>
      ) : orderedNudges.length > 0 ? (
        <div className={cn('flex flex-col', expandedNudgeId ? 'gap-3' : 'gap-2')}>
          {orderedNudges.map((bar) => {
            const tone = nudgeTone(bar);
            const isExpanded = expandedNudgeId === bar.id;
            const interventionId = interventionIdForBar(bar, data ?? null);
            const coreSetupChecklist = bar.id === 'core_setup_required'
              ? bar.actions
                .map((action) => ({ action, checklist: parseCoreSetupChecklistItem(action) }))
                .filter((item): item is { action: NowNudgeBarData['actions'][number]; checklist: CoreChecklistItem } => item.checklist !== null)
              : [];
            const visibleActions = bar.id === 'core_setup_required'
              ? []
              : bar.actions;
            const actionButtons = (
              <>
                {visibleActions.map((action, index) => {
                  const actionKey = `${bar.id}-${action.kind}-${index}`;
                  const label = nudgeActionButtonLabel(action, bar);
                  const ariaLabel = nudgeActionAriaLabel(bar, action, index, bar.actions.length);
                  if (action.kind.startsWith('open_settings')) {
                    return (
                      <ActionChipButton
                        key={actionKey}
                        onClick={() => onOpenSystem?.(nudgeOpenSystemTarget(bar, action))}
                        aria-label={ariaLabel}
                        className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[4.5rem]')}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        <span>{label}</span>
                      </ActionChipButton>
                    );
                  }
                  if (action.kind.startsWith('reschedule_today')) {
                    const commitmentIds = parseRescheduleCommitmentIds(action.kind);
                    return (
                      <ActionChipButton
                        key={actionKey}
                        onClick={() => {
                          if (commitmentIds.length === 0) {
                            return;
                          }
                          void runNudgeMutation(actionKey, () => rescheduleNowTasksToToday(commitmentIds));
                        }}
                        disabled={pendingActionKey === actionKey || commitmentIds.length === 0}
                        aria-label={ariaLabel}
                        className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[4.5rem]')}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        <span>{label}</span>
                      </ActionChipButton>
                    );
                  }
                  if (action.kind.startsWith('jump_backlog')) {
                    const anchor = parseJumpAnchor(action.kind);
                    return (
                      <ActionChipButton
                        key={actionKey}
                        onClick={() => {
                          if (activeView !== 'now' || !anchor) {
                            return;
                          }
                          document.getElementById(anchor)?.scrollIntoView({ behavior: 'smooth', block: 'start' });
                        }}
                        disabled={activeView !== 'now' || !anchor}
                        aria-label={ariaLabel}
                        className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[4.5rem]')}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        <span>{label}</span>
                      </ActionChipButton>
                    );
                  }
                  if (interventionId && (action.kind === 'accept' || action.kind === 'acknowledge')) {
                    return (
                      <ActionChipButton
                        key={actionKey}
                        onClick={() => {
                          void runNudgeMutation(actionKey, () => acknowledgeInboxItem(interventionId));
                        }}
                        disabled={pendingActionKey === actionKey}
                        className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[4.5rem]')}
                        aria-label={ariaLabel}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        <span>{label}</span>
                      </ActionChipButton>
                    );
                  }
                  if (bar.primary_thread_id && (action.kind === 'expand' || action.kind === 'escalate' || action.kind === 'edit' || action.kind === 'open_thread')) {
                    return (
                      <ActionChipButton
                        key={actionKey}
                        onClick={() => onOpenThread?.(bar.primary_thread_id!)}
                        className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[4.5rem]')}
                        aria-label={ariaLabel}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        <span>{label}</span>
                      </ActionChipButton>
                    );
                  }
                  return (
                    <ActionChipButton
                      key={actionKey}
                      aria-label={ariaLabel}
                      disabled
                      className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[4rem]')}
                    >
                      <NudgeActionIcon kind={action.kind} size={11} />
                      <span>{label}</span>
                    </ActionChipButton>
                  );
                })}
                <ActionChipButton
                  aria-label={`Defer (${bar.title}) · ${bar.id}`}
                  className={actionChipClass}
                  disabled={!interventionId || pendingActionKey === `${bar.id}-defer`}
                  onClick={() => {
                    if (!interventionId) return;
                    void runNudgeMutation(`${bar.id}-defer`, () => snoozeInboxItem(interventionId, 10));
                  }}
                >
                  <NudgeActionIcon kind="snooze" size={11} />
                  <span>Defer</span>
                </ActionChipButton>
              </>
            );

            return (
              <FloatingPill
                key={bar.id}
                decoration={<NudgeLeadOrb kind={bar.kind} urgent={bar.urgent} warmSurface={tone.warmSurface} isPrimary={bar.urgent} />}
                decorationClassName="h-[1.875rem] w-[1.875rem] rounded-none border-0 bg-transparent shadow-none"
                decorationOffsetClassName="-translate-x-[114%]"
                onPress={() => expandNudge(bar.id)}
                contentClassName={cn(
                  isExpanded ? 'items-start gap-3 py-3' : null,
                  tone.shell,
                  isExpanded ? tone.activeOutline : null,
                  flashingNudgeId === bar.id
                    ? 'ring-2 ring-[var(--vel-color-accent-strong)] ring-offset-2 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(255,107,0,0.42),0_0_32px_rgba(255,107,0,0.28)] animate-[pulse_0.38s_ease-in-out_4]'
                    : null,
                )}
              >
                {isExpanded ? (
                  <div className="flex min-w-0 flex-1 flex-col gap-3">
                    <div className="flex min-w-0 items-start justify-between gap-3">
                      <button
                        type="button"
                        className="min-w-0 flex-1 overflow-hidden pt-0.5 text-left"
                        onClick={() => expandNudge(bar.id)}
                        data-testid={`nudge-toggle-${bar.id}`}
                      >
                        <div className="flex min-w-0 flex-col gap-1">
                          {bar.timestamp ? (
                            <span className={`inline-flex items-center gap-1 text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)] ${uiFonts.mono}`}>
                              <ClockIcon size={10} />
                              {formatNudgeAge(bar.timestamp)}
                            </span>
                          ) : null}
                          <p className="text-sm font-medium whitespace-normal">{bar.title}</p>
                        </div>
                      </button>
                      <div
                        className="flex max-w-[46%] shrink-0 flex-wrap items-center justify-end gap-1 overflow-hidden pt-0.5"
                        onClick={(event) => event.stopPropagation()}
                      >
                        {actionButtons}
                      </div>
                    </div>
                    <div className="flex w-full flex-col">
                      <p className="w-full whitespace-normal text-xs leading-5 text-[var(--vel-color-muted)]">
                        {bar.summary}
                      </p>
                      {coreSetupChecklist.length > 0 ? (
                        <div className="mt-1 flex w-full flex-col gap-1">
                          {coreSetupChecklist.map(({ action, checklist }, index) => (
                            <button
                              key={`${bar.id}-check-${checklist.id}-${index}`}
                              type="button"
                              onClick={(event) => {
                                event.stopPropagation();
                                onOpenSystem?.(nudgeOpenSystemTarget(bar, action));
                              }}
                              aria-label={nudgeActionAriaLabel(bar, action, index, coreSetupChecklist.length)}
                              className="flex w-full items-center gap-2 rounded-lg px-1 py-1 text-left transition hover:bg-[var(--vel-color-panel)]/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--vel-color-accent-strong)]/40"
                            >
                              <span
                                aria-hidden
                                className={cn(
                                  'flex h-4 w-4 shrink-0 items-center justify-center rounded-sm border text-[10px] leading-none',
                                  checklist.state === 'ready'
                                    ? 'border-emerald-500/40 bg-emerald-950/35 text-emerald-100'
                                    : 'border-[var(--vel-color-border)] bg-transparent text-transparent',
                                )}
                              >
                                {checklist.state === 'ready' ? '✓' : '·'}
                              </span>
                              <span className="shrink-0 text-xs leading-5 text-[var(--vel-color-text)]">
                                {checklist.label}
                              </span>
                              {checklist.value ? (
                                <span className="min-w-0 flex-1 truncate text-[11px] leading-5 text-[var(--vel-color-muted)]">
                                  {checklist.value}
                                </span>
                              ) : null}
                              <span
                                aria-hidden
                                className="ml-auto inline-flex shrink-0 items-center text-[var(--vel-color-muted)]"
                                data-testid={`core-setup-open-icon-${checklist.id}`}
                              >
                                <OpenThreadIcon size={11} />
                              </span>
                            </button>
                          ))}
                        </div>
                      ) : null}
                    </div>
                  </div>
                ) : (
                  <>
                    <button
                      type="button"
                      className="min-w-0 flex-1 overflow-hidden text-left"
                      onClick={() => expandNudge(bar.id)}
                      data-testid={`nudge-toggle-${bar.id}`}
                    >
                      <div className="flex min-w-0 flex-col gap-1">
                        {bar.timestamp ? (
                          <span className={`inline-flex items-center gap-1 text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)] ${uiFonts.mono}`}>
                            <ClockIcon size={10} />
                            {formatNudgeAge(bar.timestamp)}
                          </span>
                        ) : null}
                        <p className="text-sm font-medium truncate">{bar.title}</p>
                      </div>
                      <p className="truncate text-xs text-[var(--vel-color-muted)]">
                        {bar.summary}
                      </p>
                    </button>
                    <div
                      className="flex max-w-[38%] shrink-0 flex-wrap items-center justify-end gap-1 overflow-hidden"
                      onClick={(event) => event.stopPropagation()}
                    >
                      {actionButtons}
                    </div>
                  </>
                )}
              </FloatingPill>
            );
          })}
        </div>
      ) : (
        <p className="px-2 text-sm text-[var(--vel-color-muted)]">No active nudges right now.</p>
      )}
    </aside>
  );
}
