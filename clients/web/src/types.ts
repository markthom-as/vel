export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: { code: string; message: string };
  warnings?: string[];
  meta: { request_id: string; degraded?: boolean };
}

export interface FreshnessEntryData {
  source: string;
  last_seen_at: number | null;
  status: 'fresh' | 'stale' | 'missing';
}

export interface DiagnosticsData {
  node_id: string;
  node_display_name: string;
  generated_at: number;
  sync_status: string;
  active_workers: number;
  capability_summary: string[];
  freshness_entries: FreshnessEntryData[];
}

export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonValue[] | { [key: string]: JsonValue };
export type JsonObject = { [key: string]: JsonValue };
export type UnixSeconds = number;
export type Rfc3339Timestamp = string;

export interface ConversationData {
  id: string;
  title: string | null;
  kind: string;
  pinned: boolean;
  archived: boolean;
  created_at: UnixSeconds;
  updated_at: UnixSeconds;
}

export interface MessageData {
  id: string;
  conversation_id: string;
  role: string;
  kind: string;
  content: JsonValue;
  status: string | null;
  importance: string | null;
  created_at: UnixSeconds;
  updated_at: UnixSeconds | null;
}

export interface CreateMessageResponse {
  user_message: MessageData;
  assistant_message?: MessageData | null;
  assistant_error?: string | null;
}

export interface InboxItemData {
  id: string;
  message_id: string;
  kind: string;
  state: string;
  surfaced_at: UnixSeconds;
  snoozed_until: UnixSeconds | null;
  confidence: number | null;
  conversation_id: string | null;
  title: string;
  summary: string;
  project_id: string | null;
  project_label: string | null;
  available_actions: AvailableActionData[];
  evidence: ActionEvidenceRefData[];
}

export interface InterventionActionData {
  id: string;
  state: string;
}

export interface SyncResultData {
  source: string;
  signals_ingested: number;
}

export interface ProvenanceData {
  message_id: string;
  events: ProvenanceEvent[];
  signals: JsonValue[];
  policy_decisions: JsonValue[];
  linked_objects: JsonValue[];
}

export interface ProvenanceEvent {
  id: string;
  event_name: string;
  created_at: UnixSeconds;
  payload: JsonValue;
}

export interface SettingsData {
  quiet_hours?: JsonValue;
  disable_proactive?: boolean;
  toggle_risks?: boolean;
  toggle_reminders?: boolean;
  timezone?: string | null;
  node_display_name?: string | null;
  tailscale_base_url?: string | null;
  lan_base_url?: string | null;
  adaptive_policy_overrides?: {
    default_prep_minutes?: number | null;
    commute_buffer_minutes?: number | null;
    default_prep_source_suggestion_id?: string | null;
    default_prep_source_title?: string | null;
    default_prep_source_accepted_at?: UnixSeconds | null;
    commute_buffer_source_suggestion_id?: string | null;
    commute_buffer_source_title?: string | null;
    commute_buffer_source_accepted_at?: UnixSeconds | null;
  };
}

export interface IntegrationCalendarData {
  id: string;
  summary: string;
  primary: boolean;
  selected: boolean;
}

export interface IntegrationGuidanceData {
  title: string;
  detail: string;
  action: string;
}

export interface GoogleCalendarIntegrationData {
  configured: boolean;
  connected: boolean;
  has_client_id: boolean;
  has_client_secret: boolean;
  calendars: IntegrationCalendarData[];
  all_calendars_selected: boolean;
  last_sync_at: UnixSeconds | null;
  last_sync_status: string | null;
  last_error: string | null;
  last_item_count: number | null;
  guidance: IntegrationGuidanceData | null;
}

export interface TodoistIntegrationData {
  configured: boolean;
  connected: boolean;
  has_api_token: boolean;
  last_sync_at: UnixSeconds | null;
  last_sync_status: string | null;
  last_error: string | null;
  last_item_count: number | null;
  guidance: IntegrationGuidanceData | null;
}

export interface LocalIntegrationData {
  configured: boolean;
  source_path: string | null;
  suggested_paths: string[];
  source_kind: string;
  last_sync_at: UnixSeconds | null;
  last_sync_status: string | null;
  last_error: string | null;
  last_item_count: number | null;
  guidance: IntegrationGuidanceData | null;
}

export interface LocalIntegrationPathSelectionData {
  source_path: string | null;
}

export interface IntegrationsData {
  google_calendar: GoogleCalendarIntegrationData;
  todoist: TodoistIntegrationData;
  activity: LocalIntegrationData;
  health: LocalIntegrationData;
  git: LocalIntegrationData;
  messaging: LocalIntegrationData;
  reminders: LocalIntegrationData;
  notes: LocalIntegrationData;
  transcripts: LocalIntegrationData;
}

export interface IntegrationConnectionSettingRefData {
  setting_key: string;
  setting_value: string;
  created_at: UnixSeconds;
}

export interface IntegrationConnectionData {
  id: string;
  family: string;
  provider_key: string;
  status: string;
  display_name: string;
  account_ref: string | null;
  metadata: JsonValue;
  created_at: UnixSeconds;
  updated_at: UnixSeconds;
  setting_refs: IntegrationConnectionSettingRefData[];
}

export interface IntegrationConnectionEventData {
  id: string;
  connection_id: string;
  event_type: string;
  payload: JsonValue;
  timestamp: UnixSeconds;
  created_at: UnixSeconds;
}

export interface ComponentData {
  id: string;
  name: string;
  description: string;
  status: string;
  last_restarted_at: UnixSeconds | null;
  last_error: string | null;
  restart_count: number;
}

export interface ClusterBootstrapData {
  node_id: string;
  node_display_name: string;
  active_authority_node_id: string;
  active_authority_epoch: number;
  sync_base_url: string;
  sync_transport: string;
  tailscale_base_url: string | null;
  lan_base_url: string | null;
  localhost_base_url: string | null;
  capabilities: string[];
  linked_nodes: LinkedNodeData[];
  projects: ProjectRecordData[];
  action_items: ActionItemData[];
}

export type ProjectFamilyData = 'personal' | 'creative' | 'work';
export type ProjectStatusData = 'active' | 'paused' | 'archived';

export interface ProjectRootRefData {
  path: string;
  label: string;
  kind: string;
}

export interface ProjectProvisionRequestData {
  create_repo: boolean;
  create_notes_root: boolean;
}

export interface ProjectRecordData {
  id: string;
  slug: string;
  name: string;
  family: ProjectFamilyData;
  status: ProjectStatusData;
  primary_repo: ProjectRootRefData;
  primary_notes_root: ProjectRootRefData;
  secondary_repos: ProjectRootRefData[];
  secondary_notes_roots: ProjectRootRefData[];
  upstream_ids: Record<string, string>;
  pending_provision: ProjectProvisionRequestData;
  created_at: Rfc3339Timestamp;
  updated_at: Rfc3339Timestamp;
  archived_at: Rfc3339Timestamp | null;
}

export interface ProjectCreateRequestData {
  slug: string;
  name: string;
  family: ProjectFamilyData;
  status?: ProjectStatusData | null;
  primary_repo: ProjectRootRefData;
  primary_notes_root: ProjectRootRefData;
  secondary_repos: ProjectRootRefData[];
  secondary_notes_roots: ProjectRootRefData[];
  upstream_ids: Record<string, string>;
  pending_provision: ProjectProvisionRequestData;
}

export interface ProjectCreateResponseData {
  project: ProjectRecordData;
}

export interface ProjectListResponseData {
  projects: ProjectRecordData[];
}

export type ActionSurfaceData = 'now' | 'inbox';
export type ActionKindData =
  | 'next_step'
  | 'intervention'
  | 'review'
  | 'freshness'
  | 'blocked'
  | 'conflict'
  | 'linking';
export type ActionStateData =
  | 'active'
  | 'acknowledged'
  | 'resolved'
  | 'dismissed'
  | 'snoozed';
export type AvailableActionData =
  | 'acknowledge'
  | 'resolve'
  | 'dismiss'
  | 'snooze'
  | 'open_thread'
  | 'open_project'
  | 'sync_now'
  | 'link_node';

export interface ActionEvidenceRefData {
  source_kind: string;
  source_id: string;
  label: string;
  detail: string | null;
}

export interface ActionItemData {
  id: string;
  surface: ActionSurfaceData;
  kind: ActionKindData;
  title: string;
  summary: string;
  project_id: string | null;
  state: ActionStateData;
  rank: number;
  surfaced_at: Rfc3339Timestamp;
  snoozed_until: Rfc3339Timestamp | null;
  evidence: ActionEvidenceRefData[];
}

export interface ReviewSnapshotData {
  open_action_count: number;
  triage_count: number;
  projects_needing_review: number;
}

export type LinkStatusData = 'pending' | 'linked' | 'revoked' | 'expired';

export interface LinkScopeData {
  read_context: boolean;
  write_safe_actions: boolean;
  execute_repo_tasks: boolean;
}

export interface PairingTokenData {
  token_id: string;
  token_code: string;
  issued_at: Rfc3339Timestamp;
  expires_at: Rfc3339Timestamp;
  issued_by_node_id: string;
  scopes: LinkScopeData;
}

export interface LinkedNodeData {
  node_id: string;
  node_display_name: string;
  status: LinkStatusData;
  scopes: LinkScopeData;
  linked_at: Rfc3339Timestamp;
  last_seen_at: Rfc3339Timestamp | null;
  transport_hint: string | null;
}

export interface NudgeData {
  nudge_id: string;
  nudge_type: string;
  level: string;
  state: string;
  related_commitment_id: string | null;
  message: string;
  created_at: UnixSeconds;
  snoozed_until: UnixSeconds | null;
  resolved_at: UnixSeconds | null;
}

