import { useEffect, useMemo, useState } from 'react';
import {
  contextQueryKeys,
  loadNow,
  rescheduleNowCalendarEvent,
  rescheduleNowTasksToToday,
} from '../../data/context';
import { invalidateQuery, setQueryData, useQuery } from '../../data/query';
import type { MainView } from '../../data/operatorSurfaces';
import type { IntegrationsData, NowData, NowEventData, NowNudgeBarData } from '../../types';
import {
  acknowledgeInboxItem,
  invalidateInboxQueries,
  snoozeInboxItem,
} from '../../data/chat';
import { loadIntegrations, updateGoogleCalendarIntegration } from '../../data/operator';
import {
  CalendarIcon,
  FileIcon,
  AttachmentIcon,
  ChevronLeftIcon,
  PersonIcon,
  ThreadsIcon,
  CloseIcon,
  MinimizeIcon,
  WarningIcon,
} from '../../core/Icons';
import { GoogleMeetBrandIcon, ZoomBrandIcon } from '../../core/Icons';
import { uiFonts } from '../../core/Theme';
import { nudgeOpenSystemTarget } from '../../views/now/nowModel';
import {
  NudgeActionIcon,
  nudgeActionAriaLabel,
  nudgeActionButtonLabel,
  nudgeActionToneClass,
  nudgeSurfaceTone,
} from '../../views/now/nowNudgePresentation';
import type { SystemNavigationTarget } from '../../views/system';
import { cn } from '../../core/cn';
import { ActionChipButton } from '../../core/FilterToggleTag';
import { SurfaceSpinner } from '../../core/SurfaceState';
import { ThreadView } from '../../views/threads';
import { CoreSetupChecklist, type CoreChecklistItem } from './CoreSetupChecklist';
import { NudgeCard } from './NudgeCard';

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

const actionChipClass =
  '!min-h-[1.1rem] !gap-1.5 !rounded-full !px-2 !py-[0.2rem] !text-[9px] !tracking-[0.1em] opacity-90';
const nudgeActionChipClass =
  `${actionChipClass} w-full justify-center`;

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

function formatCalendarDayLabel(timestamp: number, timezone: string): string {
  return new Intl.DateTimeFormat(undefined, {
    weekday: 'short',
    month: 'short',
    day: 'numeric',
    timeZone: timezone,
  }).format(new Date(timestamp * 1000));
}

function formatCalendarTime(timestamp: number, timezone: string): string {
  return new Intl.DateTimeFormat(undefined, {
    hour: 'numeric',
    minute: '2-digit',
    timeZone: timezone,
  }).format(new Date(timestamp * 1000));
}

function calendarEventDuration(event: NowEventData): number {
  if (!event.end_ts || event.end_ts <= event.start_ts) {
    return 30 * 60;
  }
  return event.end_ts - event.start_ts;
}

function formatCalendarEventLabel(event: NowEventData, timezone: string): string {
  if (event.all_day) {
    return 'All day';
  }
  const start = formatCalendarTime(event.start_ts, timezone);
  const end = event.end_ts ? formatCalendarTime(event.end_ts, timezone) : null;
  return end ? `${start}-${end}` : start;
}

function compactAttendeesLabel(event: NowEventData): string | null {
  const attendees = event.attendees ?? [];
  if (attendees.length === 0) {
    return null;
  }
  if (attendees.length === 1) {
    return attendees[0] ?? null;
  }
  if (attendees.length === 2) {
    return `${attendees[0]}, ${attendees[1]}`;
  }
  return `${attendees[0]}, ${attendees[1]} +${attendees.length - 2}`;
}

function compactNotesLabel(event: NowEventData): string | null {
  return event.notes?.replace(/\s+/g, ' ').trim() ?? null;
}

function videoProviderLabel(event: NowEventData): string | null {
  switch (event.video_provider) {
    case 'google_meet':
      return 'Google Meet';
    case 'zoom':
      return 'Zoom';
    default:
      return null;
  }
}

