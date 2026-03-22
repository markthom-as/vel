import { useEffect, useMemo, useState } from 'react';
import type { ReactNode } from 'react';
import type { InboxItemData } from '../../types';
import { chatQueryKeys, invalidateInboxQueries, loadInbox, type InboxScope } from '../../data/chat';
import { setQueryData, useQuery } from '../../data/query';
import {
  markPendingInterventionActionConfirmed,
  prunePendingInterventionActions,
  setPendingInterventionAction,
  type PendingInterventionAction,
} from '../../data/chat-state';
import { subscribeWsQuerySync } from '../../data/ws-sync';
import { Button } from '../../core/Button';
import { FilterMetricToggleTag } from '../../core/FilterToggleTag';
import {
  ClipboardCheckIcon,
  ClockIcon,
  FolderIcon,
  LayersIcon,
  OpenThreadIcon,
  RescheduleIcon,
  SparkIcon,
  TagIcon,
} from '../../core/Icons';
import {
  PanelEyebrow,
  PanelPageSection,
  PanelSectionHeaderBand,
  PanelSectionHeaderLead,
  PanelSectionHeaderTrail,
} from '../../core/PanelChrome';
import { surfaceShell } from '../../core/Theme';
import { SurfaceState } from '../../core/SurfaceState';
import { InboxItemCard } from './InboxItemCard';
import { InboxQueueSegmentedControl } from './InboxQueueSegmentedControl';

/** Inbox UI is queue-only; archive scope is not exposed in this surface. */
const INBOX_SCOPE: InboxScope = 'queue';

interface InboxViewProps {
  onOpenThread?: (conversationId: string) => void;
}

