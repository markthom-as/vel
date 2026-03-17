import { useState } from 'react';
import { apiPost } from './api/client';
import { decodeApiResponse, decodeConversationData, type ApiResponse, type ConversationData } from './types';
import { AppShell } from './components/AppShell';
import { ContextPanel } from './components/ContextPanel';
import { MainPanel } from './components/MainPanel';
import { SettingsPage } from './components/SettingsPage';
import { Sidebar } from './components/Sidebar';
import { invalidateQuery } from './data/query';
import { queryKeys } from './data/resources';

type MainView = 'now' | 'inbox' | 'threads';

function App() {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [mainView, setMainView] = useState<MainView>('now');
  const [showSettings, setShowSettings] = useState(false);
  const showInbox = mainView === 'inbox';
  const showNow = mainView === 'now';

  async function startNewConversation() {
    const res = await apiPost<ApiResponse<ConversationData>>('/api/conversations', {
      title: 'New conversation',
      kind: 'general',
    }, (value) => decodeApiResponse(value, decodeConversationData));
    if (res.ok && res.data) {
      setSelectedConversationId(res.data.id);
      setMainView('threads');
      invalidateQuery(queryKeys.conversations(), { refetch: true });
    }
  }

  if (showSettings) {
    return (
      <div className="h-screen bg-zinc-950 text-zinc-100">
        <SettingsPage onBack={() => setShowSettings(false)} />
      </div>
    );
  }

  return (
    <AppShell
      sidebar={
        <>
          <nav className="shrink-0 flex border-b border-zinc-800">
            <button
              type="button"
              onClick={() => setMainView('now')}
              className={`flex-1 px-3 py-2 text-sm ${showNow ? 'bg-zinc-800 text-white' : 'text-zinc-500 hover:text-zinc-300'}`}
            >
              Now
            </button>
            <button
              type="button"
              onClick={() => setMainView('inbox')}
              className={`flex-1 px-3 py-2 text-sm ${showInbox ? 'bg-zinc-800 text-white' : 'text-zinc-500 hover:text-zinc-300'}`}
            >
              Inbox
            </button>
            <button
              type="button"
              onClick={() => setMainView('threads')}
              className={`flex-1 px-3 py-2 text-sm ${!showInbox && !showNow ? 'bg-zinc-800 text-white' : 'text-zinc-500 hover:text-zinc-300'}`}
            >
              Threads
            </button>
          </nav>
          <Sidebar
            selectedConversationId={selectedConversationId}
            onSelectConversation={(id) => {
              setSelectedConversationId(id);
              setMainView('threads');
            }}
            onNewConversation={startNewConversation}
            onOpenNow={() => setMainView('now')}
            onOpenSettings={() => setShowSettings(true)}
          />
        </>
      }
      main={<MainPanel conversationId={selectedConversationId} showInbox={showInbox} showNow={showNow} />}
      contextPanel={<ContextPanel />}
    />
  );
}

export default App;
