import { useMemo, useState } from 'react';
import { contextQueryKeys, loadNow } from '../../data/context';
import { invalidateQuery, useQuery } from '../../data/query';
import type { MainView } from '../../data/operatorSurfaces';
import type { NowData, NowNudgeBarData } from '../../types';
import { acknowledgeInboxItem, invalidateInboxQueries, snoozeInboxItem } from '../../data/chat';
import {
  ClipboardCheckIcon,
  ClockIcon,
  OpenThreadIcon,
  RescheduleIcon,
  SettingsIcon,
  SyncIcon,
  ThreadsIcon,
  WarningIcon,
} from '../../core/Icons';
import { uiFonts } from '../../core/Theme';
import { nudgeOpenSystemTarget } from '../../views/now/nowModel';
import type { SystemNavigationTarget } from '../../views/system';
import { cn } from '../../core/cn';
import { ActionChipButton } from '../../core/FilterToggleTag';
import { FloatingPill } from '../../core/FloatingPill';
import { SurfaceSpinner } from '../../core/SurfaceState';

interface NudgeZoneProps {
  activeView: MainView;
  extraNudges?: NowNudgeBarData[];
  onOpenThread?: (conversationId: string) => void;
  onOpenSystem?: (target?: SystemNavigationTarget) => void;
}

function nudgeTone(bar: NowNudgeBarData) {
  switch (bar.kind) {
    case 'trust_warning':
      return {
        shell: 'border-amber-500/45 bg-amber-950/30 text-amber-100',
        icon: 'border-amber-500/50 bg-amber-950/75 text-amber-200 shadow-[0_0_18px_rgba(245,158,11,0.28)]',
        Icon: WarningIcon,
      };
    case 'freshness_warning':
      return {
        shell: 'border-sky-500/35 bg-sky-950/25 text-sky-100',
        icon: 'border-sky-500/45 bg-sky-950/70 text-sky-200 shadow-[0_0_18px_rgba(59,130,246,0.28)]',
        Icon: SyncIcon,
      };
    case 'needs_input':
      return {
        shell:
          'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel)]/82 text-[var(--vel-color-text)]',
        icon:
          'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] text-[var(--vel-color-accent-soft)] shadow-[0_0_18px_rgba(200,116,43,0.24)]',
        Icon: ThreadsIcon,
      };
    case 'review_request':
      return {
        shell: 'border-emerald-500/30 bg-emerald-950/20 text-emerald-100',
        icon: 'border-emerald-500/35 bg-emerald-950/70 text-emerald-200 shadow-[0_0_18px_rgba(16,185,129,0.24)]',
        Icon: ClipboardCheckIcon,
      };
    case 'reflow_proposal':
      return {
        shell: 'border-orange-500/35 bg-orange-950/20 text-orange-100',
        icon: 'border-orange-500/40 bg-orange-950/70 text-orange-200 shadow-[0_0_18px_rgba(249,115,22,0.26)]',
        Icon: RescheduleIcon,
      };
    default:
      return {
        shell:
          'border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/78 text-[var(--vel-color-text)]',
        icon:
          'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] text-[var(--vel-color-accent-soft)] shadow-[0_0_18px_rgba(200,116,43,0.24)]',
        Icon: WarningIcon,
      };
  }
}

function actionIcon(kind: string) {
  if (kind === 'open_settings') return <SettingsIcon size={11} />;
  if (kind === 'expand') return <OpenThreadIcon size={11} />;
  if (kind === 'snooze') return <RescheduleIcon size={11} />;
  return <OpenThreadIcon size={11} />;
}

const actionChipClass =
  '!min-h-[1.05rem] !gap-1 !rounded-full !px-1.5 !py-[0.2rem] !text-[9px] !tracking-[0.1em] opacity-85';