export function InboxView({ onOpenThread }: InboxViewProps) {
  const inboxKey = useMemo(() => chatQueryKeys.inbox(INBOX_SCOPE), []);
  const pendingInterventionActionsKey = useMemo(
    () => chatQueryKeys.pendingInterventionActions(),
    [],
  );
  const { data: items = [], loading, error } = useQuery<InboxItemData[]>(
    inboxKey,
    async () => {
      const response = await loadInbox(INBOX_SCOPE);
      return response.ok && response.data ? response.data : [];
    },
  );
  const { data: pendingInterventionActions = {} } = useQuery<Record<string, PendingInterventionAction>>(
    pendingInterventionActionsKey,
    async () => ({}),
    { enabled: false },
  );
  const visibleItems = items.filter((item) => pendingInterventionActions[item.id] === undefined);
  const { newCount, openedCount, archivedCount } = countInboxQueueStates(visibleItems);

  const [queueFilter, setQueueFilter] = useState<'all' | 'new' | 'opened' | 'archived'>('all');
  const [kindFilter, setKindFilter] = useState('');
  const [projectFilter, setProjectFilter] = useState('');

  const queueScopedItems = useMemo(() => {
    if (queueFilter === 'all') return visibleItems;
    return visibleItems.filter((item) => inboxQueueBucket(item.state) === queueFilter);
  }, [visibleItems, queueFilter]);

  const kindOptions = useMemo(() => {
    const kinds = new Set<string>();
    for (const item of queueScopedItems) kinds.add(item.kind);
    return Array.from(kinds).sort((a, b) => a.localeCompare(b));
  }, [queueScopedItems]);

  const itemsForProjectCounts = useMemo(() => {
    if (!kindFilter) return queueScopedItems;
    return queueScopedItems.filter((item) => item.kind === kindFilter);
  }, [queueScopedItems, kindFilter]);

  const projectOptions = useMemo(() => {
    const map = new Map<string, string>();
    for (const item of itemsForProjectCounts) {
      const key = item.project_id ?? '__unassigned__';
      const label = item.project_label?.trim() || 'Unassigned';
      if (!map.has(key)) map.set(key, label);
    }
    return Array.from(map.entries()).sort((a, b) => a[1].localeCompare(b[1]));
  }, [itemsForProjectCounts]);

  const kindCountMap = useMemo(() => {
    const map = new Map<string, number>();
    for (const item of queueScopedItems) {
      map.set(item.kind, (map.get(item.kind) ?? 0) + 1);
    }
    return map;
  }, [queueScopedItems]);

  const projectCountMap = useMemo(() => {
    const map = new Map<string, number>();
    for (const item of itemsForProjectCounts) {
      const key = item.project_id ?? '__unassigned__';
      map.set(key, (map.get(key) ?? 0) + 1);
    }
    return map;
  }, [itemsForProjectCounts]);

  const totalQueueCount = newCount + openedCount + archivedCount;

  const filteredItems = useMemo(() => {
    return visibleItems.filter((item) => {
      if (queueFilter !== 'all' && inboxQueueBucket(item.state) !== queueFilter) {
        return false;
      }
      if (kindFilter && item.kind !== kindFilter) {
        return false;
      }
      if (projectFilter) {
        const key = item.project_id ?? '__unassigned__';
        if (key !== projectFilter) return false;
      }
      return true;
    });
  }, [visibleItems, queueFilter, kindFilter, projectFilter]);

  const filtersActive = queueFilter !== 'all' || kindFilter !== '' || projectFilter !== '';

  useEffect(() => {
    return subscribeWsQuerySync();
  }, []);

  useEffect(() => {
    setQueryData<Record<string, PendingInterventionAction>>(
      pendingInterventionActionsKey,
      (prev = {}) => prunePendingInterventionActions(prev, items),
    );
  }, [items, pendingInterventionActionsKey]);

  async function runInterventionAction(
    item: InboxItemData,
    nextState: 'acknowledged' | 'dismissed' | 'snoozed' | 'resolved' | 'active',
    action: () => Promise<{ ok: boolean; data?: { state: string }; error?: { message: string } }>,
  ) {
    setQueryData<Record<string, PendingInterventionAction>>(
      pendingInterventionActionsKey,
      (prev = {}) => setPendingInterventionAction(prev, item.id, nextState),
    );

    try {
      const response = await action();
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update inbox item');
      }
      setQueryData<Record<string, PendingInterventionAction>>(
        pendingInterventionActionsKey,
        (prev = {}) =>
          markPendingInterventionActionConfirmed(prev, item.id, response.data?.state ?? nextState),
      );
      invalidateInboxQueries();
    } catch {
      setQueryData<Record<string, PendingInterventionAction>>(
        pendingInterventionActionsKey,
        (prev = {}) => {
          const next = { ...prev };
          delete next[item.id];
          return next;
        },
      );
    }
  }

  if (loading) {
    return <SurfaceState message="Loading inbox…" />;
  }
  if (error) {
    return <SurfaceState message={error} tone="danger" />;
  }

  return (
    <div className={surfaceShell.mainColumn}>
      <div className={surfaceShell.scrollColumn}>
        <div className={surfaceShell.mainContent}>
        <section className={surfaceShell.sectionStack}>
        <header className="space-y-4">
          <div className="min-w-0">
            <PanelEyebrow tracking="wide">Inbox</PanelEyebrow>
            <h1 className="mt-2 text-2xl font-semibold text-zinc-100">Inbox</h1>
          </div>
          <div className="space-y-2">
            <PanelSectionHeaderBand mode="section-header">
              <PanelSectionHeaderLead>
                <h2 className="text-lg font-medium text-zinc-100">Queue</h2>
              </PanelSectionHeaderLead>
              <PanelSectionHeaderTrail>
                <InboxQueueSegmentedControl
                  queueFilter={queueFilter}
                  onQueueFilterChange={setQueueFilter}
                  newCount={newCount}
                  openedCount={openedCount}
                  archivedCount={archivedCount}
                  totalQueueCount={totalQueueCount}
                />
              </PanelSectionHeaderTrail>
            </PanelSectionHeaderBand>
            <p className="text-xs uppercase tracking-[0.14em] text-zinc-500">Filter by read state</p>
          </div>
        </header>

        {visibleItems.length > 0 ? (
          <div
            className="flex flex-col gap-3 rounded-[16px] border border-zinc-800/90 bg-zinc-950/40 p-4"
            role="group"
            aria-label="Inbox filters"
          >
            <div className="flex flex-wrap items-center justify-between gap-2">
              <p className="text-[11px] uppercase tracking-[0.2em] text-zinc-500">Filters</p>
              {filtersActive ? (
                <button
                  type="button"
                  onClick={() => {
                    setQueueFilter('all');
                    setKindFilter('');
                    setProjectFilter('');
                  }}
                  className="text-xs text-zinc-400 underline-offset-2 hover:text-zinc-200 hover:underline"
                >
                  Clear filters
                </button>
              ) : null}
            </div>
            <div className="flex flex-col gap-3">
              <div>
                <PanelEyebrow className="mb-2 text-zinc-600">Kind</PanelEyebrow>
                <div className="flex flex-wrap gap-1.5" role="group" aria-label="Inbox kind">
                  <FilterMetricToggleTag
                    label="All kinds"
                    count={queueScopedItems.length}
                    selected={kindFilter === ''}
                    onClick={() => setKindFilter('')}
                    icon={<TagIcon size={12} className={kindFilter === '' ? 'text-amber-200/90' : 'text-zinc-500'} />}
                  />
                  {kindOptions.map((kind) => (
                    <FilterMetricToggleTag
                      key={kind}
                      label={formatKind(kind)}
                      count={kindCountMap.get(kind) ?? 0}
                      selected={kindFilter === kind}
                      onClick={() => setKindFilter(kind)}
                      icon={inboxKindIcon(kind, kindFilter === kind)}
                    />
                  ))}
                </div>
              </div>
              <div>
                <PanelEyebrow className="mb-2 text-zinc-600">Project</PanelEyebrow>
                <div className="flex flex-wrap gap-1.5" role="group" aria-label="Inbox project">
                  <FilterMetricToggleTag
                    label="All projects"
                    count={itemsForProjectCounts.length}
                    selected={projectFilter === ''}
                    onClick={() => setProjectFilter('')}
                    icon={
                      <LayersIcon size={12} className={projectFilter === '' ? 'text-amber-200/90' : 'text-zinc-500'} />
                    }
                  />
                  {projectOptions.map(([id, lab]) => (
                    <FilterMetricToggleTag
                      key={id}
                      label={lab}
                      count={projectCountMap.get(id) ?? 0}
                      selected={projectFilter === id}
                      onClick={() => setProjectFilter(id)}
                      icon={
                        <FolderIcon
                          size={12}
                          className={projectFilter === id ? 'text-amber-200/90' : 'text-zinc-500'}
                        />
                      }
                    />
                  ))}
                </div>
              </div>
            </div>
          </div>
        ) : null}

        {visibleItems.length === 0 ? (
          <PanelPageSection className="!p-6">
            <p className="text-sm text-zinc-300">No open queue items.</p>
          </PanelPageSection>
        ) : filteredItems.length === 0 ? (
          <PanelPageSection className="!p-6">
            <p className="text-sm text-zinc-300">No items match the current filters.</p>
            <Button variant="outline" size="sm" className="mt-3" onClick={() => {
              setQueueFilter('all');
              setKindFilter('');
              setProjectFilter('');
            }}>
              Reset filters
            </Button>
          </PanelPageSection>
        ) : (
          <div className="space-y-3">
            {filteredItems.map((item) => (
              <InboxItemCard
                key={item.id}
                item={item}
                onOpenThread={onOpenThread}
                runInterventionAction={runInterventionAction}
              />
            ))}
          </div>
        )}
        </section>
        </div>
      </div>
    </div>
  );
}