export interface SyncBootstrapData {
  cluster: ClusterBootstrapData;
  current_context: CurrentContextData | null;
  nudges: NudgeData[];
  commitments: CommitmentData[];
  linked_nodes: LinkedNodeData[];
  projects: ProjectRecordData[];
  action_items: ActionItemData[];
}

export interface ComponentLogEventData {
  id: string;
  component_id: string;
  event_name: string;
  status: string;
  message: string;
  payload: JsonValue;
  created_at: UnixSeconds;
}

export interface IntegrationLogEventData {
  id: string;
  integration_id: string;
  event_name: string;
  status: string;
  message: string;
  payload: JsonValue;
  created_at: UnixSeconds;
}

export interface GoogleCalendarAuthStartData {
  auth_url: string;
}

export interface RunSummaryData {
  id: string;
  kind: string;
  status: string;
  trace_id: string;
  parent_run_id: string | null;
  automatic_retry_supported: boolean;
  automatic_retry_reason: string | null;
  unsupported_retry_override: boolean;
  unsupported_retry_override_reason: string | null;
  created_at: string;
  started_at: string | null;
  finished_at: string | null;
  duration_ms: number | null;
  retry_scheduled_at: string | null;
  retry_reason: string | null;
  blocked_reason: string | null;
}

export interface LoopData {
  kind: string;
  enabled: boolean;
  interval_seconds: number;
  last_started_at: UnixSeconds | null;
  last_finished_at: UnixSeconds | null;
  last_status: string | null;
  last_error: string | null;
  next_due_at: UnixSeconds | null;
}

export interface SuggestionEvidenceData {
  id: string;
  evidence_type: string;
  ref_id: string;
  evidence: JsonValue | null;
  weight: number | null;
  created_at: UnixSeconds;
}

export interface SuggestionData {
  id: string;
  suggestion_type: string;
  state: string;
  title: string | null;
  summary: string | null;
  priority: number;
  confidence: string | null;
  evidence_count: number;
  decision_context_summary: string | null;
  decision_context: JsonValue | null;
  evidence: SuggestionEvidenceData[] | null;
  latest_feedback_outcome: string | null;
  latest_feedback_notes: string | null;
  adaptive_policy: SuggestionAdaptivePolicyData | null;
  payload: JsonValue;
  created_at: UnixSeconds;
  resolved_at: UnixSeconds | null;
}

export interface AdaptivePolicyOverrideData {
  policy_key: string;
  value_minutes: number;
  source_suggestion_id: string | null;
  source_title: string | null;
  source_accepted_at: UnixSeconds | null;
}

export interface SuggestionAdaptivePolicyData {
  policy_key: string;
  suggested_minutes: number;
  current_minutes: number | null;
  is_active_source: boolean;
  active_override: AdaptivePolicyOverrideData | null;
}

export interface UncertaintyData {
  id: string;
  subject_type: string;
  subject_id: string | null;
  decision_kind: string;
  confidence_band: string;
  confidence_score: number | null;
  reasons: JsonValue;
  missing_evidence: JsonValue | null;
  resolution_mode: string;
  status: string;
  created_at: UnixSeconds;
  resolved_at: UnixSeconds | null;
}

export interface CurrentContextData {
  computed_at: UnixSeconds;
  context: JsonValue;
}

export interface NowLabelData {
  key: string;
  label: string;
}

export interface NowRiskSummaryData {
  level: string;
  score: number | null;
  label: string;
}

export interface NowSummaryData {
  mode: NowLabelData;
  phase: NowLabelData;
  meds: NowLabelData;
  risk: NowRiskSummaryData;
}

export interface NowEventData {
  title: string;
  start_ts: UnixSeconds;
  end_ts: UnixSeconds | null;
  location: string | null;
  prep_minutes: number | null;
  travel_minutes: number | null;
  leave_by_ts: UnixSeconds | null;
}

export interface NowTaskData {
  id: string;
  text: string;
  source_type: string;
  due_at: string | null;
  project: string | null;
  commitment_kind: string | null;
}

export interface NowScheduleData {
  empty_message: string | null;
  next_event: NowEventData | null;
  upcoming_events: NowEventData[];
}

export interface NowTasksData {
  todoist: NowTaskData[];
  other_open: NowTaskData[];
  next_commitment: NowTaskData | null;
}

export interface NowAttentionData {
  state: NowLabelData;
  drift: NowLabelData;
  severity: NowLabelData;
  confidence: number | null;
  reasons: string[];
}

export interface NowSourceActivityData {
  label: string;
  timestamp: UnixSeconds;
  summary: JsonValue;
}

export interface NowSourcesData {
  git_activity: NowSourceActivityData | null;
  health: NowSourceActivityData | null;
  note_document: NowSourceActivityData | null;
  assistant_message: NowSourceActivityData | null;
}

export interface NowFreshnessEntryData {
  key: string;
  label: string;
  status: string;
  last_sync_at: UnixSeconds | null;
  age_seconds: UnixSeconds | null;
  guidance: string | null;
}

export interface NowFreshnessData {
  overall_status: string;
  sources: NowFreshnessEntryData[];
}

export interface NowDebugData {
  raw_context: JsonValue;
  signals_used: string[];
  commitments_used: string[];
  risk_used: string[];
}

export interface NowData {
  computed_at: UnixSeconds;
  timezone: string;
  summary: NowSummaryData;
  schedule: NowScheduleData;
  tasks: NowTasksData;
  attention: NowAttentionData;
  sources: NowSourcesData;
  freshness: NowFreshnessData;
  action_items: ActionItemData[];
  review_snapshot: ReviewSnapshotData;
  reasons: string[];
  debug: NowDebugData;
}

export interface CommitmentData {
  id: string;
  text: string;
  source_type: string;
  source_id: string | null;
  status: string;
  due_at: string | null;
  project: string | null;
  commitment_kind: string | null;
  created_at: string;
  resolved_at: string | null;
  metadata: JsonValue;
}

export interface SignalExplainSummary {
  signal_id: string;
  signal_type: string;
  source: string;
  timestamp: UnixSeconds;
  summary: JsonValue;
}

export interface ContextSourceSummaryData {
  timestamp: UnixSeconds;
  summary: JsonValue;
}

export interface ContextSourceSummariesData {
  git_activity: ContextSourceSummaryData | null;
  health: ContextSourceSummaryData | null;
  note_document: ContextSourceSummaryData | null;
  assistant_message: ContextSourceSummaryData | null;
}

export interface ContextExplainData {
  computed_at: UnixSeconds;
  mode: string | null;
  morning_state: string | null;
  context: JsonValue;
  source_summaries: ContextSourceSummariesData;
  adaptive_policy_overrides: AdaptivePolicyOverrideData[];
  signals_used: string[];
  signal_summaries: SignalExplainSummary[];
  commitments_used: string[];
  risk_used: string[];
  reasons: string[];
}

export interface DriftExplainData {
  attention_state: string | null;
  drift_type: string | null;
  drift_severity: string | null;
  confidence: number | null;
  reasons: string[];
  signals_used: string[];
  signal_summaries: SignalExplainSummary[];
  commitments_used: string[];
}

export interface TextMessageContent {
  text: string;
  actions?: MessageActionContent[];
}

export interface MessageActionContent {
  action_type: string;
  label: string;
  value?: string;
  url?: string;
}

export interface ReminderCardContent {
  title: string;
  due_time?: number;
  reason?: string;
  confidence?: number;
}

export interface RiskCardContent {
  commitment_title: string;
  risk_level: string;
  risk_score?: number;
  top_drivers?: string[];
  dependency_ids?: string[];
  proposed_next_step?: string;
}

export interface SuggestionCardContent {
  suggestion_text: string;
  linked_goal?: string;
  expected_benefit?: string;
}

export interface SummaryCardContent {
  title: string;
  timeframe?: string;
  top_items?: string[];
  recommended_actions?: string[];
}

export interface WsMessageNewEvent {
  type: 'messages:new';
  timestamp: Rfc3339Timestamp;
  payload: MessageData;
}

export interface WsInterventionsNewEvent {
  type: 'interventions:new';
  timestamp: Rfc3339Timestamp;
  payload: InboxItemData;
}

export interface WsInterventionsUpdatedEvent {
  type: 'interventions:updated';
  timestamp: Rfc3339Timestamp;
  payload: InterventionActionData;
}

export interface WsContextUpdatedEvent {
  type: 'context:updated';
  timestamp: Rfc3339Timestamp;
  payload: CurrentContextData;
}

export type RunUpdateEventData = RunSummaryData;

export interface WsRunsUpdatedEvent {
  type: 'runs:updated';
  timestamp: Rfc3339Timestamp;
  payload: RunUpdateEventData;
}

export interface WsComponentsUpdatedEvent {
  type: 'components:updated';
  timestamp: Rfc3339Timestamp;
  payload: ComponentData;
}

export type WsEvent =
  | WsMessageNewEvent
  | WsInterventionsNewEvent
  | WsInterventionsUpdatedEvent
  | WsContextUpdatedEvent
  | WsRunsUpdatedEvent
  | WsComponentsUpdatedEvent;

export type WsEnvelope = WsEvent;
export type InterventionEventData = InboxItemData;

export type Decoder<T> = (value: unknown) => T;

