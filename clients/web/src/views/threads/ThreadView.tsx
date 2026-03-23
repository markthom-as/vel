import { useEffect, useMemo, useRef, useState } from 'react';
import type { ConversationData, JsonValue, MessageData } from '../../types';
import {
  chatQueryKeys,
  loadConversationList,
  loadConversationMessages,
} from '../../data/chat';
import { useQuery } from '../../data/query';
import { subscribeWsQuerySync } from '../../data/ws-sync';
import { cn } from '../../core/cn';
import { FilterDenseTag, FilterToggleTag } from '../../core/FilterToggleTag';
import { LayersIcon, OpenThreadIcon, ThreadsIcon } from '../../core/Icons';
import { itemPillCard, itemPillRowSelected } from '../../core/itemPill';
import { NowItemRowLayout } from '../../core/NowItemRow';
import { PanelEyebrow, PanelKeyValueRow, PanelMutedInset } from '../../core/PanelChrome';
import { PanelMetaPill } from '../../core/PanelItem';
import { MessageRenderer } from '../../core/MessageRenderer';
import { ProvenanceDrawer } from './ProvenanceDrawer';
import { useResolvedThreadConversationId } from './useResolvedThreadConversationId';
import { SurfaceState } from '../../core/SurfaceState';

interface ThreadViewProps {
  conversationId: string | null;
  onSelectConversation?: (conversationId: string) => void;
}

function capabilityStateLabel(state: string): string {
  return state.replaceAll('_', ' ');
}

function continuationTokenLabel(value: string): string {
  return value.replaceAll('_', ' ');
}

function formatContextValue(value: JsonValue): string {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean') {
    return String(value);
  }
  if (value === null) {
    return 'unknown';
  }
  if (Array.isArray(value)) {
    return value
      .map((item) => (typeof item === 'string' ? item : JSON.stringify(item)))
      .join(' • ');
  }
  return JSON.stringify(value);
}

function continuationContextRows(conversation: ConversationData): Array<{ label: string; value: string }> {
  const context = conversation.continuation?.continuation.continuation_context;
  if (!context || typeof context !== 'object' || Array.isArray(context)) {
    return [];
  }
  return Object.entries(context)
    .filter(([, value]) => value !== null && value !== '')
    .slice(0, 3)
    .map(([label, value]) => ({
      label: label.replaceAll('_', ' '),
      value: formatContextValue(value),
    }));
}

