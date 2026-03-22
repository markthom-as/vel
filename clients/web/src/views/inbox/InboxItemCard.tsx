import type { InboxItemData } from '../../types';
import {
  acknowledgeInboxItem,
  dismissInboxItem,
  getInboxThreadPath,
  getInterventionApiId,
  reactivateInboxItem,
  resolveInboxItem,
  snoozeInboxItem,
} from '../../data/chat';
import { Button } from '../../core/Button';
import { FilterDenseTag, FilterPillButton } from '../../core/FilterToggleTag';
import { InboxIcon, OpenThreadIcon, TagIcon } from '../../core/Icons';
import { NowItemRowLayout, NowItemRowShell } from '../../core/NowItemRow';
import { formatRelativeMinutes, projectTagClasses } from '../now/nowModel';
import { surfaceActionChipNudgeClass } from '../now/nowNudgePresentation';

export type InboxInterventionAction = (
  item: InboxItemData,
  nextState: 'acknowledged' | 'dismissed' | 'snoozed' | 'resolved' | 'active',
  action: () => Promise<{ ok: boolean; data?: { state: string }; error?: { message: string } }>,
) => Promise<void>;

export function InboxItemCard({
  item,
  onOpenThread,
  runInterventionAction,
}: {
  item: InboxItemData;
  onOpenThread?: (conversationId: string) => void;
  runInterventionAction: InboxInterventionAction;
}) {
  const threadPath = getInboxThreadPath(item);
  const apiId = getInterventionApiId(item);
  const stateLower = item.state.toLowerCase();
  const hasAcknowledge = Boolean(apiId && item.available_actions.includes('acknowledge'));
  const hasSnooze = Boolean(apiId && item.available_actions.includes('snooze'));
  const hasDismiss = Boolean(apiId && item.available_actions.includes('dismiss'));
  const hasResolve = Boolean(apiId && item.available_actions.includes('resolve'));
  const hasOpenThread = Boolean(threadPath && item.conversation_id);
  const canMarkUnread = Boolean(apiId && stateLower !== 'active');
  const canArchive =
    Boolean(apiId && hasResolve) && stateLower !== 'resolved' && stateLower !== 'dismissed';

  return (
    <NowItemRowShell surface="muted" shell="laneRow" as="article">
      <NowItemRowLayout
        actions={
          <>
            {hasOpenThread ? (
              <FilterPillButton
                className={surfaceActionChipNudgeClass}
                onClick={() => onOpenThread?.(item.conversation_id as string)}
                aria-label="Open thread"
              >
                <OpenThreadIcon size={16} className="shrink-0" aria-hidden />
                <span className="capitalize">Open thread</span>
              </FilterPillButton>
            ) : null}
            {canMarkUnread ? (
              <FilterPillButton
                className={surfaceActionChipNudgeClass}
                onClick={() =>
                  void runInterventionAction(item, 'active', () => reactivateInboxItem(apiId as string))
                }
              >
                <span className="capitalize">Mark unread</span>
              </FilterPillButton>
            ) : null}
            {canArchive ? (
              <FilterPillButton
                className={surfaceActionChipNudgeClass}
                onClick={() =>
                  void runInterventionAction(item, 'resolved', () => resolveInboxItem(apiId as string))
                }
              >
                <span className="capitalize">Archive</span>
              </FilterPillButton>
            ) : null}
            {hasAcknowledge ? (
              <FilterPillButton
                className={surfaceActionChipNudgeClass}
                onClick={() =>
                  void runInterventionAction(item, 'acknowledged', () =>
                    acknowledgeInboxItem(apiId as string),
                  )
                }
              >
                <span className="capitalize">Acknowledge</span>
              </FilterPillButton>
            ) : null}
            {hasSnooze ? (
              <FilterPillButton
                className={surfaceActionChipNudgeClass}
                onClick={() =>
                  void runInterventionAction(item, 'snoozed', () => snoozeInboxItem(apiId as string, 10))
                }
              >
                <span className="capitalize">Snooze 10m</span>
              </FilterPillButton>
            ) : null}
            {hasDismiss ? (
              <Button
                variant="danger"
                size="sm"
                onClick={() =>
                  void runInterventionAction(item, 'dismissed', () => dismissInboxItem(apiId as string))
                }
              >
                Dismiss
              </Button>
            ) : null}
          </>
        }
      >
        <div className="flex min-w-0 items-center justify-between gap-2">
          <p className="min-w-0 flex-1 truncate text-sm font-medium leading-none tracking-tight text-zinc-100">
            {item.title}
          </p>
          <div className="flex min-w-0 shrink-0 flex-nowrap items-center justify-end gap-x-1.5 overflow-x-auto [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
            <FilterDenseTag className="border-zinc-800/90 bg-zinc-900/92 text-zinc-400">
              <span aria-hidden className="inline-flex shrink-0 items-center">
                <InboxIcon size={10} />
              </span>
              {formatKind(item.kind)}
            </FilterDenseTag>
            <FilterDenseTag className="border-zinc-800/90 bg-zinc-900/92 text-zinc-400">
              {item.state}
            </FilterDenseTag>
            {item.project_label ? (
              <FilterDenseTag className={projectTagClasses(item.project_label)}>
                <span aria-hidden className="inline-flex shrink-0 items-center opacity-80">
                  <TagIcon size={10} />
                </span>
                {item.project_label}
              </FilterDenseTag>
            ) : null}
            <FilterDenseTag className="!shrink-0 border-transparent bg-transparent text-zinc-600">
              {formatRelativeMinutes(item.surfaced_at)}
            </FilterDenseTag>
          </div>
        </div>
        <p className="line-clamp-2 text-xs leading-snug text-zinc-500">{item.summary}</p>
        <div className="flex flex-wrap gap-x-4 gap-y-1 text-[11px] text-zinc-500">
          {item.snoozed_until != null ? (
            <span>Snoozed until {formatTs(item.snoozed_until)}</span>
          ) : null}
          {item.confidence != null ? (
            <span>Confidence {Math.round(item.confidence * 100)}%</span>
          ) : null}
        </div>
        <div className="mt-1">
          <p className="text-[10px] uppercase tracking-[0.16em] text-zinc-500">Evidence</p>
          <div className="mt-1 flex flex-wrap gap-1.5">
            {item.evidence.length === 0 ? (
              <FilterDenseTag className="border-zinc-800/90 bg-zinc-900/92 text-zinc-500">
                No evidence labels
              </FilterDenseTag>
            ) : (
              item.evidence.map((evidence) => (
                <FilterDenseTag
                  key={`${item.id}-${evidence.source_id}-${evidence.label}`}
                  className={`!normal-case !tracking-normal ${projectTagClasses(evidence.label)}`}
                >
                  {evidence.label}
                </FilterDenseTag>
              ))
            )}
          </div>
        </div>
      </NowItemRowLayout>
    </NowItemRowShell>
  );
}

function formatTs(ts: number): string {
  return new Date(ts * 1000).toLocaleString();
}

function formatKind(kind: string): string {
  return kind.replaceAll('_', ' ');
}
