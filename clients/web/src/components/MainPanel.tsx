import { InboxView } from './InboxView';
import { NowView } from './NowView';
import type { SettingsTab } from './SettingsPage';
import { SuggestionsView } from './SuggestionsView';
import { ThreadView } from './ThreadView';

type MainView = 'now' | 'inbox' | 'suggestions' | 'threads';

interface MainPanelProps {
  conversationId: string | null;
  mainView: MainView;
  onOpenSettings: (target?: {
    tab: SettingsTab;
    integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
  }) => void;
}

export function MainPanel({ conversationId, mainView, onOpenSettings }: MainPanelProps) {
  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {mainView === 'now' ? (
        <NowView onOpenSettings={onOpenSettings} />
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