export function ThreadView({ conversationId, onSelectConversation }: ThreadViewProps) {
  const [provenanceMessageId, setProvenanceMessageId] = useState<string | null>(null);
  const [threadFilter, setThreadFilter] = useState('');
  const [filterMode, setFilterMode] = useState<'all' | 'unread' | 'active'>('all');
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data: conversations = [], loading: conversationsLoading, error: conversationsError } = useQuery(
    conversationsKey,
    async () => {
      const response = await loadConversationList();
      return response.ok && response.data ? response.data : [];
    },
  );
  const resolvedConversationId = useResolvedThreadConversationId(conversationId);
  const selectedConversation = useMemo(
    () => conversations.find((conversation) => conversation.id === resolvedConversationId) ?? null,
    [conversations, resolvedConversationId],
  );
  const threadModeCounts = useMemo(() => {
    const all = conversations.length;
    const unread = conversations.filter((c) => Boolean(c.continuation)).length;
    const active = resolvedConversationId ? 1 : 0;
    return { all, unread, active };
  }, [conversations, resolvedConversationId]);

  const filteredConversations = useMemo(() => {
    const query = threadFilter.trim().toLowerCase();
    return conversations
      .filter((conversation) => {
        if (filterMode === 'unread') {
          return Boolean(conversation.continuation);
        }
        if (filterMode === 'active') {
          return conversation.id === resolvedConversationId;
        }
        return true;
      })
      .filter((conversation) => {
        const haystacks = [
          conversation.title ?? '',
          conversation.kind,
          conversation.id,
        ];
        return haystacks.some((value) => value.toLowerCase().includes(query));
      })
      .slice(0, 8);
  }, [conversations, resolvedConversationId, filterMode, threadFilter]);
  const messagesKey = useMemo(
    () => chatQueryKeys.conversationMessages(resolvedConversationId),
    [resolvedConversationId],
  );

  const {
    data: messages = [],
    loading: messagesLoading,
    error: messagesError,
  } = useQuery<MessageData[]>(
    messagesKey,
    async () => {
      if (!resolvedConversationId) {
        return [];
      }
      const response = await loadConversationMessages(resolvedConversationId);
      return response.ok && response.data ? response.data : [];
    },
    { enabled: Boolean(resolvedConversationId) },
  );

  useEffect(() => {
    return subscribeWsQuerySync();
  }, []);

  useEffect(() => {
    const el = scrollRef.current;
    if (!el) return;
    try {
      el.scrollTo({ top: el.scrollHeight, behavior: 'smooth' });
    } catch {
      el.scrollTop = el.scrollHeight;
    }
  }, [messages]);

  if (!resolvedConversationId) {
    if (conversationsLoading) {
      return <SurfaceState message="Loading latest conversation…" layout="centered" />;
    }
    if (conversationsError) {
      return <SurfaceState message={conversationsError} layout="centered" tone="danger" />;
    }
    return <SurfaceState message="No conversations yet." layout="centered" />;
  }

  const error = conversationsError ?? messagesError;
  if (messagesLoading) {
    return <SurfaceState message="Loading conversation…" layout="centered" />;
  }
  if (error) {
    return <SurfaceState message={error} layout="centered" tone="danger" />;
  }

  return (
    <>
      <div className="flex min-h-0 flex-1">
        <aside className="flex w-full max-w-sm shrink-0 flex-col border-r border-zinc-900 bg-zinc-950/60">
          <div className="border-b border-zinc-900 px-4 py-4">
            <p className="text-[11px] uppercase tracking-[0.22em] text-zinc-500">Threads</p>
            <div className="mt-3 flex flex-wrap gap-1.5" role="group" aria-label="Thread list filter">
              {(
                [
                  {
                    mode: 'all' as const,
                    label: 'All',
                    count: threadModeCounts.all,
                    icon: (sel: boolean) => (
                      <LayersIcon size={12} className={sel ? 'text-amber-200/90' : 'text-zinc-500'} />
                    ),
                  },
                  {
                    mode: 'unread' as const,
                    label: 'Unread',
                    count: threadModeCounts.unread,
                    icon: (sel: boolean) => (
                      <ThreadsIcon size={12} className={sel ? 'text-amber-200/90' : 'text-zinc-500'} />
                    ),
                  },
                  {
                    mode: 'active' as const,
                    label: 'Active',
                    count: threadModeCounts.active,
                    icon: (sel: boolean) => (
                      <OpenThreadIcon size={12} className={sel ? 'text-amber-200/90' : 'text-zinc-500'} />
                    ),
                  },
                ] as const
              ).map(({ mode, label, count, icon }) => (
                <FilterToggleTag
                  key={mode}
                  label={label}
                  count={count}
                  selected={filterMode === mode}
                  onClick={() => setFilterMode(mode)}
                  icon={icon(filterMode === mode)}
                />
              ))}
            </div>
            <label className="mt-3 block relative">
              <span className="sr-only">Filter threads</span>
              <input
                type="text"
                value={threadFilter}
                onChange={(event) => setThreadFilter(event.target.value)}
                placeholder="Find thread"
                className="w-full rounded-xl border border-zinc-800 bg-zinc-950 px-3 py-2 pr-10 text-sm text-zinc-100 placeholder:text-zinc-500"
              />
              <span className="pointer-events-none absolute inset-y-0 right-3 flex items-center text-zinc-500">⌕</span>
            </label>
          </div>
          <div className="min-h-0 flex-1 overflow-y-auto px-2 py-2">
            {filteredConversations.length === 0 ? (
              <p className="px-2 py-3 text-sm text-zinc-500">No threads match that filter.</p>
            ) : (
              <div className="space-y-2">
                {filteredConversations.map((conversation) => (
                  <ThreadListRow
                    key={conversation.id}
                    conversation={conversation}
                    active={conversation.id === resolvedConversationId}
                    disabled={!onSelectConversation || conversation.id === resolvedConversationId}
                    onSelect={onSelectConversation}
                  />
                ))}
              </div>
            )}
          </div>
          <div className="border-t border-zinc-900 px-4 py-3 text-center text-xs text-zinc-500">
            {filteredConversations.length} thread{filteredConversations.length === 1 ? '' : 's'} in view
          </div>
        </aside>

        <section className="relative flex min-w-0 flex-1 flex-col">
          <div ref={scrollRef} className="flex-1 overflow-y-auto p-4">
            <div className="mx-auto max-w-3xl">
              <header className="mb-5 border-b border-zinc-900 pb-4">
                <div className="flex flex-wrap items-start justify-between gap-3">
                  <div className="min-w-0">
                    <PanelEyebrow className="tracking-[0.22em]">Thread</PanelEyebrow>
                    <h1 className="mt-2 truncate text-2xl font-semibold tracking-tight text-zinc-100">
                      {threadTitle(selectedConversation)}
                    </h1>
                  </div>
                  {selectedConversation?.continuation ? (
                    <div className="flex flex-wrap gap-2">
                      <PanelMetaPill tone="state">
                        {continuationTokenLabel(selectedConversation.continuation.continuation.continuation_category)}
                      </PanelMetaPill>
                      {selectedConversation.continuation.lifecycle_stage ? (
                        <PanelMetaPill tone="state">{selectedConversation.continuation.lifecycle_stage}</PanelMetaPill>
                      ) : null}
                    </div>
                  ) : null}
                </div>
                {selectedConversation?.continuation ? (
                  <PanelMutedInset className="mt-3">
                    <p className="text-sm leading-6 text-zinc-300">
                      {selectedConversation.continuation.continuation.escalation_reason}
                    </p>
                    <div className="mt-3 flex flex-wrap gap-2">
                      <PanelMetaPill tone="state">
                        {capabilityStateLabel(selectedConversation.continuation.continuation.bounded_capability_state)}
                      </PanelMetaPill>
                      <PanelMetaPill tone="state">
                        {continuationTokenLabel(selectedConversation.continuation.continuation.open_target)}
                      </PanelMetaPill>
                    </div>
                    {continuationContextRows(selectedConversation).length > 0 ? (
                      <div className="mt-3 grid gap-2 text-xs text-zinc-400">
                        {continuationContextRows(selectedConversation).map((entry) => (
                          <PanelKeyValueRow key={entry.label} label={entry.label} value={entry.value} />
                        ))}
                      </div>
                    ) : null}
                  </PanelMutedInset>
                ) : null}
                <PanelMutedInset className="mt-3">
                  <p className="text-sm leading-6 text-zinc-300">
                    Workflow invocation stays unavailable here until the backend exposes exactly one stable canonical
                    object binding for this thread.
                  </p>
                  <p className="mt-2 text-xs leading-5 text-zinc-500">
                    Attach or create an object first. `v0.5.1` does not allow floating or multi-object invocation from
                    Threads.
                  </p>
                </PanelMutedInset>
              </header>
              {messages.length === 0 && (
                <SurfaceState message="No messages in this thread yet." />
              )}
              {messages.map((message) => (
                <MessageRenderer
                  key={message.id}
                  message={message}
                  onShowWhy={setProvenanceMessageId}
                />
              ))}
            </div>
          </div>
          {provenanceMessageId && (
            <ProvenanceDrawer
              messageId={provenanceMessageId}
              onClose={() => setProvenanceMessageId(null)}
            />
          )}
        </section>
      </div>
    </>
  );
}

