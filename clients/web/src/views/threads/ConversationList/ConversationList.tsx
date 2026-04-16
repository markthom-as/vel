import { useEffect, useMemo, useState } from 'react';
import type { ConversationData } from '../../../types';
import {
  chatQueryKeys,
  loadConversationList,
  updateConversationArchive,
  updateConversationPinned,
} from '../../../data/chat';
import { invalidateQuery, setQueryData, useQuery } from '../../../data/query';
import { subscribeWsQuerySync } from '../../../data/ws-sync';
import { SurfaceState } from '../../../core/SurfaceState';
import { cn } from '../../../core/cn';
import { FilterDenseTag } from '../../../core/FilterToggleTag';
import { DotIcon, ThreadsIcon, WarningIcon } from '../../../core/Icons';

interface ConversationListProps {
  selectedId: string | null;
  onSelect: (id: string) => void;
  variant?: 'standard' | 'split-compact';
  collapsible?: boolean;
}

export function ConversationList({
  selectedId,
  onSelect,
  variant = 'standard',
  collapsible = false,
}: ConversationListProps) {
  const [collapsed, setCollapsed] = useState(false);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data: conversations = [], loading, error } = useQuery<ConversationData[]>(
    conversationsKey,
    async () => {
      const response = await loadConversationList();
      return response.ok && response.data ? response.data : [];
    },
  );

  useEffect(() => {
    return subscribeWsQuerySync();
  }, []);

  async function setConversationPinned(conversation: ConversationData, pinned: boolean) {
    setQueryData<ConversationData[]>(conversationsKey, (current = []) =>
      current.map((entry) =>
        entry.id === conversation.id ? { ...entry, pinned } : entry,
      ),
    );
    await updateConversationPinned(conversation.id, pinned);
    invalidateQuery(conversationsKey, { refetch: true });
  }

  async function archiveConversation(conversation: ConversationData) {
    setQueryData<ConversationData[]>(conversationsKey, (current = []) =>
      current.map((entry) =>
        entry.id === conversation.id ? { ...entry, archived: true } : entry,
      ),
    );
    await updateConversationArchive(conversation.id, true);
    invalidateQuery(conversationsKey, { refetch: true });
  }

  if (loading) return <SurfaceState message="Loading conversations…" />;
  if (error) return <SurfaceState message={error} tone="danger" />;

  if (collapsible && collapsed) {
    return (
      <section
        data-testid="conversation-list-shell"
        data-list-variant={variant}
        data-collapsed="true"
        className="flex min-h-0 flex-col px-2 py-2"
      >
        <button
          type="button"
          aria-label="Expand compact thread list"
          onClick={() => setCollapsed(false)}
          className="inline-flex min-h-10 items-center justify-center rounded-lg border border-[var(--vel-color-border)] px-2 text-[10px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)] transition hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]"
        >
          Threads
        </button>
      </section>
    );
  }

  return (
    <section
      data-testid="conversation-list-shell"
      data-list-variant={variant}
      data-collapsed="false"
      className="flex min-h-0 flex-1 flex-col"
    >
      {collapsible ? (
        <div className="flex items-center justify-end px-2 pt-2">
          <button
            type="button"
            aria-label="Collapse compact thread list"
            onClick={() => setCollapsed(true)}
            className="inline-flex min-h-8 items-center justify-center rounded-lg border border-[var(--vel-color-border)] px-2 text-[10px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)] transition hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]"
          >
            Hide
          </button>
        </div>
      ) : null}
      <ul
        className={cn(
          'flex-1 space-y-1 overflow-y-auto px-2 py-2',
          variant === 'split-compact' ? 'py-1' : '',
        )}
        aria-label="Conversations"
      >
        {conversations.map((conversation) => (
          <ConversationRow
            key={conversation.id}
            conversation={conversation}
            selected={selectedId === conversation.id}
            compact={variant === 'split-compact'}
            onSelect={onSelect}
            onPinnedChange={setConversationPinned}
            onArchive={archiveConversation}
          />
        ))}
        {conversations.length === 0 && (
          <li><SurfaceState message="No conversations yet." /></li>
        )}
      </ul>
    </section>
  );
}

