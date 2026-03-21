import type { ActionItemData, NowData, NowTaskData } from '../../../types';
import { CheckCircleIcon, ClipboardCheckIcon, LayersIcon } from '../../../core/Icons';
import { ActionRow } from './ActionRow';
import { CompactTaskLaneRow } from './CompactTaskLaneRow';
import { StatPill } from './StatPill';
import { TaskGroup } from './TaskGroup';

type LaneItem = NonNullable<NowData['task_lane']>['active'] extends infer T ? Exclude<T, null> : never;

export function NowTasksSection({
  taskLane,
  riskItems,
  nextTasks,
  allTaskMetadata,
  commitmentIds,
  completedCount,
  remainingCount,
  backlogCount,
  groupedTaskCount,
  pendingCommitments,
  commitmentMessages,
  onCompleteCommitment,
  onOpenInbox,
  onOpenThread,
}: {
  taskLane: NowData['task_lane'];
  riskItems: ActionItemData[];
  nextTasks: LaneItem[];
  allTaskMetadata: NowTaskData[];
  commitmentIds: Set<string>;
  completedCount: number;
  remainingCount: number;
  backlogCount: number;
  groupedTaskCount: number;
  pendingCommitments: Record<string, true>;
  commitmentMessages: Record<string, { status: 'success' | 'error'; message: string }>;
  onCompleteCommitment: (commitmentId: string) => void;
  onOpenInbox?: () => void;
  onOpenThread?: (conversationId: string) => void;
}) {
  const isCommitment = (id: string) => commitmentIds.has(id);

  return (
    <section className="space-y-4 pt-3">
      <div className="flex items-start justify-between gap-3">
        <div>
          <h2 className="text-lg font-medium text-zinc-100">Tasks</h2>
          <p className="mt-1 text-[10px] uppercase tracking-[0.16em] text-zinc-500">Today&apos;s operating queue</p>
        </div>
        <div className="flex flex-wrap items-center gap-2 text-right">
          <StatPill
            label="Completed"
            value={`${completedCount}/${Math.max(1, completedCount + remainingCount)}`}
            detail={`${Math.round((completedCount / Math.max(1, completedCount + remainingCount)) * 100)}%`}
            icon={<CheckCircleIcon size={12} />}
          />
          <StatPill label="Remaining" value={String(remainingCount)} icon={<ClipboardCheckIcon size={12} />} />
          <StatPill label="Backlog" value={String(backlogCount)} icon={<LayersIcon size={12} />} />
        </div>
      </div>

      <div className="mt-4 space-y-4">
        <TaskGroup title="NOW" visible={Boolean(taskLane?.active)}>
          {taskLane?.active ? (
            <CompactTaskLaneRow
              item={taskLane.active}
              metadata={allTaskMetadata.find((task) => task.id === taskLane.active?.id) ?? null}
              emphasis="active"
              pending={Boolean(pendingCommitments[taskLane.active.id])}
              feedback={commitmentMessages[taskLane.active.id]}
              onOpenThread={
                taskLane.active.primary_thread_id
                  ? () => onOpenThread?.(taskLane.active!.primary_thread_id!)
                  : undefined
              }
              onComplete={
                isCommitment(taskLane.active.id) ? () => void onCompleteCommitment(taskLane.active!.id) : undefined
              }
            />
          ) : null}
        </TaskGroup>

        <TaskGroup title="TODAY" visible={(taskLane?.pending.length ?? 0) > 0}>
          {taskLane?.pending.map((item) => (
            <CompactTaskLaneRow
              key={item.id}
              item={item}
              metadata={allTaskMetadata.find((task) => task.id === item.id) ?? null}
              pending={Boolean(pendingCommitments[item.id])}
              feedback={commitmentMessages[item.id]}
              onOpenThread={item.primary_thread_id ? () => onOpenThread?.(item.primary_thread_id!) : undefined}
              onComplete={isCommitment(item.id) ? () => void onCompleteCommitment(item.id) : undefined}
            />
          ))}
        </TaskGroup>

        <TaskGroup title="AT RISK" visible={riskItems.length > 0}>
          {riskItems.map((item) => (
            <ActionRow key={item.id} item={item} onOpenInbox={onOpenInbox} onOpenThread={onOpenThread} />
          ))}
        </TaskGroup>

        <TaskGroup title="NEXT" visible={nextTasks.length > 0}>
          {nextTasks.map((item) => (
            <CompactTaskLaneRow
              key={item.id}
              item={item}
              metadata={allTaskMetadata.find((task) => task.id === item.id) ?? null}
            />
          ))}
        </TaskGroup>

        {groupedTaskCount === 0 ? (
          <p className="rounded-2xl border border-zinc-800 bg-zinc-950/40 px-4 py-4 text-sm text-zinc-500">
            No current-day tasks are surfaced right now.
          </p>
        ) : null}
      </div>
    </section>
  );
}
