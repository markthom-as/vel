import { useEffect, useMemo, useState } from 'react';
import { contextQueryKeys, loadNow, updateCommitment } from '../../data/context';
import { invalidateQuery, useQuery } from '../../data/query';
import type { AssistantEntryResponse, NowData } from '../../types';
import { chatQueryKeys } from '../../data/chat';
import { MessageComposer } from '../../core/MessageComposer';
import { SurfaceState } from '../../core/SurfaceState';
import { AssistantEntryFeedback } from './components/AssistantEntryFeedback';
import { NowMetricStrip } from './components/NowMetricStrip';
import { NowNudgeStrip } from './components/NowNudgeStrip';
import { NowTasksSection } from './components/NowTasksSection';
import {
  buildCurrentStatus,
  dedupeActionItems,
  dedupeTasks,
  findActiveEvent,
  findActiveRoutineBlock,
  findNextEvent,
  scoreNudge,
} from './nowModel';

type SettingsIntegrationTarget =
  | 'google'
  | 'todoist'
  | 'activity'
  | 'git'
  | 'messaging'
  | 'notes'
  | 'transcripts';

interface NowViewProps {
  onOpenInbox?: () => void;
  onOpenThread?: (conversationId: string) => void;
  onOpenSettings?: (target: {
    tab: 'general' | 'integrations' | 'runtime';
    integrationId?: SettingsIntegrationTarget;
  }) => void;
}

