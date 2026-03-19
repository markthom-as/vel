import { useEffect, useState } from 'react';
import { ConversationList } from './ConversationList';
import {
  advancedSurfaces,
  primarySurfaces,
  secondarySurfaces,
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
    <>
      <div className="p-4 border-b border-zinc-800">
        <h1 className="font-semibold text-zinc-100">Vel</h1>
        <p className="text-xs text-zinc-500">Operator surface</p>
      </div>
      <nav className="shrink-0 border-b border-zinc-800 p-2">
        <p className="px-3 pb-2 text-[11px] uppercase tracking-[0.18em] text-zinc-500">Daily Use</p>
        <ul className="space-y-1">
          {primarySurfaces.map((item) => (
            <li key={item.view}>
              <button
                type="button"
                onClick={() => onSelectView(item.view)}
                className={`w-full rounded-md px-3 py-2 text-left text-sm transition ${
                  activeView === item.view
                    ? 'bg-zinc-800 text-zinc-100'
                    : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200'
                }`}
              >
                <span className="flex items-center gap-3">
                  <span aria-hidden="true" className="text-xs text-zinc-500">
                    {item.icon}
                  </span>
                  <span className="min-w-0">
                    <span className="block">{item.label}</span>
                    <span className={`block text-xs ${activeView === item.view ? 'text-zinc-300' : 'text-zinc-500'}`}>
                      {item.blurb}
                    </span>
                  </span>
                </span>
              </button>
            </li>
          ))}
        </ul>
        <p className="px-3 pb-2 pt-4 text-[11px] uppercase tracking-[0.18em] text-zinc-500">Support</p>
        <ul className="space-y-1">
          {secondarySurfaces.map((item) => (
            <li key={item.view}>
              <button
                type="button"
                onClick={() => onSelectView(item.view)}
                className={`w-full rounded-md px-3 py-2 text-left text-sm transition ${
                  activeView === item.view
                    ? 'bg-zinc-800 text-zinc-100'
                    : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200'
                }`}
              >
                <span className="flex items-center gap-3">
                  <span aria-hidden="true" className="text-xs text-zinc-500">
                    {item.icon}
                  </span>
                  <span className="min-w-0">
                    <span className="block">{item.label}</span>
                    <span className={`block text-xs ${activeView === item.view ? 'text-zinc-300' : 'text-zinc-500'}`}>
                      {item.blurb}
                    </span>
                  </span>
                </span>
              </button>
            </li>
          ))}
        </ul>
        <p className="px-3 pb-2 pt-4 text-[11px] uppercase tracking-[0.18em] text-zinc-500">Advanced</p>
        <ul className="space-y-1">
          {advancedSurfaces.map((item) => (
            <li key={item.view}>
              <button
                type="button"
                onClick={() => onSelectView(item.view)}
                className={`w-full rounded-md px-3 py-2 text-left text-sm transition ${
                  activeView === item.view
                    ? 'bg-zinc-800 text-zinc-100'
                    : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200'
                }`}
              >
                <span className="flex items-center gap-3">
                  <span aria-hidden="true" className="text-xs text-zinc-500">
                    {item.icon}
                  </span>
                  <span className="min-w-0">
                    <span className="block">{item.label}</span>
                    <span className={`block text-xs ${activeView === item.view ? 'text-zinc-300' : 'text-zinc-500'}`}>
                      {item.blurb}
                    </span>
                  </span>
                </span>
              </button>
            </li>
          ))}
        </ul>
      </nav>
      <div className="p-4 border-b border-zinc-800">
        <div className="flex items-center justify-between gap-3">
          <p className="text-xs uppercase tracking-[0.16em] text-zinc-500">Thread actions</p>
          <button
            type="button"
            onClick={() => setShowThreadHistory((current) => !current)}
            className="text-xs text-zinc-500 hover:text-zinc-300"
          >
            {showThreadHistory ? 'Hide history' : 'Show history'}
          </button>
        </div>
        <div className="mt-2 flex flex-wrap items-center gap-2">
          {onNewConversation && (
            <button
              type="button"
              onClick={onNewConversation}
              className="text-xs text-zinc-500 hover:text-zinc-300"
            >
              New conversation
            </button>
          )}
          {activeView !== 'threads' ? (
            <button
              type="button"
              onClick={() => onSelectView('threads')}
              className="text-xs text-zinc-500 hover:text-zinc-300"
            >
              Open threads
            </button>
          ) : null}
        </div>
      </div>
      {activeView === 'threads' && showThreadHistory ? (
        <ConversationList
          selectedId={selectedConversationId}
          onSelect={onSelectConversation}
        />
      ) : (
        <div className="p-4 text-xs text-zinc-500">
          {activeView === 'threads'
            ? 'Thread history is available but can stay collapsed while you focus on the active conversation.'
            : 'Conversation history is scoped to the Threads surface.'}
        </div>
      )}
    </>
  );
}