function CalendarEventTable({
  sectionEvents,
  timezone,
  pendingEventId,
  draggedEventKey,
  setDraggedEventKey,
  visibleEvents,
  visibleFollowingDayEvents,
  onRescheduleEvent,
}: {
  sectionEvents: NowEventData[];
  timezone: string;
  pendingEventId: string | null;
  draggedEventKey: string | null;
  setDraggedEventKey: (value: string | null) => void;
  visibleEvents: NowEventData[];
  visibleFollowingDayEvents: NowEventData[];
  onRescheduleEvent: (event: NowEventData, startTs: number) => void;
}) {
  const calendarDragMimeType = 'application/x-vel-calendar-event';

  return (
    <table className="w-full table-fixed border-collapse text-left text-[11px]">
      <thead className="text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]">
        <tr>
          <th className="w-20 px-1 py-1.5 font-medium">Time</th>
          <th className="px-1 py-1.5 font-medium">Event</th>
        </tr>
      </thead>
      <tbody>
        {sectionEvents.map((event) => {
          const eventKey = event.event_id ?? `${event.title}-${event.start_ts}`;
          const isPending = event.event_id != null && pendingEventId === event.event_id;
          const attendeesLabel = compactAttendeesLabel(event);
          const notesLabel = compactNotesLabel(event);
          const conferenceLabel = videoProviderLabel(event);
          return (
            <tr
              key={`${eventKey}-${event.start_ts}`}
              data-event-title={event.title}
              className="border-t border-[var(--vel-color-border-subtle)]/80 align-top transition-[outline]"
              onDragOver={(dragEvent) => {
                if (!draggedEventKey) {
                  return;
                }
                dragEvent.preventDefault();
              }}
              onDrop={(dragEvent) => {
                dragEvent.preventDefault();
                const droppedEventKey =
                  dragEvent.dataTransfer.getData(calendarDragMimeType) ||
                  dragEvent.dataTransfer.getData('text/plain') ||
                  draggedEventKey;
                const draggedEvent = [...visibleEvents, ...visibleFollowingDayEvents].find(
                  (item) => (item.event_id ?? `${item.title}-${item.start_ts}`) === droppedEventKey,
                );
                setDraggedEventKey(null);
                if (!draggedEvent) {
                  return;
                }
                onRescheduleEvent(draggedEvent, event.start_ts);
              }}
            >
              <td className="px-1 py-1.5 font-medium text-[var(--vel-color-muted)]">{formatCalendarEventLabel(event, timezone)}</td>
              <td className="px-1 py-1.5">
                <div
                  draggable={Boolean(event.event_id) && !isPending}
                  onDragStart={(dragEvent) => {
                    setDraggedEventKey(eventKey);
                    dragEvent.dataTransfer.effectAllowed = 'move';
                    dragEvent.dataTransfer.setData(calendarDragMimeType, eventKey);
                    dragEvent.dataTransfer.setData('text/plain', eventKey);
                  }}
                  onDragEnd={() => setDraggedEventKey(null)}
                  className={cn(
                    'transition',
                    event.event_id ? 'cursor-grab active:cursor-grabbing' : null,
                    isPending ? 'opacity-60' : null,
                  )}
                >
                  <div className="flex min-w-0 items-baseline gap-1.5">
                    {event.event_url ? (
                      <a
                        href={event.event_url}
                        target="_blank"
                        rel="noreferrer"
                        onClick={(clickEvent) => clickEvent.stopPropagation()}
                        className="min-w-0 truncate text-[12px] font-medium leading-4 text-[var(--vel-color-text)] underline-offset-2 transition hover:text-[var(--vel-color-accent-soft)] hover:underline"
                      >
                        {event.title}
                      </a>
                    ) : (
                      <p className="min-w-0 truncate text-[12px] font-medium leading-4 text-[var(--vel-color-text)]">{event.title}</p>
                    )}
                    {event.calendar_name ? (
                      <span className="min-w-0 shrink truncate text-[10px] uppercase tracking-[0.08em] text-[var(--vel-color-muted)] opacity-60">
                        {event.calendar_name}
                      </span>
                    ) : null}
                  </div>
                  {event.rescheduled ? (
                    <div className="mt-0.5 flex flex-wrap items-center gap-1 text-[10px] uppercase tracking-[0.08em] text-[var(--vel-color-muted)]">
                      <span>Moved in Vel</span>
                    </div>
                  ) : null}
                  {attendeesLabel || notesLabel || event.attachment_url || event.video_url ? (
                    <div className="mt-1 space-y-1 text-[10px] leading-4 text-[var(--vel-color-muted)]">
                      {attendeesLabel ? (
                        <p className="truncate">
                          <span className="mr-1 inline-flex align-middle text-[var(--vel-color-dim)]">
                            <PersonIcon size={11} />
                          </span>
                          {attendeesLabel}
                        </p>
                      ) : null}
                      {notesLabel ? (
                        <p className="line-clamp-2">
                          <span className="mr-1 inline-flex align-middle text-[var(--vel-color-dim)]">
                            <FileIcon size={11} />
                          </span>
                          {notesLabel}
                        </p>
                      ) : null}
                      {event.attachment_url ? (
                        <a
                          href={event.attachment_url}
                          target="_blank"
                          rel="noreferrer"
                          className="inline-flex items-center gap-1 text-[var(--vel-color-muted)] transition hover:text-[var(--vel-color-accent-soft)]"
                        >
                          <AttachmentIcon size={11} />
                        </a>
                      ) : null}
                      {event.video_url ? (
                        <a
                          href={event.video_url}
                          target="_blank"
                          rel="noreferrer"
                          className="inline-flex items-center gap-1.5 rounded-full border border-[var(--vel-color-border-subtle)] px-2 py-0.5 text-[9px] uppercase tracking-[0.08em] text-[var(--vel-color-text)] transition hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-accent-soft)]"
                        >
                          {event.video_provider === 'google_meet' ? (
                            <GoogleMeetBrandIcon size={11} />
                          ) : event.video_provider === 'zoom' ? (
                            <ZoomBrandIcon size={11} />
                          ) : null}
                          <span>{conferenceLabel ?? 'Open video'}</span>
                        </a>
                      ) : null}
                    </div>
                  ) : null}
                </div>
              </td>
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}

function CalendarSection({
  computedAt,
  timezone,
  events,
  followingDayEvents = [],
  integrations,
  pendingToggleId,
  pendingEventId,
  onToggleCalendar,
  onRescheduleEvent,
}: {
  computedAt: number;
  timezone: string;
  events: NowEventData[];
  followingDayEvents?: NowEventData[];
  integrations: IntegrationsData | null;
  pendingToggleId: string | null;
  pendingEventId: string | null;
  onToggleCalendar: (calendarId: string | null) => void;
  onRescheduleEvent: (event: NowEventData, startTs: number) => void;
}) {
  const [draggedEventKey, setDraggedEventKey] = useState<string | null>(null);
  const [showFollowingDay, setShowFollowingDay] = useState(false);
  const googleCalendar = integrations?.google_calendar ?? null;
  const visibleCalendars = useMemo(
    () => googleCalendar?.calendars.filter((calendar) => calendar.sync_enabled && calendar.display_enabled) ?? [],
    [googleCalendar],
  );
  const visibleCalendarIds = useMemo(
    () => new Set(visibleCalendars.map((calendar) => calendar.id)),
    [visibleCalendars],
  );
  const visibleEvents = useMemo(
    () => events.filter((event) => !event.calendar_id || visibleCalendarIds.has(event.calendar_id)),
    [events, visibleCalendarIds],
  );
  const visibleFollowingDayEvents = useMemo(
    () => followingDayEvents.filter((event) => !event.calendar_id || visibleCalendarIds.has(event.calendar_id)),
    [followingDayEvents, visibleCalendarIds],
  );
  const followingDayAnchorTs = visibleFollowingDayEvents[0]?.start_ts ?? (computedAt + (24 * 60 * 60));
  const currentDayLabel = formatCalendarDayLabel(computedAt, timezone);
  const followingDayLabel = formatCalendarDayLabel(followingDayAnchorTs, timezone);
  const contextualCalendarDay = (label: string) => (label === 'Today' || label === 'Tomorrow' ? label.toLowerCase() : label);

  return (
    <section
      id="sidebar-calendar"
      aria-label="Calendar"
      className="space-y-3 opacity-65 transition-opacity hover:opacity-100"
    >
      <div className="flex items-center justify-between gap-3">
        <p className={`${uiFonts.display} inline-flex items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
          <CalendarIcon size={11} />
          Calendar
        </p>
        {visibleFollowingDayEvents.length > 0 ? (
          <label className="inline-flex cursor-pointer items-center gap-1.5 select-none">
            <input
              type="checkbox"
              checked={showFollowingDay}
              onChange={() => setShowFollowingDay((current) => !current)}
              className="h-2.5 w-2.5 cursor-pointer appearance-none rounded-[2px] border border-[var(--vel-color-border)] bg-transparent checked:border-[var(--vel-color-accent-border)] checked:bg-[var(--vel-color-accent-strong)]"
            />
            <span className={`${uiFonts.mono} text-[8px] uppercase tracking-[0.1em] text-[var(--vel-color-muted)]`}>
              Next day
            </span>
          </label>
        ) : null}
      </div>

      {googleCalendar ? (
        <div className="mt-3 flex flex-wrap gap-1.5">
          <ActionChipButton
            onClick={() => onToggleCalendar(null)}
            disabled={pendingToggleId === '__all__'}
            className={cn(
              actionChipClass,
              googleCalendar.calendars
                .filter((calendar) => calendar.sync_enabled)
                .every((calendar) => calendar.display_enabled)
                ? '!border-[var(--vel-color-accent-border)] !bg-[color:var(--vel-color-panel-2)]'
                : null,
            )}
          >
            <span>All</span>
          </ActionChipButton>
          {visibleCalendars.map((calendar) => (
            <ActionChipButton
              key={calendar.id}
              onClick={() => onToggleCalendar(calendar.id)}
              disabled={pendingToggleId === calendar.id}
              className={cn(
                actionChipClass,
                calendar.display_enabled
                  ? '!border-[var(--vel-color-accent-border)] !bg-[color:var(--vel-color-panel-2)]'
                  : null,
              )}
            >
              <span>{calendar.summary}</span>
            </ActionChipButton>
          ))}
        </div>
      ) : null}

      <div className="space-y-3">
        <p className={`${uiFonts.mono} text-right text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]`}>
          {currentDayLabel}
        </p>
        {visibleEvents.length === 0 ? (
          <p className="text-xs text-[var(--vel-color-muted)] opacity-60">
            {`No calendar events for ${contextualCalendarDay(currentDayLabel)}.`}
          </p>
        ) : (
          <CalendarEventTable
            sectionEvents={visibleEvents}
            timezone={timezone}
            pendingEventId={pendingEventId}
            draggedEventKey={draggedEventKey}
            setDraggedEventKey={setDraggedEventKey}
            visibleEvents={visibleEvents}
            visibleFollowingDayEvents={visibleFollowingDayEvents}
            onRescheduleEvent={onRescheduleEvent}
          />
        )}
        {showFollowingDay ? (
          <div className="space-y-2">
            <p className={`${uiFonts.mono} text-right text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]`}>
              {followingDayLabel}
            </p>
            {visibleFollowingDayEvents.length > 0 ? (
              <CalendarEventTable
                sectionEvents={visibleFollowingDayEvents}
                timezone={timezone}
                pendingEventId={pendingEventId}
                draggedEventKey={draggedEventKey}
                setDraggedEventKey={setDraggedEventKey}
                visibleEvents={visibleEvents}
                visibleFollowingDayEvents={visibleFollowingDayEvents}
                onRescheduleEvent={onRescheduleEvent}
              />
            ) : (
              <p className="text-xs text-[var(--vel-color-muted)]">
                {`No calendar events for ${contextualCalendarDay(followingDayLabel)}.`}
              </p>
            )}
          </div>
        ) : null}
      </div>
    </section>
  );
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
  const [pendingActionKey, setPendingActionKey] = useState<string | null>(null);
  const [pendingCalendarToggleId, setPendingCalendarToggleId] = useState<string | null>(null);
  const [pendingCalendarEventId, setPendingCalendarEventId] = useState<string | null>(null);
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const integrationsKey = useMemo(() => ['integrations'] as const, []);
  const { data, loading, error } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: integrations } = useQuery<IntegrationsData | null>(
    integrationsKey,
    async () => {
      const response = await loadIntegrations();
      return response.ok ? response.data ?? null : null;
    },
  );
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

  async function toggleCalendar(calendarId: string | null) {
    if (!integrations) {
      return;
    }
    const googleCalendar = integrations.google_calendar;
    const pendingId = calendarId ?? '__all__';
    setPendingCalendarToggleId(pendingId);
    try {
    const patch = calendarId == null
        ? {
          calendar_settings: googleCalendar.calendars.map((calendar) => ({
            id: calendar.id,
            display_enabled: calendar.sync_enabled,
          })),
        }
        : {
          calendar_settings: googleCalendar.calendars
            .filter((calendar) => calendar.id === calendarId)
            .map((calendar) => ({
              id: calendar.id,
              display_enabled: !calendar.display_enabled,
            })),
        };
      const response = await updateGoogleCalendarIntegration(patch);
      if (!response.ok) {
        return;
      }
      setQueryData(integrationsKey, response.data ?? null);
      invalidateQuery(nowKey, { refetch: true });
    } finally {
      setPendingCalendarToggleId(null);
    }
  }

  async function moveCalendarEvent(event: NowEventData, startTs: number) {
    if (!event.event_id) {
      return;
    }
    setPendingCalendarEventId(event.event_id);
    try {
      const duration = calendarEventDuration(event);
      const response = await rescheduleNowCalendarEvent({
        event_id: event.event_id,
        calendar_id: event.calendar_id,
        start_ts: startTs,
        end_ts: event.end_ts ? startTs + duration : null,
      });
      if (!response.ok) {
        return;
      }
      setQueryData(nowKey, response.data ?? null);
    } finally {
      setPendingCalendarEventId(null);
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
            const tone = nudgeSurfaceTone(bar);
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
                  const actionTone = nudgeActionToneClass(action.kind);
                  if (action.kind.startsWith('open_settings')) {
                    return (
                      <ActionChipButton
                        key={actionKey}
                        onClick={() => onOpenSystem?.(nudgeOpenSystemTarget(bar, action))}
                        aria-label={ariaLabel}
                        className={cn(nudgeActionChipClass, actionTone)}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        {label ? <span>{label}</span> : null}
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
                        className={cn(nudgeActionChipClass, actionTone)}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        {label ? <span>{label}</span> : null}
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
                        className={cn(nudgeActionChipClass, actionTone)}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        {label ? <span>{label}</span> : null}
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
                        className={cn(nudgeActionChipClass, actionTone)}
                        aria-label={ariaLabel}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        {label ? <span>{label}</span> : null}
                      </ActionChipButton>
                    );
                  }
                  if (bar.primary_thread_id && (action.kind === 'expand' || action.kind === 'escalate' || action.kind === 'edit' || action.kind === 'open_thread')) {
                    return (
                      <ActionChipButton
                        key={actionKey}
                        onClick={() => onOpenThread?.(bar.primary_thread_id!)}
                        className={cn(nudgeActionChipClass, actionTone)}
                        aria-label={ariaLabel}
                      >
                        <NudgeActionIcon kind={action.kind} size={11} />
                        {label ? <span>{label}</span> : null}
                      </ActionChipButton>
                    );
                  }
                  return (
                    <ActionChipButton
                      key={actionKey}
                      aria-label={ariaLabel}
                      disabled
                      className={cn(nudgeActionChipClass, actionTone)}
                    >
                      <NudgeActionIcon kind={action.kind} size={11} />
                      {label ? <span>{label}</span> : null}
                    </ActionChipButton>
                  );
                })}
                <ActionChipButton
                  aria-label={`Defer (${bar.title}) · ${bar.id}`}
                  className={cn(nudgeActionChipClass, nudgeActionToneClass('snooze'))}
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
              <NudgeCard
                key={bar.id}
                bar={bar}
                tone={tone}
                isExpanded={isExpanded}
                isFlashing={flashingNudgeId === bar.id}
                timestampLabel={formatNudgeAge(bar.timestamp)}
                onToggle={() => toggleNudgeExpansion(bar.id)}
                actionButtons={actionButtons}
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
          <CalendarSection
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
        <section
          aria-label="Mini chat panel"
          className="absolute inset-x-0 bottom-2 z-40 flex max-h-[calc(100%-1rem)] w-full flex-col overflow-hidden border-b-[7px] border-b-[#2a160c] bg-[color:var(--vel-color-bg)]/95 py-1 font-mono ring-1 ring-[var(--vel-color-border)]/85 shadow-[0_2px_8px_rgba(0,0,0,0.26)]"
        >
          <div className="flex items-center justify-between gap-1 border-b border-[var(--vel-color-border)] px-2 py-1">
            <p className="inline-flex min-w-0 items-center gap-1.5 whitespace-nowrap text-[10px] uppercase leading-none tracking-[0.14em] text-[var(--vel-color-accent-soft)]">
              <ThreadsIcon size={14} />
              TERMINAL CHAT
            </p>
            <div className="flex shrink-0 items-center gap-1">
              <button
                type="button"
                onClick={() => onMiniChatClose?.()}
                aria-label="Return to GUI mode"
                className="inline-flex h-5 min-w-[2.35rem] shrink-0 items-center justify-center gap-1 whitespace-nowrap rounded border border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/70 px-1 !text-[8px] uppercase leading-none tracking-[0.1em] text-[var(--vel-color-accent-soft)] transition hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]"
              >
                <ChevronLeftIcon size={9} />
                <span>GUI</span>
              </button>
              <button
                type="button"
                onClick={() => onMiniChatClose?.()}
                aria-label="Minimize mini chat"
                className="mt-0.5 inline-flex h-6 w-6 items-center justify-center text-[var(--vel-color-accent-soft)] transition hover:text-[var(--vel-color-text)]"
              >
                <MinimizeIcon size={11} />
              </button>
              <button
                type="button"
                onClick={() => onMiniChatClose?.()}
                aria-label="Close mini chat"
                className="inline-flex h-6 w-6 items-center justify-center text-[var(--vel-color-accent-soft)] transition hover:text-[var(--vel-color-text)]"
              >
                <CloseIcon size={11} />
              </button>
            </div>
          </div>
          <ThreadView
            miniMode
            className="min-h-0 flex-1 px-1 pb-1"
            conversationId={miniChatThreadId ?? null}
            onMiniChatClose={onMiniChatClose}
            onSelectConversation={(conversationId) => {
              onMiniChatThreadSelect?.(conversationId);
            }}
          />
        </section>
      ) : null}
    </aside>
  );
}
