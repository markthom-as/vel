import { primarySurfaces, supportSurfaces, type MainView } from '../../data/operatorSurfaces';
import {
  InboxIcon,
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
      className="flex min-w-0 items-center gap-x-4 sm:gap-x-5"
      aria-label="Primary"
    >
      {[...primarySurfaces, ...supportSurfaces].map((item) => (
        <button
          key={item.view}
          type="button"
          onClick={() => onSelectView(item.view)}
          className={`group inline-flex shrink-0 items-center gap-1 whitespace-nowrap rounded-md px-0 py-0.5 text-[8px] font-normal uppercase tracking-[0.18em] transition ${
            activeView === item.view ? ACCENT : 'text-zinc-500 hover:text-zinc-200'
          }`}
        >
          <span
            aria-hidden="true"
            className={`opacity-80 ${
              activeView === item.view ? 'text-current' : 'text-zinc-500 group-hover:text-zinc-300'
            }`}
          >
            {surfaceIcon(item.view)}
          </span>
          <span>{item.label}</span>
        </button>
      ))}
    </nav>
  );
}

function surfaceIcon(view: MainView) {
  switch (view) {
    case 'now':
      return <SparkIcon size={11} strokeWidth={1.75} />;
    case 'inbox':
      return <InboxIcon size={11} strokeWidth={1.75} />;
    case 'threads':
      return <ThreadsIcon size={11} strokeWidth={1.75} />;
    case 'settings':
      return <SettingsIcon size={11} strokeWidth={1.75} />;
    default:
      return <SparkIcon size={11} strokeWidth={1.75} />;
  }
}