function ConversationRow({
  conversation,
  selected,
  compact,
  onSelect,
  onPinnedChange,
  onArchive,
}: {
  conversation: ConversationData;
  selected: boolean;
  compact: boolean;
  onSelect: (id: string) => void;
  onPinnedChange: (conversation: ConversationData, pinned: boolean) => Promise<void>;
  onArchive: (conversation: ConversationData) => Promise<void>;
}) {
  const [actionsOpen, setActionsOpen] = useState(false);
  const [pendingAction, setPendingAction] = useState<'pin' | 'archive' | null>(null);
  const hasContinuation = Boolean(conversation.continuation);
  const needsReview =
    Boolean(conversation.continuation?.continuation.review_requirements?.length)
    || conversation.continuation?.continuation.continuation_category === 'needs_input';
  const updatedAt = conversation.last_message_at ?? conversation.updated_at;
  const title = conversation.title?.trim() || 'Untitled';

  async function runRowAction(action: 'pin' | 'archive') {
    if (pendingAction) return;
    setPendingAction(action);
    try {
      if (action === 'pin') {
        await onPinnedChange(conversation, !conversation.pinned);
      } else {
        await onArchive(conversation);
      }
      setActionsOpen(false);
    } finally {
      setPendingAction(null);
    }
  }

  return (
    <li>
      <div
        aria-current={selected ? 'true' : undefined}
        data-conversation-id={conversation.id}
        className={cn(
          'group relative flex w-full items-stretch rounded-lg border text-left transition',
          compact ? 'min-h-11 gap-2 px-2 py-1.5' : 'min-h-14 gap-3 px-3 py-2',
          selected
            ? 'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] text-[var(--vel-color-text)] shadow-[0_0_0_1px_rgba(255,107,0,0.12)]'
            : 'border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/40 text-[var(--vel-color-muted)] hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]',
        )}
      >
        <button
          type="button"
          onClick={() => onSelect(conversation.id)}
          aria-current={selected ? 'true' : undefined}
          aria-label={`${title}${hasContinuation ? ', unread continuation' : ''}${needsReview ? ', needs review' : ''}`}
          data-conversation-id={conversation.id}
          className={cn(
            'flex min-w-0 flex-1 items-stretch text-left',
            compact ? 'min-h-11 gap-2' : 'min-h-14 gap-3',
          )}
        >
          <span className={cn('flex shrink-0 items-start justify-center', compact ? 'w-4 pt-1' : 'w-5 pt-1.5')}>
            {hasContinuation ? (
              <span aria-label="Unread continuation" role="img" className="text-[var(--vel-color-accent)]">
                <DotIcon size={10} />
              </span>
            ) : (
              <ThreadsIcon size={14} className="mt-px text-[var(--vel-color-dim)] group-hover:text-[var(--vel-color-muted)]" />
            )}
          </span>
          <span className="flex min-w-0 flex-1 flex-col justify-center gap-1">
            <span className="flex min-w-0 items-center gap-1.5">
              <span className="min-w-0 flex-1 truncate text-sm font-medium leading-tight">{title}</span>
              {conversation.pinned ? (
                <FilterDenseTag tone="brand" casing="normal" className="!tracking-normal">
                  Pinned
                </FilterDenseTag>
              ) : null}
            </span>
            <span className="flex min-w-0 flex-wrap items-center gap-1.5">
              {needsReview ? (
                <FilterDenseTag tone="brand">
                  <WarningIcon size={10} />
                  Review
                </FilterDenseTag>
              ) : hasContinuation ? (
                <FilterDenseTag tone="muted">Unread</FilterDenseTag>
              ) : null}
              {conversation.project_label ? (
                <FilterDenseTag tone="muted" casing="normal">{conversation.project_label}</FilterDenseTag>
              ) : null}
              {compact ? null : (
                <span className="shrink-0 text-[11px] text-[var(--vel-color-muted)]">{formatTs(updatedAt)}</span>
              )}
            </span>
          </span>
        </button>
        <div className="relative flex shrink-0 items-center">
          <button
            type="button"
            aria-label={`More actions for ${title}`}
            aria-expanded={actionsOpen}
            onClick={() => setActionsOpen((open) => !open)}
            className="inline-flex min-h-10 min-w-10 items-center justify-center rounded-lg border border-[var(--vel-color-border)] text-[11px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)] transition hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]"
          >
            More
          </button>
          {actionsOpen ? (
            <div
              role="menu"
              aria-label={`Actions for ${title}`}
              className="absolute right-0 top-full z-20 mt-1 grid min-w-36 gap-1 rounded-lg border border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)] p-1 shadow-lg"
            >
              <button
                type="button"
                role="menuitem"
                onClick={() => {
                  void runRowAction('pin');
                }}
                disabled={pendingAction !== null}
                className="min-h-10 rounded-md px-3 text-left text-xs text-[var(--vel-color-text)] transition hover:bg-white/5 disabled:opacity-50"
              >
                {conversation.pinned ? 'Unpin' : 'Pin'}
              </button>
              <button
                type="button"
                role="menuitem"
                onClick={() => {
                  void runRowAction('archive');
                }}
                disabled={pendingAction !== null}
                className="min-h-10 rounded-md px-3 text-left text-xs text-[var(--vel-color-text)] transition hover:bg-white/5 disabled:opacity-50"
              >
                Archive
              </button>
              <button
                type="button"
                role="menuitem"
                disabled
                className="min-h-10 rounded-md px-3 text-left text-xs text-[var(--vel-color-muted)] opacity-55"
              >
                Mute unavailable
              </button>
            </div>
          ) : null}
        </div>
      </div>
    </li>
  );
}

function formatTs(ts: number): string {
  const d = new Date(ts * 1000);
  const now = new Date();
  if (d.toDateString() === now.toDateString()) {
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
  return d.toLocaleDateString([], { month: 'short', day: 'numeric' });
}
