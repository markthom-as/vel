import { useEffect, useMemo, type ReactNode } from 'react';
import { contextQueryKeys, loadNow } from '../../data/context';
import { useQuery } from '../../data/query';
import type { MainView } from '../../data/operatorSurfaces';
import type { NowData } from '../../types';
import { FilterDenseTag } from '../../core/FilterToggleTag';
import { SyncIcon, ThreadsIcon, WarningIcon } from '../../core/Icons';
import { nowNavContextSummary } from '../../views/now/nowModel';
import { formatNavbarDateTime } from './formatNavbarDateTime';
import { NAVBAR_HEADER_CLASSNAME } from './navbarChrome';
import { NavbarBrand } from './NavbarBrand';
import { NavbarInfoButton } from './NavbarInfoButton';
import { NavbarNavLinks } from './NavbarNavLinks';

export interface NavbarProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
  onOpenDocumentation: () => void;
  infoPanelOpen: boolean;
}

export function Navbar({
  activeView,
  onSelectView,
  onOpenDocumentation,
  infoPanelOpen,
}: NavbarProps) {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const { data } = useQuery<NowData | null>(
    nowKey,
    async () => {
      try {
        const response = await loadNow();
        return response.ok ? response.data ?? null : null;
      } catch {
        return null;
      }
    },
    { enabled: true },
  );

  const computedAt = data?.computed_at ?? null;
  const timezone = data?.timezone ?? 'UTC';
  const currentTaskSummary = data ? nowNavContextSummary(data) : 'No active task';
  const titleDateTime = computedAt ? formatNavbarDateTime(computedAt, timezone) : null;
  const nudgeCount = (data?.nudge_bars ?? []).length;
  const unreadThreadCount =
    ((data?.action_items ?? []).filter((item) => item.thread_route !== null).length)
    + (data?.reflow_status?.thread_id ? 1 : 0);
  const syncCount = data?.mesh_summary?.queued_write_count ?? 0;
  const titleNotifications = nudgeCount + unreadThreadCount + syncCount;

  useEffect(() => {
    const pieces = ['Vel'];
    if (titleNotifications > 0) {
      pieces.push(`(${titleNotifications})`);
    }
    if (titleDateTime) {
      pieces.push(`· ${titleDateTime}`);
    }
    if (currentTaskSummary && currentTaskSummary !== 'No active task') {
      pieces.push(`· ${currentTaskSummary}`);
    }
    document.title = pieces.join(' ');
  }, [currentTaskSummary, titleDateTime, titleNotifications]);

  return (
    <header className={NAVBAR_HEADER_CLASSNAME} role="banner">
      <div className="flex min-w-0 flex-wrap items-center justify-between gap-x-4 gap-y-3">
        <div className="flex min-w-0 items-center gap-3 sm:gap-4">
          <NavbarBrand onSelectNow={() => onSelectView('now')} />
          <div className="hidden min-w-0 flex-col justify-center sm:flex">
            <p className="truncate text-[12px] font-medium text-zinc-300">
              {titleDateTime ?? 'Waiting for current context'}
            </p>
            <p className="truncate text-[11px] text-zinc-500">
              {currentTaskSummary}
            </p>
          </div>
        </div>

        <div className="ml-auto flex min-w-0 items-center gap-3 pr-1 sm:gap-5 sm:pr-5">
          <div className="hidden items-center gap-1.5 sm:flex" aria-label="Surface status">
            <StatusTag icon={<WarningIcon size={12} />} count={nudgeCount} tone="warn" label="Nudges" />
            <StatusTag icon={<ThreadsIcon size={12} />} count={unreadThreadCount} tone="thread" label="Threads" />
            <StatusTag icon={<SyncIcon size={12} />} count={syncCount} tone="sync" label="Sync" />
          </div>
          <NavbarNavLinks activeView={activeView} onSelectView={onSelectView} />
          <NavbarInfoButton infoPanelOpen={infoPanelOpen} onOpenDocumentation={onOpenDocumentation} />
        </div>
      </div>
    </header>
  );
}

function StatusTag({
  icon,
  count,
  tone,
  label,
}: {
  icon: ReactNode;
  count: number;
  tone: 'warn' | 'thread' | 'sync';
  label: string;
}) {
  const toneClass =
    tone === 'warn'
      ? 'border-amber-700/40 bg-amber-950/40 text-amber-200'
      : tone === 'thread'
        ? 'border-emerald-700/40 bg-emerald-950/30 text-emerald-200'
        : 'border-sky-800/40 bg-sky-950/30 text-sky-200';

  return (
    <FilterDenseTag className={`${toneClass} !gap-1.5`} aria-label={`${label}: ${count}`}>
      <span aria-hidden className="inline-flex items-center">{icon}</span>
      <span className="text-[10px] font-semibold">{count}</span>
    </FilterDenseTag>
  );
}
