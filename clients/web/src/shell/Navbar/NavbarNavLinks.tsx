import { primarySurfaces, supportSurfaces, type MainView } from '../../data/operatorSurfaces';
import { ActionChipButton } from '../../core/FilterToggleTag';
import { cn } from '../../core/cn';
import {
  InfoCircleIcon,
  SettingsIcon,
  SparkIcon,
  ThreadsIcon,
} from '../../core/Icons';
import { uiTheme } from '../../core/Theme';
import type { SystemNavigationTarget } from '../../views/system';

const ACCENT = uiTheme.brandText;

interface NavbarNavLinksProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
  onDeepLink?: (target: { view: MainView; anchor?: string; systemTarget?: SystemNavigationTarget }) => void;
}

export function NavbarNavLinks({ activeView, onSelectView, onDeepLink }: NavbarNavLinksProps) {
  return (
    <nav
      className="flex min-w-0 items-center gap-x-2 sm:gap-x-3"
      aria-label="Primary"
    >
      {[...primarySurfaces, ...supportSurfaces].map((item) => (
        <button
          key={item.view}
          type="button"
          onClick={() => onSelectView(item.view)}
          className={`group inline-flex shrink-0 items-center gap-2 whitespace-nowrap rounded-full border px-2 py-2 sm:px-3 text-xs font-medium normal-case tracking-normal transition ${
            activeView === item.view
              ? `border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] ${ACCENT}`
              : 'border-[var(--vel-color-border)] text-[var(--vel-color-muted)] hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]'
          }`}
        >
          <span
            aria-hidden="true"
            className={cn(
              'inline-flex shrink-0',
              activeView === item.view ? '' : 'opacity-90 text-[var(--vel-color-dim)] group-hover:text-[var(--vel-color-muted)]',
            )}
          >
            {surfaceIcon(item.view)}
          </span>
          <span className="hidden sm:inline leading-none">{item.label}</span>
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
