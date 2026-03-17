import { ConversationList } from './ConversationList';

export type MainView =
  | 'now'
  | 'inbox'
  | 'threads'
  | 'suggestions'
  | 'projects'
  | 'stats'
  | 'settings';

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
  const navigationItems: Array<{ view: MainView; label: string }> = [
    { view: 'now', label: 'Now' },
    { view: 'inbox', label: 'Inbox' },
    { view: 'threads', label: 'Threads' },
    { view: 'suggestions', label: 'Suggestions' },
    { view: 'projects', label: 'Projects' },
    { view: 'stats', label: 'Stats' },
    { view: 'settings', label: 'Settings' },
  ];

  return (
    <>
      <div className="p-4 border-b border-zinc-800">
        <h1 className="font-semibold text-zinc-100">Vel</h1>
        <p className="text-xs text-zinc-500">Operator surface</p>
      </div>
      <nav className="shrink-0 border-b border-zinc-800 p-2">
        <ul className="space-y-1">
          {navigationItems.map((item) => (
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
                {item.label}
              </button>
            </li>
          ))}
        </ul>
      </nav>
      <div className="p-4 border-b border-zinc-800">
        <p className="text-xs uppercase tracking-[0.16em] text-zinc-500">Thread actions</p>
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
      {activeView === 'threads' ? (
        <ConversationList
          selectedId={selectedConversationId}
          onSelect={onSelectConversation}
        />
      ) : (
        <div className="p-4 text-xs text-zinc-500">
          Conversation history is scoped to the Threads surface.
        </div>
      )}
    </>
  );
}
