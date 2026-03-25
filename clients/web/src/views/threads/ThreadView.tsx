import { useEffect, useMemo, useRef, useState } from 'react';
import type { ConversationData, JsonValue, MessageData } from '../../types';
import {
  chatQueryKeys,
  loadConversationList,
  loadConversationMessages,
  updateConversationArchive,
  updateConversationCallMode,
  updateConversationTitle,
} from '../../data/chat';
import { invalidateQuery, setQueryData, useQuery } from '../../data/query';
import { subscribeWsQuerySync } from '../../data/ws-sync';
import { cn } from '../../core/cn';
import { ActionChipButton, FilterDenseTag, FilterToggleTag } from '../../core/FilterToggleTag';
import { ArchiveIcon, DotIcon, LayersIcon, MicIcon, OpenThreadIcon, ThreadsIcon, WarningIcon } from '../../core/Icons';
import { ObjectRowFrame, ObjectRowLayout, ObjectRowTitleMetaBand } from '../../core/ObjectRow';
import { PanelEmptyRow, PanelKeyValueRow } from '../../core/PanelChrome';
import { MessageRenderer } from '../../core/MessageRenderer';
import { ProvenanceDrawer } from './ProvenanceDrawer';
import { useResolvedThreadConversationId } from './useResolvedThreadConversationId';
import { SurfaceState } from '../../core/SurfaceState';
import { uiFonts } from '../../core/Theme';
import { SearchField } from '../../core/SearchField/SearchField';

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
  const [filterMode, setFilterMode] = useState<'all' | 'unread' | 'needs_review' | 'active' | 'archived'>('all');
  const [draftTitle, setDraftTitle] = useState('');
  const [editingTitle, setEditingTitle] = useState(false);
  const [savingTitle, setSavingTitle] = useState(false);
  const [archivingConversationId, setArchivingConversationId] = useState<string | null>(null);
  const [togglingCallModeId, setTogglingCallModeId] = useState<string | null>(null);
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
  useEffect(() => {
    setDraftTitle(threadTitle(selectedConversation));
  }, [selectedConversation]);

  useEffect(() => {
    if (!conversationId && resolvedConversationId && onSelectConversation) {
      onSelectConversation(resolvedConversationId);
    }
  }, [conversationId, onSelectConversation, resolvedConversationId]);

  async function saveTitle(nextTitle: string) {
    if (!selectedConversation) return;
    const trimmed = nextTitle.trim();
    const currentTitle = threadTitle(selectedConversation);
    if (!trimmed || trimmed === currentTitle) {
      setDraftTitle(currentTitle);
      return;
    }
    setSavingTitle(true);
    setQueryData(conversationsKey, (current: ConversationData[] | undefined) =>
      (current ?? []).map((conversation) =>
        conversation.id === selectedConversation.id ? { ...conversation, title: trimmed } : conversation,
      ),
    );
    try {
      await updateConversationTitle(selectedConversation.id, trimmed);
      invalidateQuery(conversationsKey, { refetch: true });
    } finally {
      setSavingTitle(false);
    }
  }

  async function archiveConversation(conversation: ConversationData) {
    if (archivingConversationId) return;
    setArchivingConversationId(conversation.id);
    setQueryData(conversationsKey, (current: ConversationData[] | undefined) =>
      (current ?? []).map((entry) =>
        entry.id === conversation.id ? { ...entry, archived: true } : entry,
      ),
    );
    try {
      await updateConversationArchive(conversation.id, true);
      await invalidateQuery(conversationsKey, { refetch: true });
      const fallbackConversationId = conversations
        .filter((entry) => entry.id !== conversation.id && !entry.archived)
        .sort((left, right) => right.updated_at - left.updated_at)[0]?.id ?? null;
      if (fallbackConversationId && onSelectConversation) {
        onSelectConversation(fallbackConversationId);
      }
    } finally {
      setArchivingConversationId(null);
    }
  }

  async function toggleConversationCallMode(conversation: ConversationData) {
    if (togglingCallModeId) return;
    const nextCallMode = !conversation.call_mode_active;
    setTogglingCallModeId(conversation.id);
    setQueryData(conversationsKey, (current: ConversationData[] | undefined) =>
      (current ?? []).map((entry) =>
        entry.id === conversation.id ? { ...entry, call_mode_active: nextCallMode } : entry,
      ),
    );
    try {
      await updateConversationCallMode(conversation.id, nextCallMode);
      await invalidateQuery(conversationsKey, { refetch: true });
    } finally {
      setTogglingCallModeId(null);
    }
  }
  const threadModeCounts = useMemo(() => {
    const all = conversations.filter((c) => !c.archived).length;
    const unread = conversations.filter((c) => Boolean(c.continuation)).length;
    const needsReview = conversations.filter((c) =>
      Boolean(c.continuation?.continuation.review_requirements?.length)
      || c.continuation?.continuation.continuation_category === 'needs_input',
    ).length;
    const active = resolvedConversationId ? 1 : 0;
    const archived = conversations.filter((c) => c.archived).length;
    return { all, unread, needsReview, active, archived };
  }, [conversations, resolvedConversationId]);

  const filteredConversations = useMemo(() => {
    const query = threadFilter.trim().toLowerCase();
    return conversations
      .filter((conversation) => {
        if (filterMode === 'unread') {
          return Boolean(conversation.continuation);
        }
        if (filterMode === 'needs_review') {
          return Boolean(conversation.continuation?.continuation.review_requirements?.length)
            || conversation.continuation?.continuation.continuation_category === 'needs_input';
        }
        if (filterMode === 'active') {
          return conversation.id === resolvedConversationId;
        }
        if (filterMode === 'archived') {
          return conversation.archived;
        }
        if (conversation.archived) {
          return false;
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
    return (
      <section className="mx-auto flex min-h-0 w-full max-w-3xl flex-1 items-center justify-center px-4 py-10 sm:px-6">
        <div className="w-full max-w-xl rounded-[28px] border border-[var(--vel-color-border)] bg-[var(--vel-color-panel)] px-6 py-8 text-center shadow-[0_24px_60px_rgba(0,0,0,0.18)]">
          <p className={`${uiFonts.display} text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
            Threads
          </p>
          <h1 className="mt-3 text-3xl font-semibold tracking-tight text-[var(--vel-color-text)]">No thread selected yet</h1>
          <p className="mt-3 text-sm leading-6 text-[var(--vel-color-muted)]">
            Start a conversation from `Now` and the latest thread will land here. Until then, this space stays intentionally quiet.
          </p>
        </div>
      </section>
    );
  }

  const error = conversationsError ?? messagesError;
  if (messagesLoading) {
    return <SurfaceState message="Loading conversation…" layout="centered" />;
  }
  if (error) {
    return <SurfaceState message={error} layout="centered" tone="danger" />;
  }

  const boundObject = selectedConversation?.continuation ?? null;
  const contextRows = selectedConversation ? continuationContextRows(selectedConversation) : [];
  const messageCount = messages.length;
  const headerMessageCount = selectedConversation?.message_count ?? messageCount;
  const openedHeaderTags = [
    selectedConversation?.kind ? selectedConversation.kind.replaceAll('_', ' ') : null,
    selectedConversation?.project_label ?? null,
    boundObject?.lifecycle_stage ?? null,
    selectedConversation?.call_mode_active ? 'call mode active' : null,
  ].filter(Boolean) as string[];

  return (
    <>
      <div className="flex min-h-0 flex-1">
        <aside
          className="hidden shrink-0 border-r border-[var(--vel-color-border)] lg:block w-full max-w-[20rem]"
        >
          <div className="sticky top-[5.25rem] flex min-h-[32rem] flex-col">
            <div className="flex items-center justify-between border-b border-[var(--vel-color-border)] px-3 py-3">
              <p className={`${uiFonts.display} inline-flex items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-muted)]`}>
                <ThreadsIcon size={12} />
                THREADS ({threadModeCounts.all})
              </p>
            </div>
            <>
                <div className="border-b border-[var(--vel-color-border)] px-3 py-3">
                  <div className="flex flex-wrap gap-1.5" role="group" aria-label="Thread list filter">
                    {(
                      [
                        {
                          mode: 'all' as const,
                          label: 'All',
                          count: threadModeCounts.all,
                          icon: (sel: boolean) => (
                            <LayersIcon size={12} className={sel ? 'text-[var(--vel-color-accent-soft)]' : 'text-[var(--vel-color-dim)]'} />
                          ),
                        },
                        {
                          mode: 'unread' as const,
                          label: 'Unread',
                          count: threadModeCounts.unread,
                          icon: (sel: boolean) => (
                            <ThreadsIcon size={12} className={sel ? 'text-[var(--vel-color-accent-soft)]' : 'text-[var(--vel-color-dim)]'} />
                          ),
                        },
                        {
                          mode: 'needs_review' as const,
                          label: 'Needs Review',
                          count: threadModeCounts.needsReview,
                          icon: (sel: boolean) => (
                            <WarningIcon size={12} className={sel ? 'text-[var(--vel-color-accent-soft)]' : 'text-[var(--vel-color-dim)]'} />
                          ),
                        },
                        {
                          mode: 'active' as const,
                          label: 'Active',
                          count: threadModeCounts.active,
                          icon: (sel: boolean) => (
                            <OpenThreadIcon size={12} className={sel ? 'text-[var(--vel-color-accent-soft)]' : 'text-[var(--vel-color-dim)]'} />
                          ),
                        },
                        {
                          mode: 'archived' as const,
                          label: 'Archived',
                          count: threadModeCounts.archived,
                          icon: (sel: boolean) => (
                            <ArchiveIcon size={12} className={sel ? 'text-[var(--vel-color-accent-soft)]' : 'text-[var(--vel-color-dim)]'} />
                          ),
                        },
                      ] as const
                    ).map(({ mode, label, count, icon }) => (
                      <FilterToggleTag
                        key={mode}
                        label={label}
                        count={count}
                        size="dense"
                        selected={filterMode === mode}
                        onClick={() => setFilterMode(mode)}
                        icon={icon(filterMode === mode)}
                      />
                    ))}
                  </div>
                  <SearchField
                    className="mt-3"
                    aria-label="Filter threads"
                    value={threadFilter}
                    onChange={(event) => setThreadFilter(event.target.value)}
                    placeholder="Find thread"
                  />
                </div>

                <div className="relative flex-1 overflow-visible">
                  <div className="pointer-events-none absolute inset-x-0 top-0 z-10 h-5 bg-gradient-to-b from-[var(--vel-color-bg)] to-transparent" />
                  <div className="pointer-events-none absolute inset-x-0 bottom-0 z-10 h-5 bg-gradient-to-t from-[var(--vel-color-bg)] to-transparent" />
                  {filteredConversations.length === 0 ? (
                    <PanelEmptyRow>No threads match that filter.</PanelEmptyRow>
                  ) : (
                    filteredConversations.map((conversation) => (
                      <ThreadListRow
                        key={conversation.id}
                        conversation={conversation}
                        active={conversation.id === resolvedConversationId}
                        disabled={!onSelectConversation || conversation.id === resolvedConversationId}
                        onSelect={onSelectConversation}
                      />
                    ))
                  )}
                </div>
            </>
          </div>
        </aside>

        <section className="relative flex min-w-0 flex-1 flex-col">
          <div ref={scrollRef} className="flex-1 overflow-y-auto">
            <div className="mx-auto flex max-w-5xl flex-col gap-4 px-4 py-4 sm:px-6">
              <section className="space-y-3 pb-1">
                <div className="flex flex-wrap items-start justify-between gap-3 px-1 py-2">
                  <div className="min-w-0 space-y-2">
                    <div className={`${uiFonts.display} flex flex-wrap items-center gap-2 text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)]`}>
                      <span>CURRENT THREAD | {headerMessageCount} {headerMessageCount === 1 ? 'MESSAGE' : 'MESSAGES'} | PARTICIPANTS</span>
                      <div className="flex items-center gap-1.5 text-[11px] leading-none">
                        <ParticipantDot label="Y" className="border-emerald-700/60 bg-emerald-950/50 text-emerald-100" />
                        <ParticipantDot label="V" className="border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] text-[var(--vel-color-accent-soft)]" />
                      </div>
                    </div>
                    <div className="flex flex-wrap items-center gap-2 text-[11px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">
                      <span>LATEST {formatAbsoluteTimestamp(lastMessageAt(messages, selectedConversation?.updated_at ?? null))}</span>
                      <span aria-hidden>|</span>
                      <span>CREATED {formatAbsoluteTimestamp(selectedConversation?.created_at ?? null)}</span>
                    </div>
                    {editingTitle ? (
                      <input
                        type="text"
                        value={draftTitle}
                        onChange={(event) => setDraftTitle(event.target.value)}
                        onBlur={() => {
                          setEditingTitle(false);
                          void saveTitle(draftTitle);
                        }}
                        onKeyDown={(event) => {
                          if (event.key === 'Enter') {
                            setEditingTitle(false);
                            void saveTitle(draftTitle);
                          }
                          if (event.key === 'Escape') {
                            setDraftTitle(threadTitle(selectedConversation));
                            setEditingTitle(false);
                          }
                        }}
                        autoFocus
                        className="w-full bg-transparent text-2xl font-semibold tracking-tight text-[var(--vel-color-text)] outline-none"
                      />
                    ) : (
                      <button
                        type="button"
                        onClick={() => setEditingTitle(true)}
                        className="truncate text-left text-2xl font-semibold tracking-tight text-[var(--vel-color-text)] hover:text-[var(--vel-color-accent-soft)]"
                      >
                        {draftTitle}
                      </button>
                    )}
                    {savingTitle ? <p className="text-[11px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">SAVING…</p> : null}
                    {boundObject?.continuation.escalation_reason ? (
                      <p className="max-w-3xl text-sm leading-6 text-[var(--vel-color-muted)]">
                        {boundObject.continuation.escalation_reason}
                      </p>
                    ) : null}
                  </div>
                  <div className="flex flex-wrap items-center justify-end gap-2">
                    {openedHeaderTags.map((tag) => (
                      <FilterDenseTag key={tag} tone="muted">
                        {tag}
                      </FilterDenseTag>
                    ))}
                    <ActionChipButton
                      tone={selectedConversation?.call_mode_active ? 'brand' : 'ghost'}
                      disabled={!selectedConversation || Boolean(togglingCallModeId)}
                      onClick={() => {
                        if (selectedConversation) {
                          void toggleConversationCallMode(selectedConversation);
                        }
                      }}
                    >
                      <MicIcon size={12} />
                      {selectedConversation?.call_mode_active ? 'End call' : 'Start call'}
                    </ActionChipButton>
                    <ActionChipButton
                      tone="ghost"
                      aria-label="Archive thread"
                      className="border border-[var(--vel-color-border)] bg-transparent text-[var(--vel-color-muted)] hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-accent-soft)]"
                      disabled={!selectedConversation || Boolean(archivingConversationId)}
                      onClick={() => {
                        if (selectedConversation) {
                          void archiveConversation(selectedConversation);
                        }
                      }}
                    >
                      <ArchiveIcon size={12} />
                      Archive
                    </ActionChipButton>
                  </div>
                </div>
              </section>

              {boundObject || contextRows.length > 0 || selectedConversation?.call_mode_active ? (
                <section className="space-y-3 border-b border-[var(--vel-color-border)] pb-4">
                  {selectedConversation?.call_mode_active ? (
                    <p className="max-w-3xl text-sm leading-6 text-[var(--vel-color-muted)]">
                      Call mode is active for this thread. Browser speech-to-text still goes through the normal assistant path, and new assistant replies on this thread can speak back locally.
                    </p>
                  ) : null}
                  <div className="flex flex-wrap items-center gap-2">
                    {boundObject ? (
                      <FilterDenseTag tone="muted">
                        {boundObject.thread_type.replaceAll('_', ' ')}
                      </FilterDenseTag>
                    ) : null}
                    {boundObject ? (
                      <FilterDenseTag tone="muted">
                        {capabilityStateLabel(boundObject.continuation.bounded_capability_state)}
                      </FilterDenseTag>
                    ) : null}
                    {boundObject ? (
                      <FilterDenseTag tone="muted">
                        {continuationTokenLabel(boundObject.continuation.open_target)}
                      </FilterDenseTag>
                    ) : null}
                  </div>
                  {contextRows.length > 0 ? (
                    <div className="grid gap-2 text-xs text-[var(--vel-color-muted)]">
                      {contextRows.map((entry) => (
                        <PanelKeyValueRow key={entry.label} label={entry.label} value={entry.value} />
                      ))}
                    </div>
                  ) : null}
                </section>
              ) : null}

              <section className="min-w-0 space-y-4">
                {messages.length === 0 ? (
                  <SurfaceState message="No messages in this thread yet." />
                ) : (
                  messages.map((message) => (
                    <MessageRenderer
                      key={message.id}
                      message={message}
                      onShowWhy={setProvenanceMessageId}
                    />
                  ))
                )}
              </section>
            </div>
          </div>
          {provenanceMessageId ? (
            <ProvenanceDrawer
              messageId={provenanceMessageId}
              onClose={() => setProvenanceMessageId(null)}
            />
          ) : null}
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

function formatAbsoluteTimestamp(ts: number | null): string {
  if (!ts) {
    return 'Unknown';
  }
  return new Intl.DateTimeFormat('en-US', {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  }).format(new Date(ts * 1000));
}

function formatRelativeFreshness(ts: number): string {
  const diffSeconds = Math.max(0, Math.round(Date.now() / 1000) - ts);
  if (diffSeconds < 60) {
    return 'now';
  }
  if (diffSeconds < 3600) {
    return `${Math.floor(diffSeconds / 60)}m ago`;
  }
  if (diffSeconds < 86_400) {
    return `${Math.floor(diffSeconds / 3600)}h ago`;
  }
  return `${Math.floor(diffSeconds / 86_400)}d ago`;
}

function lastMessageAt(messages: MessageData[], fallback: number | null | undefined): number | null {
  if (messages.length === 0) {
    return fallback ?? null;
  }
  return [...messages].sort((left, right) => right.created_at - left.created_at)[0]?.created_at ?? fallback ?? null;
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
  const lastMessageTimestamp = lastMessageAt(
    messages,
    conversation.last_message_at ?? conversation.updated_at,
  );

  return (
    <button
      type="button"
      onClick={() => onSelect?.(conversation.id)}
      disabled={disabled}
      className={cn('w-full px-3 py-2 text-left transition disabled:cursor-default', active ? '' : 'opacity-70 hover:opacity-100')}
      aria-current={active ? 'true' : undefined}
    >
      <ObjectRowFrame
        as="div"
        tone={active ? 'activeBrand' : 'neutral'}
        density="button"
        className={cn(
          active
            ? 'rounded-[1.05rem] px-3 py-2 ring-1 ring-[color:var(--vel-color-accent-border)]/70 shadow-[0_0_0_1px_rgba(255,107,0,0.12),0_0_0_5px_rgba(255,107,0,0.08)]'
            : 'border-none bg-transparent p-0 shadow-none',
        )}
      >
        <ObjectRowLayout
          leading={
            <span className="mt-1 flex h-3 w-3 shrink-0 items-center justify-center" role={unreadCount > 0 ? 'img' : undefined} aria-label={unreadCount > 0 ? 'Unread continuation' : undefined}>
              {unreadCount > 0 ? <DotIcon size={8} className="text-[var(--vel-color-accent)]" /> : null}
            </span>
          }
          actionsLayout="inline"
          actions={
            <div className="text-right">
              <div className={cn('text-xs font-medium', active ? 'text-[var(--vel-color-accent-strong)]' : 'text-[var(--vel-color-muted)]')}>
                {formatRelativeFreshness(lastMessageTimestamp ?? conversation.updated_at)}
              </div>
            </div>
          }
        >
          <ObjectRowTitleMetaBand
            title={threadTitle(conversation)}
            meta={
              conversation.project_label ? (
                <FilterDenseTag tone="muted">{conversation.project_label}</FilterDenseTag>
              ) : null
            }
          />
          <p className={cn('line-clamp-2 text-xs leading-snug', active ? 'text-[var(--vel-color-text)]' : 'text-[var(--vel-color-muted)]')}>
            {previewText(conversation, messages)}
          </p>
        </ObjectRowLayout>
      </ObjectRowFrame>
    </button>
  );
}

function ParticipantDot({ label, className }: { label: string; className?: string }) {
  return (
    <span className={cn('inline-flex h-[1.05rem] w-[1.05rem] items-center justify-center rounded-full border text-[11px] font-medium uppercase leading-none align-middle [font-variant-numeric:tabular-nums]', className)}>
      {label}
    </span>
  );
}
