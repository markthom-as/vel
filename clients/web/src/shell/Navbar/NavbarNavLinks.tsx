import { primarySurfaces, type MainView } from '../../data/operatorSurfaces';
import { ActionChipButton } from '../../core/FilterToggleTag';
import { cn } from '../../core/cn';
import {
  InfoCircleIcon,
  SettingsIcon,
  SparkIcon,
  WarningIcon,
  ThreadsIcon,
} from '../../core/Icons';
import { uiTheme } from '../../core/Theme';
import type { SystemNavigationTarget } from '../../views/system';
import type { ViewportSurface } from '../../core/hooks/useViewportSurface';

const ACCENT = uiTheme.brandText;

interface NavbarNavLinksProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
  onDeepLink?: (target: { view: MainView; anchor?: string; systemTarget?: SystemNavigationTarget }) => void;
  surface?: ViewportSurface;
}

export function NavbarNavLinks({ activeView, onSelectView, onDeepLink, surface = 'desktop' }: NavbarNavLinksProps) {
  const isMobile = surface === 'mobile';

  const primaryItems = [...primarySurfaces].map((item) => ({
    key: item.view,
    label: item.label,
    icon: surfaceIcon(item.view),
    isActive: activeView === item.view,
    onActivate: () => onSelectView(item.view),
  }));

  const nudgeItem = {
    key: 'nudges',
    label: 'Nudges',
    icon: <WarningIcon size={15} strokeWidth={1.85} />,
    isActive: false,
    onActivate: () => onDeepLink?.({ view: 'now', anchor: 'nudges-section' }) ?? onSelectView('now'),
  };

  const navItems = isMobile ? [...primaryItems, nudgeItem] : primaryItems;

  return (
    <nav
      className={cn(
        'flex min-w-0 items-center',
        isMobile ? 'w-full justify-around gap-x-1' : 'items-center gap-x-2 sm:gap-x-3',
      )}
      aria-label="Primary"
      role="tablist"
    >
      {navItems.map((item) => (
        <button
          key={item.key}
          type="button"
          onClick={item.onActivate}
          role="tab"
          aria-selected={item.isActive}
          aria-label={item.label}
          className={cn(
            'group inline-flex shrink-0 items-center gap-2 whitespace-nowrap rounded-full border transition',
            isMobile
              ? 'min-h-[2.85rem] w-full flex-1 flex-col justify-center px-2 py-2 text-[8px] font-medium uppercase tracking-[0.06em]'
              : 'rounded-full border px-2 py-2 sm:px-3 text-xs font-medium normal-case tracking-normal',
            item.isActive
              ? `border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] ${ACCENT}`
              : 'border-[var(--vel-color-border)] text-[var(--vel-color-muted)] hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]',
          )}
        >
          <span
            aria-hidden="true"
            className={cn(
              'inline-flex shrink-0',
              item.isActive ? '' : 'opacity-90 text-[var(--vel-color-dim)] group-hover:text-[var(--vel-color-muted)]',
            )}
          >
            {item.icon}
          </span>
          <span className={cn(isMobile ? 'leading-tight' : 'hidden sm:inline leading-none')}>{item.label}</span>
        </button>
      ))}
      <ActionChipButton
        tone="ghost"
        iconOnly
        onClick={() =>
          onDeepLink?.({
            view: 'system',
            systemTarget: { section: 'overview', subsection: 'trust' },
            anchor: 'system-docs',
          }) ?? onSelectView('system')
        }
        aria-label="System documentation"
        title="Open system documentation"
        className="ml-1 text-[var(--vel-color-dim)] hover:text-[var(--vel-color-text)]"
      >
        <InfoCircleIcon size={16} />
        {isMobile ? null : <span className="sr-only">System documentation</span>}
      </ActionChipButton>
    </nav>
  );
}

function surfaceIcon(view: MainView) {
  const common = {
    size: 15,
    strokeWidth: 1.85,
  };
  switch (view) {
    case 'now':
      return <SparkIcon {...common} />;
    case 'threads':
      return <ThreadsIcon {...common} />;
    case 'system':
      return <SettingsIcon {...common} />;
    default:
      return <SparkIcon {...common} />;
  }
}