export function decodeApiResponse<T>(value: unknown, decodeData: Decoder<T>): ApiResponse<T> {
  const record = expectRecord(value, 'api response');
  const ok = expectBoolean(record.ok, 'api response.ok');
  const metaRecord = expectRecord(record.meta, 'api response.meta');
  const meta = {
    request_id: expectString(metaRecord.request_id, 'api response.meta.request_id'),
    degraded:
      metaRecord.degraded === undefined
        ? undefined
        : expectBoolean(metaRecord.degraded, 'api response.meta.degraded'),
  };
  const warnings =
    record.warnings === undefined ? undefined : decodeArray(record.warnings, (item) => expectString(item, 'api response.warning'));

  let data: T | undefined;
  if (record.data !== undefined) {
    data = decodeData(record.data);
  }

  let error: ApiResponse<T>['error'];
  if (record.error !== undefined) {
    const errorRecord = expectRecord(record.error, 'api response.error');
    error = {
      code: expectString(errorRecord.code, 'api response.error.code'),
      message: expectString(errorRecord.message, 'api response.error.message'),
    };
  }

  return { ok, data, error, warnings, meta };
}

export function decodeNullable<T>(value: unknown, decode: Decoder<T>): T | null {
  return value === null ? null : decode(value);
}

export function decodeArray<T>(value: unknown, decodeItem: Decoder<T>): T[] {
  if (!Array.isArray(value)) {
    throw new Error('Expected array');
  }
  return value.map((item) => decodeItem(item));
}

export function decodeConversationData(value: unknown): ConversationData {
  const record = expectRecord(value, 'conversation');
  return {
    id: expectString(record.id, 'conversation.id'),
    title: expectNullableString(record.title, 'conversation.title'),
    kind: expectString(record.kind, 'conversation.kind'),
    pinned: expectBoolean(record.pinned, 'conversation.pinned'),
    archived: expectBoolean(record.archived, 'conversation.archived'),
    created_at: expectUnixSeconds(record.created_at, 'conversation.created_at'),
    updated_at: expectUnixSeconds(record.updated_at, 'conversation.updated_at'),
  };
}

export function decodeMessageData(value: unknown): MessageData {
  const record = expectRecord(value, 'message');
  return {
    id: expectString(record.id, 'message.id'),
    conversation_id: expectString(record.conversation_id, 'message.conversation_id'),
    role: expectString(record.role, 'message.role'),
    kind: expectString(record.kind, 'message.kind'),
    content: decodeJsonValue(record.content),
    status: expectNullableString(record.status, 'message.status'),
    importance: expectNullableString(record.importance, 'message.importance'),
    created_at: expectUnixSeconds(record.created_at, 'message.created_at'),
    updated_at: expectNullableUnixSeconds(record.updated_at, 'message.updated_at'),
  };
}

export function decodeCreateMessageResponse(value: unknown): CreateMessageResponse {
  const record = expectRecord(value, 'create message response');
  return {
    user_message: decodeMessageData(record.user_message),
    assistant_message:
      record.assistant_message === undefined
        ? undefined
        : decodeNullable(record.assistant_message, decodeMessageData),
    assistant_error:
      record.assistant_error === undefined
        ? undefined
        : expectNullableString(record.assistant_error, 'create message response.assistant_error'),
  };
}

export function decodeProjectFamilyData(value: unknown): ProjectFamilyData {
  return expectEnumString(value, 'project family', ['personal', 'creative', 'work']);
}

export function decodeProjectStatusData(value: unknown): ProjectStatusData {
  return expectEnumString(value, 'project status', ['active', 'paused', 'archived']);
}

export function decodeProjectRootRefData(value: unknown): ProjectRootRefData {
  const record = expectRecord(value, 'project root ref');
  return {
    path: expectString(record.path, 'project root ref.path'),
    label: expectString(record.label, 'project root ref.label'),
    kind: expectString(record.kind, 'project root ref.kind'),
  };
}

export function decodeProjectProvisionRequestData(value: unknown): ProjectProvisionRequestData {
  const record = expectRecord(value, 'project provision request');
  return {
    create_repo: expectBoolean(record.create_repo, 'project provision request.create_repo'),
    create_notes_root: expectBoolean(
      record.create_notes_root,
      'project provision request.create_notes_root',
    ),
  };
}

export function decodeProjectRecordData(value: unknown): ProjectRecordData {
  const record = expectRecord(value, 'project record');
  return {
    id: expectString(record.id, 'project record.id'),
    slug: expectString(record.slug, 'project record.slug'),
    name: expectString(record.name, 'project record.name'),
    family: decodeProjectFamilyData(record.family),
    status: decodeProjectStatusData(record.status),
    primary_repo: decodeProjectRootRefData(record.primary_repo),
    primary_notes_root: decodeProjectRootRefData(record.primary_notes_root),
    secondary_repos: decodeArray(record.secondary_repos ?? [], decodeProjectRootRefData),
    secondary_notes_roots: decodeArray(
      record.secondary_notes_roots ?? [],
      decodeProjectRootRefData,
    ),
    upstream_ids: decodeStringRecord(record.upstream_ids, 'project record.upstream_ids'),
    pending_provision: decodeProjectProvisionRequestData(
      record.pending_provision ?? { create_repo: false, create_notes_root: false },
    ),
    created_at: expectRfc3339Timestamp(record.created_at, 'project record.created_at'),
    updated_at: expectRfc3339Timestamp(record.updated_at, 'project record.updated_at'),
    archived_at: expectNullableRfc3339Timestamp(record.archived_at, 'project record.archived_at'),
  };
}

export function decodeProjectCreateRequestData(value: unknown): ProjectCreateRequestData {
  const record = expectRecord(value, 'project create request');
  return {
    slug: expectString(record.slug, 'project create request.slug'),
    name: expectString(record.name, 'project create request.name'),
    family: decodeProjectFamilyData(record.family),
    status:
      record.status === undefined
        ? undefined
        : expectNullableEnumString(record.status, 'project create request.status', [
            'active',
            'paused',
            'archived',
          ]),
    primary_repo: decodeProjectRootRefData(record.primary_repo),
    primary_notes_root: decodeProjectRootRefData(record.primary_notes_root),
    secondary_repos: decodeArray(record.secondary_repos ?? [], decodeProjectRootRefData),
    secondary_notes_roots: decodeArray(
      record.secondary_notes_roots ?? [],
      decodeProjectRootRefData,
    ),
    upstream_ids: decodeStringRecord(record.upstream_ids, 'project create request.upstream_ids'),
    pending_provision: decodeProjectProvisionRequestData(
      record.pending_provision ?? { create_repo: false, create_notes_root: false },
    ),
  };
}

export function decodeProjectCreateResponseData(value: unknown): ProjectCreateResponseData {
  const record = expectRecord(value, 'project create response');
  return {
    project: decodeProjectRecordData(record.project),
  };
}

export function decodeProjectListResponseData(value: unknown): ProjectListResponseData {
  const record = expectRecord(value, 'project list response');
  return {
    projects: decodeArray(record.projects ?? [], decodeProjectRecordData),
  };
}

export function decodeActionSurfaceData(value: unknown): ActionSurfaceData {
  return expectEnumString(value, 'action surface', ['now', 'inbox']);
}

export function decodeActionKindData(value: unknown): ActionKindData {
  return expectEnumString(value, 'action kind', [
    'next_step',
    'intervention',
    'review',
    'freshness',
    'blocked',
    'conflict',
    'linking',
  ]);
}

export function decodeActionStateData(value: unknown): ActionStateData {
  return expectEnumString(value, 'action state', [
    'active',
    'acknowledged',
    'resolved',
    'dismissed',
    'snoozed',
  ]);
}

export function decodeAvailableActionData(value: unknown): AvailableActionData {
  return expectEnumString(value, 'available action', [
    'acknowledge',
    'resolve',
    'dismiss',
    'snooze',
    'open_thread',
    'open_project',
    'sync_now',
    'link_node',
  ]);
}

export function decodeActionEvidenceRefData(value: unknown): ActionEvidenceRefData {
  const record = expectRecord(value, 'action evidence');
  return {
    source_kind: expectString(record.source_kind, 'action evidence.source_kind'),
    source_id: expectString(record.source_id, 'action evidence.source_id'),
    label: expectString(record.label, 'action evidence.label'),
    detail: expectNullableString(record.detail, 'action evidence.detail'),
  };
}

export function decodeActionItemData(value: unknown): ActionItemData {
  const record = expectRecord(value, 'action item');
  return {
    id: expectString(record.id, 'action item.id'),
    surface: decodeActionSurfaceData(record.surface),
    kind: decodeActionKindData(record.kind),
    title: expectString(record.title, 'action item.title'),
    summary: expectString(record.summary, 'action item.summary'),
    project_id: expectNullableString(record.project_id, 'action item.project_id'),
    state: decodeActionStateData(record.state),
    rank: expectNumber(record.rank, 'action item.rank'),
    surfaced_at: expectRfc3339Timestamp(record.surfaced_at, 'action item.surfaced_at'),
    snoozed_until: expectNullableRfc3339Timestamp(
      record.snoozed_until,
      'action item.snoozed_until',
    ),
    evidence: decodeArray(record.evidence ?? [], decodeActionEvidenceRefData),
  };
}

export function decodeReviewSnapshotData(value: unknown): ReviewSnapshotData {
  const record = expectRecord(value, 'review snapshot');
  return {
    open_action_count: expectNumber(record.open_action_count, 'review snapshot.open_action_count'),
    triage_count: expectNumber(record.triage_count, 'review snapshot.triage_count'),
    projects_needing_review: expectNumber(
      record.projects_needing_review,
      'review snapshot.projects_needing_review',
    ),
  };
}

export function decodeLinkStatusData(value: unknown): LinkStatusData {
  return expectEnumString(value, 'link status', ['pending', 'linked', 'revoked', 'expired']);
}

