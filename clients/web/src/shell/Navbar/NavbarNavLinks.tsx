import { useId } from 'react';
import { primarySurfaces, supportSurfaces, type MainView } from '../../data/operatorSurfaces';
import { cn } from '../../core/cn';
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
  const gradientId = `vel-nav-shimmer-${useId().replace(/:/g, '')}`;

  return (
    <nav
      className="flex min-w-0 items-center gap-x-4 sm:gap-x-5"
      aria-label="Primary"
    >
      <svg
        aria-hidden
        className="pointer-events-none fixed h-0 w-0 overflow-hidden"
        focusable="false"
      >
        <defs>
          <linearGradient
            id={gradientId}
            gradientUnits="objectBoundingBox"
            x1={0}
            y1={0}
            x2={1}
            y2={0}
            gradientTransform="rotate(128 0.5 0.5)"
          >
            <stop offset="0%" stopColor="rgb(194, 65, 12)" stopOpacity="var(--vel-brand-shimmer-opacity)" />
            <stop offset="38%" stopColor="rgb(255, 201, 154)" stopOpacity="var(--vel-brand-shimmer-opacity)" />
            <stop offset="58%" stopColor="rgb(255, 107, 0)" stopOpacity="var(--vel-brand-shimmer-opacity)" />
            <stop offset="100%" stopColor="rgb(254, 215, 170)" stopOpacity="var(--vel-brand-shimmer-opacity)" />
          </linearGradient>
        </defs>
      </svg>
      {[...primarySurfaces, ...supportSurfaces].map((item) => (
        <button
          key={item.view}
          type="button"
          onClick={() => onSelectView(item.view)}
          className={`group inline-flex shrink-0 items-center gap-1.5 whitespace-nowrap rounded-md px-0 py-0.5 text-[8px] font-normal normal-case tracking-normal transition ${
            activeView === item.view ? ACCENT : 'text-zinc-500 hover:text-zinc-200'
          }`}
        >
          <span
            aria-hidden="true"
            className={cn(
              'inline-flex shrink-0',
              activeView === item.view ? '' : 'opacity-90 text-zinc-500 group-hover:text-zinc-300',
            )}
          >
            {surfaceIcon(item.view, activeView === item.view, gradientId)}
          </span>
          <span>{item.label}</span>
        </button>
      ))}
    </nav>
  );
}

function surfaceIcon(view: MainView, active: boolean, gradientId: string) {
  const common = {
    size: 15,
    strokeWidth: 1.85,
    ...(active ? { stroke: `url(#${gradientId})` as const } : {}),
  };
  switch (view) {
    case 'now':
      return <SparkIcon {...common} />;
    case 'inbox':
      return <InboxIcon {...common} />;
    case 'threads':
      return <ThreadsIcon {...common} />;
    case 'settings':
      return <SettingsIcon {...common} />;
    default:
      return <SparkIcon {...common} />;
  }
}
