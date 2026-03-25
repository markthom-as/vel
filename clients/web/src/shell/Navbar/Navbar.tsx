import type { MainView } from '../../data/operatorSurfaces';
import { contextQueryKeys, loadNow } from '../../data/context';
import { useMemo } from 'react';
import { useQuery } from '../../data/query';
import { cn } from '../../core/cn';
import { ActionChipButton } from '../../core/FilterToggleTag';
import { CalendarIcon, CheckCircleIcon, SparkIcon, SyncIcon, WarningIcon } from '../../core/Icons';
import { uiFonts } from '../../core/Theme';
import {
  NAVBAR_HEADER_CLASSNAME,
  NAVBAR_INNER_CLASSNAME,
  NAVBAR_MOBILE_BAR_CLASSNAME,
  NAVBAR_MOBILE_BAR_INNER_CLASSNAME,
} from './navbarChrome';
import { NavbarBrand } from './NavbarBrand';
import { NavbarNavLinks } from './NavbarNavLinks';
import { formatNavbarDateTime } from './formatNavbarDateTime';
import { findActiveEvent } from '../../views/now/nowModel';
import type { ViewportSurface } from '../../core/hooks/useViewportSurface';

import type { SystemNavigationTarget } from '../../views/system';

export interface NavbarProps {
  activeView: MainView;
  surface?: ViewportSurface;
  onSelectView: (view: MainView) => void;
  onDeepLink?: (target: { view: MainView; anchor?: string; systemTarget?: SystemNavigationTarget }) => void;
}