export function decodeLinkScopeData(value: unknown): LinkScopeData {
  const record = expectRecord(value, 'link scope');
  return {
    read_context: expectBoolean(record.read_context, 'link scope.read_context'),
    write_safe_actions: expectBoolean(record.write_safe_actions, 'link scope.write_safe_actions'),
    execute_repo_tasks: expectBoolean(
      record.execute_repo_tasks,
      'link scope.execute_repo_tasks',
    ),
  };
}

export function decodePairingTokenData(value: unknown): PairingTokenData {
  const record = expectRecord(value, 'pairing token');
  return {
    token_id: expectString(record.token_id, 'pairing token.token_id'),
    token_code: expectString(record.token_code, 'pairing token.token_code'),
    issued_at: expectRfc3339Timestamp(record.issued_at, 'pairing token.issued_at'),
    expires_at: expectRfc3339Timestamp(record.expires_at, 'pairing token.expires_at'),
    issued_by_node_id: expectString(
      record.issued_by_node_id,
      'pairing token.issued_by_node_id',
    ),
    scopes: decodeLinkScopeData(record.scopes),
  };
}

export function decodeLinkedNodeData(value: unknown): LinkedNodeData {
  const record = expectRecord(value, 'linked node');
  return {
    node_id: expectString(record.node_id, 'linked node.node_id'),
    node_display_name: expectString(record.node_display_name, 'linked node.node_display_name'),
    status: decodeLinkStatusData(record.status),
    scopes: decodeLinkScopeData(record.scopes),
    linked_at: expectRfc3339Timestamp(record.linked_at, 'linked node.linked_at'),
    last_seen_at: expectNullableRfc3339Timestamp(record.last_seen_at, 'linked node.last_seen_at'),
    transport_hint: expectNullableString(record.transport_hint, 'linked node.transport_hint'),
  };
}

export function decodeNudgeData(value: unknown): NudgeData {
  const record = expectRecord(value, 'nudge');
  return {
    nudge_id: expectString(record.nudge_id, 'nudge.nudge_id'),
    nudge_type: expectString(record.nudge_type, 'nudge.nudge_type'),
    level: expectString(record.level, 'nudge.level'),
    state: expectString(record.state, 'nudge.state'),
    related_commitment_id: expectNullableString(
      record.related_commitment_id,
      'nudge.related_commitment_id',
    ),
    message: expectString(record.message, 'nudge.message'),
    created_at: expectUnixSeconds(record.created_at, 'nudge.created_at'),
    snoozed_until: expectNullableUnixSeconds(record.snoozed_until, 'nudge.snoozed_until'),
    resolved_at: expectNullableUnixSeconds(record.resolved_at, 'nudge.resolved_at'),
  };
}

export function decodeInboxItemData(value: unknown): InboxItemData {
  const record = expectRecord(value, 'inbox item');
  return {
    id: expectString(record.id, 'inbox item.id'),
    message_id: expectString(record.message_id, 'inbox item.message_id'),
    kind: expectString(record.kind, 'inbox item.kind'),
    state: expectString(record.state, 'inbox item.state'),
    surfaced_at: expectUnixSeconds(record.surfaced_at, 'inbox item.surfaced_at'),
    snoozed_until: expectNullableUnixSeconds(record.snoozed_until, 'inbox item.snoozed_until'),
    confidence: expectNullableNumber(record.confidence, 'inbox item.confidence'),
    conversation_id: expectNullableString(record.conversation_id, 'inbox item.conversation_id'),
    title: expectString(record.title, 'inbox item.title'),
    summary: expectString(record.summary, 'inbox item.summary'),
    project_id: expectNullableString(record.project_id, 'inbox item.project_id'),
    project_label: expectNullableString(record.project_label, 'inbox item.project_label'),
    available_actions: decodeArray(
      record.available_actions ?? [],
      decodeAvailableActionData,
    ),
    evidence: decodeArray(record.evidence ?? [], decodeActionEvidenceRefData),
  };
}

export function decodeInterventionActionData(value: unknown): InterventionActionData {
  const record = expectRecord(value, 'intervention action');
  return {
    id: expectString(record.id, 'intervention action.id'),
    state: expectString(record.state, 'intervention action.state'),
  };
}

export function decodeSyncResultData(value: unknown): SyncResultData {
  const record = expectRecord(value, 'sync result');
  return {
    source: expectString(record.source, 'sync result.source'),
    signals_ingested: expectNumber(record.signals_ingested, 'sync result.signals_ingested'),
  };
}

export function decodeRunUpdateEventData(value: unknown): RunUpdateEventData {
  return decodeRunSummaryData(value);
}

export function decodeCurrentContextData(value: unknown): CurrentContextData {
  const record = expectRecord(value, 'current context');
  return {
    computed_at: expectUnixSeconds(record.computed_at, 'current context.computed_at'),
    context: decodeJsonValue(record.context),
  };
}

export function decodeNowLabelData(value: unknown): NowLabelData {
  const record = expectRecord(value, 'now label');
  return {
    key: expectString(record.key, 'now label.key'),
    label: expectString(record.label, 'now label.label'),
  };
}

export function decodeNowRiskSummaryData(value: unknown): NowRiskSummaryData {
  const record = expectRecord(value, 'now risk summary');
  return {
    level: expectString(record.level, 'now risk summary.level'),
    score: expectNullableNumber(record.score, 'now risk summary.score'),
    label: expectString(record.label, 'now risk summary.label'),
  };
}

export function decodeNowEventData(value: unknown): NowEventData {
  const record = expectRecord(value, 'now event');
  return {
    title: expectString(record.title, 'now event.title'),
    start_ts: expectUnixSeconds(record.start_ts, 'now event.start_ts'),
    end_ts: expectNullableUnixSeconds(record.end_ts, 'now event.end_ts'),
    location: expectNullableString(record.location, 'now event.location'),
    prep_minutes: expectNullableNumber(record.prep_minutes, 'now event.prep_minutes'),
    travel_minutes: expectNullableNumber(record.travel_minutes, 'now event.travel_minutes'),
    leave_by_ts: expectNullableUnixSeconds(record.leave_by_ts, 'now event.leave_by_ts'),
  };
}

export function decodeNowTaskData(value: unknown): NowTaskData {
  const record = expectRecord(value, 'now task');
  return {
    id: expectString(record.id, 'now task.id'),
    text: expectString(record.text, 'now task.text'),
    source_type: expectString(record.source_type, 'now task.source_type'),
    due_at: expectNullableRfc3339Timestamp(record.due_at, 'now task.due_at'),
    project: expectNullableString(record.project, 'now task.project'),
    commitment_kind: expectNullableString(record.commitment_kind, 'now task.commitment_kind'),
  };
}

function decodeNowSourceActivityData(value: unknown): NowSourceActivityData {
  const record = expectRecord(value, 'now source activity');
  return {
    label: expectString(record.label, 'now source activity.label'),
    timestamp: expectUnixSeconds(record.timestamp, 'now source activity.timestamp'),
    summary: decodeJsonValue(record.summary),
  };
}

export function decodeNowData(value: unknown): NowData {
  const record = expectRecord(value, 'now data');
  const summary = expectRecord(record.summary, 'now data.summary');
  const schedule = expectRecord(record.schedule, 'now data.schedule');
  const tasks = expectRecord(record.tasks, 'now data.tasks');
  const attention = expectRecord(record.attention, 'now data.attention');
  const sources = expectRecord(record.sources, 'now data.sources');
  const freshness = expectRecord(record.freshness, 'now data.freshness');
  const debug = expectRecord(record.debug, 'now data.debug');
  return {
    computed_at: expectUnixSeconds(record.computed_at, 'now data.computed_at'),
    timezone: expectString(record.timezone, 'now data.timezone'),
    summary: {
      mode: decodeNowLabelData(summary.mode),
      phase: decodeNowLabelData(summary.phase),
      meds: decodeNowLabelData(summary.meds),
      risk: decodeNowRiskSummaryData(summary.risk),
    },
    schedule: {
      empty_message: expectNullableString(schedule.empty_message, 'now data.schedule.empty_message'),
      next_event: decodeNullable(schedule.next_event, decodeNowEventData),
      upcoming_events: decodeArray(schedule.upcoming_events ?? [], decodeNowEventData),
    },
    tasks: {
      todoist: decodeArray(tasks.todoist ?? [], decodeNowTaskData),
      other_open: decodeArray(tasks.other_open ?? [], decodeNowTaskData),
      next_commitment: decodeNullable(tasks.next_commitment, decodeNowTaskData),
    },
    attention: {
      state: decodeNowLabelData(attention.state),
      drift: decodeNowLabelData(attention.drift),
      severity: decodeNowLabelData(attention.severity),
      confidence: expectNullableNumber(attention.confidence, 'now data.attention.confidence'),
      reasons: decodeArray(attention.reasons ?? [], (item) => expectString(item, 'now data.attention.reasons')),
    },
    sources: {
      git_activity: decodeNullable(sources.git_activity, decodeNowSourceActivityData),
      health: decodeNullable(sources.health, decodeNowSourceActivityData),
      note_document: decodeNullable(sources.note_document, decodeNowSourceActivityData),
      assistant_message: decodeNullable(
        sources.assistant_message,
        decodeNowSourceActivityData,
      ),
    },
    freshness: {
      overall_status: expectString(freshness.overall_status, 'now data.freshness.overall_status'),
      sources: decodeArray(freshness.sources ?? [], (item) => {
        const source = expectRecord(item, 'now freshness source');
        return {
          key: expectString(source.key, 'now freshness source.key'),
          label: expectString(source.label, 'now freshness source.label'),
          status: expectString(source.status, 'now freshness source.status'),
          last_sync_at: expectNullableUnixSeconds(source.last_sync_at, 'now freshness source.last_sync_at'),
          age_seconds: expectNullableUnixSeconds(source.age_seconds, 'now freshness source.age_seconds'),
          guidance: expectNullableString(source.guidance, 'now freshness source.guidance'),
        };
      }),
    },
    action_items: decodeArray(record.action_items ?? [], decodeActionItemData),
    review_snapshot: decodeReviewSnapshotData(
      record.review_snapshot ?? {
        open_action_count: 0,
        triage_count: 0,
        projects_needing_review: 0,
      },
    ),
    reasons: decodeArray(record.reasons ?? [], (item) => expectString(item, 'now data.reasons')),
    debug: {
      raw_context: decodeJsonValue(debug.raw_context),
      signals_used: decodeArray(debug.signals_used ?? [], (item) => expectString(item, 'now data.debug.signals_used')),
      commitments_used: decodeArray(debug.commitments_used ?? [], (item) => expectString(item, 'now data.debug.commitments_used')),
      risk_used: decodeArray(debug.risk_used ?? [], (item) => expectString(item, 'now data.debug.risk_used')),
    },
  };
}

