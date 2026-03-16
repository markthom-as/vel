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
import './App.css';

function App() {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [showInbox, setShowInbox] = useState(false);
  const [showSettings, setShowSettings] = useState(false);

  async function startNewConversation() {
    const res = await apiPost<ApiResponse<ConversationData>>('/api/conversations', {
      title: 'New conversation',
      kind: 'general',
    }, (value) => decodeApiResponse(value, decodeConversationData));
    if (res.ok && res.data) {
      setSelectedConversationId(res.data.id);
      setShowInbox(false);
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
              onClick={() => setShowInbox(true)}
              className={`flex-1 px-3 py-2 text-sm ${showInbox ? 'bg-zinc-800 text-white' : 'text-zinc-500 hover:text-zinc-300'}`}
            >
              Inbox
            </button>
            <button
              type="button"
              onClick={() => setShowInbox(false)}
              className={`flex-1 px-3 py-2 text-sm ${!showInbox ? 'bg-zinc-800 text-white' : 'text-zinc-500 hover:text-zinc-300'}`}
            >
              Threads
            </button>
          </nav>
          <Sidebar
            selectedConversationId={selectedConversationId}
            onSelectConversation={(id) => { setSelectedConversationId(id); setShowInbox(false); }}
            onNewConversation={startNewConversation}
            onOpenSettings={() => setShowSettings(true)}
          />
        </>
      }
      main={<MainPanel conversationId={selectedConversationId} showInbox={showInbox} />}
      contextPanel={<ContextPanel />}
    />
  );
}

export default App;
