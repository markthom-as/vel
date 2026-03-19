import { InboxView } from './InboxView';
import { NowView } from './NowView';
import { ProjectsView } from './ProjectsView';
import type { SettingsTab } from './SettingsPage';
import { SettingsPage } from './SettingsPage';
import { SuggestionsView } from './SuggestionsView';
import { StatsView } from './StatsView';
import { ThreadView } from './ThreadView';
import { getSurfaceDefinition, type MainView } from '../data/operatorSurfaces';

type SettingsNavigationTarget = {
  tab: SettingsTab;
  integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
};

interface MainPanelProps {
  conversationId: string | null;
  mainView: MainView;
  onNavigate: (view: MainView) => void;
  onOpenThread: (conversationId: string) => void;
  settingsTarget: SettingsNavigationTarget;
  onOpenSettings: (target?: {
    tab: SettingsTab;
    integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
  }) => void;
}

export function MainPanel({
  conversationId,
  mainView,
  onNavigate,
  onOpenThread,
  settingsTarget,
  onOpenSettings,
}: MainPanelProps) {
  const surface = getSurfaceDefinition(mainView);

  if (mainView === 'now') {
    return (
      <div className="flex-1 flex flex-col overflow-hidden">
        <NowView onOpenSettings={onOpenSettings} />
      </div>
    );
  }
  if (mainView === 'inbox') {
    return (
      <div className="flex-1 flex flex-col overflow-hidden">
        <InboxView onOpenThread={onOpenThread} />
      </div>
    );
  }
  if (mainView === 'suggestions') {
    return (
      <div className="flex-1 flex flex-col overflow-hidden">
        <SuggestionsView />
      </div>
    );
  }
  if (mainView === 'threads') {
    return (
      <div className="flex-1 flex flex-col overflow-hidden">
        <ThreadView conversationId={conversationId} />
      </div>
    );
  }
  if (mainView === 'settings') {
    return (
      <div className="flex-1 flex flex-col overflow-hidden bg-zinc-950 text-zinc-100">
        <SettingsPage
          onBack={() => onNavigate('now')}
          initialTab={settingsTarget.tab}
          initialIntegrationId={settingsTarget.integrationId}
        />
      </div>
    );
  }
  if (mainView === 'projects') {
    return (
      <div className="flex-1 flex flex-col overflow-hidden">
        <ProjectsView />
      </div>
    );
  }

  if (mainView === 'stats') {
    return (
      <div className="flex-1 flex flex-col overflow-hidden">
        <StatsView />
      </div>
    );
  }

  return (
    <SurfacePlaceholder
      title={surface.label}
      subtitle={`${surface.label} is still a detail surface.`}
      body={surface.blurb}
    />
  );
}

interface SurfacePlaceholderProps {
  title: string;
  subtitle: string;
  body: string;
  action?: {
    label: string;
    onClick: () => void;
  };
}

function SurfacePlaceholder({ title, subtitle, body, action }: SurfacePlaceholderProps) {
  return (
    <div className="flex-1 overflow-y-auto bg-zinc-950">
      <div className="mx-auto max-w-3xl px-6 py-10">
        <p className="text-xs uppercase tracking-[0.24em] text-zinc-500">{title}</p>
        <h1 className="mt-2 text-3xl font-semibold text-zinc-100">{subtitle}</h1>
        <p className="mt-4 text-sm leading-6 text-zinc-300">{body}</p>
        {action ? (
          <button
            type="button"
            onClick={action.onClick}
            className="mt-6 rounded-md border border-zinc-700 bg-zinc-900 px-4 py-2 text-sm text-zinc-200 hover:border-zinc-600 hover:text-zinc-100"
          >
            {action.label}
          </button>
        ) : null}
      </div>
    </div>
  );
}