export function decodeSignalExplainSummary(value: unknown): SignalExplainSummary {
  const record = expectRecord(value, 'signal explain summary');
  return {
    signal_id: expectString(record.signal_id, 'signal explain summary.signal_id'),
    signal_type: expectString(record.signal_type, 'signal explain summary.signal_type'),
    source: expectString(record.source, 'signal explain summary.source'),
    timestamp: expectUnixSeconds(record.timestamp, 'signal explain summary.timestamp'),
    summary: decodeJsonValue(record.summary),
  };
}

export function decodeContextExplainData(value: unknown): ContextExplainData {
  const record = expectRecord(value, 'context explain');
  const sourceSummaries = expectRecord(record.source_summaries, 'context explain.source_summaries');
  return {
    computed_at: expectUnixSeconds(record.computed_at, 'context explain.computed_at'),
    mode: expectNullableString(record.mode, 'context explain.mode'),
    morning_state: expectNullableString(record.morning_state, 'context explain.morning_state'),
    context: decodeJsonValue(record.context),
    source_summaries: {
      git_activity: decodeNullable(sourceSummaries.git_activity, (item) => {
        const summary = expectRecord(item, 'context explain.source_summaries.git_activity');
        return {
          timestamp: expectUnixSeconds(summary.timestamp, 'context explain.source_summaries.git_activity.timestamp'),
          summary: decodeJsonValue(summary.summary),
        };
      }),
      health: decodeNullable(sourceSummaries.health, (item) => {
        const summary = expectRecord(item, 'context explain.source_summaries.health');
        return {
          timestamp: expectUnixSeconds(summary.timestamp, 'context explain.source_summaries.health.timestamp'),
          summary: decodeJsonValue(summary.summary),
        };
      }),
      note_document: decodeNullable(sourceSummaries.note_document, (item) => {
        const summary = expectRecord(item, 'context explain.source_summaries.note_document');
        return {
          timestamp: expectUnixSeconds(summary.timestamp, 'context explain.source_summaries.note_document.timestamp'),
          summary: decodeJsonValue(summary.summary),
        };
      }),
      assistant_message: decodeNullable(sourceSummaries.assistant_message, (item) => {
        const summary = expectRecord(item, 'context explain.source_summaries.assistant_message');
        return {
          timestamp: expectUnixSeconds(summary.timestamp, 'context explain.source_summaries.assistant_message.timestamp'),
          summary: decodeJsonValue(summary.summary),
        };
      }),
    },
    adaptive_policy_overrides: decodeArray(record.adaptive_policy_overrides ?? [], (item) => {
      const override = expectRecord(item, 'context explain.adaptive_policy_overrides');
      return {
        policy_key: expectString(override.policy_key, 'context explain.adaptive_policy_overrides.policy_key'),
        value_minutes: expectNumber(override.value_minutes, 'context explain.adaptive_policy_overrides.value_minutes'),
        source_suggestion_id: expectNullableString(
          override.source_suggestion_id,
          'context explain.adaptive_policy_overrides.source_suggestion_id',
        ),
        source_title: expectNullableString(
          override.source_title,
          'context explain.adaptive_policy_overrides.source_title',
        ),
        source_accepted_at: expectNullableUnixSeconds(
          override.source_accepted_at,
          'context explain.adaptive_policy_overrides.source_accepted_at',
        ),
      };
    }),
    signals_used: decodeArray(record.signals_used ?? [], (item) => expectString(item, 'context explain.signals_used')),
    signal_summaries: decodeArray(record.signal_summaries ?? [], decodeSignalExplainSummary),
    commitments_used: decodeArray(record.commitments_used ?? [], (item) => expectString(item, 'context explain.commitments_used')),
    risk_used: decodeArray(record.risk_used ?? [], (item) => expectString(item, 'context explain.risk_used')),
    reasons: decodeArray(record.reasons ?? [], (item) => expectString(item, 'context explain.reasons')),
  };
}

export function decodeDriftExplainData(value: unknown): DriftExplainData {
  const record = expectRecord(value, 'drift explain');
  return {
    attention_state: expectNullableString(record.attention_state, 'drift explain.attention_state'),
    drift_type: expectNullableString(record.drift_type, 'drift explain.drift_type'),
    drift_severity: expectNullableString(record.drift_severity, 'drift explain.drift_severity'),
    confidence: expectNullableNumber(record.confidence, 'drift explain.confidence'),
    reasons: decodeArray(record.reasons ?? [], (item) => expectString(item, 'drift explain.reasons')),
    signals_used: decodeArray(record.signals_used ?? [], (item) => expectString(item, 'drift explain.signals_used')),
    signal_summaries: decodeArray(record.signal_summaries ?? [], decodeSignalExplainSummary),
    commitments_used: decodeArray(record.commitments_used ?? [], (item) => expectString(item, 'drift explain.commitments_used')),
  };
}

export function decodeSettingsData(value: unknown): SettingsData {
  const record = expectRecord(value, 'settings');
  const adaptiveOverrides =
    record.adaptive_policy_overrides === undefined
      ? undefined
      : expectRecord(record.adaptive_policy_overrides, 'settings.adaptive_policy_overrides');
  return {
    quiet_hours:
      record.quiet_hours === undefined ? undefined : decodeJsonValue(record.quiet_hours),
    disable_proactive:
      record.disable_proactive === undefined
        ? undefined
        : expectBoolean(record.disable_proactive, 'settings.disable_proactive'),
    toggle_risks:
      record.toggle_risks === undefined
        ? undefined
        : expectBoolean(record.toggle_risks, 'settings.toggle_risks'),
    toggle_reminders:
      record.toggle_reminders === undefined
        ? undefined
        : expectBoolean(record.toggle_reminders, 'settings.toggle_reminders'),
    timezone:
      record.timezone === undefined
        ? undefined
        : expectNullableString(record.timezone, 'settings.timezone'),
    node_display_name:
      record.node_display_name === undefined
        ? undefined
        : expectNullableString(record.node_display_name, 'settings.node_display_name'),
    tailscale_base_url:
      record.tailscale_base_url === undefined
        ? undefined
        : expectNullableString(record.tailscale_base_url, 'settings.tailscale_base_url'),
    lan_base_url:
      record.lan_base_url === undefined
        ? undefined
        : expectNullableString(record.lan_base_url, 'settings.lan_base_url'),
    adaptive_policy_overrides:
      adaptiveOverrides === undefined
        ? undefined
        : {
            default_prep_minutes:
              adaptiveOverrides.default_prep_minutes === undefined
                ? undefined
                : expectNullableNumber(
                    adaptiveOverrides.default_prep_minutes,
                    'settings.adaptive_policy_overrides.default_prep_minutes',
                  ),
            commute_buffer_minutes:
              adaptiveOverrides.commute_buffer_minutes === undefined
                ? undefined
                : expectNullableNumber(
                    adaptiveOverrides.commute_buffer_minutes,
                    'settings.adaptive_policy_overrides.commute_buffer_minutes',
                  ),
            default_prep_source_suggestion_id:
              adaptiveOverrides.default_prep_source_suggestion_id === undefined
                ? undefined
                : expectNullableString(
                    adaptiveOverrides.default_prep_source_suggestion_id,
                    'settings.adaptive_policy_overrides.default_prep_source_suggestion_id',
                  ),
            default_prep_source_title:
              adaptiveOverrides.default_prep_source_title === undefined
                ? undefined
                : expectNullableString(
                    adaptiveOverrides.default_prep_source_title,
                    'settings.adaptive_policy_overrides.default_prep_source_title',
                  ),
            default_prep_source_accepted_at:
              adaptiveOverrides.default_prep_source_accepted_at === undefined
                ? undefined
                : expectNullableUnixSeconds(
                    adaptiveOverrides.default_prep_source_accepted_at,
                    'settings.adaptive_policy_overrides.default_prep_source_accepted_at',
                  ),
            commute_buffer_source_suggestion_id:
              adaptiveOverrides.commute_buffer_source_suggestion_id === undefined
                ? undefined
                : expectNullableString(
                    adaptiveOverrides.commute_buffer_source_suggestion_id,
                    'settings.adaptive_policy_overrides.commute_buffer_source_suggestion_id',
                  ),
            commute_buffer_source_title:
              adaptiveOverrides.commute_buffer_source_title === undefined
                ? undefined
                : expectNullableString(
                    adaptiveOverrides.commute_buffer_source_title,
                    'settings.adaptive_policy_overrides.commute_buffer_source_title',
                  ),
            commute_buffer_source_accepted_at:
              adaptiveOverrides.commute_buffer_source_accepted_at === undefined
                ? undefined
                : expectNullableUnixSeconds(
                    adaptiveOverrides.commute_buffer_source_accepted_at,
                    'settings.adaptive_policy_overrides.commute_buffer_source_accepted_at',
                  ),
          },
  };
}