function compactActionLabel(label: string): string {
  return label
    .replace(/^continue in /i, '')
    .replace(/^open /i, '')
    .replace(/^view /i, '')
    .replace(/^related threads$/i, '')
    .trim();
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
  activeView: _activeView,
  extraNudges = [],
  onOpenThread,
  onOpenSystem,
}: NudgeZoneProps) {
  const [expanded, setExpanded] = useState(false);
  const [expandedNudgeId, setExpandedNudgeId] = useState<string | null>(null);
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
  const visibleNudgeCount = expanded ? orderedNudges.length : Math.min(orderedNudges.length, 4);
  const hiddenNudgeCount = Math.max(0, orderedNudges.length - visibleNudgeCount);
  const deferredCount = (data?.header?.buckets ?? []).find((bucket) => bucket.kind === 'snoozed')?.count ?? 0;

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

  return (
    <aside id="nudges-section" aria-label="Nudges" className="flex flex-col gap-2">
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
        <div className="flex flex-col gap-2">
          {orderedNudges.slice(0, visibleNudgeCount).map((bar) => {
            const tone = nudgeTone(bar);
            const Icon = tone.Icon;
            const isExpanded = expandedNudgeId === bar.id;
            const interventionId = interventionIdForBar(bar, data ?? null);

            return (
              <FloatingPill
                key={bar.id}
                decoration={<Icon size={12} className="block shrink-0" />}
                decorationClassName={tone.icon}
                decorationOffsetClassName="-translate-x-[64%]"
                contentClassName={tone.shell}
              >
                  <div
                    className="min-w-0 flex-1 overflow-hidden cursor-pointer"
                    onClick={() => setExpandedNudgeId((current) => current === bar.id ? null : bar.id)}
                  >
                    <div className="flex min-w-0 items-center gap-2">
                      <p className={cn('text-sm font-medium', isExpanded ? 'whitespace-normal' : 'truncate')}>{bar.title}</p>
                      {bar.timestamp ? (
                        <span className={`inline-flex items-center gap-1 text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)] ${uiFonts.mono}`}>
                          <ClockIcon size={10} />
                          {formatNudgeAge(bar.timestamp)}
                        </span>
                      ) : null}
                    </div>
                    <p className={cn('text-xs text-[var(--vel-color-muted)]', isExpanded ? 'whitespace-normal leading-5' : 'truncate')}>
                      {bar.summary}
                    </p>
                  </div>
                  <div
                    className={cn('flex max-w-[38%] shrink-0 flex-wrap items-center justify-end gap-1 overflow-hidden', isExpanded ? 'max-w-[46%]' : '')}
                    onClick={(event) => event.stopPropagation()}
                  >
                    {bar.actions.map((action, index) => {
                      const actionKey = `${bar.id}-${action.kind}-${index}`;
                      if (action.kind === 'open_settings') {
                        return (
                          <ActionChipButton
                            key={actionKey}
                            onClick={() => onOpenSystem?.(nudgeOpenSystemTarget(bar))}
                            iconOnly
                            aria-label="Open settings"
                            className={actionChipClass}
                          >
                            {actionIcon(action.kind)}
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
                            iconOnly={compactActionLabel(action.label).length === 0}
                            className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[3.5rem]')}
                            aria-label={compactActionLabel(action.label) || action.kind}
                          >
                            {actionIcon(action.kind)}
                            {compactActionLabel(action.label).length > 0 ? <span>{compactActionLabel(action.label)}</span> : null}
                          </ActionChipButton>
                        );
                      }
                      if (bar.primary_thread_id && (action.kind === 'expand' || action.kind === 'escalate' || action.kind === 'edit')) {
                        return (
                          <ActionChipButton
                            key={actionKey}
                            onClick={() => onOpenThread?.(bar.primary_thread_id!)}
                            iconOnly={action.kind === 'expand'}
                            className={cn(actionChipClass, action.kind === 'expand' ? '' : '[&>span]:truncate [&>span]:max-w-[3.5rem]')}
                            aria-label={action.kind === 'expand' ? 'Open thread' : undefined}
                          >
                            {actionIcon(action.kind)}
                            {action.kind === 'expand' ? null : <span>{compactActionLabel(action.label)}</span>}
                          </ActionChipButton>
                        );
                      }
                      return (
                        <ActionChipButton
                          key={actionKey}
                          iconOnly={compactActionLabel(action.label).length === 0}
                          aria-label={compactActionLabel(action.label) || action.kind}
                          disabled
                          className={cn(actionChipClass, '[&>span]:truncate [&>span]:max-w-[3rem]')}
                        >
                          {actionIcon(action.kind)}
                          {compactActionLabel(action.label).length > 0 ? <span>{compactActionLabel(action.label)}</span> : null}
                        </ActionChipButton>
                      );
                    })}
                    <ActionChipButton
                      aria-label="Defer nudge"
                      iconOnly
                      className={actionChipClass}
                      disabled={!interventionId || pendingActionKey === `${bar.id}-defer`}
                      onClick={() => {
                        if (!interventionId) return;
                        void runNudgeMutation(`${bar.id}-defer`, () => snoozeInboxItem(interventionId, 10));
                      }}
                    >
                      <RescheduleIcon size={11} />
                    </ActionChipButton>
                  </div>
              </FloatingPill>
            );
          })}
          {hiddenNudgeCount > 0 ? (
            <div className="flex justify-end px-2">
              <ActionChipButton
                onClick={() => setExpanded(true)}
                aria-label={`Show ${hiddenNudgeCount} more nudges`}
                className="!px-2.5 !py-1 !text-[10px] !tracking-[0.12em]"
              >
                +{hiddenNudgeCount} more
              </ActionChipButton>
            </div>
          ) : null}
          {expanded && orderedNudges.length > 4 ? (
            <div className="flex justify-end px-2">
              <ActionChipButton
                onClick={() => setExpanded(false)}
                aria-label="Collapse extra nudges"
                className="!px-2.5 !py-1 !text-[10px] !tracking-[0.12em]"
              >
                Show fewer
              </ActionChipButton>
            </div>
          ) : null}
        </div>
      ) : (
        <p className="px-2 text-sm text-[var(--vel-color-muted)]">No active nudges right now.</p>
      )}
    </aside>
  );
}
