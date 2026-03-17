import { InboxView } from './InboxView';
import { NowView } from './NowView';
import { SuggestionsView } from './SuggestionsView';
import { ThreadView } from './ThreadView';

type MainView = 'now' | 'inbox' | 'suggestions' | 'threads';

interface MainPanelProps {
  conversationId: string | null;
  mainView: MainView;
}

export function MainPanel({ conversationId, mainView }: MainPanelProps) {
  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {mainView === 'now' ? (
        <NowView />
      ) : mainView === 'inbox' ? (
        <InboxView />
      ) : mainView === 'suggestions' ? (
        <SuggestionsView />
      ) : (
        <ThreadView conversationId={conversationId} />
      )}
    </div>
  );
}