export function decodeIntegrationCalendarData(value: unknown): IntegrationCalendarData {
  const record = expectRecord(value, 'integration calendar');
  return {
    id: expectString(record.id, 'integration calendar.id'),
    summary: expectString(record.summary, 'integration calendar.summary'),
    primary: expectBoolean(record.primary, 'integration calendar.primary'),
    selected: expectBoolean(record.selected, 'integration calendar.selected'),
  };
}

export function decodeIntegrationGuidanceData(value: unknown): IntegrationGuidanceData {
  const record = expectRecord(value, 'integration guidance');
  return {
    title: expectString(record.title, 'integration guidance.title'),
    detail: expectString(record.detail, 'integration guidance.detail'),
    action: expectString(record.action, 'integration guidance.action'),
  };
}

export function decodeClusterBootstrapData(value: unknown): ClusterBootstrapData {
  const record = expectRecord(value, 'cluster bootstrap');
  return {
    node_id: expectString(record.node_id, 'cluster bootstrap.node_id'),
    node_display_name: expectString(
      record.node_display_name,
      'cluster bootstrap.node_display_name',
    ),
    active_authority_node_id: expectString(
      record.active_authority_node_id,
      'cluster bootstrap.active_authority_node_id',
    ),
    active_authority_epoch: expectNumber(
      record.active_authority_epoch,
      'cluster bootstrap.active_authority_epoch',
    ),
    sync_base_url: expectString(record.sync_base_url, 'cluster bootstrap.sync_base_url'),
    sync_transport: expectString(record.sync_transport, 'cluster bootstrap.sync_transport'),
    tailscale_base_url: expectNullableString(
      record.tailscale_base_url,
      'cluster bootstrap.tailscale_base_url',
    ),
    lan_base_url: expectNullableString(
      record.lan_base_url,
      'cluster bootstrap.lan_base_url',
    ),
    localhost_base_url: expectNullableString(
      record.localhost_base_url,
      'cluster bootstrap.localhost_base_url',
    ),
    capabilities: decodeArray(record.capabilities ?? [], (item) =>
      expectString(item, 'cluster bootstrap.capabilities'),
    ),
    linked_nodes: decodeArray(record.linked_nodes ?? [], decodeLinkedNodeData),
    projects: decodeArray(record.projects ?? [], decodeProjectRecordData),
    action_items: decodeArray(record.action_items ?? [], decodeActionItemData),
  };
}

export function decodeSyncBootstrapData(value: unknown): SyncBootstrapData {
  const record = expectRecord(value, 'sync bootstrap');
  return {
    cluster: decodeClusterBootstrapData(record.cluster),
    current_context: decodeNullable(record.current_context, decodeCurrentContextData),
    nudges: decodeArray(record.nudges ?? [], decodeNudgeData),
    commitments: decodeArray(record.commitments ?? [], decodeCommitmentData),
    linked_nodes: decodeArray(record.linked_nodes ?? [], decodeLinkedNodeData),
    projects: decodeArray(record.projects ?? [], decodeProjectRecordData),
    action_items: decodeArray(record.action_items ?? [], decodeActionItemData),
  };
}

export function decodeGoogleCalendarIntegrationData(value: unknown): GoogleCalendarIntegrationData {
  const record = expectRecord(value, 'google calendar integration');
  return {
    configured: expectBoolean(record.configured, 'google calendar integration.configured'),
    connected: expectBoolean(record.connected, 'google calendar integration.connected'),
    has_client_id: expectBoolean(record.has_client_id, 'google calendar integration.has_client_id'),
    has_client_secret: expectBoolean(
      record.has_client_secret,
      'google calendar integration.has_client_secret',
    ),
    calendars: decodeArray(record.calendars ?? [], decodeIntegrationCalendarData),
    all_calendars_selected: expectBoolean(
      record.all_calendars_selected,
      'google calendar integration.all_calendars_selected',
    ),
    last_sync_at: expectNullableUnixSeconds(record.last_sync_at, 'google calendar integration.last_sync_at'),
    last_sync_status: expectNullableString(
      record.last_sync_status,
      'google calendar integration.last_sync_status',
    ),
    last_error: expectNullableString(record.last_error, 'google calendar integration.last_error'),
    last_item_count: expectNullableNumber(
      record.last_item_count,
      'google calendar integration.last_item_count',
    ),
    guidance: decodeNullable(record.guidance, decodeIntegrationGuidanceData),
  };
}

export function decodeTodoistIntegrationData(value: unknown): TodoistIntegrationData {
  const record = expectRecord(value, 'todoist integration');
  return {
    configured: expectBoolean(record.configured, 'todoist integration.configured'),
    connected: expectBoolean(record.connected, 'todoist integration.connected'),
    has_api_token: expectBoolean(record.has_api_token, 'todoist integration.has_api_token'),
    last_sync_at: expectNullableUnixSeconds(record.last_sync_at, 'todoist integration.last_sync_at'),
    last_sync_status: expectNullableString(record.last_sync_status, 'todoist integration.last_sync_status'),
    last_error: expectNullableString(record.last_error, 'todoist integration.last_error'),
    last_item_count: expectNullableNumber(record.last_item_count, 'todoist integration.last_item_count'),
    guidance: decodeNullable(record.guidance, decodeIntegrationGuidanceData),
  };
}

export function decodeLocalIntegrationData(value: unknown): LocalIntegrationData {
  const record = expectRecord(value, 'local integration');
  return {
    configured: expectBoolean(record.configured, 'local integration.configured'),
    source_path: expectNullableString(record.source_path, 'local integration.source_path'),
    suggested_paths: decodeArray(record.suggested_paths ?? [], (item) =>
      expectString(item, 'local integration.suggested_paths[]')),
    source_kind: expectString(record.source_kind ?? 'path', 'local integration.source_kind'),
    last_sync_at: expectNullableUnixSeconds(record.last_sync_at, 'local integration.last_sync_at'),
    last_sync_status: expectNullableString(record.last_sync_status, 'local integration.last_sync_status'),
    last_error: expectNullableString(record.last_error, 'local integration.last_error'),
    last_item_count: expectNullableNumber(record.last_item_count, 'local integration.last_item_count'),
    guidance: decodeNullable(record.guidance, decodeIntegrationGuidanceData),
  };
}

export function decodeLocalIntegrationPathSelectionData(
  value: unknown,
): LocalIntegrationPathSelectionData {
  const record = expectRecord(value, 'local integration path selection');
  return {
    source_path: expectNullableString(record.source_path, 'local integration path selection.source_path'),
  };
}

export function decodeIntegrationsData(value: unknown): IntegrationsData {
  const record = expectRecord(value, 'integrations');
  return {
    google_calendar: decodeGoogleCalendarIntegrationData(
      record.google_calendar ?? {},
    ),
    todoist: decodeTodoistIntegrationData(record.todoist ?? {}),
    activity: decodeLocalIntegrationData(record.activity ?? {}),
    health: decodeLocalIntegrationData(record.health ?? {}),
    git: decodeLocalIntegrationData(record.git ?? {}),
    messaging: decodeLocalIntegrationData(record.messaging ?? {}),
    reminders: decodeLocalIntegrationData(record.reminders ?? {}),
    notes: decodeLocalIntegrationData(record.notes ?? {}),
    transcripts: decodeLocalIntegrationData(record.transcripts ?? {}),
  };
}

export function decodeIntegrationConnectionSettingRefData(
  value: unknown,
): IntegrationConnectionSettingRefData {
  const record = expectRecord(value, 'integration connection setting ref');
  return {
    setting_key: expectString(
      record.setting_key,
      'integration connection setting ref.setting_key',
    ),
    setting_value: expectString(
      record.setting_value,
      'integration connection setting ref.setting_value',
    ),
    created_at: expectUnixSeconds(
      record.created_at,
      'integration connection setting ref.created_at',
    ),
  };
}

export function decodeIntegrationConnectionData(
  value: unknown,
): IntegrationConnectionData {
  const record = expectRecord(value, 'integration connection');
  return {
    id: expectString(record.id, 'integration connection.id'),
    family: expectString(record.family, 'integration connection.family'),
    provider_key: expectString(
      record.provider_key,
      'integration connection.provider_key',
    ),
    status: expectString(record.status, 'integration connection.status'),
    display_name: expectString(
      record.display_name,
      'integration connection.display_name',
    ),
    account_ref: expectNullableString(
      record.account_ref,
      'integration connection.account_ref',
    ),
    metadata: decodeJsonValue(record.metadata ?? null),
    created_at: expectUnixSeconds(
      record.created_at,
      'integration connection.created_at',
    ),
    updated_at: expectUnixSeconds(
      record.updated_at,
      'integration connection.updated_at',
    ),
    setting_refs: decodeArray(
      record.setting_refs ?? [],
      decodeIntegrationConnectionSettingRefData,
    ),
  };
}

export function decodeIntegrationConnectionEventData(
  value: unknown,
): IntegrationConnectionEventData {
  const record = expectRecord(value, 'integration connection event');
  return {
    id: expectString(record.id, 'integration connection event.id'),
    connection_id: expectString(
      record.connection_id,
      'integration connection event.connection_id',
    ),
    event_type: expectString(
      record.event_type,
      'integration connection event.event_type',
    ),
    payload: decodeJsonValue(record.payload ?? null),
    timestamp: expectUnixSeconds(
      record.timestamp,
      'integration connection event.timestamp',
    ),
    created_at: expectUnixSeconds(
      record.created_at,
      'integration connection event.created_at',
    ),
  };
}