function inboxQueueBucket(state: string): 'new' | 'opened' | 'archived' {
  const s = state.toLowerCase();
  if (s === 'active') return 'new';
  if (s === 'resolved' || s === 'dismissed') return 'archived';
  return 'opened';
}

function countInboxQueueStates(items: InboxItemData[]): {
  newCount: number;
  openedCount: number;
  archivedCount: number;
} {
  let newCount = 0;
  let openedCount = 0;
  let archivedCount = 0;
  for (const item of items) {
    switch (inboxQueueBucket(item.state)) {
      case 'new':
        newCount++;
        break;
      case 'opened':
        openedCount++;
        break;
      case 'archived':
        archivedCount++;
        break;
    }
  }
  return { newCount, openedCount, archivedCount };
}

function inboxKindIcon(kind: string, selected: boolean): ReactNode {
  const cls = selected ? 'text-amber-200/90' : 'text-zinc-500';
  const k = kind.toLowerCase();
  if (k.includes('reminder') || k.includes('due')) {
    return <RescheduleIcon size={12} className={cls} />;
  }
  if (k.includes('follow')) {
    return <OpenThreadIcon size={12} className={cls} />;
  }
  if (k.includes('commit') || k.includes('task')) {
    return <ClipboardCheckIcon size={12} className={cls} />;
  }
  if (k.includes('intervention') || k.includes('assistant') || k.includes('synth')) {
    return <SparkIcon size={12} className={cls} />;
  }
  if (k.includes('snooze') || k.includes('wait')) {
    return <ClockIcon size={12} className={cls} />;
  }
  return <TagIcon size={12} className={cls} />;
}

function formatKind(kind: string): string {
  return kind.replaceAll('_', ' ');
}
