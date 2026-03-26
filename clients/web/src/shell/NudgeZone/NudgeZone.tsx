import { useEffect, useMemo, useState } from 'react';
import type { MainView } from '../../data/operatorSurfaces';
import type { NowData, NowNudgeBarData } from '../../types';
import { nudgeOpenSystemTarget } from '../../views/now/nowModel';
import {
  WarningIcon,
} from '../../core/Icons';
import { uiFonts } from '../../core/Theme';
import { buildNudgeViewModel } from '../../views/now/nudgeViewModel';
import type { SystemNavigationTarget } from '../../views/system';
import { cn } from '../../core/cn';
import { SurfaceSpinner } from '../../core/SurfaceState';
import { NudgeActions } from './NudgeActions';
import { CalendarRail } from './CalendarRail';
import { CoreSetupChecklist, type CoreChecklistItem } from './CoreSetupChecklist';
import { MiniChatPanel } from './MiniChatPanel';
import { NudgeCard } from './NudgeCard';
import { useNudgeZoneData } from './useNudgeZoneData';

interface NudgeZoneProps {
  activeView: MainView;
  extraNudges?: NowNudgeBarData[];
  highlightedNudgeId?: string | null;
  highlightedNudgeNonce?: number | null;
  onOpenThread?: (conversationId: string) => void;
  miniChatOpen?: boolean;
  miniChatThreadId?: string | null;
  onMiniChatThreadSelect?: (conversationId: string) => void;
  onMiniChatClose?: () => void;
  onOpenSystem?: (target?: SystemNavigationTarget) => void;
}

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
  miniChatOpen = false,
  miniChatThreadId,
  onMiniChatThreadSelect,
  onMiniChatClose,
  onOpenSystem,
}: NudgeZoneProps) {
  const [expandedNudgeId, setExpandedNudgeId] = useState<string | null>(null);
  const [flashingNudgeId, setFlashingNudgeId] = useState<string | null>(null);
  const {
    data,
    loading,
    error,
    integrations,
    moveCalendarEvent,
    pendingActionKey,
    pendingCalendarEventId,
    pendingCalendarToggleId,
    runNudgeMutation,
    toggleCalendar,
  } = useNudgeZoneData();
  const nudgeBars = [...extraNudges, ...(data?.nudge_bars ?? [])];
  const orderedNudges = nudgeBars
    .filter((bar, index) => nudgeBars.findIndex((item) => item.id === bar.id) === index)
    .sort((a, b) => Number(b.urgent) - Number(a.urgent));
  const showingLocalNudgeFallback = Boolean(error) && !data;
  const deferredCount = (data?.header?.buckets ?? []).find((bucket) => bucket.kind === 'snoozed')?.count ?? 0;
  const orderedNudgeIdsKey = orderedNudges.map((bar) => bar.id).join('|');

  function toggleNudgeExpansion(nudgeId: string) {
    setExpandedNudgeId((current) => (current === nudgeId ? null : nudgeId));
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

  return (
    <aside id="nudges-section" aria-label="Nudges" className="relative min-h-[calc(100vh-6rem)] flex flex-col gap-2 overflow-visible pl-6 pr-3">
      <div className="flex items-center justify-between gap-3 px-2">
        <p className={`${uiFonts.display} inline-flex items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
          <WarningIcon size={11} />
          NUDGES ({orderedNudges.length})
          {deferredCount > 0 ? <span className="ml-2 text-[var(--vel-color-muted)]">| DEFERRED ({deferredCount})</span> : null}
        </p>
      </div>

      {loading && orderedNudges.length === 0 ? (
        <div className="px-2 py-1 text-sm text-[var(--vel-color-muted)]">
          <SurfaceSpinner className="mb-1 h-4 w-4" />
          <p>Loading signals…</p>
        </div>
      ) : orderedNudges.length > 0 ? (
        <div className={cn('flex flex-col', expandedNudgeId ? 'gap-3' : 'gap-2')}>
          {showingLocalNudgeFallback ? (
            <p className="px-2 text-xs text-[var(--vel-color-muted)] opacity-70">
              Live context is unavailable. Showing local nudges only.
            </p>
          ) : null}
          {orderedNudges.map((bar) => {
            const viewModel = buildNudgeViewModel(bar);
            const isExpanded = expandedNudgeId === bar.id;
            const interventionId = interventionIdForBar(bar, data ?? null);
            const coreSetupChecklist = bar.id === 'core_setup_required'
              ? bar.actions
                .map((action) => ({ action, checklist: parseCoreSetupChecklistItem(action) }))
                .filter((item): item is { action: NowNudgeBarData['actions'][number]; checklist: CoreChecklistItem } => item.checklist !== null)
              : [];
            return (
              <NudgeCard
                key={bar.id}
                bar={bar}
                viewModel={viewModel}
                isExpanded={isExpanded}
                isFlashing={flashingNudgeId === bar.id}
                timestampLabel={formatNudgeAge(bar.timestamp)}
                onToggle={() => toggleNudgeExpansion(bar.id)}
                actionButtons={(
                  <NudgeActions
                    bar={bar}
                    activeView={activeView}
                    interventionId={interventionId}
                    pendingActionKey={pendingActionKey}
                    onOpenThread={onOpenThread}
                    onOpenSystem={onOpenSystem}
                    runMutation={runNudgeMutation}
                  />
                )}
                checklistContent={
                  <CoreSetupChecklist
                    bar={bar}
                    items={coreSetupChecklist}
                    onOpenSystemAction={(action) => {
                      onOpenSystem?.(nudgeOpenSystemTarget(bar, action));
                    }}
                  />
                }
              />
            );
          })}
        </div>
      ) : showingLocalNudgeFallback ? (
        <p className="px-2 text-sm text-[var(--vel-color-muted)] opacity-70">
          Live context is unavailable, and there are no local nudges to show.
        </p>
      ) : (
        <p className="px-2 text-sm text-[var(--vel-color-muted)]">No active nudges right now.</p>
      )}

      {data ? (
        <div className="mt-5 space-y-4">
          <div className="border-t border-[var(--vel-color-border)]/85" aria-hidden="true" />
          <CalendarRail
            computedAt={data.computed_at}
            timezone={data.timezone}
            events={data.schedule.upcoming_events}
            followingDayEvents={data.schedule.following_day_events}
            integrations={integrations ?? null}
            pendingToggleId={pendingCalendarToggleId}
            pendingEventId={pendingCalendarEventId}
            onToggleCalendar={toggleCalendar}
            onRescheduleEvent={moveCalendarEvent}
          />
        </div>
      ) : null}
      {miniChatOpen ? (
        <MiniChatPanel
          miniChatThreadId={miniChatThreadId}
          onMiniChatClose={onMiniChatClose}
          onMiniChatThreadSelect={onMiniChatThreadSelect}
        />
      ) : null}
    </aside>
  );
}