export function decodeComponentData(value: unknown): ComponentData {
  const record = expectRecord(value, 'component');
  return {
    id: expectString(record.id, 'component.id'),
    name: expectString(record.name, 'component.name'),
    description: expectString(record.description, 'component.description'),
    status: expectString(record.status, 'component.status'),
    last_restarted_at: expectNullableUnixSeconds(
      record.last_restarted_at,
      'component.last_restarted_at',
    ),
    last_error: expectNullableString(record.last_error, 'component.last_error'),
    restart_count: expectNumber(record.restart_count, 'component.restart_count'),
  };
}

export function decodeComponentLogEventData(value: unknown): ComponentLogEventData {
  const record = expectRecord(value, 'component log event');
  return {
    id: expectString(record.id, 'component log event.id'),
    component_id: expectString(record.component_id, 'component log event.component_id'),
    event_name: expectString(record.event_name, 'component log event.event_name'),
    status: expectString(record.status, 'component log event.status'),
    message: expectString(record.message, 'component log event.message'),
    payload: decodeJsonValue(record.payload ?? null),
    created_at: expectUnixSeconds(record.created_at, 'component log event.created_at'),
  };
}

export function decodeIntegrationLogEventData(value: unknown): IntegrationLogEventData {
  const record = expectRecord(value, 'integration log event');
  return {
    id: expectString(record.id, 'integration log event.id'),
    integration_id: expectString(record.integration_id, 'integration log event.integration_id'),
    event_name: expectString(record.event_name, 'integration log event.event_name'),
    status: expectString(record.status, 'integration log event.status'),
    message: expectString(record.message, 'integration log event.message'),
    payload: decodeJsonValue(record.payload ?? null),
    created_at: expectUnixSeconds(record.created_at, 'integration log event.created_at'),
  };
}

export function decodeGoogleCalendarAuthStartData(value: unknown): GoogleCalendarAuthStartData {
  const record = expectRecord(value, 'google auth start');
  return {
    auth_url: expectString(record.auth_url, 'google auth start.auth_url'),
  };
}

export function decodeRunSummaryData(value: unknown): RunSummaryData {
  const record = expectRecord(value, 'run summary');
  const id = expectString(record.id, 'run summary.id');
  return {
    id,
    kind: expectString(record.kind, 'run summary.kind'),
    status: expectString(record.status, 'run summary.status'),
    trace_id: expectNullableString(record.trace_id ?? null, 'run summary.trace_id') ?? id,
    parent_run_id: expectNullableString(record.parent_run_id ?? null, 'run summary.parent_run_id'),
    automatic_retry_supported: expectBoolean(
      record.automatic_retry_supported,
      'run summary.automatic_retry_supported',
    ),
    automatic_retry_reason: expectNullableString(
      record.automatic_retry_reason,
      'run summary.automatic_retry_reason',
    ),
    unsupported_retry_override: expectBoolean(
      record.unsupported_retry_override,
      'run summary.unsupported_retry_override',
    ),
    unsupported_retry_override_reason: expectNullableString(
      record.unsupported_retry_override_reason,
      'run summary.unsupported_retry_override_reason',
    ),
    created_at: expectRfc3339Timestamp(record.created_at, 'run summary.created_at'),
    started_at: expectNullableRfc3339Timestamp(record.started_at, 'run summary.started_at'),
    finished_at: expectNullableRfc3339Timestamp(record.finished_at, 'run summary.finished_at'),
    duration_ms: expectNullableNumber(record.duration_ms, 'run summary.duration_ms'),
    retry_scheduled_at: expectNullableRfc3339Timestamp(
      record.retry_scheduled_at,
      'run summary.retry_scheduled_at',
    ),
    retry_reason: expectNullableString(record.retry_reason, 'run summary.retry_reason'),
    blocked_reason: expectNullableString(record.blocked_reason, 'run summary.blocked_reason'),
  };
}

export function decodeLoopData(value: unknown): LoopData {
  const record = expectRecord(value, 'loop');
  return {
    kind: expectString(record.kind, 'loop.kind'),
    enabled: expectBoolean(record.enabled, 'loop.enabled'),
    interval_seconds: expectNumber(record.interval_seconds, 'loop.interval_seconds'),
    last_started_at: expectNullableUnixSeconds(record.last_started_at, 'loop.last_started_at'),
    last_finished_at: expectNullableUnixSeconds(record.last_finished_at, 'loop.last_finished_at'),
    last_status: expectNullableString(record.last_status, 'loop.last_status'),
    last_error: expectNullableString(record.last_error, 'loop.last_error'),
    next_due_at: expectNullableUnixSeconds(record.next_due_at, 'loop.next_due_at'),
  };
}

export function decodeSuggestionEvidenceData(value: unknown): SuggestionEvidenceData {
  const record = expectRecord(value, 'suggestion evidence');
  return {
    id: expectString(record.id, 'suggestion evidence.id'),
    evidence_type: expectString(record.evidence_type, 'suggestion evidence.evidence_type'),
    ref_id: expectString(record.ref_id, 'suggestion evidence.ref_id'),
    evidence: decodeNullable(record.evidence, decodeJsonValue),
    weight: expectNullableNumber(record.weight, 'suggestion evidence.weight'),
    created_at: expectUnixSeconds(record.created_at, 'suggestion evidence.created_at'),
  };
}

export function decodeSuggestionData(value: unknown): SuggestionData {
  const record = expectRecord(value, 'suggestion');
  const adaptivePolicy = decodeNullable(record.adaptive_policy ?? null, (item) => {
    const policy = expectRecord(item, 'suggestion.adaptive_policy');
    const activeOverride = decodeNullable(policy.active_override, (overrideValue) => {
      const override = expectRecord(overrideValue, 'suggestion.adaptive_policy.active_override');
      return {
        policy_key: expectString(
          override.policy_key,
          'suggestion.adaptive_policy.active_override.policy_key',
        ),
        value_minutes: expectNumber(
          override.value_minutes,
          'suggestion.adaptive_policy.active_override.value_minutes',
        ),
        source_suggestion_id: expectNullableString(
          override.source_suggestion_id,
          'suggestion.adaptive_policy.active_override.source_suggestion_id',
        ),
        source_title: expectNullableString(
          override.source_title,
          'suggestion.adaptive_policy.active_override.source_title',
        ),
        source_accepted_at: expectNullableUnixSeconds(
          override.source_accepted_at,
          'suggestion.adaptive_policy.active_override.source_accepted_at',
        ),
      };
    });
    return {
      policy_key: expectString(policy.policy_key, 'suggestion.adaptive_policy.policy_key'),
      suggested_minutes: expectNumber(
        policy.suggested_minutes,
        'suggestion.adaptive_policy.suggested_minutes',
      ),
      current_minutes: expectNullableNumber(
        policy.current_minutes,
        'suggestion.adaptive_policy.current_minutes',
      ),
      is_active_source: expectBoolean(
        policy.is_active_source,
        'suggestion.adaptive_policy.is_active_source',
      ),
      active_override: activeOverride,
    };
  });
  return {
    id: expectString(record.id, 'suggestion.id'),
    suggestion_type: expectString(record.suggestion_type, 'suggestion.suggestion_type'),
    state: expectString(record.state, 'suggestion.state'),
    title: expectNullableString(record.title, 'suggestion.title'),
    summary: expectNullableString(record.summary, 'suggestion.summary'),
    priority: expectNumber(record.priority, 'suggestion.priority'),
    confidence: expectNullableString(record.confidence, 'suggestion.confidence'),
    evidence_count: expectNumber(record.evidence_count, 'suggestion.evidence_count'),
    decision_context_summary: expectNullableString(
      record.decision_context_summary,
      'suggestion.decision_context_summary',
    ),
    decision_context: decodeNullable(record.decision_context, decodeJsonValue),
    evidence: decodeNullable(record.evidence, (items) => decodeArray(items, decodeSuggestionEvidenceData)),
    latest_feedback_outcome: expectNullableString(
      record.latest_feedback_outcome ?? null,
      'suggestion.latest_feedback_outcome',
    ),
    latest_feedback_notes: expectNullableString(
      record.latest_feedback_notes ?? null,
      'suggestion.latest_feedback_notes',
    ),
    adaptive_policy: adaptivePolicy,
    payload: decodeJsonValue(record.payload),
    created_at: expectUnixSeconds(record.created_at, 'suggestion.created_at'),
    resolved_at: expectNullableUnixSeconds(record.resolved_at, 'suggestion.resolved_at'),
  };
}

export function decodeUncertaintyData(value: unknown): UncertaintyData {
  const record = expectRecord(value, 'uncertainty');
  return {
    id: expectString(record.id, 'uncertainty.id'),
    subject_type: expectString(record.subject_type, 'uncertainty.subject_type'),
    subject_id: expectNullableString(record.subject_id, 'uncertainty.subject_id'),
    decision_kind: expectString(record.decision_kind, 'uncertainty.decision_kind'),
    confidence_band: expectString(record.confidence_band, 'uncertainty.confidence_band'),
    confidence_score: expectNullableNumber(record.confidence_score, 'uncertainty.confidence_score'),
    reasons: decodeJsonValue(record.reasons),
    missing_evidence: decodeNullable(record.missing_evidence, decodeJsonValue),
    resolution_mode: expectString(record.resolution_mode, 'uncertainty.resolution_mode'),
    status: expectString(record.status, 'uncertainty.status'),
    created_at: expectUnixSeconds(record.created_at, 'uncertainty.created_at'),
    resolved_at: expectNullableUnixSeconds(record.resolved_at, 'uncertainty.resolved_at'),
  };
}

