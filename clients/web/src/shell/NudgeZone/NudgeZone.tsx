import { useEffect, useState } from 'react';
import type { MainView } from '../../data/operatorSurfaces';
import type { NowData, NowNudgeBarData } from '../../types';
import {
  WarningIcon,
} from '../../core/Icons';
import { uiFonts } from '../../core/Theme';
import { buildNudgeViewModel } from '../../views/now/nudgeViewModel';
import { nudgeOpenSystemTarget } from '../../views/now/nowModel';
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
  variant?: 'rail' | 'compact';
  compactInitiallyOpen?: boolean;
  railCollapsible?: boolean;
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
  variant = 'rail',
  compactInitiallyOpen = false,
  railCollapsible = false,
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
  const [compactDrawerOpen, setCompactDrawerOpen] = useState(compactInitiallyOpen);
  const [railCollapsed, setRailCollapsed] = useState(false);
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

  function toggleRailCollapse() {
    setRailCollapsed((current) => {
      const nextCollapsed = !current;
      if (nextCollapsed) {
        onMiniChatClose?.();
      }
      return nextCollapsed;
    });
  }

  const nudgeListContent = (
    <>
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
    </>
  );

  const calendarRailContent = data ? (
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
  ) : null;

  useEffect(() => {
    if (!highlightedNudgeId || highlightedNudgeNonce == null) {
      return;
    }
    if (!orderedNudgeIdsKey.split('|').includes(highlightedNudgeId)) {
      return;
    }
    const animationFrame = window.requestAnimationFrame(() => {
      setExpandedNudgeId(highlightedNudgeId);
      setFlashingNudgeId(highlightedNudgeId);
    });
    const timeoutId = window.setTimeout(() => {
      setFlashingNudgeId((current) => (current === highlightedNudgeId ? null : current));
    }, 1600);
    return () => {
      window.cancelAnimationFrame(animationFrame);
      window.clearTimeout(timeoutId);
    };
  }, [highlightedNudgeId, highlightedNudgeNonce, orderedNudgeIdsKey]);

  if (variant === 'compact') {
    return (
      <section
        id="nudges-section"
        aria-label="Nudges"
        data-nudge-zone-variant="compact"
        className="w-full px-3 py-2"
      >
        <button
          type="button"
          aria-controls="mobile-nudge-drawer"
          aria-expanded={compactDrawerOpen}
          onClick={() => setCompactDrawerOpen((current) => !current)}
          className="flex min-h-11 w-full items-center justify-between gap-3 rounded-lg border border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/80 px-3 py-2 text-left"
        >
          <span className={`${uiFonts.display} inline-flex min-w-0 items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
            <WarningIcon size={11} />
            Nudges
          </span>
          <span className="inline-flex shrink-0 items-center gap-2 text-xs text-[var(--vel-color-muted)]">
            <span className="inline-flex min-w-6 justify-center rounded-full border border-[var(--vel-color-border)] px-2 py-0.5 font-mono text-[10px] text-[var(--vel-color-text)]">
              {orderedNudges.length}
            </span>
            <span>{compactDrawerOpen ? 'Close' : 'Open'}</span>
          </span>
        </button>

        {compactDrawerOpen ? (
          <div
            id="mobile-nudge-drawer"
            role="region"
            aria-label="Mobile nudge drawer"
            className="mt-2 max-h-[min(70vh,34rem)] overflow-y-auto rounded-lg border border-[var(--vel-color-border)] bg-[color:var(--vel-color-bg)]/96 p-3"
          >
            <div className="mb-3 flex items-center justify-between gap-3">
              <p className={`${uiFonts.display} text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
                Active nudges ({orderedNudges.length})
              </p>
              <button
                type="button"
                onClick={() => setCompactDrawerOpen(false)}
                className="min-h-9 rounded-lg border border-[var(--vel-color-border)] px-3 text-xs text-[var(--vel-color-muted)]"
              >
                Dismiss
              </button>
            </div>
            {nudgeListContent}
          </div>
        ) : null}

        {miniChatOpen ? (
          <MiniChatPanel
            miniChatThreadId={miniChatThreadId}
            onMiniChatClose={onMiniChatClose}
            onMiniChatThreadSelect={onMiniChatThreadSelect}
          />
        ) : null}
      </section>
    );
  }

  return (
    <aside
      id="nudges-section"
      aria-label="Nudges"
      data-nudge-zone-variant="rail"
      data-nudge-rail-collapsed={railCollapsed ? 'true' : undefined}
      className="relative min-h-[calc(100vh-6rem)] flex flex-col gap-2 overflow-visible pl-6 pr-3"
    >
      <div className="flex items-center justify-between gap-3 px-2">
        <p className={`${uiFonts.display} inline-flex items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
          <WarningIcon size={11} />
          NUDGES ({orderedNudges.length})
          {deferredCount > 0 ? <span className="ml-2 text-[var(--vel-color-muted)]">| DEFERRED ({deferredCount})</span> : null}
        </p>
        {railCollapsible ? (
          <button
            type="button"
            aria-expanded={!railCollapsed}
            aria-controls="nudge-rail-content"
            onClick={toggleRailCollapse}
            className="min-h-9 shrink-0 rounded-lg border border-[var(--vel-color-border)] px-3 text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)] transition hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]"
          >
            {railCollapsed ? 'Open' : 'Collapse'}
          </button>
        ) : null}
      </div>

      {railCollapsed ? (
        <div
          id="nudge-rail-content"
          role="region"
          aria-label="Nudge rail collapsed summary"
          className="mx-2 rounded-lg border border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/45 px-3 py-2 text-xs text-[var(--vel-color-muted)]"
        >
          Docked rail collapsed. {orderedNudges.length} active.
        </div>
      ) : (
        <div id="nudge-rail-content">
          {nudgeListContent}
          {calendarRailContent}
        </div>
      )}

      {!railCollapsed && miniChatOpen ? (
        <MiniChatPanel
          miniChatThreadId={miniChatThreadId}
          onMiniChatClose={onMiniChatClose}
          onMiniChatThreadSelect={onMiniChatThreadSelect}
        />
      ) : null}
    </aside>
  );
}
