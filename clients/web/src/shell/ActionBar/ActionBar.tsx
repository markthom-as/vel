import type { ReactNode } from 'react';
import type { MainView } from '../../data/operatorSurfaces';
import { cn } from '../../core/cn';
import { MicIcon, SearchIcon, SettingsIcon, SparkIcon, ThreadsIcon } from '../../core/Icons';
import { shellChrome, uiFonts } from '../../core/Theme';

interface ActionBarProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
}

interface ActionItem {
  key: string;
  label: string;
  icon: ReactNode;
  onClick: () => void;
  active?: boolean;
}

function focusComposerInput() {
  document.querySelector<HTMLTextAreaElement>('[data-vel-composer-input="true"]')?.focus();
}

function focusVoiceTrigger() {
  document.querySelector<HTMLButtonElement>('[data-vel-voice-trigger="true"]')?.focus();
}

export function ActionBar({ activeView, onSelectView }: ActionBarProps) {
  const contextualSlot = contextualAction(activeView, onSelectView);
  const items: ActionItem[] = [
    {
      key: 'voice',
      label: 'Voice',
      icon: <MicIcon size={15} strokeWidth={2} />,
      onClick: focusVoiceTrigger,
    },
    {
      key: 'capture',
      label: 'Capture',
      icon: <SparkIcon size={15} strokeWidth={2} />,
      onClick: focusComposerInput,
      active: activeView === 'now',
    },
    {
      key: 'ask',
      label: 'Ask',
      icon: <SearchIcon size={15} strokeWidth={2} />,
      onClick: focusComposerInput,
    },
    {
      key: 'command',
      label: 'Command',
      icon: <SettingsIcon size={15} strokeWidth={2} />,
      onClick: () => onSelectView('system'),
      active: activeView === 'system',
    },
    contextualSlot,
  ];

  return (
    <nav aria-label="Quick actions" className={shellChrome.actionBarDock}>
      <div className={shellChrome.actionBarInner}>
        {items.map((item) => (
          <button
            key={item.key}
            type="button"
            onClick={item.onClick}
            className={cn(
              `${uiFonts.display} inline-flex min-w-0 flex-1 items-center justify-center gap-2 rounded-full border px-3 py-2 text-[11px] leading-none tracking-[0.08em] text-[var(--vel-color-muted)] transition sm:flex-initial sm:px-4`,
              item.active
                ? 'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] text-[var(--vel-color-accent-soft)]'
                : 'border-transparent bg-white/3 hover:border-[var(--vel-color-border)] hover:text-[var(--vel-color-text)]',
            )}
          >
            <span aria-hidden className="inline-flex shrink-0 items-center justify-center">
              {item.icon}
            </span>
            <span className="truncate">{item.label}</span>
          </button>
        ))}
      </div>
    </nav>
  );
}

function contextualAction(activeView: MainView, onSelectView: (view: MainView) => void): ActionItem {
  if (activeView === 'now') {
    return {
      key: 'quick-threads',
      label: 'Threads',
      icon: <ThreadsIcon size={15} strokeWidth={2} />,
      onClick: () => onSelectView('threads'),
    };
  }

  if (activeView === 'threads') {
    return {
      key: 'quick-now',
      label: 'Now',
      icon: <SparkIcon size={15} strokeWidth={2} />,
      onClick: () => onSelectView('now'),
    };
  }

  return {
    key: 'quick-threads',
    label: 'Threads',
    icon: <ThreadsIcon size={15} strokeWidth={2} />,
    onClick: () => onSelectView('threads'),
  };
}
