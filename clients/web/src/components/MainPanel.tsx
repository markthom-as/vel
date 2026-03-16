import { InboxView } from './InboxView';
import { ThreadView } from './ThreadView';

interface MainPanelProps {
  conversationId: string | null;
  showInbox: boolean;
}

export function MainPanel({ conversationId, showInbox }: MainPanelProps) {
  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {conversationId && !showInbox && (
        <header className="shrink-0 border-b border-zinc-800 px-4 py-2 text-sm text-zinc-400">
          Thread
        </header>
      )}
      {showInbox ? <InboxView /> : <ThreadView conversationId={conversationId} />}
    </div>
  );
}
