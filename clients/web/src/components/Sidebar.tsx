import { useEffect, useState } from 'react';
import { ConversationList } from './ConversationList';
import {
  primarySurfaces,
  supportSurfaces,
  type MainView,
} from '../data/operatorSurfaces';

interface SidebarProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
  selectedConversationId: string | null;
  onSelectConversation: (id: string) => void;
  onNewConversation?: () => void | Promise<void>;
}

export function Sidebar({
  activeView,
  onSelectView,
  selectedConversationId,
  onSelectConversation,
  onNewConversation,
}: SidebarProps) {
  const [showThreadHistory, setShowThreadHistory] = useState(activeView === 'threads');

  useEffect(() => {
    if (activeView === 'threads') {
      setShowThreadHistory(true);
    }
  }, [activeView]);

  return (
    <div className="flex h-full flex-col">
      <div className="flex items-center justify-center border-b border-zinc-800 px-2 py-4">
        <div className="flex h-12 w-12 items-center justify-center rounded-2xl border border-zinc-800 bg-zinc-900 text-sm font-semibold text-zinc-100">
          Vel
        </div>
      </div>
      <nav className="shrink-0 border-b border-zinc-800 px-2 py-3">
        <p className="pb-2 text-center text-[10px] uppercase tracking-[0.22em] text-zinc-500">Daily Use</p>
        <ul className="space-y-2">
          {primarySurfaces.map((item) => (
            <li key={item.view}>
              <button
                type="button"
                onClick={() => onSelectView(item.view)}
                aria-label={item.label}
                title={item.label}
                className={`flex w-full flex-col items-center gap-1 rounded-xl px-2 py-2 text-center text-[11px] transition ${
                  activeView === item.view
                    ? 'bg-zinc-800 text-zinc-100'
                    : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200'
                }`}
              >
                <span aria-hidden="true" className="text-sm text-zinc-500">
                  {item.icon}
                </span>
                <span>{item.label}</span>
              </button>
            </li>
          ))}
        </ul>
        <p className="pb-2 pt-4 text-center text-[10px] uppercase tracking-[0.22em] text-zinc-500">Support</p>
        <ul className="space-y-2">
          {supportSurfaces.map((item) => (
            <li key={item.view}>
              <button
                type="button"
                onClick={() => onSelectView(item.view)}
                aria-label={item.label}
                title={item.label}
                className={`flex w-full flex-col items-center gap-1 rounded-xl px-2 py-2 text-center text-[11px] transition ${
                  activeView === item.view
                    ? 'bg-zinc-800 text-zinc-100'
                    : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200'
                }`}
              >
                <span aria-hidden="true" className="text-sm text-zinc-500">
                  {item.icon}
                </span>
                <span>{item.label}</span>
              </button>
            </li>
          ))}
        </ul>
      </nav>
      <div className="mt-auto border-t border-zinc-800 px-2 py-3">
        {activeView === 'threads' ? (
          <div className="space-y-2">
            <p className="text-center text-[10px] uppercase tracking-[0.22em] text-zinc-500">Threads</p>
            <button
              type="button"
              onClick={() => setShowThreadHistory((current) => !current)}
              className="flex w-full flex-col items-center gap-1 rounded-xl px-2 py-2 text-center text-[11px] text-zinc-400 transition hover:bg-zinc-900 hover:text-zinc-200"
            >
              <span aria-hidden="true" className="text-sm text-zinc-500">
                {showThreadHistory ? '◍' : '○'}
              </span>
              <span>{showThreadHistory ? 'Hide history' : 'Show history'}</span>
            </button>
            {onNewConversation ? (
              <button
                type="button"
                onClick={onNewConversation}
                className="flex w-full flex-col items-center gap-1 rounded-xl px-2 py-2 text-center text-[11px] text-zinc-400 transition hover:bg-zinc-900 hover:text-zinc-200"
              >
                <span aria-hidden="true" className="text-sm text-zinc-500">＋</span>
                <span>New thread</span>
              </button>
            ) : null}
          </div>
        ) : (
          <div className="px-1 text-center text-[11px] leading-5 text-zinc-500">
            Threads stay contextual.
          </div>
        )}
      </div>
      {activeView === 'threads' && showThreadHistory ? (
        <ConversationList selectedId={selectedConversationId} onSelect={onSelectConversation} />
      ) : (
        <div className="px-3 py-4 text-center text-[11px] leading-5 text-zinc-500">
          {activeView === 'threads'
            ? 'History can stay collapsed while you focus on the active thread.'
            : 'Conversation history is scoped to Threads.'}
        </div>
      )}
    </div>
  );
}
