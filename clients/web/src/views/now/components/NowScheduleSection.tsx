import type { NowEventData, NowScheduleData } from '../../../types';
import {
  PanelSectionHeaderBand,
  PanelSectionHeaderLead,
} from '../../../core/PanelChrome';
import { FilterDenseTag } from '../../../core/FilterToggleTag';
import { CalendarIcon, ClockIcon } from '../../../core/Icons';
import { NowItemRowLayout, NowItemRowShell } from '../../../core/NowItemRow';
import { formatTime } from '../nowModel';

function scheduleRows(schedule: NowScheduleData): NowEventData[] {
  if (schedule.upcoming_events.length > 0) {
    return schedule.upcoming_events.slice(0, 3);
  }
  return schedule.next_event ? [schedule.next_event] : [];
}

function eventTimeLabel(event: NowEventData, timezone: string): string {
  const start = formatTime(event.start_ts, timezone);
  const end = event.end_ts ? formatTime(event.end_ts, timezone) : null;
  return end ? `${start} - ${end}` : start;
}

export function NowScheduleSection({
  schedule,
  timezone,
}: {
  schedule: NowScheduleData;
  timezone: string;
}) {
  const events = scheduleRows(schedule);

  return (
    <section className="space-y-4">
      <div className="space-y-2">
        <PanelSectionHeaderBand mode="section-header">
          <PanelSectionHeaderLead>
            <h2 className="text-lg font-medium text-zinc-100">Calendar</h2>
          </PanelSectionHeaderLead>
        </PanelSectionHeaderBand>
        <p className="text-xs uppercase tracking-[0.14em] text-zinc-500">
          Today-first canonical constraint surface
        </p>
      </div>

      {events.length === 0 ? (
        <p className="rounded-2xl border border-zinc-800 bg-zinc-950/40 px-4 py-4 text-sm text-zinc-500">
          {schedule.empty_message ?? 'No calendar commitments are surfaced right now.'}
        </p>
      ) : (
        <div className="space-y-2">
          {events.map((event) => (
            <NowItemRowShell
              key={`${event.title}-${event.start_ts}`}
              surface="muted"
              shell="laneRow"
            >
              <NowItemRowLayout
                leading={
                  <span className="mt-0.5 flex h-6 w-6 shrink-0 self-start items-center justify-center rounded border border-zinc-800 bg-zinc-950 text-zinc-500">
                    <CalendarIcon size={12} />
                  </span>
                }
              >
                <div className="flex min-w-0 items-center justify-between gap-2">
                  <p className="min-w-0 flex-1 truncate text-sm font-medium leading-tight tracking-tight text-zinc-100">
                    {event.title}
                  </p>
                  <div className="flex min-w-0 shrink-0 flex-nowrap items-center justify-end gap-x-1.5 overflow-x-auto [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
                    <FilterDenseTag tone="neutral">
                      <span aria-hidden className="inline-flex shrink-0 items-center">
                        <ClockIcon size={10} />
                      </span>
                      {eventTimeLabel(event, timezone)}
                    </FilterDenseTag>
                    {event.location ? (
                      <FilterDenseTag tone="neutral">
                        {event.location}
                      </FilterDenseTag>
                    ) : null}
                  </div>
                </div>
              </NowItemRowLayout>
            </NowItemRowShell>
          ))}
        </div>
      )}
    </section>
  );
}