export function NowView({ onOpenInbox, onOpenThread, onOpenSettings }: NowViewProps) {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const commitmentsKey = useMemo(() => contextQueryKeys.commitments(25), []);
  const inboxKey = useMemo(() => chatQueryKeys.inbox(), []);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data, loading, error, refetch } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );

  const [assistantEntryMessage, setAssistantEntryMessage] = useState<{
    status: 'success' | 'error';
    message: string;
  } | null>(null);
  const [assistantInlineResponse, setAssistantInlineResponse] = useState<AssistantEntryResponse | null>(null);
  const [assistantEntryThreadId, setAssistantEntryThreadId] = useState<string | null>(null);
  const [pendingCommitments, setPendingCommitments] = useState<Record<string, true>>({});
  const [commitmentMessages, setCommitmentMessages] = useState<
    Record<string, { status: 'success' | 'error'; message: string }>
  >({});

  useEffect(() => {
    const handleFocus = () => {
      void refetch();
    };
    const handleVisibilityChange = () => {
      if (document.visibilityState === 'visible') {
        void refetch();
      }
    };
    const interval = window.setInterval(() => {
      void refetch();
    }, 60_000);

    window.addEventListener('focus', handleFocus);
    document.addEventListener('visibilitychange', handleVisibilityChange);
    return () => {
      window.clearInterval(interval);
      window.removeEventListener('focus', handleFocus);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  }, [refetch]);

  const handleAssistantEntry = async (response: AssistantEntryResponse) => {
    invalidateQuery(conversationsKey, { refetch: true });
    invalidateQuery(inboxKey, { refetch: true });
    setAssistantEntryMessage(null);
    setAssistantInlineResponse(null);
    setAssistantEntryThreadId(response.conversation.id);

    if (response.route_target === 'threads') {
      onOpenThread?.(response.conversation.id);
      return;
    }
    if (response.route_target === 'inbox') {
      setAssistantEntryMessage({
        status: 'success',
        message: 'Saved to Inbox for follow-up.',
      });
      onOpenInbox?.();
      return;
    }
    setAssistantInlineResponse(response);
    if (response.assistant_error) {
      setAssistantEntryMessage({
        status: 'error',
        message: response.assistant_error,
      });
      return;
    }
    setAssistantEntryMessage({
      status: 'success',
      message: 'Handled here in Now.',
    });
  };

  if (loading) {
    return <SurfaceState message="Loading your current state…" layout="centered" />;
  }

  if (error) {
    return <SurfaceState message={error} layout="centered" tone="warning" />;
  }

  if (!data) {
    return (
      <SurfaceState
        message="No current context yet. Sync integrations or run an evaluation."
        layout="centered"
      />
    );
  }

  const actionItems = dedupeActionItems(
    [...(data.action_items ?? [])]
      .filter((item) => item.surface === 'now')
      .sort((left, right) => left.rank - right.rank),
  );
  const header = data.header;
  const meshSummary = data.mesh_summary;
  const contextLine = data.context_line;
  const nudgeBars = data.nudge_bars ?? [];
  const taskLane = data.task_lane;
  const nowTs = data.computed_at;
  const activeEvent = findActiveEvent(data.schedule.upcoming_events, nowTs);
  const nextScheduledEvent = findNextEvent(data.schedule.upcoming_events, nowTs);
  const activeRoutineBlock = findActiveRoutineBlock(data.day_plan, nowTs);
  const commitmentRows = dedupeTasks([data.tasks.next_commitment, ...(data.tasks.other_open ?? [])]);
  const commitmentIds = new Set(commitmentRows.map((t) => t.id));
  const pullableTasks = dedupeTasks(data.tasks.todoist ?? []);
  const currentStatus = buildCurrentStatus(
    data,
    activeEvent,
    activeRoutineBlock,
    commitmentRows[0] ?? null,
    nextScheduledEvent,
  );

  const completeCommitment = async (commitmentId: string) => {
    setPendingCommitments((current) => ({ ...current, [commitmentId]: true }));
    setCommitmentMessages((current) => {
      const next = { ...current };
      delete next[commitmentId];
      return next;
    });
    try {
      const response = await updateCommitment(commitmentId, { status: 'done' });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to complete commitment');
      }
      invalidateQuery(nowKey, { refetch: true });
      invalidateQuery(commitmentsKey, { refetch: true });
      await refetch();
      setCommitmentMessages((current) => ({
        ...current,
        [commitmentId]: { status: 'success', message: 'Completed.' },
      }));
    } catch (commitmentError) {
      setCommitmentMessages((current) => ({
        ...current,
        [commitmentId]: {
          status: 'error',
          message: commitmentError instanceof Error ? commitmentError.message : String(commitmentError),
        },
      }));
    } finally {
      setPendingCommitments((current) => {
        const next = { ...current };
        delete next[commitmentId];
        return next;
      });
    }
  };

  const handleNowBarAction = (
    bar: NowData['nudge_bars'][number],
    action: NowData['nudge_bars'][number]['actions'][number],
  ) => {
    if ((action.kind === 'open_thread' || action.kind === 'expand') && bar.primary_thread_id) {
      onOpenThread?.(bar.primary_thread_id);
      return;
    }
    if (action.kind === 'open_settings') {
      onOpenSettings?.({ tab: 'runtime' });
      return;
    }
    if (action.kind === 'open_inbox') {
      onOpenInbox?.();
    }
  };

  const activeDescription = contextLine?.text ?? currentStatus.summary;
  const backupNudge =
    data.trust_readiness.backup.level !== 'ok'
      ? {
          id: 'backup_trust_warning',
          kind: 'trust_warning',
          title: 'No trustworthy backup',
          summary: data.trust_readiness.backup.detail,
          urgent: data.trust_readiness.backup.level === 'fail',
          primary_thread_id: null,
          actions: [{ kind: 'open_settings', label: 'Open backups' }],
        }
      : null;
  const prioritizedNudges = [...(backupNudge ? [backupNudge] : []), ...nudgeBars]
    .sort((left, right) => scoreNudge(right) - scoreNudge(left))
    .slice(0, 4);
  const riskItems = actionItems
    .filter((item) => ['recovery', 'blocked', 'conflict', 'freshness', 'linking'].includes(item.kind))
    .slice(0, 3);
  const nextTasks = pullableTasks
    .filter((task) => task.id !== taskLane?.active?.id && !taskLane?.pending.some((item) => item.id === task.id))
    .slice(0, 3)
    .map((task) => ({
      id: task.id,
      task_kind: 'task' as const,
      text: task.text,
      state: 'pending',
      project: task.project,
      primary_thread_id: null,
    }));
  const groupedTaskCount =
    (taskLane?.active ? 1 : 0) + (taskLane?.pending.length ?? 0) + riskItems.length + nextTasks.length;
  const allTaskMetadata = [...commitmentRows, ...pullableTasks];
  const completedCount = taskLane?.recent_completed.length ?? 0;
  const remainingCount =
    (taskLane?.pending.length ?? 0) + riskItems.length + nextTasks.length + (taskLane?.active ? 1 : 0);
  const backlogCount = Math.max(0, pullableTasks.length - ((taskLane?.pending.length ?? 0) + nextTasks.length));
  const threadAttentionCount =
    actionItems.filter((item) => item.thread_route !== null).length + (data.reflow_status?.thread_id ? 1 : 0);

  return (
    <div className="flex min-h-full flex-col bg-zinc-950">
      <div className="mx-auto w-full max-w-5xl flex-1 px-4 py-6 pb-36 sm:px-6">
        <section className="space-y-4">
          <div className="flex items-start justify-between gap-3">
            <div className="space-y-0.5">
              <h1 className="text-2xl font-semibold tracking-tight text-zinc-100">
                {header?.title ?? 'Now'}
              </h1>
              <p className="truncate text-[11px] text-zinc-500">{activeDescription}</p>
            </div>
            <NowMetricStrip
              nudgeCount={prioritizedNudges.length}
              threadAttentionCount={threadAttentionCount}
              queuedWriteCount={meshSummary?.queued_write_count ?? 0}
            />
          </div>

          <NowNudgeStrip
            bars={prioritizedNudges}
            nowTs={nowTs}
            actionItems={actionItems}
            onBarAction={handleNowBarAction}
          />

          <NowTasksSection
            taskLane={taskLane}
            riskItems={riskItems}
            nextTasks={nextTasks}
            allTaskMetadata={allTaskMetadata}
            commitmentIds={commitmentIds}
            completedCount={completedCount}
            remainingCount={remainingCount}
            backlogCount={backlogCount}
            groupedTaskCount={groupedTaskCount}
            pendingCommitments={pendingCommitments}
            commitmentMessages={commitmentMessages}
            onCompleteCommitment={completeCommitment}
            onOpenInbox={onOpenInbox}
            onOpenThread={onOpenThread}
          />

          <AssistantEntryFeedback
            message={assistantEntryMessage}
            inlineResponse={assistantInlineResponse}
            assistantEntryThreadId={assistantEntryThreadId}
            onOpenThread={onOpenThread}
          />
        </section>

        <MessageComposer
          compact
          floating
          hideHelperText
          onSent={(_, response) => {
            handleAssistantEntry(response);
            invalidateQuery(nowKey, { refetch: true });
          }}
          onSendFailed={() => {
            setAssistantEntryMessage({
              status: 'error',
              message: 'Failed to send assistant entry.',
            });
          }}
        />
      </div>
    </div>
  );
}
