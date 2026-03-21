import { useState } from 'react';
import type { NowData, NowTaskData } from '../../../types';
import {
  CalendarIcon,
  ClockIcon,
  OpenThreadIcon,
  RescheduleIcon,
  TagIcon,
} from '../../../core/Icons';
import { SurfaceActionChip, SurfaceTagChip } from '../../../core/SurfaceChips';
import { formatTaskDate } from '../nowModel';
import { taskKindIcon } from '../nowNudgePresentation';

export function CompactTaskLaneRow({
  item,
  metadata = null,
  emphasis = 'default',
  pending = false,
  feedback,
  onOpenThread,
  onComplete,
}: {
  item: NonNullable<NowData['task_lane']>['active'] extends infer T ? Exclude<T, null> : never;
  metadata?: NowTaskData | null;
  emphasis?: 'active' | 'default' | 'completed';
  pending?: boolean;
  feedback?: { status: 'success' | 'error'; message: string };
  onOpenThread?: () => void;
  onComplete?: () => void;
}) {
  const completed = emphasis === 'completed' || item.state === 'completed';
  const canComplete = Boolean(onComplete) && !completed;
  const [rescheduleOpen, setRescheduleOpen] = useState(false);

  return (
    <div
      className={`rounded-xl border px-4 py-2.5 ${
        emphasis === 'active'
          ? 'border-[#ff5a2f]/50 bg-[#4a1a14]/20 shadow-[0_0_26px_rgba(255,90,47,0.16)]'
          : completed
            ? 'border-zinc-900/80 bg-transparent'
            : 'border-zinc-900/80 bg-transparent'
      }`}
    >
      <div className="flex items-start gap-3">
        <button
          type="button"
          disabled={!canComplete || pending}
          onClick={onComplete}
          aria-label={completed ? `${item.text} completed` : `Complete ${item.text}`}
          className={`mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded border text-[11px] ${
            completed
              ? 'border-emerald-600 bg-emerald-600 text-zinc-950'
              : canComplete
                ? 'border-zinc-600 bg-zinc-950 text-zinc-500'
                : 'border-zinc-800 bg-zinc-900 text-zinc-700'
          }`}
        >
          {completed ? '✓' : ''}
        </button>
        <div className="min-w-0 flex-1">
          <div className="flex flex-wrap items-center gap-2">
            <p className={`text-sm font-medium ${completed ? 'text-zinc-500 line-through' : 'text-zinc-100'}`}>
              {item.text}
            </p>
            <SurfaceTagChip square>
              {taskKindIcon(item.task_kind)}
              {item.task_kind}
            </SurfaceTagChip>
            {item.project ? (
              <SurfaceTagChip tone="project" square>
                <TagIcon size={11} />
                {item.project}
              </SurfaceTagChip>
            ) : null}
            {metadata?.due_at ? (
              <SurfaceTagChip square>
                <CalendarIcon size={11} />
                {formatTaskDate(metadata.due_at)}
              </SurfaceTagChip>
            ) : null}
            {metadata?.commitment_kind ? (
              <SurfaceTagChip square>
                <ClockIcon size={11} />
                {metadata.commitment_kind}
              </SurfaceTagChip>
            ) : null}
          </div>
          {feedback ? (
            <p className={`mt-2 text-xs ${feedback.status === 'error' ? 'text-rose-300' : 'text-emerald-300'}`}>
              {feedback.message}
            </p>
          ) : null}
          <div className="mt-2 flex flex-wrap gap-2">
            {onOpenThread ? (
              <SurfaceActionChip onClick={onOpenThread}>
                <OpenThreadIcon size={12} />
                Open thread
              </SurfaceActionChip>
            ) : null}
            <SurfaceActionChip onClick={() => setRescheduleOpen((current) => !current)}>
              <RescheduleIcon size={12} />
              Reschedule
            </SurfaceActionChip>
          </div>
          {rescheduleOpen ? (
            <div className="mt-3 flex flex-wrap gap-2">
              {['Tomorrow', 'Later This Week', 'Pick Day', 'Unschedule'].map((label) => (
                <SurfaceActionChip key={label} compact>
                  <CalendarIcon size={11} />
                  {label}
                </SurfaceActionChip>
              ))}
            </div>
          ) : null}
        </div>
      </div>
    </div>
  );
}
