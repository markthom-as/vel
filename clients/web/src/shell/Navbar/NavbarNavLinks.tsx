import { primarySurfaces, supportSurfaces, type MainView } from '../../data/operatorSurfaces';
import { cn } from '../../core/cn';
import {
  InfoCircleIcon,
  SettingsIcon,
  SparkIcon,
  ThreadsIcon,
} from '../../core/Icons';
import { uiTheme } from '../../core/Theme';
import systemDocUrl from '../../../../../docs/user/system.md?url';

const ACCENT = uiTheme.brandText;

interface NavbarNavLinksProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
}

export function NavbarNavLinks({ activeView, onSelectView }: NavbarNavLinksProps) {
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
          className={`group inline-flex shrink-0 items-center gap-2 whitespace-nowrap rounded-full border px-3 py-2 text-xs font-medium normal-case tracking-normal transition ${
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
          <span className="leading-none">{item.label}</span>
        </button>
      ))}
      <a
        href={systemDocUrl}
        aria-label="System documentation"
        title="Open system documentation"
        target="_blank"
        rel="noreferrer"
        className="inline-flex shrink-0 items-center justify-center pl-1 text-[var(--vel-color-dim)] transition hover:text-[var(--vel-color-text)]"
      >
        <InfoCircleIcon size={16} />
      </a>
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
