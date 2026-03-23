import { primarySurfaces, supportSurfaces, type MainView } from '../../data/operatorSurfaces';
import { cn } from '../../core/cn';
import {
  SettingsIcon,
  SparkIcon,
  ThreadsIcon,
} from '../../core/Icons';
import { uiTheme } from '../../core/Theme';

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
              ? `border-[#ff6b00]/45 bg-[#2d1608]/85 ${ACCENT}`
              : 'border-zinc-800/90 text-zinc-400 hover:border-zinc-700 hover:text-zinc-100'
          }`}
        >
          <span
            aria-hidden="true"
            className={cn(
              'inline-flex shrink-0',
              activeView === item.view ? '' : 'opacity-90 text-zinc-500 group-hover:text-zinc-300',
            )}
          >
            {surfaceIcon(item.view)}
          </span>
          <span className="leading-none">{item.label}</span>
        </button>
      ))}
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
