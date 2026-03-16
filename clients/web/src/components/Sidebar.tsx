import { ConversationList } from './ConversationList';

interface SidebarProps {
  selectedConversationId: string | null;
  onSelectConversation: (id: string) => void;
  onOpenSettings?: () => void;
}

export function Sidebar({ selectedConversationId, onSelectConversation, onOpenSettings }: SidebarProps) {
  return (
    <>
      <div className="p-3 border-b border-zinc-800">
        <h1 className="font-semibold text-zinc-100">Vel</h1>
        <p className="text-xs text-zinc-500">Conversations</p>
        <button
          type="button"
          onClick={onOpenSettings}
          className="mt-2 text-xs text-zinc-500 hover:text-zinc-300"
        >
          Settings
        </button>
      </div>
      <ConversationList
        selectedId={selectedConversationId}
        onSelect={onSelectConversation}
      />
    </>
  );
}
