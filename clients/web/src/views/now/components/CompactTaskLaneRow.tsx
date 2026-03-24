import type { NowData, NowTaskData } from '../../../types';
import {
  ClockIcon,
  OpenThreadIcon,
  TagIcon,
} from '../../../core/Icons';
import { ActionChipButton, FilterDenseTag, ProjectTag } from '../../../core/FilterToggleTag';
import { NowItemRowLayout, NowItemRowShell, type NowItemRowSurface } from '../../../core/NowItemRow';
import { formatTaskDate } from '../nowModel';
import {
  formatTaskKindLabel,
  taskKindIcon,
} from '../nowNudgePresentation';

function laneItemStateIsCompleted(state: string): boolean {
  const normalized = state.trim().toLowerCase();
  return normalized === 'completed' || normalized === 'done' || normalized === 'closed';
}

export function CompactTaskLaneRow({
  item,
  metadata = null,
  emphasis = 'default',
  /** When true (TODAY / NEXT), row uses flat chrome with no shadow or glow wash. */
  flat = false,
  pending = false,
  feedback,
  onOpenThread,
  onComplete,
}: {
  item: NonNullable<NowData['task_lane']>['active'] extends infer T ? Exclude<T, null> : never;
  metadata?: NowTaskData | null;
  emphasis?: 'active' | 'default' | 'completed';
  flat?: boolean;
  pending?: boolean;
  feedback?: { status: 'success' | 'error'; message: string };
  onOpenThread?: () => void;
  onComplete?: () => void;
}) {
  const completed =
    emphasis === 'completed' || laneItemStateIsCompleted(item.state) || feedback?.status === 'success';
  const canComplete = Boolean(onComplete) && !completed;

  const surface: NowItemRowSurface =
    emphasis === 'active' && !completed ? 'emphasis' : completed ? 'ghost' : flat ? 'queue' : 'muted';

  return (
    <NowItemRowShell surface={surface} shell="laneRow">
      <NowItemRowLayout
        leading={
          <button
            type="button"
            disabled={!canComplete || pending}
            onClick={onComplete}
            aria-label={completed ? `${item.text} completed` : `Complete ${item.text}`}
            className={`mt-0.5 flex h-6 w-6 shrink-0 self-start items-center justify-center rounded border text-xs ${
              completed
                ? 'border-emerald-600 bg-emerald-600 text-zinc-950'
                : canComplete
                  ? 'border-zinc-600 bg-zinc-950 text-zinc-500'
                  : 'border-zinc-800 bg-zinc-900 text-zinc-700'
            }`}
          >
            {completed ? '✓' : ''}
          </button>
        }
        actions={
          <>
            {onOpenThread ? (
              <ActionChipButton onClick={onOpenThread} aria-label="Open thread">
                <OpenThreadIcon size={16} className="shrink-0" aria-hidden />
                <span className="capitalize">Open thread</span>
              </ActionChipButton>
            ) : null}
          </>
        }
      >
        <div className="flex min-w-0 items-center justify-between gap-2">
          <p
            className={`min-w-0 flex-1 truncate text-sm font-medium leading-tight tracking-tight ${
              completed ? 'text-zinc-500 line-through' : 'text-zinc-100'
            }`}
          >
            {item.text}
          </p>
          <div className="flex min-w-0 shrink-0 flex-nowrap items-center justify-end gap-x-1.5 overflow-x-auto [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
            <FilterDenseTag tone="neutral">
              <span aria-hidden className="inline-flex shrink-0 items-center">
                {taskKindIcon(item.task_kind)}
              </span>
              {formatTaskKindLabel(item.task_kind)}
            </FilterDenseTag>
            {item.project ? (
              <ProjectTag label={item.project}>
                <span aria-hidden className="inline-flex shrink-0 items-center opacity-80">
                  <TagIcon size={10} />
                </span>
                {item.project}
              </ProjectTag>
            ) : null}
            {metadata?.commitment_kind ? (
              <FilterDenseTag tone="neutral">
                <span aria-hidden className="inline-flex shrink-0 items-center">
                  <ClockIcon size={10} />
                </span>
                {metadata.commitment_kind}
              </FilterDenseTag>
            ) : null}
            {metadata?.due_at ? (
              <FilterDenseTag tone="ghost">
                Due {formatTaskDate(metadata.due_at)}
              </FilterDenseTag>
            ) : null}
          </div>
        </div>
        {feedback ? (
          <p className={`line-clamp-2 text-xs leading-snug ${feedback.status === 'error' ? 'text-rose-300' : 'text-emerald-300'}`}>
            {feedback.message}
          </p>
        ) : null}
      </NowItemRowLayout>
    </NowItemRowShell>
  );
}