function threadTitle(conversation: ConversationData | null): string {
  if (conversation?.title?.trim()) {
    return conversation.title.trim();
  }
  if (conversation?.continuation?.continuation.escalation_reason?.trim()) {
    return conversation.continuation.continuation.escalation_reason.trim();
  }
  if (conversation?.continuation?.continuation.open_target?.trim()) {
    return continuationTokenLabel(conversation.continuation.continuation.open_target);
  }
  return 'Untitled thread';
}

function previewText(conversation: ConversationData, messages: MessageData[]): string {
  const latestMessage = [...messages].sort((left, right) => right.created_at - left.created_at)[0];
  const messageText =
    typeof latestMessage?.content === 'object' && latestMessage?.content !== null && !Array.isArray(latestMessage.content)
      ? latestMessage.content.text
      : null;
  if (typeof messageText === 'string' && messageText.trim().length > 0) {
    return messageText.trim();
  }
  if (conversation.continuation?.continuation.escalation_reason?.trim()) {
    return conversation.continuation.continuation.escalation_reason.trim();
  }
  return conversation.kind.replaceAll('_', ' ');
}

function ThreadListRow({
  conversation,
  active,
  disabled,
  onSelect,
}: {
  conversation: ConversationData;
  active: boolean;
  disabled: boolean;
  onSelect?: (conversationId: string) => void;
}) {
  const messagesKey = useMemo(
    () => chatQueryKeys.conversationMessages(conversation.id),
    [conversation.id],
  );
  const { data: messages = [] } = useQuery<MessageData[]>(
    messagesKey,
    async () => {
      try {
        const response = await loadConversationMessages(conversation.id);
        return response.ok && response.data ? response.data : [];
      } catch {
        return [];
      }
    },
    { enabled: true },
  );
  const unreadCount = conversation.continuation ? 1 : 0;
  const tagFrame = active
    ? 'border-zinc-300/90 bg-white/90 text-zinc-800'
    : 'border-zinc-800/90 bg-zinc-900/92 text-zinc-400';
  const metaMuted = active ? 'text-zinc-600' : 'text-zinc-600';

  return (
    <button
      type="button"
      onClick={() => onSelect?.(conversation.id)}
      disabled={disabled}
      className={cn(
        active
          ? itemPillRowSelected
          : cn(itemPillCard('muted', 'laneRow'), 'text-zinc-100 hover:border-zinc-700'),
        'w-full text-left transition disabled:cursor-default disabled:opacity-60',
      )}
    >
      <NowItemRowLayout
        leading={
          unreadCount > 0 ? (
            <span
              className="mt-1 flex h-2 w-2 shrink-0 items-center justify-center"
              role="img"
              aria-label="Unread continuation"
            >
              <span className="h-2 w-2 rounded-full bg-emerald-400 ring-2 ring-emerald-500/25" />
            </span>
          ) : (
            <span className="mt-1 w-2 shrink-0" aria-hidden />
          )
        }
        actionsLayout="inline"
        actions={
          unreadCount > 0 ? (
            <FilterDenseTag
              className={cn(
                active ? 'border-emerald-600/40 bg-emerald-50 text-emerald-900' : 'border-emerald-700/50 bg-emerald-950/50 text-emerald-200',
              )}
            >
              {unreadCount}
            </FilterDenseTag>
          ) : null
        }
      >
        <div className="flex min-w-0 items-center justify-between gap-2">
          <p
            className={cn(
              'min-w-0 flex-1 truncate text-sm font-medium leading-tight tracking-tight',
              active ? 'text-zinc-950' : 'text-zinc-100',
            )}
          >
            {threadTitle(conversation)}
          </p>
          <div className="flex min-w-0 shrink-0 flex-nowrap items-center justify-end gap-x-1.5 overflow-x-auto [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
            <FilterDenseTag className={tagFrame}>
              <span aria-hidden className="inline-flex shrink-0 items-center opacity-80">
                <ThreadsIcon size={10} />
              </span>
              {conversation.kind.replaceAll('_', ' ')}
            </FilterDenseTag>
            {conversation.continuation ? (
              <FilterDenseTag className={tagFrame}>
                {continuationTokenLabel(conversation.continuation.continuation.continuation_category)}
              </FilterDenseTag>
            ) : null}
            <FilterDenseTag className={cn('!shrink-0 border-transparent bg-transparent', metaMuted)}>
              {formatThreadDate(conversation.created_at)} · last {formatThreadDate(latestThreadTimestamp(conversation, messages))}
            </FilterDenseTag>
          </div>
        </div>
        <p className={cn('line-clamp-2 text-xs leading-snug', active ? 'text-zinc-700' : 'text-zinc-500')}>
          {previewText(conversation, messages)}
        </p>
      </NowItemRowLayout>
    </button>
  );
}

function latestThreadTimestamp(conversation: ConversationData, messages: MessageData[]): number {
  const latestMessage = [...messages].sort((left, right) => right.created_at - left.created_at)[0];
  return latestMessage?.created_at ?? conversation.updated_at;
}

function formatThreadDate(timestamp: number): string {
  try {
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
    }).format(new Date(timestamp * 1000));
  } catch {
    return String(timestamp);
  }
}
