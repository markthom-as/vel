import type {
  AgentInspectData,
  IntegrationConnectionData,
  IntegrationsData,
  SemanticAliasOverridesData,
  SettingsData,
} from '../../types';
import { CoreSettingsDetail } from './SystemCoreSettingsSection';
import {
  OperationsActivityDetail,
  OperationsRecoveryDetail,
} from './SystemOperationsSections';
import {
  OverviewHorizonDetail,
  OverviewTrustDetail,
} from './SystemOverviewSections';
import {
  ControlCapabilitiesDetail,
  ControlProjectsDetail,
  PreferencesAccessibilityDetail,
} from './SystemSupportSections';
import { IntegrationsAccountsDetail } from './SystemAccountsSection';
import {
  IntegrationsProvidersDetail,
  type IntegrationActionId,
  type IntegrationProviderSummary,
} from './SystemProvidersSection';
import { PreferencesAppearanceDetail } from './SystemPreferencesSection';
import type {
  SystemNavigationTarget,
  SystemSubsectionKey,
} from './systemNavigation';

function providerNeedsRecovery(provider: IntegrationProviderSummary): boolean {
  const status = provider.status.toLowerCase();
  return status !== 'connected' && status !== 'configured';
}

function visibleBlockers(
  blockers: AgentInspectData['blockers'],
  developerMode: boolean,
) {
  const developerOnlyCodes = new Set([
    'writeback_disabled',
    'no_matching_write_grant',
  ]);
  return developerMode
    ? blockers
    : blockers.filter((blocker) => !developerOnlyCodes.has(blocker.code));
}

export function renderSystemSubsection({
  subsection,
  inspect,
  providers,
  integrations,
  connections,
  projects,
  capabilityGroups,
  settings,
  pendingAction,
  onRunIntegrationAction,
  blockers,
  preferences,
  onTogglePreference,
  onUpdateSemanticAliases,
  onCommitSettingField,
  onUpdateCoreSettings,
  developerMode,
  onUpdateLlmSettings,
  onPatchGoogleCalendar,
  onPatchTodoist,
  onStartGoogleAuth,
  onJumpToTarget,
}: {
  subsection: SystemSubsectionKey;
  inspect: AgentInspectData;
  providers: IntegrationProviderSummary[];
  integrations: IntegrationsData;
  connections: IntegrationConnectionData[];
  projects: AgentInspectData['grounding']['projects'];
  capabilityGroups: AgentInspectData['capabilities']['groups'];
  settings: SettingsData | null;
  pendingAction: IntegrationActionId | null;
  onRunIntegrationAction: (actionId: IntegrationActionId) => void | Promise<void>;
  blockers: AgentInspectData['blockers'];
  preferences: {
    denseRows: boolean;
    tabularNumbers: boolean;
    reducedMotion: boolean;
    strongFocus: boolean;
    dockedActionBar: boolean;
    semanticAliases: SemanticAliasOverridesData;
  };
  onTogglePreference: (key: 'denseRows' | 'tabularNumbers' | 'reducedMotion' | 'strongFocus' | 'dockedActionBar') => void;
  onUpdateSemanticAliases: (aliases: SemanticAliasOverridesData) => void | Promise<void>;
  onCommitSettingField: (key: 'node_display_name' | 'timezone' | 'tailscale_base_url' | 'lan_base_url', value: string) => Promise<void>;
  onUpdateCoreSettings: (patch: Record<string, unknown>) => Promise<void>;
  developerMode: boolean;
  onUpdateLlmSettings: (patch: Record<string, unknown>) => Promise<void>;
  onPatchGoogleCalendar: (patch: Record<string, unknown>) => Promise<void>;
  onPatchTodoist: (patch: Record<string, unknown>) => Promise<void>;
  onStartGoogleAuth: () => Promise<void>;
  onJumpToTarget: (target: SystemNavigationTarget) => void;
}) {
  if (subsection === 'core_settings') {
    return (
      <CoreSettingsDetail
        settings={settings}
        integrations={integrations}
        onCommitSettingField={onCommitSettingField}
        onUpdateCoreSettings={onUpdateCoreSettings}
        onJumpToTarget={onJumpToTarget}
      />
    );
  }
  if (subsection === 'trust') {
    return (
      <OverviewTrustDetail
        inspect={inspect}
        degradedProviderCount={providers.filter((provider) => provider.status !== 'connected' && provider.configured).length}
        filteredBlockers={visibleBlockers(blockers, developerMode)}
      />
    );
  }
  if (subsection === 'horizon') {
    return <OverviewHorizonDetail inspect={inspect} />;
  }
  if (subsection === 'activity') {
    return <OperationsActivityDetail providers={providers} connectionCount={connections.length} />;
  }
  if (subsection === 'recovery') {
    return (
      <OperationsRecoveryDetail
        recoveryProviders={providers.filter((provider) => providerNeedsRecovery(provider))}
        filteredBlockers={visibleBlockers(blockers, developerMode)}
      />
    );
  }
  if (subsection === 'providers') {
    return (
      <IntegrationsProvidersDetail
        providers={providers}
        settings={settings}
        integrations={integrations}
        pendingAction={pendingAction}
        onRunIntegrationAction={onRunIntegrationAction}
        onUpdateLlmSettings={onUpdateLlmSettings}
        onPatchGoogleCalendar={onPatchGoogleCalendar}
        onPatchTodoist={onPatchTodoist}
        onStartGoogleAuth={onStartGoogleAuth}
      />
    );
  }
  if (subsection === 'accounts') {
    return (
      <IntegrationsAccountsDetail
        connections={connections}
      />
    );
  }
  if (subsection === 'projects') {
    return (
      <ControlProjectsDetail
        projects={projects}
      />
    );
  }
  if (subsection === 'capabilities') {
    return (
      <ControlCapabilitiesDetail
        capabilityGroups={capabilityGroups}
        blockers={blockers}
        developerMode={developerMode}
      />
    );
  }
  if (subsection === 'appearance') {
    return <PreferencesAppearanceDetail preferences={preferences} onToggle={onTogglePreference} onUpdateSemanticAliases={onUpdateSemanticAliases} />;
  }
  if (subsection === 'accessibility') {
    return (
      <PreferencesAccessibilityDetail
        preferences={preferences}
        onToggle={onTogglePreference}
      />
    );
  }
  return null;
}
