import type { ActionItemData, NowData, NowTaskData } from '../../../types';
import {
  PanelSectionHeaderBand,
  PanelSectionHeaderLead,
  PanelSectionHeaderTrail,
} from '../../../core/PanelChrome';
import { ActionRow } from './ActionRow';
import { CompactTaskLaneRow } from './CompactTaskLaneRow';
import { NowTasksMetricStrip } from './NowTasksMetricStrip';
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
      <div className="space-y-2">
        <PanelSectionHeaderBand mode="section-header">
          <PanelSectionHeaderLead>
            <h2 className="text-lg font-medium text-zinc-100">Tasks</h2>
          </PanelSectionHeaderLead>
          <PanelSectionHeaderTrail>
            <NowTasksMetricStrip
              completedCount={completedCount}
              remainingCount={remainingCount}
              backlogCount={backlogCount}
            />
          </PanelSectionHeaderTrail>
        </PanelSectionHeaderBand>
        <p className="text-xs uppercase tracking-[0.14em] text-zinc-500">Today&apos;s operating queue</p>
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
              flat
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
              flat
              metadata={allTaskMetadata.find((task) => task.id === item.id) ?? null}
            />
          ))}
        </TaskGroup>

        <TaskGroup title="COMPLETED" visible={(taskLane?.recent_completed.length ?? 0) > 0}>
          {taskLane?.recent_completed.map((item) => (
            <CompactTaskLaneRow
              key={item.id}
              item={item}
              flat
              emphasis="completed"
              metadata={allTaskMetadata.find((task) => task.id === item.id) ?? null}
              onOpenThread={item.primary_thread_id ? () => onOpenThread?.(item.primary_thread_id!) : undefined}
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