export function Navbar({ activeView, surface = 'desktop', onSelectView, onDeepLink }: NavbarProps) {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const { data } = useQuery(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );
  const dateTime = formatNavbarDateTime(data?.computed_at ?? Math.floor(Date.now() / 1000), data?.timezone ?? 'America/Denver');
  const activeEvent = data ? findActiveEvent(data.schedule.upcoming_events, data.computed_at) : null;
  const activeTask = data?.task_lane?.active?.text ?? 'No active task';
  const clientName = data?.mesh_summary?.authority_label ?? 'Client Unknown';
  const location = activeEvent?.location?.trim() || data?.schedule?.upcoming_events?.[0]?.location?.trim() || 'Location Unknown';
  const completedCount = data?.task_lane?.recent_completed?.length ?? 0;
  const taskTotal = completedCount + (data?.task_lane?.pending?.length ?? 0) + (data?.task_lane?.active ? 1 : 0);
  const nudgeCount = data?.nudge_bars?.length ?? 0;
  const syncTone = data?.mesh_summary?.sync_state?.replaceAll('_', ' ') ?? 'unknown';
  const syncBadgeClassName =
    syncTone.includes('error') || syncTone.includes('blocked')
      ? 'border-red-500/28 bg-red-950/25 text-red-300/68 hover:border-red-400/45 hover:text-red-200'
      : syncTone.includes('sync')
        ? 'border-amber-500/28 bg-amber-950/20 text-amber-300/68 hover:border-amber-400/45 hover:text-amber-200'
        : 'border-emerald-500/20 bg-emerald-950/20 text-emerald-200/62 hover:border-emerald-400/38 hover:text-emerald-100/82';

  const isMobileSurface = surface === 'mobile';

  return (
    <>
      <header className={NAVBAR_HEADER_CLASSNAME} role="banner">
        <div className={NAVBAR_INNER_CLASSNAME}>
          <div className="flex min-w-0 items-center gap-3">
            <div className="mr-3 sm:mr-7">
              <NavbarBrand onSelectNow={() => onSelectView('now')} />
            </div>
            <div className="min-w-0">
              <p className={`${uiFonts.display} text-[11px] uppercase tracking-[0.16em] text-[var(--vel-color-accent-soft)] truncate`}>
                <span className={isMobileSurface ? 'hidden' : 'inline'}>{clientName}</span>
                <span className="hidden sm:inline">{clientName} | {location}</span>
              </p>
              <p className={`hidden sm:block truncate text-xs text-[var(--vel-color-muted)] ${uiFonts.mono}`}>{dateTime}</p>
            </div>
          </div>
          {isMobileSurface ? null : (
            <>
              <div className="mx-auto hidden min-w-0 items-center gap-2 lg:flex">
                {activeView !== 'now' ? (
                  <ActionChipButton
                    tone="ghost"
                    onClick={() => onDeepLink?.({ view: 'now', anchor: 'now-next-up' }) ?? onSelectView('now')}
                    aria-label={`Current event ${activeEvent?.title ?? 'No current event'}`}
                    className="max-w-[12rem] !gap-1.5 !px-2 !py-1 !text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]"
                  >
                    <CalendarIcon size={11} className="shrink-0" />
                    <span className="truncate">{activeEvent?.title ?? 'No current event'}</span>
                  </ActionChipButton>
                ) : null}
                <button
                  type="button"
                  onClick={() => onDeepLink?.({ view: 'now', anchor: 'now-completed' }) ?? onSelectView('now')}
                  aria-label={`Completed tasks ${completedCount} of ${taskTotal || completedCount}`}
                  className="inline-flex min-h-[1.15rem] items-center justify-center gap-1 rounded-full border border-emerald-500/18 bg-emerald-950/18 px-1.5 py-[0.18rem] text-emerald-200/50 transition hover:border-emerald-400/35 hover:text-emerald-100/85"
                >
                  <CheckCircleIcon size={11} />
                  <span className="inline-flex items-center justify-center text-[9px] uppercase leading-none tracking-[0.12em] [line-height:1]">
                    {completedCount}/{taskTotal || completedCount}
                  </span>
                </button>
                <button
                  type="button"
                  onClick={() => onDeepLink?.({ view: 'now', anchor: 'nudges-section' }) ?? onSelectView('now')}
                  aria-label={`Nudges ${nudgeCount}`}
                  className="inline-flex min-h-[1.15rem] items-center justify-center gap-1 rounded-full border border-[color:var(--vel-color-accent-border)]/24 bg-[color:var(--vel-color-panel-2)]/25 px-1.5 py-[0.18rem] text-[var(--vel-color-accent-soft)]/54 transition hover:border-[var(--vel-color-accent-border)]/42 hover:text-[var(--vel-color-accent-soft)]"
                >
                  <WarningIcon size={11} />
                  <span className="inline-flex items-center justify-center text-[9px] uppercase leading-none tracking-[0.12em] [line-height:1]">{nudgeCount}</span>
                </button>
                <button
                  type="button"
                  onClick={() => onDeepLink?.({ view: 'system', systemTarget: { section: 'overview', subsection: 'trust' }, anchor: 'trust' }) ?? onSelectView('system')}
                  aria-label={`Sync status ${syncTone}`}
                  className={cn('inline-flex min-h-[1.15rem] items-center justify-center rounded-full border px-1.5 py-[0.18rem] transition', syncBadgeClassName)}
                >
                  <SyncIcon size={11} className={syncTone.includes('sync') ? 'animate-spin' : ''} />
                </button>
                {activeView !== 'now' ? (
                  <ActionChipButton
                    tone="ghost"
                    onClick={() => onDeepLink?.({ view: 'now', anchor: 'now-active' }) ?? onSelectView('now')}
                    aria-label={`Active task ${activeTask}`}
                    className="max-w-[12rem] !gap-1.5 !px-2 !py-1 !text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]"
                  >
                    <SparkIcon size={11} className="shrink-0 text-[var(--vel-color-accent-soft)]" />
                    <span className="truncate">{activeTask}</span>
                  </ActionChipButton>
                ) : null}
              </div>
              <div className="flex min-w-0 items-center gap-3">
                <NavbarNavLinks activeView={activeView} onSelectView={onSelectView} onDeepLink={onDeepLink} surface={surface} />
              </div>
            </>
          )}
        </div>
      </header>
      {isMobileSurface ? (
        <div className={NAVBAR_MOBILE_BAR_CLASSNAME}>
          <div className={NAVBAR_MOBILE_BAR_INNER_CLASSNAME}>
            <NavbarNavLinks activeView={activeView} onSelectView={onSelectView} onDeepLink={onDeepLink} surface={surface} />
          </div>
        </div>
      ) : null}
    </>
  );
}
