# Feature Capability Matrix

## iPhone

- supportsChat: yes
- supportsVoicePushToTalk: yes
- supportsDashboard: yes (compact)
- supportsProjectInspector: limited/no
- supportsWidgets: yes
- supportsLiveActivities: yes
- supportsComplications: no
- supportsAmbientHUD: no
- supportsScreenAwareness: no
- supportsQuickCapture: yes
- supportsNotificationActions: yes
- supportsSplitViewWorkspace: no

## iPad

- supportsChat: yes
- supportsVoicePushToTalk: yes
- supportsDashboard: yes
- supportsProjectInspector: yes
- supportsWidgets: yes
- supportsLiveActivities: optional
- supportsComplications: no
- supportsAmbientHUD: no
- supportsScreenAwareness: no
- supportsQuickCapture: yes
- supportsNotificationActions: yes
- supportsSplitViewWorkspace: yes

## Watch

- supportsChat: no/very limited
- supportsVoicePushToTalk: limited
- supportsDashboard: no (use compact snapshot instead)
- supportsProjectInspector: no
- supportsWidgets: watch surfaces
- supportsLiveActivities: no
- supportsComplications: yes
- supportsAmbientHUD: no
- supportsScreenAwareness: no
- supportsQuickCapture: yes
- supportsNotificationActions: yes
- supportsSplitViewWorkspace: no
- runtimeRole: edge client of `veld`
- bridgeRole: iPhone bridge preferred for cache, transport, and reconciliation
- ownsPolicyOrSynthesis: no
- supportsSensorEventEmission: yes (through approved event-log lanes)
- supportsHapticEscalation: yes
- supportsGlanceableRiskState: yes
- supportsDirectRemoteAuthority: no/prefer bridged access

Wave-3 reduced watch surface (implemented):
- supportsActiveNudges: yes (done/snooze actions only)
- supportsThreadAppend: yes (keyboard submit + voice transcript)
- supportsThreadManagement: no (handoff to phone/Mac for deeper flows)
- supportsFullThreadReadingOrEditing: no
- supportsLocalPlannerOrLLMLogic: no

## macOS (scaffold)

- supportsChat: yes
- supportsVoicePushToTalk: future
- supportsDashboard: yes
- supportsProjectInspector: yes
- supportsWidgets: optional
- supportsLiveActivities: no
- supportsComplications: no
- supportsAmbientHUD: yes
- supportsScreenAwareness: future
- supportsQuickCapture: yes
- supportsNotificationActions: yes
- supportsSplitViewWorkspace: yes
