import { InboxView } from './InboxView';
import { NowView } from './NowView';
import { ThreadView } from './ThreadView';

interface MainPanelProps {
  conversationId: string | null;
  showInbox: boolean;
  showNow: boolean;
}

export function MainPanel({ conversationId, showInbox, showNow }: MainPanelProps) {
  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {conversationId && !showInbox && !showNow && (
        <header className="shrink-0 border-b border-zinc-800 px-4 py-2 text-sm text-zinc-400">
          Thread
        </header>
      )}
      {showNow ? <NowView /> : showInbox ? <InboxView /> : <ThreadView conversationId={conversationId} />}
    </div>
  );
}