export function decodeCommitmentData(value: unknown): CommitmentData {
  const record = expectRecord(value, 'commitment');
  return {
    id: expectString(record.id, 'commitment.id'),
    text: expectString(record.text, 'commitment.text'),
    source_type: expectString(record.source_type, 'commitment.source_type'),
    source_id: expectNullableString(record.source_id, 'commitment.source_id'),
    status: expectString(record.status, 'commitment.status'),
    due_at: decodeNullableDateTimeString(record.due_at, 'commitment.due_at'),
    project: expectNullableString(record.project, 'commitment.project'),
    commitment_kind: expectNullableString(record.commitment_kind, 'commitment.commitment_kind'),
    created_at: decodeDateTimeString(record.created_at, 'commitment.created_at'),
    resolved_at: decodeNullableDateTimeString(record.resolved_at, 'commitment.resolved_at'),
    metadata: decodeJsonValue(record.metadata ?? null),
  };
}

function decodeDateTimeString(value: unknown, label: string): string {
  return expectRfc3339Timestamp(value, label);
}

function decodeNullableDateTimeString(value: unknown, label: string): string | null {
  if (value === null || value === undefined) {
    return null;
  }
  return decodeDateTimeString(value, label);
}

export function decodeProvenanceEvent(value: unknown): ProvenanceEvent {
  const record = expectRecord(value, 'provenance event');
  return {
    id: expectString(record.id, 'provenance event.id'),
    event_name: expectString(record.event_name, 'provenance event.event_name'),
    created_at: expectUnixSeconds(record.created_at, 'provenance event.created_at'),
    payload: decodeJsonValue(record.payload),
  };
}

export function decodeProvenanceData(value: unknown): ProvenanceData {
  const record = expectRecord(value, 'provenance');
  return {
    message_id: expectString(record.message_id, 'provenance.message_id'),
    events: decodeArray(record.events, decodeProvenanceEvent),
    signals: decodeArray(record.signals ?? [], decodeJsonValue),
    policy_decisions: decodeArray(record.policy_decisions ?? [], decodeJsonValue),
    linked_objects: decodeArray(record.linked_objects ?? [], decodeJsonValue),
  };
}

export function decodeWsEvent(value: unknown): WsEvent {
  const record = expectRecord(value, 'websocket event');
  const type = expectString(record.type, 'websocket event.type');
  const timestamp = expectRfc3339Timestamp(record.timestamp, 'websocket event.timestamp');

  switch (type) {
    case 'messages:new':
      return {
        type,
        timestamp,
        payload: decodeMessageData(record.payload),
      };
    case 'interventions:new':
      return {
        type,
        timestamp,
        payload: decodeInboxItemData(record.payload),
      };
    case 'interventions:updated':
      return {
        type,
        timestamp,
        payload: decodeInterventionActionData(record.payload),
      };
    case 'context:updated':
      return {
        type,
        timestamp,
        payload: decodeCurrentContextData(record.payload),
      };
    case 'runs:updated':
      return {
        type,
        timestamp,
        payload: decodeRunUpdateEventData(record.payload),
      };
    case 'components:updated':
      return {
        type,
        timestamp,
        payload: decodeComponentData(record.payload),
      };
    default:
      throw new Error(`Unsupported websocket event type: ${type}`);
  }
}

export function decodeTextMessageContent(value: JsonValue): TextMessageContent | null {
  const record = asRecord(value);
  if (!record || typeof record.text !== 'string') {
    return null;
  }
  return {
    text: record.text,
    actions: decodeMessageActions(record.actions),
  };
}

function decodeMessageActions(value: JsonValue | undefined): MessageActionContent[] | undefined {
  if (!Array.isArray(value)) {
    return undefined;
  }

  const actions: MessageActionContent[] = [];
  value.forEach((item) => {
      const record = asRecord(item);
      const actionType = optionalString(record?.action_type);
      const label = optionalString(record?.label);
      if (!record || !actionType || !label) {
        return;
      }
      actions.push({
        action_type: actionType,
        label,
        value: optionalString(record.value),
        url: optionalString(record.url),
      });
    });

  return actions.length > 0 ? actions : undefined;
}

export function decodeReminderCardContent(value: JsonValue): ReminderCardContent | null {
  const record = asRecord(value);
  if (!record || typeof record.title !== 'string') {
    return null;
  }
  return {
    title: record.title,
    due_time: optionalNumber(record.due_time),
    reason: optionalString(record.reason),
    confidence: optionalNumber(record.confidence),
  };
}

export function decodeRiskCardContent(value: JsonValue): RiskCardContent | null {
  const record = asRecord(value);
  const factors = asRecord(record?.factors as JsonValue);
  const commitmentTitle = optionalString(record?.commitment_title) ?? optionalString(record?.commitment_id);
  const riskLevel = optionalString(record?.risk_level);
  if (!record || !commitmentTitle || !riskLevel) {
    return null;
  }
  const topDrivers = optionalStringArray(record.top_drivers)
    ?? optionalStringArray(factors?.reasons)
    ?? undefined;
  const dependencyIds = optionalStringArray(factors?.dependency_ids) ?? undefined;
  return {
    commitment_title: commitmentTitle,
    risk_level: riskLevel,
    risk_score: optionalNumber(record.risk_score),
    top_drivers: topDrivers,
    dependency_ids: dependencyIds,
    proposed_next_step: optionalString(record.proposed_next_step),
  };
}

export function decodeSuggestionCardContent(value: JsonValue): SuggestionCardContent | null {
  const record = asRecord(value);
  if (!record || typeof record.suggestion_text !== 'string') {
    return null;
  }
  return {
    suggestion_text: record.suggestion_text,
    linked_goal: optionalString(record.linked_goal),
    expected_benefit: optionalString(record.expected_benefit),
  };
}

export function decodeSummaryCardContent(value: JsonValue): SummaryCardContent | null {
  const record = asRecord(value);
  if (!record || typeof record.title !== 'string') {
    return null;
  }
  return {
    title: record.title,
    timeframe: optionalString(record.timeframe),
    top_items: optionalStringArray(record.top_items),
    recommended_actions: optionalStringArray(record.recommended_actions),
  };
}

function decodeJsonValue(value: unknown): JsonValue {
  if (
    value === null
    || typeof value === 'string'
    || typeof value === 'number'
    || typeof value === 'boolean'
  ) {
    return value;
  }

  if (Array.isArray(value)) {
    return value.map((item) => decodeJsonValue(item));
  }

  if (typeof value === 'object') {
    const next: JsonObject = {};
    for (const [key, child] of Object.entries(value)) {
      next[key] = decodeJsonValue(child);
    }
    return next;
  }

  throw new Error('Expected JSON value');
}

function expectRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    throw new Error(`Expected ${label} to be an object`);
  }
  return value as Record<string, unknown>;
}

function asRecord(value: JsonValue): Record<string, JsonValue> | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return null;
  }
  return value;
}

function expectString(value: unknown, label: string): string {
  if (typeof value !== 'string') {
    throw new Error(`Expected ${label} to be a string`);
  }
  return value;
}

function expectRfc3339Timestamp(value: unknown, label: string): Rfc3339Timestamp {
  const timestamp = expectString(value, label);
  const isRfc3339 =
    /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})$/.test(timestamp) &&
    !Number.isNaN(Date.parse(timestamp));
  if (!isRfc3339) {
    throw new Error(`Expected ${label} to be an RFC3339 timestamp`);
  }
  return timestamp;
}

function expectNullableRfc3339Timestamp(value: unknown, label: string): Rfc3339Timestamp | null {
  if (value === null || value === undefined) {
    return null;
  }
  return expectRfc3339Timestamp(value, label);
}

function expectNullableString(value: unknown, label: string): string | null {
  if (value === null || value === undefined) {
    return null;
  }
  return expectString(value, label);
}

function expectNumber(value: unknown, label: string): number {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    throw new Error(`Expected ${label} to be a number`);
  }
  return value;
}

function expectUnixSeconds(value: unknown, label: string): UnixSeconds {
  return expectNumber(value, label);
}

function expectNullableNumber(value: unknown, label: string): number | null {
  if (value === null || value === undefined) {
    return null;
  }
  return expectNumber(value, label);
}

function expectNullableUnixSeconds(value: unknown, label: string): UnixSeconds | null {
  if (value === null || value === undefined) {
    return null;
  }
  return expectUnixSeconds(value, label);
}

function expectBoolean(value: unknown, label: string): boolean {
  if (typeof value !== 'boolean') {
    throw new Error(`Expected ${label} to be a boolean`);
  }
  return value;
}

function expectEnumString<T extends string>(
  value: unknown,
  label: string,
  allowed: readonly T[],
): T {
  const next = expectString(value, label);
  if (!allowed.includes(next as T)) {
    throw new Error(`Expected ${label} to be one of: ${allowed.join(', ')}`);
  }
  return next as T;
}

function expectNullableEnumString<T extends string>(
  value: unknown,
  label: string,
  allowed: readonly T[],
): T | null {
  if (value === null || value === undefined) {
    return null;
  }
  return expectEnumString(value, label, allowed);
}

function decodeStringRecord(value: unknown, label: string): Record<string, string> {
  const record = expectRecord(value ?? {}, label);
  const next: Record<string, string> = {};
  for (const [key, item] of Object.entries(record)) {
    next[key] = expectString(item, `${label}.${key}`);
  }
  return next;
}

function optionalString(value: JsonValue | undefined): string | undefined {
  return typeof value === 'string' ? value : undefined;
}

function optionalNumber(value: JsonValue | undefined): number | undefined {
  return typeof value === 'number' ? value : undefined;
}

function optionalStringArray(value: JsonValue | undefined): string[] | undefined {
  if (!Array.isArray(value) || value.some((item) => typeof item !== 'string')) {
    return undefined;
  }
  return value as string[];
}
