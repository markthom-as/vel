import { useEffect, useMemo } from 'react';
import { contextQueryKeys, loadNow } from '../../data/context';
import { useQuery } from '../../data/query';
import type { MainView } from '../../data/operatorSurfaces';
import type { NowData } from '../../types';
import { formatNavbarDateTime } from './formatNavbarDateTime';
import { NAVBAR_HEADER_CLASSNAME } from './navbarChrome';
import { NavbarBrand } from './NavbarBrand';
import { NavbarInfoButton } from './NavbarInfoButton';
import { NavbarNavLinks } from './NavbarNavLinks';
import { NavbarStatus } from './NavbarStatus';

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
  const currentTaskSummary = data?.task_lane?.active?.text
    ?? data?.context_line?.text
    ?? 'No active task';
  const titleDateTime = computedAt ? formatNavbarDateTime(computedAt, timezone) : null;
  const titleNotifications = ((data?.nudge_bars ?? []).length)
    + (((data?.action_items ?? []).filter((item) => item.thread_route !== null).length) + (data?.reflow_status?.thread_id ? 1 : 0))
    + (data?.mesh_summary?.queued_write_count ?? 0);

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
      <div className="flex min-w-0 items-center justify-between gap-4 whitespace-nowrap">
        <div className="min-w-0 shrink-0">
          <div className="flex min-w-0 items-end gap-3">
            <NavbarBrand onSelectNow={() => onSelectView('now')} />
            <NavbarStatus
              dateTimeLabel={titleDateTime ?? 'No clock context'}
              contextLine={currentTaskSummary}
            />
          </div>
        </div>

        <div className="ml-auto flex min-w-0 items-center gap-5 pr-1 sm:gap-6 sm:pr-5">
          <NavbarNavLinks activeView={activeView} onSelectView={onSelectView} />
          <NavbarInfoButton infoPanelOpen={infoPanelOpen} onOpenDocumentation={onOpenDocumentation} />
        </div>
      </div>
    </header>
  );
}
