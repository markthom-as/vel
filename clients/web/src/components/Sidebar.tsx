import { ConversationList } from './ConversationList';

interface SidebarProps {
  selectedConversationId: string | null;
  onSelectConversation: (id: string) => void;
  onNewConversation?: () => void | Promise<void>;
  onOpenSettings?: () => void;
}

export function Sidebar({
  selectedConversationId,
  onSelectConversation,
  onNewConversation,
  onOpenSettings,
}: SidebarProps) {
  return (
    <>
      <div className="p-4 border-b border-zinc-800">
        <h1 className="font-semibold text-zinc-100">Vel</h1>
        <p className="text-xs text-zinc-500">Operator threads</p>
        <div className="mt-2 flex flex-wrap items-center gap-2">
          {onNewConversation && (
            <>
              <button
                type="button"
                onClick={onNewConversation}
                className="text-xs text-zinc-500 hover:text-zinc-300"
              >
                New conversation
              </button>
              {onOpenSettings ? <span className="text-zinc-600">·</span> : null}
            </>
          )}
          {onOpenSettings ? (
            <button
              type="button"
              onClick={onOpenSettings}
              className="text-xs text-zinc-500 hover:text-zinc-300"
            >
              Settings
            </button>
          ) : null}
        </div>
      </div>
      <ConversationList
        selectedId={selectedConversationId}
        onSelect={onSelectConversation}
      />
    </>
  );
}
