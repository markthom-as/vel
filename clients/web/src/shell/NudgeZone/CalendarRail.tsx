import { useMemo, useState } from 'react';
import type { IntegrationsData, NowEventData } from '../../types';
import { ActionChipButton } from '../../core/FilterToggleTag';
import {
  AttachmentIcon,
  FileIcon,
  PersonIcon,
} from '../../core/Icons';
import { GoogleMeetBrandIcon, ZoomBrandIcon } from '../../core/Icons';
import { SemanticIcon } from '../../core/Icons/SemanticIcon';
import { cn } from '../../core/cn';
import { uiFonts } from '../../core/Theme';
import { resolveCalendarSemantic } from '../../core/Theme/semanticRegistry';

const calendarActionChipClass =
  '!min-h-[1.1rem] !gap-1.5 !rounded-full !px-2 !py-[0.2rem] !text-[9px] !tracking-[0.1em] opacity-90';

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
              <td className="px-1 py-1.5 font-medium text-[var(--vel-color-muted)]">
                {formatCalendarEventLabel(event, timezone)}
              </td>
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
                      <p className="min-w-0 truncate text-[12px] font-medium leading-4 text-[var(--vel-color-text)]">
                        {event.title}
                      </p>
                    )}
                    {event.calendar_name ? (
                      <span className="min-w-0 shrink truncate text-[10px] uppercase tracking-[0.08em] text-[var(--vel-color-muted)] opacity-60">
                        {resolveCalendarSemantic(event.calendar_name).label}
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

export function CalendarRail({
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
  const calendarHeader = resolveCalendarSemantic('Calendar');
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
          <SemanticIcon icon={calendarHeader.icon} size={11} />
          {calendarHeader.label}
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
          {(() => {
            const allSemantic = resolveCalendarSemantic('All', googleCalendar.calendars
              .filter((calendar) => calendar.sync_enabled)
              .every((calendar) => calendar.display_enabled));
            return (
          <ActionChipButton
            onClick={() => onToggleCalendar(null)}
            disabled={pendingToggleId === '__all__'}
            className={cn(calendarActionChipClass, allSemantic.chipClassName)}
          >
            <span>{allSemantic.label}</span>
          </ActionChipButton>
            );
          })()}
          {visibleCalendars.map((calendar) => {
            const semantic = resolveCalendarSemantic(calendar.summary, calendar.display_enabled);
            return (
            <ActionChipButton
              key={calendar.id}
              onClick={() => onToggleCalendar(calendar.id)}
              disabled={pendingToggleId === calendar.id}
              className={cn(calendarActionChipClass, semantic.chipClassName)}
            >
              <span>{semantic.label}</span>
            </ActionChipButton>
            );
          })}
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
