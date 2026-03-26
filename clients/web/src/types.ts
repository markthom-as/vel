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

export function decodeFreshnessEntryData(value: unknown): FreshnessEntryData {
  const record = expectRecord(value, 'freshness entry');
  return {
    source: expectString(record.source, 'freshness entry.source'),
    last_seen_at: expectNullableUnixSeconds(record.last_seen_at, 'freshness entry.last_seen_at'),
    status: expectEnumString(record.status, 'freshness entry.status', ['fresh', 'stale', 'missing']),
  };
}

export function decodeDiagnosticsData(value: unknown): DiagnosticsData {
  const record = expectRecord(value, 'diagnostics');
  return {
    node_id: expectString(record.node_id, 'diagnostics.node_id'),
    node_display_name: expectString(record.node_display_name, 'diagnostics.node_display_name'),
    generated_at: expectUnixSeconds(record.generated_at, 'diagnostics.generated_at'),
    sync_status: expectString(record.sync_status, 'diagnostics.sync_status'),
    active_workers: expectNumber(record.active_workers, 'diagnostics.active_workers'),
    capability_summary: decodeArray(
      record.capability_summary ?? [],
      (item) => expectString(item, 'diagnostics.capability_summary[]'),
    ),
    freshness_entries: decodeArray(
      record.freshness_entries ?? [],
      decodeFreshnessEntryData,
    ),
  };
}

export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonValue[] | { [key: string]: JsonValue };
export type JsonObject = { [key: string]: JsonValue };
export type UnixSeconds = number;
export type Rfc3339Timestamp = string;

export interface ThreadContinuationData {
  escalation_reason: string;
  continuation_context: JsonValue;
  review_requirements: string[];
  bounded_capability_state: string;
  continuation_category: NowHeaderBucketKindData;
  open_target: string;
}

export interface ConversationContinuationData {
  thread_id: string;
  thread_type: string;
  lifecycle_stage: string | null;
  continuation: ThreadContinuationData;
}

export interface ThreadLinkData {
  id: string;
  entity_type: string;
  entity_id: string;
  relation_type: string;
}

export interface ThreadData {
  id: string;
  thread_type: string;
  title: string;
  status: string;
  planning_kind?: string | null;
  lifecycle_stage?: string | null;
  created_at: UnixSeconds;
  updated_at: UnixSeconds;
  continuation?: ThreadContinuationData | null;
  metadata?: JsonValue | null;
  links?: ThreadLinkData[] | null;
  project_id?: string | null;
  project_label?: string | null;
}

export interface ConversationData {
  id: string;
  title: string | null;
  kind: string;
  pinned: boolean;
  archived: boolean;
  call_mode_active: boolean;
  created_at: UnixSeconds;
  updated_at: UnixSeconds;
  message_count: number;
  last_message_at?: UnixSeconds | null;
  project_label?: string | null;
  continuation: ConversationContinuationData | null;
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
  assistant_error_retryable?: boolean;
  assistant_context?: AssistantContextData | null;
}

export type AssistantEntryRouteTargetData = 'inbox' | 'threads' | 'inline';

export interface AssistantEntryVoiceProvenanceData {
  surface?: string | null;
  source_device?: string | null;
  locale?: string | null;
  transcript_origin?: string | null;
  recorded_at?: string | null;
  offline_captured_at?: string | null;
  queued_at?: string | null;
}

export type AssistantEntryAttachmentKindData =
  | 'file'
  | 'image'
  | 'person'
  | 'event'
  | 'task'
  | 'video'
  | 'audio'
  | 'link'
  | 'markdown';

export interface AssistantEntryAttachmentData {
  kind: AssistantEntryAttachmentKindData;
  label?: string | null;
  object_id?: string | null;
  mime_type?: string | null;
  metadata?: JsonValue | null;
}

export interface AssistantEntryRequest {
  text: string;
  conversation_id?: string | null;
  intent?: NowDockedInputIntentData | null;
  attachments?: AssistantEntryAttachmentData[] | null;
  voice?: AssistantEntryVoiceProvenanceData | null;
}

export interface AssistantEntryResponse {
  route_target: AssistantEntryRouteTargetData;
  entry_intent?: NowDockedInputIntentData | null;
  continuation_category?: NowHeaderBucketKindData | null;
  follow_up?: AssistantEntryFollowUpData | null;
  user_message: MessageData;
  assistant_message?: MessageData | null;
  assistant_error?: string | null;
  assistant_error_retryable?: boolean;
  assistant_context?: AssistantContextData | null;
  conversation: ConversationData;
  proposal?: AssistantActionProposalData | null;
  planning_profile_proposal?: PlanningProfileEditProposalData | null;
  daily_loop_session?: DailyLoopSessionData | null;
  end_of_day?: EndOfDayData | null;
}

export interface AssistantEntryFollowUpData {
  intervention_id: string;
  message_id: string;
  conversation_id: string;
  kind: string;
  state: string;
  surfaced_at: UnixSeconds;
  snoozed_until?: UnixSeconds | null;
  confidence?: number | null;
}

export interface AssistantActionProposalData {
  action_item_id: string;
  state: AssistantProposalStateData;
  kind: ActionKindData;
  permission_mode: ActionPermissionModeData;
  scope_affinity: ActionScopeAffinityData;
  title: string;
  summary: string;
  project_id: string | null;
  project_label: string | null;
  project_family: ProjectFamilyData | null;
  thread_route?: ActionThreadRouteData | null;
}

export type AssistantProposalStateData =
  | 'staged'
  | 'approved'
  | 'applied'
  | 'failed'
  | 'reversed';

export interface ContextCapture {
  capture_id: string;
  capture_type: string;
  content_text: string;
  occurred_at: Rfc3339Timestamp;
  source_device?: string | null;
}

export interface EndOfDayData {
  date: string;
  what_was_done: ContextCapture[];
  what_remains_open: string[];
  what_may_matter_tomorrow: string[];
}

export type SemanticSourceKindData =
  | 'capture'
  | 'artifact'
  | 'project'
  | 'note'
  | 'transcript_note'
  | 'thread'
  | 'message'
  | 'person';

export interface RecallContextSourceCountData {
  source_kind: SemanticSourceKindData;
  count: number;
}

export interface RecallContextHitData {
  record_id: string;
  source_kind: SemanticSourceKindData;
  source_id: string;
  snippet: string;
  lexical_score: number;
  semantic_score: number;
  combined_score: number;
  provenance: JsonObject;
}

export interface RecallContextData {
  query_text: string;
  hit_count: number;
  source_counts: RecallContextSourceCountData[];
  hits: RecallContextHitData[];
}

export type ReflowChangeKindData = 'moved' | 'unscheduled' | 'needs_judgment';

export type ScheduleRuleFacetKindData =
  | 'block_target'
  | 'duration'
  | 'calendar_free'
  | 'fixed_start'
  | 'time_window'
  | 'local_urgency'
  | 'local_defer';

export interface ScheduleRuleFacetData {
  kind: ScheduleRuleFacetKindData;
  label: string;
  detail?: string | null;
}

export interface ReflowChangeData {
  kind: ReflowChangeKindData;
  title: string;
  detail: string;
  project_label?: string | null;
  scheduled_start_ts?: UnixSeconds | null;
}

export interface ReflowProposalData {
  headline: string;
  summary: string;
  moved_count: number;
  unscheduled_count: number;
  needs_judgment_count: number;
  changes: ReflowChangeData[];
  rule_facets: ScheduleRuleFacetData[];
}

export interface AssistantContextData {
  query_text: string;
  summary: string;
  focus_lines: string[];
  commitments: CommitmentData[];
  recall: RecallContextData;
}

export type ScheduleTimeWindowData =
  | 'prenoon'
  | 'afternoon'
  | 'evening'
  | 'night'
  | 'day';

export interface CanonicalScheduleRulesData {
  block_target: string | null;
  duration_minutes: number | null;
  calendar_free: boolean;
  fixed_start: boolean;
  time_window: ScheduleTimeWindowData | null;
  local_urgency: boolean;
  local_defer: boolean;
}

export interface DurableRoutineBlockData {
  id: string;
  label: string;
  source: RoutineBlockSourceKindData;
  local_timezone: string;
  start_local_time: string;
  end_local_time: string;
  days_of_week: number[];
  protected: boolean;
  active: boolean;
}

export type PlanningConstraintKindData =
  | 'max_scheduled_items'
  | 'reserve_buffer_before_calendar'
  | 'reserve_buffer_after_calendar'
  | 'default_time_window'
  | 'require_judgment_for_overflow';

export interface PlanningConstraintData {
  id: string;
  label: string;
  kind: PlanningConstraintKindData;
  detail: string | null;
  time_window: ScheduleTimeWindowData | null;
  minutes: number | null;
  max_items: number | null;
  active: boolean;
}

export interface RoutinePlanningProfileData {
  routine_blocks: DurableRoutineBlockData[];
  planning_constraints: PlanningConstraintData[];
}

export interface PlanningProfileRemoveTargetData {
  id: string;
}

export type PlanningProfileMutationData =
  | { kind: 'upsert_routine_block'; data: DurableRoutineBlockData }
  | { kind: 'remove_routine_block'; data: PlanningProfileRemoveTargetData }
  | { kind: 'upsert_planning_constraint'; data: PlanningConstraintData }
  | { kind: 'remove_planning_constraint'; data: PlanningProfileRemoveTargetData };

export interface PlanningProfileMutationRequestData {
  mutation: PlanningProfileMutationData;
}

export interface PlanningProfileResponseData {
  profile: RoutinePlanningProfileData;
  proposal_summary?: PlanningProfileProposalSummaryData | null;
}

export type PlanningProfileSurfaceData =
  | 'web_settings'
  | 'cli'
  | 'apple'
  | 'assistant'
  | 'voice';

export type PlanningProfileContinuityData = 'inline' | 'thread';

export interface PlanningProfileEditProposalData {
  source_surface: PlanningProfileSurfaceData;
  state: AssistantProposalStateData;
  mutation: PlanningProfileMutationData;
  summary: string;
  requires_confirmation: boolean;
  continuity: PlanningProfileContinuityData;
  outcome_summary?: string | null;
  thread_id?: string | null;
  thread_type?: string | null;
}

export interface PlanningProfileProposalSummaryItemData {
  thread_id: string;
  state: AssistantProposalStateData;
  title: string;
  summary: string;
  outcome_summary?: string | null;
  updated_at: UnixSeconds;
}

export interface PlanningProfileProposalSummaryData {
  pending_count: number;
  latest_pending?: PlanningProfileProposalSummaryItemData | null;
  latest_applied?: PlanningProfileProposalSummaryItemData | null;
  latest_failed?: PlanningProfileProposalSummaryItemData | null;
}

export interface CommitmentSchedulingProposalSummaryItemData {
  thread_id: string;
  state: AssistantProposalStateData;
  title: string;
  summary: string;
  outcome_summary?: string | null;
  updated_at: UnixSeconds;
}

export interface CommitmentSchedulingProposalSummaryData {
  pending_count: number;
  latest_pending?: CommitmentSchedulingProposalSummaryItemData | null;
  latest_applied?: CommitmentSchedulingProposalSummaryItemData | null;
  latest_failed?: CommitmentSchedulingProposalSummaryItemData | null;
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
  writeback_enabled?: boolean;
  tailscale_preferred?: boolean;
  tailscale_base_url?: string | null;
  tailscale_base_url_auto_discovered?: boolean;
  lan_base_url?: string | null;
  lan_base_url_auto_discovered?: boolean;
  llm?: LlmSettingsData;
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
  backup?: BackupSettingsData;
  web_settings?: WebSettingsData;
  core_settings?: CoreSettingsData;
}

export interface WebSettingsData {
  dense_rows: boolean;
  tabular_numbers: boolean;
  reduced_motion: boolean;
  strong_focus: boolean;
  docked_action_bar: boolean;
  semantic_aliases?: SemanticAliasOverridesData;
}

export interface SemanticAliasOverridesData {
  project?: Record<string, string>;
  calendar?: Record<string, string>;
  nudge?: Record<string, string>;
  alert?: Record<string, string>;
  mode?: Record<string, string>;
  provider?: Record<string, string>;
}

export interface AgentProfileSettingsData {
  role: string | null;
  preferences: string | null;
  constraints: string | null;
  freeform: string | null;
}

export interface CoreSettingsData {
  user_display_name: string | null;
  client_location_label: string | null;
  developer_mode: boolean;
  bypass_setup_gate: boolean;
  agent_profile: AgentProfileSettingsData;
}

export interface LlmProfileSettingsData {
  id: string;
  provider: string;
  base_url: string;
  model: string;
  context_window: number | null;
  enabled: boolean;
  editable: boolean;
  has_api_key?: boolean;
}

export interface LlmSettingsData {
  models_dir: string;
  default_chat_profile_id: string | null;
  fallback_chat_profile_id: string | null;
  profiles: LlmProfileSettingsData[];
}

export interface LlmProfileHealthData {
  profile_id: string;
  healthy: boolean;
  message: string;
}

export interface LlmOpenAiOauthLaunchRequestData {
  profile_id?: string | null;
  base_url: string;
}

export interface LlmProfileHandshakeRequestData {
  profile_id?: string | null;
  provider: string;
  base_url: string;
  model: string;
  context_window?: number | null;
  api_key?: string | null;
}

export type BackupStatusStateData = 'ready' | 'stale' | 'missing' | 'degraded';
export type BackupTrustLevelData = 'ok' | 'warn' | 'fail';
export type BackupFreshnessStateData = 'current' | 'stale' | 'missing';

export interface BackupCoverageData {
  included: string[];
  omitted: string[];
  notes: string[];
}

export interface BackupSecretOmissionFlagsData {
  settings_secrets_omitted: boolean;
  integration_tokens_omitted: boolean;
  local_key_material_omitted: boolean;
  notes: string[];
}

export interface BackupVerificationData {
  verified: boolean;
  checksum_algorithm: string;
  checksum: string;
  checked_paths: string[];
  notes: string[];
}

export interface BackupStatusData {
  state: BackupStatusStateData;
  last_backup_id: string | null;
  last_backup_at: Rfc3339Timestamp | null;
  output_root: string | null;
  artifact_coverage: BackupCoverageData | null;
  config_coverage: BackupCoverageData | null;
  verification_summary: BackupVerificationData | null;
  warnings: string[];
}

export interface BackupFreshnessData {
  state: BackupFreshnessStateData;
  age_seconds: number | null;
  stale_after_seconds: number;
}

export interface BackupTrustData {
  level: BackupTrustLevelData;
  status: BackupStatusData;
  freshness: BackupFreshnessData;
  guidance: string[];
}

export interface BackupSettingsData {
  default_output_root: string;
  trust: BackupTrustData;
}

export interface IntegrationCalendarData {
  id: string;
  summary: string;
  primary: boolean;
  sync_enabled: boolean;
  display_enabled: boolean;
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

export interface TodoistWriteCapabilitiesData {
  completion_status: boolean;
  due_date: boolean;
  tags: boolean;
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
  write_capabilities: TodoistWriteCapabilitiesData;
}

export interface LocalIntegrationData {
  configured: boolean;
  source_path: string | null;
  selected_paths?: string[];
  available_paths?: string[];
  internal_paths?: string[];
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

export interface IntegrationSourceRefData {
  family: string;
  provider_key: string;
  connection_id: string;
  external_id: string;
}

export interface WritebackTargetRefData {
  family: string;
  provider_key: string;
  project_id: string | null;
  connection_id: string | null;
  external_id: string | null;
}

export interface WritebackOperationData {
  id: string;
  kind: string;
  risk: string;
  status: string;
  target: WritebackTargetRefData;
  requested_payload: JsonValue;
  result_payload: JsonValue | null;
  provenance: IntegrationSourceRefData[];
  conflict_case_id: string | null;
  requested_by_node_id: string;
  requested_at: Rfc3339Timestamp;
  applied_at: Rfc3339Timestamp | null;
  updated_at: Rfc3339Timestamp;
}

export interface ConflictCaseData {
  id: string;
  kind: string;
  status: string;
  target: WritebackTargetRefData;
  summary: string;
  local_payload: JsonValue;
  upstream_payload: JsonValue | null;
  resolution_payload: JsonValue | null;
  opened_at: Rfc3339Timestamp;
  resolved_at: Rfc3339Timestamp | null;
  updated_at: Rfc3339Timestamp;
}

export interface PersonAliasData {
  platform: string;
  handle: string;
  display: string;
  source_ref: IntegrationSourceRefData | null;
}

export interface PersonLinkRefData {
  kind: string;
  id: string;
  label: string;
}

export interface PersonRecordData {
  id: string;
  display_name: string;
  given_name: string | null;
  family_name: string | null;
  relationship_context: string | null;
  birthday: string | null;
  last_contacted_at: Rfc3339Timestamp | null;
  aliases: PersonAliasData[];
  links: PersonLinkRefData[];
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

export type ExecutionTaskKindData =
  | 'planning'
  | 'implementation'
  | 'debugging'
  | 'review'
  | 'research'
  | 'documentation';
export type AgentProfileData = 'budget' | 'balanced' | 'quality' | 'inherit';
export type TokenBudgetClassData = 'small' | 'medium' | 'large' | 'xlarge';
export type ExecutionReviewGateData =
  | 'none'
  | 'operator_approval'
  | 'operator_preview'
  | 'post_run_review';
export type ExecutionHandoffOriginKindData = 'human_to_agent' | 'agent_to_agent';
export type ExecutionHandoffReviewStateData = 'pending_review' | 'approved' | 'rejected';

export interface RepoWorktreeRefData {
  path: string;
  label: string;
  branch: string | null;
  head_rev: string | null;
}

export interface HandoffEnvelopeData {
  task_id: string;
  trace_id: string;
  from_agent: string;
  to_agent: string;
  objective: string;
  inputs: JsonValue;
  constraints: string[];
  read_scopes: string[];
  write_scopes: string[];
  project_id: string | null;
  task_kind: ExecutionTaskKindData | null;
  agent_profile: AgentProfileData | null;
  token_budget: TokenBudgetClassData | null;
  review_gate: ExecutionReviewGateData | null;
  repo_root: RepoWorktreeRefData | null;
  allowed_tools: string[];
  capability_scope: JsonValue;
  deadline: Rfc3339Timestamp | null;
  expected_output_schema: JsonValue;
}

export interface ExecutionHandoffData {
  handoff: HandoffEnvelopeData;
  project_id: string;
  task_kind: ExecutionTaskKindData;
  agent_profile: AgentProfileData;
  token_budget: TokenBudgetClassData;
  review_gate: ExecutionReviewGateData;
  repo: RepoWorktreeRefData;
  notes_root: ProjectRootRefData;
  manifest_id: string | null;
}

export interface ExecutionRoutingReasonData {
  code: string;
  message: string;
}

export interface ExecutionRoutingDecisionData {
  task_kind: ExecutionTaskKindData;
  agent_profile: AgentProfileData;
  token_budget: TokenBudgetClassData;
  review_gate: ExecutionReviewGateData;
  read_scopes: string[];
  write_scopes: string[];
  allowed_tools: string[];
  reasons: ExecutionRoutingReasonData[];
}

export interface ExecutionHandoffRecordData {
  id: string;
  project_id: string;
  origin_kind: ExecutionHandoffOriginKindData;
  review_state: ExecutionHandoffReviewStateData;
  handoff: ExecutionHandoffData;
  routing: ExecutionRoutingDecisionData;
  manifest_id: string | null;
  requested_by: string;
  reviewed_by: string | null;
  decision_reason: string | null;
  reviewed_at: Rfc3339Timestamp | null;
  launched_at: Rfc3339Timestamp | null;
  created_at: Rfc3339Timestamp;
  updated_at: Rfc3339Timestamp;
}

export interface ExecutionLaunchPreviewData {
  handoff_id: string;
  review_state: ExecutionHandoffReviewStateData;
  launch_ready: boolean;
  blockers: string[];
  handoff: ExecutionHandoffData;
  routing: ExecutionRoutingDecisionData;
}

export type AgentCapabilityGroupKindData =
  | 'read_context'
  | 'review_actions'
  | 'mutation_actions';

export interface AgentBlockerData {
  code: string;
  message: string;
  escalation_hint: string | null;
}

export interface AgentCapabilityEntryData {
  key: string;
  label: string;
  summary: string;
  available: boolean;
  blocked_reason: AgentBlockerData | null;
  requires_review_gate: ExecutionReviewGateData | null;
  requires_writeback_enabled: boolean;
}

export interface AgentCapabilityGroupData {
  kind: AgentCapabilityGroupKindData;
  label: string;
  entries: AgentCapabilityEntryData[];
}

export interface AgentCapabilitySummaryData {
  groups: AgentCapabilityGroupData[];
}

export interface AgentContextRefData {
  computed_at: UnixSeconds;
  mode: string | null;
  morning_state: string | null;
  current_context_path: string;
  explain_context_path: string;
  explain_drift_path: string;
}

export interface AgentReviewObligationsData {
  review_snapshot: ReviewSnapshotData;
  pending_writebacks: WritebackOperationData[];
  conflicts: ConflictCaseData[];
  pending_execution_handoffs: ExecutionHandoffRecordData[];
}

export interface AgentGroundingPackData {
  generated_at: UnixSeconds;
  now: NowData;
  current_context: AgentContextRefData | null;
  projects: ProjectRecordData[];
  people: PersonRecordData[];
  commitments: CommitmentData[];
  review: AgentReviewObligationsData;
}

export interface AgentInspectExplainabilityData {
  persisted_record_kinds: string[];
  supporting_paths: string[];
  raw_context_json_supporting_only: boolean;
}

export interface AgentInspectData {
  grounding: AgentGroundingPackData;
  capabilities: AgentCapabilitySummaryData;
  blockers: AgentBlockerData[];
  explainability: AgentInspectExplainabilityData;
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
  | 'recovery'
  | 'intervention'
  | 'check_in'
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
export type ActionPermissionModeData =
  | 'auto_allowed'
  | 'user_confirm'
  | 'blocked'
  | 'unavailable';
export type ActionScopeAffinityData =
  | 'global'
  | 'project'
  | 'thread'
  | 'connector'
  | 'daily_loop';
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

export type ActionThreadRouteTargetData = 'existing_thread' | 'filtered_threads';

export interface ActionThreadRouteData {
  target: ActionThreadRouteTargetData;
  label: string;
  thread_id: string | null;
  thread_type: string | null;
  project_id: string | null;
}

export interface ActionItemData {
  id: string;
  surface: ActionSurfaceData;
  kind: ActionKindData;
  permission_mode: ActionPermissionModeData;
  scope_affinity: ActionScopeAffinityData;
  title: string;
  summary: string;
  project_id: string | null;
  project_label: string | null;
  project_family: ProjectFamilyData | null;
  state: ActionStateData;
  rank: number;
  surfaced_at: Rfc3339Timestamp;
  snoozed_until: Rfc3339Timestamp | null;
  evidence: ActionEvidenceRefData[];
  thread_route: ActionThreadRouteData | null;
}

export interface ReviewSnapshotData {
  open_action_count: number;
  triage_count: number;
  projects_needing_review: number;
  pending_execution_reviews: number;
}

export type CheckInSourceKindData = 'daily_loop';
export type CheckInSubmitTargetKindData = 'daily_loop_turn';
export type CheckInEscalationTargetData = 'threads';
export type CheckInTransitionKindData = 'submit' | 'bypass' | 'escalate';
export type CheckInTransitionTargetKindData = 'daily_loop_turn' | 'threads';

export interface CheckInSubmitTargetData {
  kind: CheckInSubmitTargetKindData;
  reference_id: string;
}

export interface CheckInEscalationData {
  target: CheckInEscalationTargetData;
  label: string;
  thread_id?: string | null;
}

export interface CheckInTransitionData {
  kind: CheckInTransitionKindData;
  label: string;
  target: CheckInTransitionTargetKindData;
  reference_id: string | null;
  requires_response: boolean;
  requires_note: boolean;
}

export interface CheckInCardData {
  id: string;
  source_kind: CheckInSourceKindData;
  phase: DailyLoopPhaseData;
  session_id: string;
  title: string;
  summary: string;
  prompt_id: string;
  prompt_text: string;
  suggested_action_label: string | null;
  suggested_response: string | null;
  allow_skip: boolean;
  blocking: boolean;
  submit_target: CheckInSubmitTargetData;
  escalation: CheckInEscalationData | null;
  transitions: CheckInTransitionData[];
}

export type ReflowTriggerKindData =
  | 'stale_schedule'
  | 'missed_event'
  | 'slipped_planned_block'
  | 'major_sync_change'
  | 'task_no_longer_fits';
export type ReflowSeverityData = 'medium' | 'high' | 'critical';
export type ReflowAcceptModeData = 'direct_accept' | 'confirm_required';
export type ReflowTransitionKindData = 'accept' | 'edit';
export type ReflowTransitionTargetKindData = 'apply_suggestion' | 'threads';

export interface ReflowEditTargetData {
  target: CheckInEscalationTargetData;
  label: string;
}

export interface ReflowTransitionData {
  kind: ReflowTransitionKindData;
  label: string;
  target: ReflowTransitionTargetKindData;
  confirm_required: boolean;
}

export interface ReflowCardData {
  id: string;
  title: string;
  summary: string;
  trigger: ReflowTriggerKindData;
  severity: ReflowSeverityData;
  accept_mode: ReflowAcceptModeData;
  suggested_action_label: string;
  preview_lines: string[];
  edit_target: ReflowEditTargetData;
  proposal?: ReflowProposalData | null;
  transitions: ReflowTransitionData[];
}

export type CurrentContextReflowStatusKindData = 'applied' | 'editing';

export interface CurrentContextReflowStatusData {
  kind: CurrentContextReflowStatusKindData;
  trigger: ReflowTriggerKindData;
  severity: ReflowSeverityData;
  headline: string;
  detail: string;
  recorded_at: UnixSeconds;
  preview_lines: string[];
  thread_id: string | null;
}

export interface TrustReadinessFacetData {
  level: string;
  label: string;
  detail: string;
}

export interface TrustReadinessReviewData {
  open_action_count: number;
  pending_execution_reviews: number;
  pending_writeback_count: number;
  conflict_count: number;
}

export interface TrustReadinessData {
  level: string;
  headline: string;
  summary: string;
  backup: TrustReadinessFacetData;
  freshness: TrustReadinessFacetData;
  review: TrustReadinessReviewData;
  guidance: string[];
  follow_through: ActionItemData[];
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
  suggested_targets: LinkTargetSuggestionData[];
}

export interface LinkTargetSuggestionData {
  label: string;
  base_url: string;
  transport_hint: string;
  recommended: boolean;
  redeem_command_hint: string;
}

export interface LinkingPromptData {
  target_node_id: string;
  target_node_display_name: string | null;
  issued_by_node_id: string;
  issued_by_node_display_name: string | null;
  issued_at: Rfc3339Timestamp;
  expires_at: Rfc3339Timestamp;
  scopes: LinkScopeData;
  issuer_sync_base_url: string;
  issuer_sync_transport: string;
  issuer_tailscale_base_url: string | null;
  issuer_lan_base_url: string | null;
  issuer_localhost_base_url: string | null;
  issuer_public_base_url: string | null;
}

export interface LinkedNodeData {
  node_id: string;
  node_display_name: string;
  status: LinkStatusData;
  scopes: LinkScopeData;
  linked_at: Rfc3339Timestamp;
  last_seen_at: Rfc3339Timestamp | null;
  transport_hint: string | null;
  sync_base_url: string | null;
  tailscale_base_url: string | null;
  lan_base_url: string | null;
  localhost_base_url: string | null;
  public_base_url: string | null;
}

export interface WorkerCapacityData {
  max_concurrency: number;
  current_load: number;
  available_concurrency: number;
}

export interface WorkerPresenceData {
  worker_id: string;
  node_id: string;
  node_display_name: string;
  client_kind: string | null;
  client_version: string | null;
  protocol_version: string | null;
  build_id: string | null;
  worker_classes: string[];
  capabilities: string[];
  status: string;
  queue_depth: number;
  reachability: string;
  latency_class: string;
  compute_class: string;
  power_class: string;
  recent_failure_rate: number;
  tailscale_preferred: boolean;
  last_heartbeat_at: UnixSeconds;
  started_at: UnixSeconds;
  sync_base_url: string;
  sync_transport: string;
  tailscale_base_url: string | null;
  preferred_tailnet_endpoint: string | null;
  tailscale_reachable: boolean;
  lan_base_url: string | null;
  localhost_base_url: string | null;
  ping_ms: number | null;
  sync_status: string | null;
  last_upstream_sync_at: UnixSeconds | null;
  last_downstream_sync_at: UnixSeconds | null;
  last_sync_error: string | null;
  incoming_linking_prompt: LinkingPromptData | null;
  capacity: WorkerCapacityData;
}

export interface ClusterWorkersData {
  active_authority_node_id: string;
  active_authority_epoch: number;
  generated_at: UnixSeconds;
  workers: WorkerPresenceData[];
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
  event_id: string | null;
  calendar_id: string | null;
  calendar_name: string | null;
  title: string;
  start_ts: UnixSeconds;
  end_ts: UnixSeconds | null;
  all_day: boolean;
  event_url: string | null;
  attachment_url: string | null;
  location: string | null;
  notes: string | null;
  attendees: string[];
  video_url: string | null;
  video_provider: string | null;
  prep_minutes: number | null;
  travel_minutes: number | null;
  leave_by_ts: UnixSeconds | null;
  rescheduled: boolean;
}

export interface NowTaskData {
  id: string;
  text: string;
  title: string;
  description: string | null;
  tags: string[];
  source_type: string;
  due_at: string | null;
  deadline: string | null;
  project: string | null;
  commitment_kind: string | null;
}

export interface NowScheduleData {
  empty_message: string | null;
  next_event: NowEventData | null;
  upcoming_events: NowEventData[];
  following_day_events: NowEventData[];
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
  mood: NowSourceActivityData | null;
  pain: NowSourceActivityData | null;
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

export type DayPlanChangeKindData =
  | 'scheduled'
  | 'deferred'
  | 'did_not_fit'
  | 'needs_judgment';

export type RoutineBlockSourceKindData = 'operator_declared' | 'inferred' | 'imported';

export interface RoutineBlockData {
  id: string;
  label: string;
  source: RoutineBlockSourceKindData;
  start_ts: UnixSeconds;
  end_ts: UnixSeconds;
  protected: boolean;
}

export interface DayPlanChangeData {
  kind: DayPlanChangeKindData;
  title: string;
  detail: string;
  project_label: string | null;
  scheduled_start_ts: UnixSeconds | null;
  rule_facets: ScheduleRuleFacetData[];
}

export interface DayPlanProposalData {
  headline: string;
  summary: string;
  scheduled_count: number;
  deferred_count: number;
  did_not_fit_count: number;
  needs_judgment_count: number;
  changes: DayPlanChangeData[];
  routine_blocks: RoutineBlockData[];
}

export interface NowData {
  computed_at: UnixSeconds;
  timezone: string;
  header?: NowHeaderData | null;
  mesh_summary?: NowMeshSummaryData | null;
  status_row?: NowStatusRowData | null;
  context_line?: NowContextLineData | null;
  nudge_bars: NowNudgeBarData[];
  task_lane?: NowTaskLaneData | null;
  next_up_items: NowNextUpItemData[];
  progress: NowProgressData;
  docked_input?: NowDockedInputData | null;
  overview: NowOverviewData;
  summary: NowSummaryData;
  schedule: NowScheduleData;
  tasks: NowTasksData;
  attention: NowAttentionData;
  sources: NowSourcesData;
  freshness: NowFreshnessData;
  trust_readiness: TrustReadinessData;
  planning_profile_summary?: PlanningProfileProposalSummaryData | null;
  commitment_scheduling_summary?: CommitmentSchedulingProposalSummaryData | null;
  check_in: CheckInCardData | null;
  day_plan: DayPlanProposalData | null;
  reflow: ReflowCardData | null;
  reflow_status: CurrentContextReflowStatusData | null;
  action_items: ActionItemData[];
  review_snapshot: ReviewSnapshotData;
  pending_writebacks: WritebackOperationData[];
  conflicts: ConflictCaseData[];
  people: PersonRecordData[];
  reasons: string[];
  debug: NowDebugData;
}

export type NowHeaderBucketKindData =
  | 'threads_by_type'
  | 'needs_input'
  | 'new_nudges'
  | 'search_filter'
  | 'snoozed'
  | 'review_apply'
  | 'reflow'
  | 'follow_up';

export type NowCountDisplayModeData =
  | 'always_show'
  | 'show_nonzero'
  | 'hidden_until_active';

export interface NowThreadFilterTargetData {
  bucket: NowHeaderBucketKindData;
  thread_id?: string | null;
}

export interface NowHeaderBucketData {
  kind: NowHeaderBucketKindData;
  count: number;
  count_display: NowCountDisplayModeData;
  urgent: boolean;
  route_target: NowThreadFilterTargetData;
}

export interface NowHeaderData {
  title: string;
  buckets: NowHeaderBucketData[];
}

export type NowMeshSyncStateData = 'synced' | 'stale' | 'local_only' | 'offline';

export type NowRepairRouteTargetData =
  | 'settings_sync'
  | 'settings_linking'
  | 'settings_recovery';

export interface NowRepairRouteData {
  target: NowRepairRouteTargetData;
  summary: string;
}

export interface NowMeshSummaryData {
  authority_node_id: string;
  authority_label: string;
  sync_state: NowMeshSyncStateData;
  linked_node_count: number;
  queued_write_count: number;
  last_sync_at?: UnixSeconds | null;
  urgent: boolean;
  repair_route?: NowRepairRouteData | null;
}

export interface NowStatusRowData {
  date_label: string;
  time_label: string;
  context_label: string;
  elapsed_label: string;
}

export interface NowContextLineData {
  text: string;
  thread_id?: string | null;
  fallback_used: boolean;
}

export type NowNudgeBarKindData =
  | 'nudge'
  | 'needs_input'
  | 'review_request'
  | 'reflow_proposal'
  | 'thread_continuation'
  | 'trust_warning'
  | 'freshness_warning';

export interface NowNudgeActionData {
  kind: string;
  label: string;
}

export interface NowNudgeBarData {
  id: string;
  kind: NowNudgeBarKindData;
  title: string;
  summary: string;
  timestamp?: UnixSeconds | null;
  urgent: boolean;
  primary_thread_id?: string | null;
  actions: NowNudgeActionData[];
}

export type NowTaskKindData = 'task' | 'commitment' | 'event';

export interface NowTaskLaneItemData {
  id: string;
  task_kind: NowTaskKindData;
  text: string;
  title: string;
  description?: string | null;
  tags: string[];
  state: string;
  lane?: string | null;
  sort_order?: number | null;
  project?: string | null;
  primary_thread_id?: string | null;
  due_at?: string | null;
  deadline?: string | null;
  due_label?: string | null;
  is_overdue: boolean;
  deadline_label?: string | null;
  deadline_passed: boolean;
}

export interface NowTaskLaneData {
  active: NowTaskLaneItemData | null;
  pending: NowTaskLaneItemData[];
  active_items?: NowTaskLaneItemData[];
  next_up?: NowTaskLaneItemData[];
  inbox?: NowTaskLaneItemData[];
  if_time_allows?: NowTaskLaneItemData[];
  completed?: NowTaskLaneItemData[];
  recent_completed: NowTaskLaneItemData[];
  overflow_count: number;
}

export interface NowNextUpItemData {
  kind: NowTaskKindData;
  id: string;
  title: string;
  meta?: string | null;
  detail?: string | null;
  task?: NowTaskLaneItemData | null;
}

export interface NowProgressData {
  base_count: number;
  completed_count: number;
  backlog_count: number;
  completed_ratio: number;
  backlog_ratio: number;
}

export type NowDockedInputIntentData =
  | 'task'
  | 'url'
  | 'question'
  | 'note'
  | 'command'
  | 'continuation'
  | 'reflection'
  | 'scheduling';

export interface NowDockedInputData {
  supported_intents: NowDockedInputIntentData[];
  day_thread_id?: string | null;
  raw_capture_thread_id?: string | null;
}

export interface NowOverviewActionData {
  kind: string;
  title: string;
  summary: string;
  reference_id?: string | null;
}

export interface NowOverviewTimelineEntryData {
  kind: string;
  title: string;
  timestamp: UnixSeconds;
  detail?: string | null;
}

export interface NowOverviewNudgeData {
  kind: string;
  title: string;
  summary: string;
}

export interface NowOverviewWhyStateData {
  label: string;
  detail: string;
}

export interface NowOverviewSuggestionData {
  id: string;
  kind: string;
  title: string;
  summary: string;
}

export interface NowOverviewData {
  dominant_action: NowOverviewActionData | null;
  today_timeline: NowOverviewTimelineEntryData[];
  visible_nudge: NowOverviewNudgeData | null;
  why_state: NowOverviewWhyStateData[];
  suggestions: NowOverviewSuggestionData[];
  decision_options: string[];
}

export type DailyLoopPhaseData = 'morning_overview' | 'standup';
export type DailyLoopStatusData =
  | 'active'
  | 'waiting_for_input'
  | 'completed'
  | 'cancelled';
export type DailyLoopStartSourceData = 'manual' | 'automatic';
export type DailyLoopSurfaceData = 'cli' | 'web' | 'apple_voice' | 'apple_text';
export type DailyLoopTurnActionData = 'submit' | 'skip' | 'resume';
export type DailyLoopTurnStateData = 'in_progress' | 'waiting_for_input' | 'completed';
export type DailyLoopCommitmentActionData = 'accept' | 'defer' | 'choose' | 'close';
export type DailyLoopPromptKindData =
  | 'intent_question'
  | 'commitment_reduction'
  | 'constraint_check';
export type DailyStandupBucketData = 'must' | 'should' | 'stretch';
export type DailyLoopCheckInResolutionKindData = 'submitted' | 'bypassed';

export interface DailyLoopStartMetadataData {
  source: DailyLoopStartSourceData;
  surface: DailyLoopSurfaceData;
}

export interface DailyLoopStartRequestData {
  phase: DailyLoopPhaseData;
  session_date: string;
  start: DailyLoopStartMetadataData;
}

export interface DailyLoopTurnRequestData {
  session_id: string;
  action: DailyLoopTurnActionData;
  response_text: string | null;
}

export interface DailyLoopPromptData {
  prompt_id: string;
  kind: DailyLoopPromptKindData;
  text: string;
  ordinal: number;
  allow_skip: boolean;
}

export interface MorningFrictionCalloutData {
  label: string;
  detail: string;
}

export type MorningIntentSignalData =
  | { kind: 'must_do_hint'; text: string }
  | { kind: 'focus_intent'; text: string }
  | { kind: 'meeting_doubt'; text: string };

export interface DailyLoopCheckInResolutionData {
  prompt_id: string;
  ordinal: number;
  kind: DailyLoopCheckInResolutionKindData;
  response_text: string | null;
  note_text: string | null;
}

export interface DailyLoopCheckInEventData {
  event_id: string;
  session_id: string;
  prompt_id: string;
  check_in_type: string;
  session_phase: string;
  source: string;
  answered_at: UnixSeconds | null;
  text: string | null;
  scale: number | null;
  scale_min: number;
  scale_max: number;
  keywords_json: JsonValue;
  confidence: number | null;
  schema_version: number;
  skipped: boolean;
  skip_reason_code: string | null;
  skip_reason_text: string | null;
  replaced_by_event_id: string | null;
  meta_json: JsonValue;
  created_at: UnixSeconds;
  updated_at: UnixSeconds;
  run_id: string | null;
}

export interface DailyLoopCheckInEventsQueryData {
  check_in_type?: string;
  session_phase?: string;
  include_skipped?: boolean;
  limit?: number;
}

export interface DailyLoopCheckInSkipRequestData {
  source?: string;
  answered_at?: UnixSeconds;
  reason_code?: string;
  reason_text?: string;
}

export interface DailyLoopCheckInSkipResponseData {
  check_in_event_id: string;
  session_id: string;
  status: string;
  supersedes_event_id: string | null;
}

export interface MorningOverviewStateData {
  snapshot: string;
  friction_callouts: MorningFrictionCalloutData[];
  signals: MorningIntentSignalData[];
  check_in_history: DailyLoopCheckInResolutionData[];
}

export interface DailyCommitmentDraftData {
  title: string;
  bucket: DailyStandupBucketData;
  source_ref: string | null;
}

export interface DailyDeferredTaskData {
  title: string;
  source_ref: string | null;
  reason: string;
}

export interface DailyFocusBlockProposalData {
  label: string;
  start_at: Rfc3339Timestamp;
  end_at: Rfc3339Timestamp;
  reason: string;
}

export interface DailyStandupOutcomeData {
  commitments: DailyCommitmentDraftData[];
  deferred_tasks: DailyDeferredTaskData[];
  confirmed_calendar: string[];
  focus_blocks: DailyFocusBlockProposalData[];
  check_in_history: DailyLoopCheckInResolutionData[];
}

export type DailyLoopSessionOutcomeData =
  | {
      phase: 'morning_overview';
      signals: MorningIntentSignalData[];
      check_in_history: DailyLoopCheckInResolutionData[];
    }
  | ({ phase: 'standup' } & DailyStandupOutcomeData);

export type DailyLoopSessionStateData =
  | ({ phase: 'morning_overview' } & MorningOverviewStateData)
  | ({ phase: 'standup' } & DailyStandupOutcomeData);

export interface DailyLoopSessionData {
  id: string;
  session_date: string;
  phase: DailyLoopPhaseData;
  status: DailyLoopStatusData;
  start: DailyLoopStartMetadataData;
  turn_state: DailyLoopTurnStateData;
  current_prompt: DailyLoopPromptData | null;
  continuity_summary: string;
  allowed_actions: DailyLoopCommitmentActionData[];
  state: DailyLoopSessionStateData;
  outcome: DailyLoopSessionOutcomeData | null;
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
  scheduler_rules: CanonicalScheduleRulesData;
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
  attachments?: AssistantEntryAttachmentData[];
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

export interface WsLinkingUpdatedEvent {
  type: 'linking:updated';
  timestamp: Rfc3339Timestamp;
  payload: JsonValue;
}

export type WsEvent =
  | WsMessageNewEvent
  | WsInterventionsNewEvent
  | WsInterventionsUpdatedEvent
  | WsContextUpdatedEvent
  | WsRunsUpdatedEvent
  | WsComponentsUpdatedEvent
  | WsLinkingUpdatedEvent;

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
  return value === null || value === undefined ? null : decode(value);
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
    call_mode_active:
      record.call_mode_active === undefined
        ? false
        : expectBoolean(record.call_mode_active, 'conversation.call_mode_active'),
    created_at: expectUnixSeconds(record.created_at, 'conversation.created_at'),
    updated_at: expectUnixSeconds(record.updated_at, 'conversation.updated_at'),
    message_count:
      record.message_count === undefined
        ? 0
        : expectNumber(record.message_count, 'conversation.message_count'),
    last_message_at:
      record.last_message_at === undefined
        ? undefined
        : expectNullableUnixSeconds(record.last_message_at, 'conversation.last_message_at'),
    project_label:
      record.project_label === undefined
        ? undefined
        : expectNullableString(record.project_label, 'conversation.project_label'),
    continuation:
      record.continuation === undefined
        ? null
        : decodeNullable(record.continuation, decodeConversationContinuationData),
  };
}

export function decodeThreadLinkData(value: unknown): ThreadLinkData {
  const record = expectRecord(value, 'thread link');
  return {
    id: expectString(record.id, 'thread link.id'),
    entity_type: expectString(record.entity_type, 'thread link.entity_type'),
    entity_id: expectString(record.entity_id, 'thread link.entity_id'),
    relation_type: expectString(record.relation_type, 'thread link.relation_type'),
  };
}

export function decodeThreadData(value: unknown): ThreadData {
  const record = expectRecord(value, 'thread');
  return {
    id: expectString(record.id, 'thread.id'),
    thread_type: expectString(record.thread_type, 'thread.thread_type'),
    title: expectString(record.title, 'thread.title'),
    status: expectString(record.status, 'thread.status'),
    planning_kind:
      record.planning_kind === undefined
        ? undefined
        : expectNullableString(record.planning_kind, 'thread.planning_kind'),
    lifecycle_stage:
      record.lifecycle_stage === undefined
        ? undefined
        : expectNullableString(record.lifecycle_stage, 'thread.lifecycle_stage'),
    created_at: expectUnixSeconds(record.created_at, 'thread.created_at'),
    updated_at: expectUnixSeconds(record.updated_at, 'thread.updated_at'),
    continuation:
      record.continuation === undefined
        ? undefined
        : decodeNullable(record.continuation, decodeThreadContinuationData),
    metadata:
      record.metadata === undefined ? undefined : decodeJsonValue(record.metadata),
    links:
      record.links === undefined
        ? undefined
        : decodeNullable(record.links, (value) => decodeArray(value, decodeThreadLinkData)),
    project_id:
      record.project_id === undefined
        ? undefined
        : expectNullableString(record.project_id, 'thread.project_id'),
    project_label:
      record.project_label === undefined
        ? undefined
        : expectNullableString(record.project_label, 'thread.project_label'),
  };
}

export function decodeConversationContinuationData(value: unknown): ConversationContinuationData {
  const record = expectRecord(value, 'conversation continuation');
  return {
    thread_id: expectString(record.thread_id, 'conversation continuation.thread_id'),
    thread_type: expectString(record.thread_type, 'conversation continuation.thread_type'),
    lifecycle_stage: expectNullableString(
      record.lifecycle_stage,
      'conversation continuation.lifecycle_stage',
    ),
    continuation: decodeThreadContinuationData(record.continuation),
  };
}

export function decodeThreadContinuationData(value: unknown): ThreadContinuationData {
  const record = expectRecord(value, 'thread continuation');
  return {
    escalation_reason: expectString(record.escalation_reason, 'thread continuation.escalation_reason'),
    continuation_context: decodeJsonValue(record.continuation_context),
    review_requirements: decodeArray(
      record.review_requirements ?? [],
      (item) => expectString(item, 'thread continuation.review_requirements[]'),
    ),
    bounded_capability_state: expectString(
      record.bounded_capability_state,
      'thread continuation.bounded_capability_state',
    ),
    continuation_category: decodeNowHeaderBucketKindData(record.continuation_category),
    open_target: expectString(record.open_target, 'thread continuation.open_target'),
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
    assistant_error_retryable:
      record.assistant_error_retryable === undefined
        ? undefined
        : expectBoolean(record.assistant_error_retryable, 'create message response.assistant_error_retryable'),
    assistant_context:
      record.assistant_context === undefined
        ? undefined
        : decodeNullable(record.assistant_context, decodeAssistantContextData),
  };
}

export function decodeAssistantEntryRouteTargetData(value: unknown): AssistantEntryRouteTargetData {
  return expectEnumString(value, 'assistant entry route target', ['inbox', 'threads', 'inline']);
}

export function decodeAssistantEntryResponse(value: unknown): AssistantEntryResponse {
  const record = expectRecord(value, 'assistant entry response');
  return {
    route_target: decodeAssistantEntryRouteTargetData(record.route_target),
    entry_intent:
      record.entry_intent === undefined
        ? undefined
        : decodeNullable(record.entry_intent, decodeNowDockedInputIntentData),
    continuation_category:
      record.continuation_category === undefined
        ? undefined
        : decodeNullable(record.continuation_category, decodeNowHeaderBucketKindData),
    follow_up:
      record.follow_up === undefined
        ? undefined
        : decodeNullable(record.follow_up, decodeAssistantEntryFollowUpData),
    user_message: decodeMessageData(record.user_message),
    assistant_message:
      record.assistant_message === undefined
        ? undefined
        : decodeNullable(record.assistant_message, decodeMessageData),
    assistant_error:
      record.assistant_error === undefined
        ? undefined
        : expectNullableString(record.assistant_error, 'assistant entry response.assistant_error'),
    assistant_error_retryable:
      record.assistant_error_retryable === undefined
        ? undefined
        : expectBoolean(record.assistant_error_retryable, 'assistant entry response.assistant_error_retryable'),
    assistant_context:
      record.assistant_context === undefined
        ? undefined
        : decodeNullable(record.assistant_context, decodeAssistantContextData),
    conversation: decodeConversationData(record.conversation),
    proposal:
      record.proposal === undefined
        ? undefined
        : decodeNullable(record.proposal, decodeAssistantActionProposalData),
    planning_profile_proposal:
      record.planning_profile_proposal === undefined
        ? undefined
        : decodeNullable(record.planning_profile_proposal, decodePlanningProfileEditProposalData),
    daily_loop_session:
      record.daily_loop_session === undefined
        ? undefined
        : decodeNullable(record.daily_loop_session, decodeDailyLoopSessionData),
    end_of_day:
      record.end_of_day === undefined
        ? undefined
        : decodeNullable(record.end_of_day, decodeEndOfDayData),
  };
}

export function decodeAssistantEntryFollowUpData(value: unknown): AssistantEntryFollowUpData {
  const record = expectRecord(value, 'assistant entry follow-up');
  return {
    intervention_id: expectString(
      record.intervention_id,
      'assistant entry follow-up.intervention_id',
    ),
    message_id: expectString(record.message_id, 'assistant entry follow-up.message_id'),
    conversation_id: expectString(
      record.conversation_id,
      'assistant entry follow-up.conversation_id',
    ),
    kind: expectString(record.kind, 'assistant entry follow-up.kind'),
    state: expectString(record.state, 'assistant entry follow-up.state'),
    surfaced_at: expectUnixSeconds(record.surfaced_at, 'assistant entry follow-up.surfaced_at'),
    snoozed_until:
      record.snoozed_until === undefined
        ? undefined
        : expectNullableUnixSeconds(record.snoozed_until, 'assistant entry follow-up.snoozed_until'),
    confidence:
      record.confidence === undefined
        ? undefined
        : expectNullableNumber(record.confidence, 'assistant entry follow-up.confidence'),
  };
}

export function decodeAssistantActionProposalData(value: unknown): AssistantActionProposalData {
  const record = expectRecord(value, 'assistant action proposal');
  return {
    action_item_id: expectString(record.action_item_id, 'assistant action proposal.action_item_id'),
    state: decodeAssistantProposalStateData(record.state),
    kind: decodeActionKindData(record.kind),
    permission_mode: decodeActionPermissionModeData(record.permission_mode),
    scope_affinity: decodeActionScopeAffinityData(record.scope_affinity),
    title: expectString(record.title, 'assistant action proposal.title'),
    summary: expectString(record.summary, 'assistant action proposal.summary'),
    project_id: expectNullableString(record.project_id, 'assistant action proposal.project_id'),
    project_label: expectNullableString(
      record.project_label,
      'assistant action proposal.project_label',
    ),
    project_family:
      record.project_family == null ? null : decodeProjectFamilyData(record.project_family),
    thread_route:
      record.thread_route === undefined
        ? undefined
        : decodeNullable(record.thread_route, decodeActionThreadRouteData),
  };
}

export function decodeAssistantProposalStateData(value: unknown): AssistantProposalStateData {
  return expectEnumString(value, 'assistant proposal state', [
    'staged',
    'approved',
    'applied',
    'failed',
    'reversed',
  ]);
}

export function decodeContextCapture(value: unknown): ContextCapture {
  const record = expectRecord(value, 'context capture');
  return {
    capture_id: expectString(record.capture_id, 'context capture.capture_id'),
    capture_type: expectString(record.capture_type, 'context capture.capture_type'),
    content_text: expectString(record.content_text, 'context capture.content_text'),
    occurred_at: expectString(record.occurred_at, 'context capture.occurred_at'),
    source_device:
      record.source_device === undefined
        ? undefined
        : expectNullableString(record.source_device, 'context capture.source_device'),
  };
}

export function decodeEndOfDayData(value: unknown): EndOfDayData {
  const record = expectRecord(value, 'end-of-day data');
  return {
    date: expectString(record.date, 'end-of-day data.date'),
    what_was_done: decodeArray(record.what_was_done, decodeContextCapture),
    what_remains_open: decodeArray(record.what_remains_open, (item) =>
      expectString(item, 'end-of-day data.what_remains_open[]'),
    ),
    what_may_matter_tomorrow: decodeArray(record.what_may_matter_tomorrow, (item) =>
      expectString(item, 'end-of-day data.what_may_matter_tomorrow[]'),
    ),
  };
}

export function decodeSemanticSourceKindData(value: unknown): SemanticSourceKindData {
  return expectEnumString(value, 'semantic source kind', [
    'capture',
    'artifact',
    'project',
    'note',
    'transcript_note',
    'thread',
    'message',
    'person',
  ]);
}

export function decodeRecallContextSourceCountData(value: unknown): RecallContextSourceCountData {
  const record = expectRecord(value, 'recall context source count');
  return {
    source_kind: decodeSemanticSourceKindData(record.source_kind),
    count: expectNumber(record.count, 'recall context source count.count'),
  };
}

export function decodeRecallContextHitData(value: unknown): RecallContextHitData {
  const record = expectRecord(value, 'recall context hit');
  return {
    record_id: expectString(record.record_id, 'recall context hit.record_id'),
    source_kind: decodeSemanticSourceKindData(record.source_kind),
    source_id: expectString(record.source_id, 'recall context hit.source_id'),
    snippet: expectString(record.snippet, 'recall context hit.snippet'),
    lexical_score: expectNumber(record.lexical_score, 'recall context hit.lexical_score'),
    semantic_score: expectNumber(record.semantic_score, 'recall context hit.semantic_score'),
    combined_score: expectNumber(record.combined_score, 'recall context hit.combined_score'),
    provenance: decodeJsonValue(expectRecord(record.provenance, 'recall context hit.provenance')) as JsonObject,
  };
}

export function decodeRecallContextData(value: unknown): RecallContextData {
  const record = expectRecord(value, 'recall context');
  return {
    query_text: expectString(record.query_text, 'recall context.query_text'),
    hit_count: expectNumber(record.hit_count, 'recall context.hit_count'),
    source_counts: decodeArray(record.source_counts, decodeRecallContextSourceCountData),
    hits: decodeArray(record.hits, decodeRecallContextHitData),
  };
}

export function decodeAssistantContextData(value: unknown): AssistantContextData {
  const record = expectRecord(value, 'assistant context');
  return {
    query_text: expectString(record.query_text, 'assistant context.query_text'),
    summary: expectString(record.summary, 'assistant context.summary'),
    focus_lines: decodeArray(record.focus_lines, (item) =>
      expectString(item, 'assistant context.focus_lines[]'),
    ),
    commitments: decodeArray(record.commitments ?? [], decodeCommitmentData),
    recall: decodeRecallContextData(record.recall),
  };
}

export function decodeScheduleTimeWindowData(value: unknown): ScheduleTimeWindowData {
  return expectEnumString(value, 'schedule time window', [
    'prenoon',
    'afternoon',
    'evening',
    'night',
    'day',
  ]);
}

export function decodeCanonicalScheduleRulesData(value: unknown): CanonicalScheduleRulesData {
  const record = expectRecord(value, 'canonical schedule rules');
  return {
    block_target: expectNullableString(record.block_target, 'canonical schedule rules.block_target'),
    duration_minutes:
      record.duration_minutes === null || record.duration_minutes === undefined
        ? null
        : expectNumber(record.duration_minutes, 'canonical schedule rules.duration_minutes'),
    calendar_free: expectBoolean(record.calendar_free, 'canonical schedule rules.calendar_free'),
    fixed_start: expectBoolean(record.fixed_start, 'canonical schedule rules.fixed_start'),
    time_window:
      record.time_window === null || record.time_window === undefined
        ? null
        : decodeScheduleTimeWindowData(record.time_window),
    local_urgency: expectBoolean(record.local_urgency, 'canonical schedule rules.local_urgency'),
    local_defer: expectBoolean(record.local_defer, 'canonical schedule rules.local_defer'),
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

export function decodeExecutionTaskKindData(value: unknown): ExecutionTaskKindData {
  return expectEnumString(value, 'execution task kind', [
    'planning',
    'implementation',
    'debugging',
    'review',
    'research',
    'documentation',
  ]);
}

export function decodeAgentProfileData(value: unknown): AgentProfileData {
  return expectEnumString(value, 'agent profile', ['budget', 'balanced', 'quality', 'inherit']);
}

export function decodeTokenBudgetClassData(value: unknown): TokenBudgetClassData {
  return expectEnumString(value, 'token budget class', ['small', 'medium', 'large', 'xlarge']);
}

export function decodeExecutionReviewGateData(value: unknown): ExecutionReviewGateData {
  return expectEnumString(value, 'execution review gate', [
    'none',
    'operator_approval',
    'operator_preview',
    'post_run_review',
  ]);
}

export function decodeRepoWorktreeRefData(value: unknown): RepoWorktreeRefData {
  const record = expectRecord(value, 'repo worktree ref');
  return {
    path: expectString(record.path, 'repo worktree ref.path'),
    label: expectString(record.label, 'repo worktree ref.label'),
    branch: expectNullableString(record.branch, 'repo worktree ref.branch'),
    head_rev: expectNullableString(record.head_rev, 'repo worktree ref.head_rev'),
  };
}

export function decodeHandoffEnvelopeData(value: unknown): HandoffEnvelopeData {
  const record = expectRecord(value, 'handoff envelope');
  return {
    task_id: expectString(record.task_id, 'handoff envelope.task_id'),
    trace_id: expectString(record.trace_id, 'handoff envelope.trace_id'),
    from_agent: expectString(record.from_agent, 'handoff envelope.from_agent'),
    to_agent: expectString(record.to_agent, 'handoff envelope.to_agent'),
    objective: expectString(record.objective, 'handoff envelope.objective'),
    inputs: decodeJsonValue(record.inputs ?? {}),
    constraints: decodeArray(record.constraints ?? [], (item) =>
      expectString(item, 'handoff envelope.constraints'),
    ),
    read_scopes: decodeArray(record.read_scopes ?? [], (item) =>
      expectString(item, 'handoff envelope.read_scopes'),
    ),
    write_scopes: decodeArray(record.write_scopes ?? [], (item) =>
      expectString(item, 'handoff envelope.write_scopes'),
    ),
    project_id: expectNullableString(record.project_id, 'handoff envelope.project_id'),
    task_kind: decodeNullable(record.task_kind, decodeExecutionTaskKindData),
    agent_profile: decodeNullable(record.agent_profile, decodeAgentProfileData),
    token_budget: decodeNullable(record.token_budget, decodeTokenBudgetClassData),
    review_gate: decodeNullable(record.review_gate, decodeExecutionReviewGateData),
    repo_root: decodeNullable(record.repo_root, decodeRepoWorktreeRefData),
    allowed_tools: decodeArray(record.allowed_tools ?? [], (item) =>
      expectString(item, 'handoff envelope.allowed_tools'),
    ),
    capability_scope: decodeJsonValue(record.capability_scope ?? {}),
    deadline: expectNullableRfc3339Timestamp(record.deadline, 'handoff envelope.deadline'),
    expected_output_schema: decodeJsonValue(record.expected_output_schema ?? {}),
  };
}

export function decodeExecutionHandoffData(value: unknown): ExecutionHandoffData {
  const record = expectRecord(value, 'execution handoff');
  return {
    handoff: decodeHandoffEnvelopeData(record.handoff),
    project_id: expectString(record.project_id, 'execution handoff.project_id'),
    task_kind: decodeExecutionTaskKindData(record.task_kind),
    agent_profile: decodeAgentProfileData(record.agent_profile),
    token_budget: decodeTokenBudgetClassData(record.token_budget),
    review_gate: decodeExecutionReviewGateData(record.review_gate),
    repo: decodeRepoWorktreeRefData(record.repo),
    notes_root: decodeProjectRootRefData(record.notes_root),
    manifest_id: expectNullableString(record.manifest_id, 'execution handoff.manifest_id'),
  };
}

export function decodeExecutionRoutingDecisionData(value: unknown): ExecutionRoutingDecisionData {
  const record = expectRecord(value, 'execution routing decision');
  return {
    task_kind: decodeExecutionTaskKindData(record.task_kind),
    agent_profile: decodeAgentProfileData(record.agent_profile),
    token_budget: decodeTokenBudgetClassData(record.token_budget),
    review_gate: decodeExecutionReviewGateData(record.review_gate),
    read_scopes: decodeArray(record.read_scopes ?? [], (item) =>
      expectString(item, 'execution routing decision.read_scopes'),
    ),
    write_scopes: decodeArray(record.write_scopes ?? [], (item) =>
      expectString(item, 'execution routing decision.write_scopes'),
    ),
    allowed_tools: decodeArray(record.allowed_tools ?? [], (item) =>
      expectString(item, 'execution routing decision.allowed_tools'),
    ),
    reasons: decodeArray(record.reasons ?? [], (item) => {
      const reason = expectRecord(item, 'execution routing reason');
      return {
        code: expectString(reason.code, 'execution routing reason.code'),
        message: expectString(reason.message, 'execution routing reason.message'),
      };
    }),
  };
}

export function decodeExecutionHandoffRecordData(value: unknown): ExecutionHandoffRecordData {
  const record = expectRecord(value, 'execution handoff record');
  return {
    id: expectString(record.id, 'execution handoff record.id'),
    project_id: expectString(record.project_id, 'execution handoff record.project_id'),
    origin_kind: expectEnumString(record.origin_kind, 'execution handoff record.origin_kind', [
      'human_to_agent',
      'agent_to_agent',
    ]),
    review_state: expectEnumString(record.review_state, 'execution handoff record.review_state', [
      'pending_review',
      'approved',
      'rejected',
    ]),
    handoff: decodeExecutionHandoffData(record.handoff),
    routing: decodeExecutionRoutingDecisionData(record.routing),
    manifest_id: expectNullableString(record.manifest_id, 'execution handoff record.manifest_id'),
    requested_by: expectString(record.requested_by, 'execution handoff record.requested_by'),
    reviewed_by: expectNullableString(record.reviewed_by, 'execution handoff record.reviewed_by'),
    decision_reason: expectNullableString(
      record.decision_reason,
      'execution handoff record.decision_reason',
    ),
    reviewed_at: expectNullableRfc3339Timestamp(
      record.reviewed_at,
      'execution handoff record.reviewed_at',
    ),
    launched_at: expectNullableRfc3339Timestamp(
      record.launched_at,
      'execution handoff record.launched_at',
    ),
    created_at: expectRfc3339Timestamp(record.created_at, 'execution handoff record.created_at'),
    updated_at: expectRfc3339Timestamp(record.updated_at, 'execution handoff record.updated_at'),
  };
}

export function decodeExecutionLaunchPreviewData(value: unknown): ExecutionLaunchPreviewData {
  const record = expectRecord(value, 'execution launch preview');
  return {
    handoff_id: expectString(record.handoff_id, 'execution launch preview.handoff_id'),
    review_state: expectEnumString(
      record.review_state,
      'execution launch preview.review_state',
      ['pending_review', 'approved', 'rejected'],
    ),
    launch_ready: expectBoolean(record.launch_ready, 'execution launch preview.launch_ready'),
    blockers: decodeArray(record.blockers ?? [], (item) =>
      expectString(item, 'execution launch preview.blockers'),
    ),
    handoff: decodeExecutionHandoffData(record.handoff),
    routing: decodeExecutionRoutingDecisionData(record.routing),
  };
}

export function decodeAgentBlockerData(value: unknown): AgentBlockerData {
  const record = expectRecord(value, 'agent blocker');
  return {
    code: expectString(record.code, 'agent blocker.code'),
    message: expectString(record.message, 'agent blocker.message'),
    escalation_hint: expectNullableString(record.escalation_hint, 'agent blocker.escalation_hint'),
  };
}

export function decodeAgentCapabilityGroupKindData(value: unknown): AgentCapabilityGroupKindData {
  return expectEnumString(value, 'agent capability group kind', [
    'read_context',
    'review_actions',
    'mutation_actions',
  ]);
}

export function decodeAgentCapabilityEntryData(value: unknown): AgentCapabilityEntryData {
  const record = expectRecord(value, 'agent capability entry');
  return {
    key: expectString(record.key, 'agent capability entry.key'),
    label: expectString(record.label, 'agent capability entry.label'),
    summary: expectString(record.summary, 'agent capability entry.summary'),
    available: expectBoolean(record.available, 'agent capability entry.available'),
    blocked_reason: decodeNullable(record.blocked_reason, decodeAgentBlockerData),
    requires_review_gate: decodeNullable(
      record.requires_review_gate,
      decodeExecutionReviewGateData,
    ),
    requires_writeback_enabled: expectBoolean(
      record.requires_writeback_enabled,
      'agent capability entry.requires_writeback_enabled',
    ),
  };
}

export function decodeAgentCapabilityGroupData(value: unknown): AgentCapabilityGroupData {
  const record = expectRecord(value, 'agent capability group');
  return {
    kind: decodeAgentCapabilityGroupKindData(record.kind),
    label: expectString(record.label, 'agent capability group.label'),
    entries: decodeArray(record.entries ?? [], decodeAgentCapabilityEntryData),
  };
}

export function decodeAgentCapabilitySummaryData(value: unknown): AgentCapabilitySummaryData {
  const record = expectRecord(value, 'agent capability summary');
  return {
    groups: decodeArray(record.groups ?? [], decodeAgentCapabilityGroupData),
  };
}

export function decodeAgentContextRefData(value: unknown): AgentContextRefData {
  const record = expectRecord(value, 'agent context ref');
  return {
    computed_at: expectUnixSeconds(record.computed_at, 'agent context ref.computed_at'),
    mode: expectNullableString(record.mode, 'agent context ref.mode'),
    morning_state: expectNullableString(record.morning_state, 'agent context ref.morning_state'),
    current_context_path: expectString(
      record.current_context_path,
      'agent context ref.current_context_path',
    ),
    explain_context_path: expectString(
      record.explain_context_path,
      'agent context ref.explain_context_path',
    ),
    explain_drift_path: expectString(
      record.explain_drift_path,
      'agent context ref.explain_drift_path',
    ),
  };
}

export function decodeAgentReviewObligationsData(value: unknown): AgentReviewObligationsData {
  const record = expectRecord(value, 'agent review obligations');
  return {
    review_snapshot: decodeReviewSnapshotData(record.review_snapshot ?? {}),
    pending_writebacks: decodeArray(record.pending_writebacks ?? [], decodeWritebackOperationData),
    conflicts: decodeArray(record.conflicts ?? [], decodeConflictCaseData),
    pending_execution_handoffs: decodeArray(
      record.pending_execution_handoffs ?? [],
      decodeExecutionHandoffRecordData,
    ),
  };
}

export function decodeAgentGroundingPackData(value: unknown): AgentGroundingPackData {
  const record = expectRecord(value, 'agent grounding pack');
  return {
    generated_at: expectUnixSeconds(record.generated_at, 'agent grounding pack.generated_at'),
    now: decodeNowData(record.now),
    current_context: decodeNullable(record.current_context, decodeAgentContextRefData),
    projects: decodeArray(record.projects ?? [], decodeProjectRecordData),
    people: decodeArray(record.people ?? [], decodePersonRecordData),
    commitments: decodeArray(record.commitments ?? [], decodeCommitmentData),
    review: decodeAgentReviewObligationsData(record.review ?? {}),
  };
}

export function decodeAgentInspectExplainabilityData(
  value: unknown,
): AgentInspectExplainabilityData {
  const record = expectRecord(value, 'agent inspect explainability');
  return {
    persisted_record_kinds: decodeArray(record.persisted_record_kinds ?? [], (item) =>
      expectString(item, 'agent inspect explainability.persisted_record_kinds'),
    ),
    supporting_paths: decodeArray(record.supporting_paths ?? [], (item) =>
      expectString(item, 'agent inspect explainability.supporting_paths'),
    ),
    raw_context_json_supporting_only: expectBoolean(
      record.raw_context_json_supporting_only,
      'agent inspect explainability.raw_context_json_supporting_only',
    ),
  };
}

export function decodeAgentInspectData(value: unknown): AgentInspectData {
  const record = expectRecord(value, 'agent inspect');
  return {
    grounding: decodeAgentGroundingPackData(record.grounding),
    capabilities: decodeAgentCapabilitySummaryData(record.capabilities),
    blockers: decodeArray(record.blockers ?? [], decodeAgentBlockerData),
    explainability: decodeAgentInspectExplainabilityData(record.explainability),
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
          ] as const),
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
    'recovery',
    'intervention',
    'check_in',
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

export function decodeActionThreadRouteTargetData(value: unknown): ActionThreadRouteTargetData {
  return expectEnumString(value, 'action thread route target', ['existing_thread', 'filtered_threads']);
}

export function decodeActionThreadRouteData(value: unknown): ActionThreadRouteData {
  const record = expectRecord(value, 'action thread route');
  return {
    target: decodeActionThreadRouteTargetData(record.target),
    label: expectString(record.label, 'action thread route.label'),
    thread_id: expectNullableString(record.thread_id, 'action thread route.thread_id'),
    thread_type: expectNullableString(record.thread_type, 'action thread route.thread_type'),
    project_id: expectNullableString(record.project_id, 'action thread route.project_id'),
  };
}

export function decodeActionItemData(value: unknown): ActionItemData {
  const record = expectRecord(value, 'action item');
  return {
    id: expectString(record.id, 'action item.id'),
    surface: decodeActionSurfaceData(record.surface),
    kind: decodeActionKindData(record.kind),
    permission_mode: decodeActionPermissionModeData(record.permission_mode),
    scope_affinity: decodeActionScopeAffinityData(record.scope_affinity),
    title: expectString(record.title, 'action item.title'),
    summary: expectString(record.summary, 'action item.summary'),
    project_id: expectNullableString(record.project_id, 'action item.project_id'),
    project_label: expectNullableString(record.project_label, 'action item.project_label'),
    project_family:
      record.project_family == null ? null : decodeProjectFamilyData(record.project_family),
    state: decodeActionStateData(record.state),
    rank: expectNumber(record.rank, 'action item.rank'),
    surfaced_at: expectRfc3339Timestamp(record.surfaced_at, 'action item.surfaced_at'),
    snoozed_until: expectNullableRfc3339Timestamp(
      record.snoozed_until,
      'action item.snoozed_until',
    ),
    evidence: decodeArray(record.evidence ?? [], decodeActionEvidenceRefData),
    thread_route: decodeNullable(record.thread_route ?? null, decodeActionThreadRouteData),
  };
}

export function decodeActionPermissionModeData(value: unknown): ActionPermissionModeData {
  const mode = expectString(value, 'action item.permission_mode');
  switch (mode) {
    case 'auto_allowed':
    case 'user_confirm':
    case 'blocked':
    case 'unavailable':
      return mode;
    default:
      throw new Error(`Unsupported action permission mode: ${mode}`);
  }
}

export function decodeActionScopeAffinityData(value: unknown): ActionScopeAffinityData {
  const affinity = expectString(value, 'action item.scope_affinity');
  switch (affinity) {
    case 'global':
    case 'project':
    case 'thread':
    case 'connector':
    case 'daily_loop':
      return affinity;
    default:
      throw new Error(`Unsupported action scope affinity: ${affinity}`);
  }
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
    pending_execution_reviews: expectNumber(
      record.pending_execution_reviews ?? 0,
      'review snapshot.pending_execution_reviews',
    ),
  };
}

export function decodeTrustReadinessFacetData(value: unknown): TrustReadinessFacetData {
  const record = expectRecord(value, 'trust readiness facet');
  return {
    level: expectString(record.level, 'trust readiness facet.level'),
    label: expectString(record.label, 'trust readiness facet.label'),
    detail: expectString(record.detail, 'trust readiness facet.detail'),
  };
}

export function decodeTrustReadinessReviewData(value: unknown): TrustReadinessReviewData {
  const record = expectRecord(value, 'trust readiness review');
  return {
    open_action_count: expectNumber(
      record.open_action_count,
      'trust readiness review.open_action_count',
    ),
    pending_execution_reviews: expectNumber(
      record.pending_execution_reviews ?? 0,
      'trust readiness review.pending_execution_reviews',
    ),
    pending_writeback_count: expectNumber(
      record.pending_writeback_count,
      'trust readiness review.pending_writeback_count',
    ),
    conflict_count: expectNumber(record.conflict_count, 'trust readiness review.conflict_count'),
  };
}

export function decodeTrustReadinessData(value: unknown): TrustReadinessData {
  const record = expectRecord(value, 'trust readiness');
  return {
    level: expectString(record.level, 'trust readiness.level'),
    headline: expectString(record.headline, 'trust readiness.headline'),
    summary: expectString(record.summary, 'trust readiness.summary'),
    backup: decodeTrustReadinessFacetData(record.backup),
    freshness: decodeTrustReadinessFacetData(record.freshness),
    review: decodeTrustReadinessReviewData(record.review),
    guidance: decodeArray(record.guidance ?? [], (item) =>
      expectString(item, 'trust readiness.guidance'),
    ),
    follow_through: decodeArray(record.follow_through ?? [], decodeActionItemData),
  };
}

export function decodeCheckInSourceKindData(value: unknown): CheckInSourceKindData {
  return expectEnumString(value, 'check-in source kind', ['daily_loop']);
}

export function decodeCheckInSubmitTargetKindData(value: unknown): CheckInSubmitTargetKindData {
  return expectEnumString(value, 'check-in submit target kind', ['daily_loop_turn']);
}

export function decodeCheckInEscalationTargetData(value: unknown): CheckInEscalationTargetData {
  return expectEnumString(value, 'check-in escalation target', ['threads']);
}

export function decodeCheckInSubmitTargetData(value: unknown): CheckInSubmitTargetData {
  const record = expectRecord(value, 'check-in submit target');
  return {
    kind: decodeCheckInSubmitTargetKindData(record.kind),
    reference_id: expectString(record.reference_id, 'check-in submit target.reference_id'),
  };
}

export function decodeCheckInEscalationData(value: unknown): CheckInEscalationData {
  const record = expectRecord(value, 'check-in escalation');
  return {
    target: decodeCheckInEscalationTargetData(record.target),
    label: expectString(record.label, 'check-in escalation.label'),
    thread_id: expectNullableString(record.thread_id ?? null, 'check-in escalation.thread_id'),
  };
}

export function decodeCheckInTransitionKindData(value: unknown): CheckInTransitionKindData {
  return expectEnumString(value, 'check-in transition kind', ['submit', 'bypass', 'escalate']);
}

export function decodeCheckInTransitionTargetKindData(
  value: unknown,
): CheckInTransitionTargetKindData {
  return expectEnumString(value, 'check-in transition target', ['daily_loop_turn', 'threads']);
}

export function decodeCheckInTransitionData(value: unknown): CheckInTransitionData {
  const record = expectRecord(value, 'check-in transition');
  return {
    kind: decodeCheckInTransitionKindData(record.kind),
    label: expectString(record.label, 'check-in transition.label'),
    target: decodeCheckInTransitionTargetKindData(record.target),
    reference_id: expectNullableString(record.reference_id ?? null, 'check-in transition.reference_id'),
    requires_response: expectBoolean(
      record.requires_response,
      'check-in transition.requires_response',
    ),
    requires_note: expectBoolean(record.requires_note, 'check-in transition.requires_note'),
  };
}

export function decodeCheckInCardData(value: unknown): CheckInCardData {
  const record = expectRecord(value, 'check-in card');
  return {
    id: expectString(record.id, 'check-in card.id'),
    source_kind: decodeCheckInSourceKindData(record.source_kind),
    phase: decodeDailyLoopPhaseData(record.phase),
    session_id: expectString(record.session_id, 'check-in card.session_id'),
    title: expectString(record.title, 'check-in card.title'),
    summary: expectString(record.summary, 'check-in card.summary'),
    prompt_id: expectString(record.prompt_id, 'check-in card.prompt_id'),
    prompt_text: expectString(record.prompt_text, 'check-in card.prompt_text'),
    suggested_action_label: expectNullableString(
      record.suggested_action_label,
      'check-in card.suggested_action_label',
    ),
    suggested_response: expectNullableString(
      record.suggested_response,
      'check-in card.suggested_response',
    ),
    allow_skip: expectBoolean(record.allow_skip, 'check-in card.allow_skip'),
    blocking: expectBoolean(record.blocking, 'check-in card.blocking'),
    submit_target: decodeCheckInSubmitTargetData(record.submit_target),
    escalation: decodeNullable(record.escalation ?? null, decodeCheckInEscalationData),
    transitions: decodeArray(record.transitions ?? [], decodeCheckInTransitionData),
  };
}

export function decodeReflowTriggerKindData(value: unknown): ReflowTriggerKindData {
  return expectEnumString(value, 'reflow trigger', [
    'stale_schedule',
    'missed_event',
    'slipped_planned_block',
    'major_sync_change',
    'task_no_longer_fits',
  ]);
}

export function decodeReflowSeverityData(value: unknown): ReflowSeverityData {
  return expectEnumString(value, 'reflow severity', ['medium', 'high', 'critical']);
}

export function decodeReflowAcceptModeData(value: unknown): ReflowAcceptModeData {
  return expectEnumString(value, 'reflow accept mode', ['direct_accept', 'confirm_required']);
}

export function decodeReflowTransitionKindData(value: unknown): ReflowTransitionKindData {
  return expectEnumString(value, 'reflow transition kind', ['accept', 'edit']);
}

export function decodeReflowTransitionTargetKindData(
  value: unknown,
): ReflowTransitionTargetKindData {
  return expectEnumString(value, 'reflow transition target', ['apply_suggestion', 'threads']);
}

export function decodeReflowEditTargetData(value: unknown): ReflowEditTargetData {
  const record = expectRecord(value, 'reflow edit target');
  return {
    target: decodeCheckInEscalationTargetData(record.target),
    label: expectString(record.label, 'reflow edit target.label'),
  };
}

export function decodeReflowChangeKindData(value: unknown): ReflowChangeKindData {
  return expectEnumString(value, 'reflow change kind', [
    'moved',
    'unscheduled',
    'needs_judgment',
  ]);
}

export function decodeScheduleRuleFacetKindData(value: unknown): ScheduleRuleFacetKindData {
  return expectEnumString(value, 'schedule rule facet kind', [
    'block_target',
    'duration',
    'calendar_free',
    'fixed_start',
    'time_window',
    'local_urgency',
    'local_defer',
  ]);
}

export function decodeScheduleRuleFacetData(value: unknown): ScheduleRuleFacetData {
  const record = expectRecord(value, 'schedule rule facet');
  return {
    kind: decodeScheduleRuleFacetKindData(record.kind),
    label: expectString(record.label, 'schedule rule facet.label'),
    detail: expectNullableString(record.detail ?? null, 'schedule rule facet.detail'),
  };
}

export function decodeReflowChangeData(value: unknown): ReflowChangeData {
  const record = expectRecord(value, 'reflow change');
  return {
    kind: decodeReflowChangeKindData(record.kind),
    title: expectString(record.title, 'reflow change.title'),
    detail: expectString(record.detail, 'reflow change.detail'),
    project_label: expectNullableString(record.project_label ?? null, 'reflow change.project_label'),
    scheduled_start_ts: expectNullableUnixSeconds(
      record.scheduled_start_ts ?? null,
      'reflow change.scheduled_start_ts',
    ),
  };
}

export function decodeReflowProposalData(value: unknown): ReflowProposalData {
  const record = expectRecord(value, 'reflow proposal');
  return {
    headline: expectString(record.headline, 'reflow proposal.headline'),
    summary: expectString(record.summary, 'reflow proposal.summary'),
    moved_count: expectNumber(record.moved_count, 'reflow proposal.moved_count'),
    unscheduled_count: expectNumber(
      record.unscheduled_count,
      'reflow proposal.unscheduled_count',
    ),
    needs_judgment_count: expectNumber(
      record.needs_judgment_count,
      'reflow proposal.needs_judgment_count',
    ),
    changes: decodeArray(record.changes ?? [], decodeReflowChangeData),
    rule_facets: decodeArray(record.rule_facets ?? [], decodeScheduleRuleFacetData),
  };
}

export function decodeReflowTransitionData(value: unknown): ReflowTransitionData {
  const record = expectRecord(value, 'reflow transition');
  return {
    kind: decodeReflowTransitionKindData(record.kind),
    label: expectString(record.label, 'reflow transition.label'),
    target: decodeReflowTransitionTargetKindData(record.target),
    confirm_required: expectBoolean(
      record.confirm_required,
      'reflow transition.confirm_required',
    ),
  };
}

export function decodeReflowCardData(value: unknown): ReflowCardData {
  const record = expectRecord(value, 'reflow card');
  return {
    id: expectString(record.id, 'reflow card.id'),
    title: expectString(record.title, 'reflow card.title'),
    summary: expectString(record.summary, 'reflow card.summary'),
    trigger: decodeReflowTriggerKindData(record.trigger),
    severity: decodeReflowSeverityData(record.severity),
    accept_mode: decodeReflowAcceptModeData(record.accept_mode),
    suggested_action_label: expectString(
      record.suggested_action_label,
      'reflow card.suggested_action_label',
    ),
    preview_lines: decodeArray(record.preview_lines ?? [], (item) =>
      expectString(item, 'reflow card.preview_lines'),
    ),
    edit_target: decodeReflowEditTargetData(record.edit_target),
    proposal: decodeNullable(record.proposal ?? null, decodeReflowProposalData),
    transitions: decodeArray(record.transitions ?? [], decodeReflowTransitionData),
  };
}

export function decodeCurrentContextReflowStatusKindData(
  value: unknown,
): CurrentContextReflowStatusKindData {
  return expectEnumString(value, 'reflow status kind', ['applied', 'editing']);
}

export function decodeCurrentContextReflowStatusData(
  value: unknown,
): CurrentContextReflowStatusData {
  const record = expectRecord(value, 'reflow status');
  return {
    kind: decodeCurrentContextReflowStatusKindData(record.kind),
    trigger: decodeReflowTriggerKindData(record.trigger),
    severity: decodeReflowSeverityData(record.severity),
    headline: expectString(record.headline, 'reflow status.headline'),
    detail: expectString(record.detail, 'reflow status.detail'),
    recorded_at: expectNumber(record.recorded_at, 'reflow status.recorded_at'),
    preview_lines: decodeArray(record.preview_lines ?? [], (item) =>
      expectString(item, 'reflow status.preview_lines'),
    ),
    thread_id: decodeNullable(record.thread_id ?? null, (item) =>
      expectString(item, 'reflow status.thread_id'),
    ),
  };
}

export function decodeDayPlanChangeKindData(value: unknown): DayPlanChangeKindData {
  return expectEnumString(value, 'day plan change kind', [
    'scheduled',
    'deferred',
    'did_not_fit',
    'needs_judgment',
  ]);
}

export function decodeRoutineBlockSourceKindData(value: unknown): RoutineBlockSourceKindData {
  return expectEnumString(value, 'routine block source', [
    'operator_declared',
    'inferred',
    'imported',
  ]);
}

export function decodeRoutineBlockData(value: unknown): RoutineBlockData {
  const record = expectRecord(value, 'routine block');
  return {
    id: expectString(record.id, 'routine block.id'),
    label: expectString(record.label, 'routine block.label'),
    source: decodeRoutineBlockSourceKindData(record.source),
    start_ts: expectUnixSeconds(record.start_ts, 'routine block.start_ts'),
    end_ts: expectUnixSeconds(record.end_ts, 'routine block.end_ts'),
    protected: expectBoolean(record.protected, 'routine block.protected'),
  };
}

export function decodeDayPlanChangeData(value: unknown): DayPlanChangeData {
  const record = expectRecord(value, 'day plan change');
  return {
    kind: decodeDayPlanChangeKindData(record.kind),
    title: expectString(record.title, 'day plan change.title'),
    detail: expectString(record.detail, 'day plan change.detail'),
    project_label: expectNullableString(
      record.project_label ?? null,
      'day plan change.project_label',
    ),
    scheduled_start_ts: expectNullableUnixSeconds(
      record.scheduled_start_ts ?? null,
      'day plan change.scheduled_start_ts',
    ),
    rule_facets: decodeArray(record.rule_facets ?? [], decodeScheduleRuleFacetData),
  };
}

export function decodeDayPlanProposalData(value: unknown): DayPlanProposalData {
  const record = expectRecord(value, 'day plan proposal');
  return {
    headline: expectString(record.headline, 'day plan proposal.headline'),
    summary: expectString(record.summary, 'day plan proposal.summary'),
    scheduled_count: expectNumber(record.scheduled_count, 'day plan proposal.scheduled_count'),
    deferred_count: expectNumber(record.deferred_count, 'day plan proposal.deferred_count'),
    did_not_fit_count: expectNumber(
      record.did_not_fit_count,
      'day plan proposal.did_not_fit_count',
    ),
    needs_judgment_count: expectNumber(
      record.needs_judgment_count,
      'day plan proposal.needs_judgment_count',
    ),
    changes: decodeArray(record.changes ?? [], decodeDayPlanChangeData),
    routine_blocks: decodeArray(record.routine_blocks ?? [], decodeRoutineBlockData),
  };
}

export function decodeDurableRoutineBlockData(value: unknown): DurableRoutineBlockData {
  const record = expectRecord(value, 'durable routine block');
  return {
    id: expectString(record.id, 'durable routine block.id'),
    label: expectString(record.label, 'durable routine block.label'),
    source: decodeRoutineBlockSourceKindData(record.source),
    local_timezone: expectString(record.local_timezone, 'durable routine block.local_timezone'),
    start_local_time: expectString(
      record.start_local_time,
      'durable routine block.start_local_time',
    ),
    end_local_time: expectString(record.end_local_time, 'durable routine block.end_local_time'),
    days_of_week: decodeArray(record.days_of_week ?? [], (item) =>
      expectNumber(item, 'durable routine block.days_of_week'),
    ),
    protected: expectBoolean(record.protected, 'durable routine block.protected'),
    active: expectBoolean(record.active, 'durable routine block.active'),
  };
}

export function decodePlanningConstraintKindData(value: unknown): PlanningConstraintKindData {
  return expectEnumString(value, 'planning constraint kind', [
    'max_scheduled_items',
    'reserve_buffer_before_calendar',
    'reserve_buffer_after_calendar',
    'default_time_window',
    'require_judgment_for_overflow',
  ]);
}

export function decodePlanningConstraintData(value: unknown): PlanningConstraintData {
  const record = expectRecord(value, 'planning constraint');
  return {
    id: expectString(record.id, 'planning constraint.id'),
    label: expectString(record.label, 'planning constraint.label'),
    kind: decodePlanningConstraintKindData(record.kind),
    detail: expectNullableString(record.detail ?? null, 'planning constraint.detail'),
    time_window: decodeNullable(record.time_window, decodeScheduleTimeWindowData),
    minutes: expectNullableNumber(record.minutes ?? null, 'planning constraint.minutes'),
    max_items: expectNullableNumber(record.max_items ?? null, 'planning constraint.max_items'),
    active: expectBoolean(record.active, 'planning constraint.active'),
  };
}

export function decodeRoutinePlanningProfileData(value: unknown): RoutinePlanningProfileData {
  const record = expectRecord(value, 'routine planning profile');
  return {
    routine_blocks: decodeArray(record.routine_blocks ?? [], decodeDurableRoutineBlockData),
    planning_constraints: decodeArray(
      record.planning_constraints ?? [],
      decodePlanningConstraintData,
    ),
  };
}

export function decodePlanningProfileRemoveTargetData(
  value: unknown,
): PlanningProfileRemoveTargetData {
  const record = expectRecord(value, 'planning profile remove target');
  return {
    id: expectString(record.id, 'planning profile remove target.id'),
  };
}

export function decodePlanningProfileMutationData(value: unknown): PlanningProfileMutationData {
  const record = expectRecord(value, 'planning profile mutation');
  const kind = expectEnumString(record.kind, 'planning profile mutation.kind', [
    'upsert_routine_block',
    'remove_routine_block',
    'upsert_planning_constraint',
    'remove_planning_constraint',
  ]);
  const data = record.data;
  switch (kind) {
    case 'upsert_routine_block':
      return { kind, data: decodeDurableRoutineBlockData(data) };
    case 'remove_routine_block':
      return { kind, data: decodePlanningProfileRemoveTargetData(data) };
    case 'upsert_planning_constraint':
      return { kind, data: decodePlanningConstraintData(data) };
    case 'remove_planning_constraint':
      return { kind, data: decodePlanningProfileRemoveTargetData(data) };
  }
}

export function decodePlanningProfileMutationRequestData(
  value: unknown,
): PlanningProfileMutationRequestData {
  const record = expectRecord(value, 'planning profile mutation request');
  return {
    mutation: decodePlanningProfileMutationData(record.mutation),
  };
}

export function decodePlanningProfileResponseData(value: unknown): PlanningProfileResponseData {
  const record = expectRecord(value, 'planning profile response');
  return {
    profile: decodeRoutinePlanningProfileData(record.profile),
    proposal_summary:
      record.proposal_summary === undefined
        ? undefined
        : decodeNullable(record.proposal_summary, decodePlanningProfileProposalSummaryData),
  };
}

export function decodePlanningProfileSurfaceData(value: unknown): PlanningProfileSurfaceData {
  return expectEnumString(value, 'planning profile surface', [
    'web_settings',
    'cli',
    'apple',
    'assistant',
    'voice',
  ]);
}

export function decodePlanningProfileContinuityData(
  value: unknown,
): PlanningProfileContinuityData {
  return expectEnumString(value, 'planning profile continuity', ['inline', 'thread']);
}

export function decodePlanningProfileEditProposalData(
  value: unknown,
): PlanningProfileEditProposalData {
  const record = expectRecord(value, 'planning profile edit proposal');
  return {
    source_surface: decodePlanningProfileSurfaceData(record.source_surface),
    state: decodeAssistantProposalStateData(record.state),
    mutation: decodePlanningProfileMutationData(record.mutation),
    summary: expectString(record.summary, 'planning profile edit proposal.summary'),
    requires_confirmation: expectBoolean(
      record.requires_confirmation,
      'planning profile edit proposal.requires_confirmation',
    ),
    continuity: decodePlanningProfileContinuityData(record.continuity),
    outcome_summary:
      record.outcome_summary === undefined
        ? undefined
        : expectNullableString(
            record.outcome_summary,
            'planning profile edit proposal.outcome_summary',
          ),
    thread_id:
      record.thread_id === undefined
        ? undefined
        : expectNullableString(record.thread_id, 'planning profile edit proposal.thread_id'),
    thread_type:
      record.thread_type === undefined
        ? undefined
        : expectNullableString(record.thread_type, 'planning profile edit proposal.thread_type'),
  };
}

export function decodePlanningProfileProposalSummaryItemData(
  value: unknown,
): PlanningProfileProposalSummaryItemData {
  const record = expectRecord(value, 'planning profile proposal summary item');
  return {
    thread_id: expectString(record.thread_id, 'planning profile proposal summary item.thread_id'),
    state: decodeAssistantProposalStateData(record.state),
    title: expectString(record.title, 'planning profile proposal summary item.title'),
    summary: expectString(record.summary, 'planning profile proposal summary item.summary'),
    outcome_summary:
      record.outcome_summary === undefined
        ? undefined
        : expectNullableString(
            record.outcome_summary,
            'planning profile proposal summary item.outcome_summary',
          ),
    updated_at: expectNumber(
      record.updated_at,
      'planning profile proposal summary item.updated_at',
    ),
  };
}

export function decodePlanningProfileProposalSummaryData(
  value: unknown,
): PlanningProfileProposalSummaryData {
  const record = expectRecord(value, 'planning profile proposal summary');
  return {
    pending_count: expectNumber(record.pending_count, 'planning profile proposal summary.pending_count'),
    latest_pending:
      record.latest_pending === undefined
        ? undefined
        : decodeNullable(
            record.latest_pending,
            decodePlanningProfileProposalSummaryItemData,
          ),
    latest_applied:
      record.latest_applied === undefined
        ? undefined
        : decodeNullable(
            record.latest_applied,
            decodePlanningProfileProposalSummaryItemData,
          ),
    latest_failed:
      record.latest_failed === undefined
        ? undefined
        : decodeNullable(
            record.latest_failed,
            decodePlanningProfileProposalSummaryItemData,
          ),
  };
}

export function decodeCommitmentSchedulingProposalSummaryItemData(
  value: unknown,
): CommitmentSchedulingProposalSummaryItemData {
  const record = expectRecord(value, 'commitment scheduling proposal summary item');
  return {
    thread_id: expectString(
      record.thread_id,
      'commitment scheduling proposal summary item.thread_id',
    ),
    state: decodeAssistantProposalStateData(record.state),
    title: expectString(
      record.title,
      'commitment scheduling proposal summary item.title',
    ),
    summary: expectString(
      record.summary,
      'commitment scheduling proposal summary item.summary',
    ),
    outcome_summary:
      record.outcome_summary === undefined
        ? undefined
        : expectNullableString(
            record.outcome_summary,
            'commitment scheduling proposal summary item.outcome_summary',
          ),
    updated_at: expectNumber(
      record.updated_at,
      'commitment scheduling proposal summary item.updated_at',
    ),
  };
}

export function decodeCommitmentSchedulingProposalSummaryData(
  value: unknown,
): CommitmentSchedulingProposalSummaryData {
  const record = expectRecord(value, 'commitment scheduling proposal summary');
  return {
    pending_count: expectNumber(
      record.pending_count,
      'commitment scheduling proposal summary.pending_count',
    ),
    latest_pending:
      record.latest_pending === undefined
        ? undefined
        : decodeNullable(
            record.latest_pending,
            decodeCommitmentSchedulingProposalSummaryItemData,
          ),
    latest_applied:
      record.latest_applied === undefined
        ? undefined
        : decodeNullable(
            record.latest_applied,
            decodeCommitmentSchedulingProposalSummaryItemData,
          ),
    latest_failed:
      record.latest_failed === undefined
        ? undefined
        : decodeNullable(
            record.latest_failed,
            decodeCommitmentSchedulingProposalSummaryItemData,
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
    suggested_targets: decodeArray(
      record.suggested_targets ?? [],
      decodeLinkTargetSuggestionData,
    ),
  };
}

export function decodeLinkTargetSuggestionData(value: unknown): LinkTargetSuggestionData {
  const record = expectRecord(value, 'link target suggestion');
  return {
    label: expectString(record.label, 'link target suggestion.label'),
    base_url: expectString(record.base_url, 'link target suggestion.base_url'),
    transport_hint: expectString(record.transport_hint, 'link target suggestion.transport_hint'),
    recommended: expectBoolean(record.recommended, 'link target suggestion.recommended'),
    redeem_command_hint: expectString(
      record.redeem_command_hint,
      'link target suggestion.redeem_command_hint',
    ),
  };
}

export function decodeLinkingPromptData(value: unknown): LinkingPromptData {
  const record = expectRecord(value, 'linking prompt');
  return {
    target_node_id: expectString(record.target_node_id, 'linking prompt.target_node_id'),
    target_node_display_name: expectNullableString(
      record.target_node_display_name,
      'linking prompt.target_node_display_name',
    ),
    issued_by_node_id: expectString(record.issued_by_node_id, 'linking prompt.issued_by_node_id'),
    issued_by_node_display_name: expectNullableString(
      record.issued_by_node_display_name,
      'linking prompt.issued_by_node_display_name',
    ),
    issued_at: expectRfc3339Timestamp(record.issued_at, 'linking prompt.issued_at'),
    expires_at: expectRfc3339Timestamp(record.expires_at, 'linking prompt.expires_at'),
    scopes: decodeLinkScopeData(record.scopes),
    issuer_sync_base_url:
      record.issuer_sync_base_url === undefined
        ? ''
        : expectString(record.issuer_sync_base_url, 'linking prompt.issuer_sync_base_url'),
    issuer_sync_transport:
      record.issuer_sync_transport === undefined
        ? ''
        : expectString(record.issuer_sync_transport, 'linking prompt.issuer_sync_transport'),
    issuer_tailscale_base_url:
      record.issuer_tailscale_base_url === undefined
        ? null
        : expectNullableString(
            record.issuer_tailscale_base_url,
            'linking prompt.issuer_tailscale_base_url',
          ),
    issuer_lan_base_url:
      record.issuer_lan_base_url === undefined
        ? null
        : expectNullableString(record.issuer_lan_base_url, 'linking prompt.issuer_lan_base_url'),
    issuer_localhost_base_url:
      record.issuer_localhost_base_url === undefined
        ? null
        : expectNullableString(
            record.issuer_localhost_base_url,
            'linking prompt.issuer_localhost_base_url',
          ),
    issuer_public_base_url:
      record.issuer_public_base_url === undefined
        ? null
        : expectNullableString(
            record.issuer_public_base_url,
            'linking prompt.issuer_public_base_url',
          ),
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
    sync_base_url:
      record.sync_base_url === undefined
        ? null
        : expectNullableString(record.sync_base_url, 'linked node.sync_base_url'),
    tailscale_base_url:
      record.tailscale_base_url === undefined
        ? null
        : expectNullableString(record.tailscale_base_url, 'linked node.tailscale_base_url'),
    lan_base_url:
      record.lan_base_url === undefined
        ? null
        : expectNullableString(record.lan_base_url, 'linked node.lan_base_url'),
    localhost_base_url:
      record.localhost_base_url === undefined
        ? null
        : expectNullableString(record.localhost_base_url, 'linked node.localhost_base_url'),
    public_base_url:
      record.public_base_url === undefined
        ? null
        : expectNullableString(record.public_base_url, 'linked node.public_base_url'),
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
    event_id: expectNullableString(record.event_id ?? null, 'now event.event_id'),
    calendar_id: expectNullableString(record.calendar_id ?? null, 'now event.calendar_id'),
    calendar_name: expectNullableString(record.calendar_name ?? null, 'now event.calendar_name'),
    title: expectString(record.title, 'now event.title'),
    start_ts: expectUnixSeconds(record.start_ts, 'now event.start_ts'),
    end_ts: expectNullableUnixSeconds(record.end_ts, 'now event.end_ts'),
    all_day: expectBoolean(record.all_day ?? false, 'now event.all_day'),
    event_url: expectNullableString(record.event_url ?? null, 'now event.event_url'),
    attachment_url: expectNullableString(record.attachment_url ?? null, 'now event.attachment_url'),
    location: expectNullableString(record.location, 'now event.location'),
    notes: expectNullableString(record.notes ?? null, 'now event.notes'),
    attendees: decodeArray(record.attendees ?? [], (item) => expectString(item, 'now event.attendees[]')),
    video_url: expectNullableString(record.video_url ?? null, 'now event.video_url'),
    video_provider: expectNullableString(record.video_provider ?? null, 'now event.video_provider'),
    prep_minutes: expectNullableNumber(record.prep_minutes, 'now event.prep_minutes'),
    travel_minutes: expectNullableNumber(record.travel_minutes, 'now event.travel_minutes'),
    leave_by_ts: expectNullableUnixSeconds(record.leave_by_ts, 'now event.leave_by_ts'),
    rescheduled: expectBoolean(record.rescheduled ?? false, 'now event.rescheduled'),
  };
}

export function decodeNowTaskData(value: unknown): NowTaskData {
  const record = expectRecord(value, 'now task');
  return {
    id: expectString(record.id, 'now task.id'),
    text: expectString(record.text, 'now task.text'),
    title: expectString(record.title ?? record.text, 'now task.title'),
    description: expectNullableString(record.description ?? null, 'now task.description'),
    tags: decodeArray(record.tags ?? [], (item) => expectString(item, 'now task.tags[]')),
    source_type: expectString(record.source_type, 'now task.source_type'),
    due_at: expectNullableRfc3339Timestamp(record.due_at, 'now task.due_at'),
    deadline: expectNullableRfc3339Timestamp(record.deadline ?? null, 'now task.deadline'),
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

function decodeNowOverviewActionData(value: unknown): NowOverviewActionData {
  const record = expectRecord(value, 'now overview action');
  return {
    kind: expectString(record.kind, 'now overview action.kind'),
    title: expectString(record.title, 'now overview action.title'),
    summary: expectString(record.summary, 'now overview action.summary'),
    reference_id: expectNullableString(record.reference_id, 'now overview action.reference_id'),
  };
}

function decodeNowOverviewTimelineEntryData(value: unknown): NowOverviewTimelineEntryData {
  const record = expectRecord(value, 'now overview timeline entry');
  return {
    kind: expectString(record.kind, 'now overview timeline entry.kind'),
    title: expectString(record.title, 'now overview timeline entry.title'),
    timestamp: expectUnixSeconds(record.timestamp, 'now overview timeline entry.timestamp'),
    detail: expectNullableString(record.detail, 'now overview timeline entry.detail'),
  };
}

function decodeNowOverviewNudgeData(value: unknown): NowOverviewNudgeData {
  const record = expectRecord(value, 'now overview nudge');
  return {
    kind: expectString(record.kind, 'now overview nudge.kind'),
    title: expectString(record.title, 'now overview nudge.title'),
    summary: expectString(record.summary, 'now overview nudge.summary'),
  };
}

function decodeNowOverviewWhyStateData(value: unknown): NowOverviewWhyStateData {
  const record = expectRecord(value, 'now overview why_state');
  return {
    label: expectString(record.label, 'now overview why_state.label'),
    detail: expectString(record.detail, 'now overview why_state.detail'),
  };
}

function decodeNowOverviewSuggestionData(value: unknown): NowOverviewSuggestionData {
  const record = expectRecord(value, 'now overview suggestion');
  return {
    id: expectString(record.id, 'now overview suggestion.id'),
    kind: expectString(record.kind, 'now overview suggestion.kind'),
    title: expectString(record.title, 'now overview suggestion.title'),
    summary: expectString(record.summary, 'now overview suggestion.summary'),
  };
}

function decodeNowHeaderBucketKindData(value: unknown): NowHeaderBucketKindData {
  return expectEnumString(value, 'now header bucket kind', [
    'threads_by_type',
    'needs_input',
    'new_nudges',
    'search_filter',
    'snoozed',
    'review_apply',
    'reflow',
    'follow_up',
  ]);
}

function decodeNowCountDisplayModeData(value: unknown): NowCountDisplayModeData {
  return expectEnumString(value, 'now count display mode', [
    'always_show',
    'show_nonzero',
    'hidden_until_active',
  ]);
}

function decodeNowThreadFilterTargetData(value: unknown): NowThreadFilterTargetData {
  const record = expectRecord(value, 'now thread filter target');
  return {
    bucket: decodeNowHeaderBucketKindData(record.bucket),
    thread_id: expectNullableString(record.thread_id ?? null, 'now thread filter target.thread_id'),
  };
}

function decodeNowHeaderBucketData(value: unknown): NowHeaderBucketData {
  const record = expectRecord(value, 'now header bucket');
  return {
    kind: decodeNowHeaderBucketKindData(record.kind),
    count: expectNumber(record.count, 'now header bucket.count'),
    count_display: decodeNowCountDisplayModeData(record.count_display),
    urgent: expectBoolean(record.urgent, 'now header bucket.urgent'),
    route_target: decodeNowThreadFilterTargetData(record.route_target),
  };
}

function decodeNowHeaderData(value: unknown): NowHeaderData {
  const record = expectRecord(value, 'now header');
  return {
    title: expectString(record.title, 'now header.title'),
    buckets: decodeArray(record.buckets ?? [], decodeNowHeaderBucketData),
  };
}

function decodeNowMeshSyncStateData(value: unknown): NowMeshSyncStateData {
  return expectEnumString(value, 'now mesh sync state', [
    'synced',
    'stale',
    'local_only',
    'offline',
  ]);
}

function decodeNowRepairRouteTargetData(value: unknown): NowRepairRouteTargetData {
  return expectEnumString(value, 'now repair route target', [
    'settings_sync',
    'settings_linking',
    'settings_recovery',
  ]);
}

function decodeNowRepairRouteData(value: unknown): NowRepairRouteData {
  const record = expectRecord(value, 'now repair route');
  return {
    target: decodeNowRepairRouteTargetData(record.target),
    summary: expectString(record.summary, 'now repair route.summary'),
  };
}

function decodeNowMeshSummaryData(value: unknown): NowMeshSummaryData {
  const record = expectRecord(value, 'now mesh summary');
  return {
    authority_node_id: expectString(record.authority_node_id, 'now mesh summary.authority_node_id'),
    authority_label: expectString(record.authority_label, 'now mesh summary.authority_label'),
    sync_state: decodeNowMeshSyncStateData(record.sync_state),
    linked_node_count: expectNumber(record.linked_node_count, 'now mesh summary.linked_node_count'),
    queued_write_count: expectNumber(
      record.queued_write_count,
      'now mesh summary.queued_write_count',
    ),
    last_sync_at: expectNullableUnixSeconds(
      record.last_sync_at ?? null,
      'now mesh summary.last_sync_at',
    ),
    urgent: expectBoolean(record.urgent, 'now mesh summary.urgent'),
    repair_route:
      record.repair_route === undefined
        ? undefined
        : decodeNullable(record.repair_route, decodeNowRepairRouteData),
  };
}

function decodeNowStatusRowData(value: unknown): NowStatusRowData {
  const record = expectRecord(value, 'now status row');
  return {
    date_label: expectString(record.date_label, 'now status row.date_label'),
    time_label: expectString(record.time_label, 'now status row.time_label'),
    context_label: expectString(record.context_label, 'now status row.context_label'),
    elapsed_label: expectString(record.elapsed_label, 'now status row.elapsed_label'),
  };
}

function decodeNowContextLineData(value: unknown): NowContextLineData {
  const record = expectRecord(value, 'now context line');
  return {
    text: expectString(record.text, 'now context line.text'),
    thread_id: expectNullableString(record.thread_id ?? null, 'now context line.thread_id'),
    fallback_used: expectBoolean(record.fallback_used, 'now context line.fallback_used'),
  };
}

function decodeNowNudgeBarKindData(value: unknown): NowNudgeBarKindData {
  return expectEnumString(value, 'now nudge bar kind', [
    'nudge',
    'needs_input',
    'review_request',
    'reflow_proposal',
    'thread_continuation',
    'trust_warning',
    'freshness_warning',
  ]);
}

function decodeNowNudgeActionData(value: unknown): NowNudgeActionData {
  const record = expectRecord(value, 'now nudge action');
  return {
    kind: expectString(record.kind, 'now nudge action.kind'),
    label: expectString(record.label, 'now nudge action.label'),
  };
}

function decodeNowNudgeBarData(value: unknown): NowNudgeBarData {
  const record = expectRecord(value, 'now nudge bar');
  return {
    id: expectString(record.id, 'now nudge bar.id'),
    kind: decodeNowNudgeBarKindData(record.kind),
    title: expectString(record.title, 'now nudge bar.title'),
    summary: expectString(record.summary, 'now nudge bar.summary'),
    timestamp: expectNullableNumber(record.timestamp ?? null, 'now nudge bar.timestamp'),
    urgent: expectBoolean(record.urgent, 'now nudge bar.urgent'),
    primary_thread_id: expectNullableString(
      record.primary_thread_id ?? null,
      'now nudge bar.primary_thread_id',
    ),
    actions: decodeArray(record.actions ?? [], decodeNowNudgeActionData),
  };
}

function decodeNowTaskKindData(value: unknown): NowTaskKindData {
  return expectEnumString(value, 'now task kind', ['task', 'commitment', 'event']);
}

function decodeNowTaskLaneItemData(value: unknown): NowTaskLaneItemData {
  const record = expectRecord(value, 'now task lane item');
  return {
    id: expectString(record.id, 'now task lane item.id'),
    task_kind: decodeNowTaskKindData(record.task_kind),
    text: expectString(record.text, 'now task lane item.text'),
    title: expectString(record.title ?? record.text, 'now task lane item.title'),
    description: expectNullableString(record.description ?? null, 'now task lane item.description'),
    tags: decodeArray(record.tags ?? [], (item) => expectString(item, 'now task lane item.tags[]')),
    state: expectString(record.state, 'now task lane item.state'),
    ...(record.lane === undefined
      ? {}
      : { lane: expectNullableString(record.lane, 'now task lane item.lane') }),
    ...(record.sort_order === undefined
      ? {}
      : {
          sort_order: expectNullableNumber(
            record.sort_order,
            'now task lane item.sort_order',
          ),
        }),
    project: expectNullableString(record.project ?? null, 'now task lane item.project'),
    primary_thread_id: expectNullableString(
      record.primary_thread_id ?? null,
      'now task lane item.primary_thread_id',
    ),
    due_at: expectNullableRfc3339Timestamp(record.due_at ?? null, 'now task lane item.due_at'),
    deadline: expectNullableRfc3339Timestamp(record.deadline ?? null, 'now task lane item.deadline'),
    due_label: expectNullableString(record.due_label ?? null, 'now task lane item.due_label'),
    is_overdue: expectBoolean(record.is_overdue ?? false, 'now task lane item.is_overdue'),
    deadline_label: expectNullableString(
      record.deadline_label ?? null,
      'now task lane item.deadline_label',
    ),
    deadline_passed: expectBoolean(
      record.deadline_passed ?? false,
      'now task lane item.deadline_passed',
    ),
  };
}

function decodeNowTaskLaneData(value: unknown): NowTaskLaneData {
  const record = expectRecord(value, 'now task lane');
  return {
    active: decodeNullable(record.active ?? null, decodeNowTaskLaneItemData),
    pending: decodeArray(record.pending ?? [], decodeNowTaskLaneItemData),
    ...(record.active_items === undefined
      ? {}
      : { active_items: decodeArray(record.active_items, decodeNowTaskLaneItemData) }),
    ...(record.next_up === undefined
      ? {}
      : { next_up: decodeArray(record.next_up, decodeNowTaskLaneItemData) }),
    ...(record.inbox === undefined
      ? {}
      : { inbox: decodeArray(record.inbox, decodeNowTaskLaneItemData) }),
    ...(record.if_time_allows === undefined
      ? {}
      : { if_time_allows: decodeArray(record.if_time_allows, decodeNowTaskLaneItemData) }),
    ...(record.completed === undefined
      ? {}
      : { completed: decodeArray(record.completed, decodeNowTaskLaneItemData) }),
    recent_completed: decodeArray(
      record.recent_completed ?? [],
      decodeNowTaskLaneItemData,
    ),
    overflow_count: expectNumber(record.overflow_count, 'now task lane.overflow_count'),
  };
}

function decodeNowNextUpItemData(value: unknown): NowNextUpItemData {
  const record = expectRecord(value, 'now next up item');
  return {
    kind: decodeNowTaskKindData(record.kind),
    id: expectString(record.id, 'now next up item.id'),
    title: expectString(record.title, 'now next up item.title'),
    meta: expectNullableString(record.meta ?? null, 'now next up item.meta'),
    detail: expectNullableString(record.detail ?? null, 'now next up item.detail'),
    task: decodeNullable(record.task ?? null, decodeNowTaskLaneItemData),
  };
}

function decodeWebSettingsData(value: unknown): WebSettingsData {
  const record = expectRecord(value, 'web settings');
  return {
    dense_rows: expectBoolean(record.dense_rows, 'web settings.dense_rows'),
    tabular_numbers: expectBoolean(record.tabular_numbers, 'web settings.tabular_numbers'),
    reduced_motion: expectBoolean(record.reduced_motion, 'web settings.reduced_motion'),
    strong_focus: expectBoolean(record.strong_focus, 'web settings.strong_focus'),
    docked_action_bar: expectBoolean(record.docked_action_bar, 'web settings.docked_action_bar'),
    semantic_aliases: decodeNullableRecordOfStringRecords(record.semantic_aliases ?? null, 'web settings.semantic_aliases') ?? undefined,
  };
}

function decodeNullableRecordOfStringRecords(
  value: unknown,
  label: string,
): Record<string, Record<string, string>> | null {
  if (value == null) {
    return null;
  }
  const record = expectRecord(value, label);
  return Object.fromEntries(
    Object.entries(record).map(([key, entry]) => {
      const entryRecord = expectRecord(entry, `${label}.${key}`);
      return [
        key,
        Object.fromEntries(
          Object.entries(entryRecord).map(([entryKey, entryValue]) => [
            entryKey,
            expectString(entryValue, `${label}.${key}.${entryKey}`),
          ]),
        ),
      ];
    }),
  );
}

function decodeAgentProfileSettingsData(value: unknown): AgentProfileSettingsData {
  const record = expectRecord(value, 'agent profile settings');
  return {
    role: expectNullableString(record.role ?? null, 'agent profile settings.role'),
    preferences: expectNullableString(record.preferences ?? null, 'agent profile settings.preferences'),
    constraints: expectNullableString(record.constraints ?? null, 'agent profile settings.constraints'),
    freeform: expectNullableString(record.freeform ?? null, 'agent profile settings.freeform'),
  };
}

function decodeCoreSettingsData(value: unknown): CoreSettingsData {
  const record = expectRecord(value, 'core settings');
  return {
    user_display_name: expectNullableString(record.user_display_name ?? null, 'core settings.user_display_name'),
    client_location_label: expectNullableString(record.client_location_label ?? null, 'core settings.client_location_label'),
    developer_mode: expectBoolean(record.developer_mode ?? false, 'core settings.developer_mode'),
    bypass_setup_gate: expectBoolean(record.bypass_setup_gate ?? false, 'core settings.bypass_setup_gate'),
    agent_profile: decodeAgentProfileSettingsData(record.agent_profile ?? {}),
  };
}

function decodeNowDockedInputIntentData(value: unknown): NowDockedInputIntentData {
  return expectEnumString(value, 'now docked input intent', [
    'task',
    'url',
    'question',
    'note',
    'command',
    'continuation',
    'reflection',
    'scheduling',
  ]);
}

function decodeNowDockedInputData(value: unknown): NowDockedInputData {
  const record = expectRecord(value, 'now docked input');
  return {
    supported_intents: decodeArray(
      record.supported_intents ?? [],
      decodeNowDockedInputIntentData,
    ),
    day_thread_id: expectNullableString(
      record.day_thread_id ?? null,
      'now docked input.day_thread_id',
    ),
    raw_capture_thread_id: expectNullableString(
      record.raw_capture_thread_id ?? null,
      'now docked input.raw_capture_thread_id',
    ),
  };
}

export function decodeNowData(value: unknown): NowData {
  const record = expectRecord(value, 'now data');
  const progress = expectRecord(record.progress ?? {}, 'now data.progress');
  const overview = expectRecord(record.overview, 'now data.overview');
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
    header:
      record.header === undefined
        ? undefined
        : decodeNullable(record.header, decodeNowHeaderData),
    mesh_summary:
      record.mesh_summary === undefined
        ? undefined
        : decodeNullable(record.mesh_summary, decodeNowMeshSummaryData),
    status_row:
      record.status_row === undefined
        ? undefined
        : decodeNullable(record.status_row, decodeNowStatusRowData),
    context_line:
      record.context_line === undefined
        ? undefined
        : decodeNullable(record.context_line, decodeNowContextLineData),
    nudge_bars: decodeArray(record.nudge_bars ?? [], decodeNowNudgeBarData),
    task_lane:
      record.task_lane === undefined
        ? undefined
        : decodeNullable(record.task_lane, decodeNowTaskLaneData),
    next_up_items: decodeArray(record.next_up_items ?? [], decodeNowNextUpItemData),
    progress: {
      base_count: expectNumber(progress.base_count ?? 1, 'now data.progress.base_count'),
      completed_count: expectNumber(
        progress.completed_count ?? 0,
        'now data.progress.completed_count',
      ),
      backlog_count: expectNumber(
        progress.backlog_count ?? 0,
        'now data.progress.backlog_count',
      ),
      completed_ratio: expectNumber(
        progress.completed_ratio ?? 0,
        'now data.progress.completed_ratio',
      ),
      backlog_ratio: expectNumber(
        progress.backlog_ratio ?? 0,
        'now data.progress.backlog_ratio',
      ),
    },
    docked_input:
      record.docked_input === undefined
        ? undefined
        : decodeNullable(record.docked_input, decodeNowDockedInputData),
    overview: {
      dominant_action: decodeNullable(
        overview.dominant_action ?? null,
        decodeNowOverviewActionData,
      ),
      today_timeline: decodeArray(
        overview.today_timeline ?? [],
        decodeNowOverviewTimelineEntryData,
      ),
      visible_nudge: decodeNullable(
        overview.visible_nudge ?? null,
        decodeNowOverviewNudgeData,
      ),
      why_state: decodeArray(overview.why_state ?? [], decodeNowOverviewWhyStateData),
      suggestions: decodeArray(
        overview.suggestions ?? [],
        decodeNowOverviewSuggestionData,
      ),
      decision_options: decodeArray(
        overview.decision_options ?? [],
        (item) => expectString(item, 'now data.overview.decision_options'),
      ),
    },
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
      following_day_events: decodeArray(schedule.following_day_events ?? [], decodeNowEventData),
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
      git_activity: decodeNullable(sources.git_activity ?? null, decodeNowSourceActivityData),
      health: decodeNullable(sources.health ?? null, decodeNowSourceActivityData),
      mood: decodeNullable(sources.mood ?? null, decodeNowSourceActivityData),
      pain: decodeNullable(sources.pain ?? null, decodeNowSourceActivityData),
      note_document: decodeNullable(sources.note_document ?? null, decodeNowSourceActivityData),
      assistant_message: decodeNullable(
        sources.assistant_message ?? null,
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
    trust_readiness: decodeTrustReadinessData(record.trust_readiness),
    planning_profile_summary:
      record.planning_profile_summary === undefined
        ? undefined
        : decodeNullable(
            record.planning_profile_summary,
            decodePlanningProfileProposalSummaryData,
          ),
    commitment_scheduling_summary:
      record.commitment_scheduling_summary === undefined
        ? undefined
        : decodeNullable(
            record.commitment_scheduling_summary,
            decodeCommitmentSchedulingProposalSummaryData,
          ),
    check_in: decodeNullable(record.check_in ?? null, decodeCheckInCardData),
    day_plan: decodeNullable(record.day_plan ?? null, decodeDayPlanProposalData),
    reflow: decodeNullable(record.reflow ?? null, decodeReflowCardData),
    reflow_status: decodeNullable(
      record.reflow_status ?? null,
      decodeCurrentContextReflowStatusData,
    ),
    action_items: decodeArray(record.action_items ?? [], decodeActionItemData),
    review_snapshot: decodeReviewSnapshotData(
      record.review_snapshot ?? {
        open_action_count: 0,
        triage_count: 0,
        projects_needing_review: 0,
        pending_execution_reviews: 0,
      },
    ),
    pending_writebacks: decodeArray(
      record.pending_writebacks ?? [],
      decodeWritebackOperationData,
    ),
    conflicts: decodeArray(record.conflicts ?? [], decodeConflictCaseData),
    people: decodeArray(record.people ?? [], decodePersonRecordData),
    reasons: decodeArray(record.reasons ?? [], (item) => expectString(item, 'now data.reasons')),
    debug: {
      raw_context: decodeJsonValue(debug.raw_context),
      signals_used: decodeArray(debug.signals_used ?? [], (item) => expectString(item, 'now data.debug.signals_used')),
      commitments_used: decodeArray(debug.commitments_used ?? [], (item) => expectString(item, 'now data.debug.commitments_used')),
      risk_used: decodeArray(debug.risk_used ?? [], (item) => expectString(item, 'now data.debug.risk_used')),
    },
  };
}

export function decodeDailyLoopPhaseData(value: unknown): DailyLoopPhaseData {
  return expectEnumString(value, 'daily loop phase', ['morning_overview', 'standup']);
}

export function decodeDailyLoopStatusData(value: unknown): DailyLoopStatusData {
  return expectEnumString(value, 'daily loop status', [
    'active',
    'waiting_for_input',
    'completed',
    'cancelled',
  ]);
}

export function decodeDailyLoopStartSourceData(value: unknown): DailyLoopStartSourceData {
  return expectEnumString(value, 'daily loop start source', ['manual', 'automatic']);
}

export function decodeDailyLoopSurfaceData(value: unknown): DailyLoopSurfaceData {
  return expectEnumString(value, 'daily loop surface', ['cli', 'web', 'apple_voice', 'apple_text']);
}

export function decodeDailyLoopTurnActionData(value: unknown): DailyLoopTurnActionData {
  return expectEnumString(value, 'daily loop turn action', ['submit', 'skip', 'resume']);
}

export function decodeDailyLoopTurnStateData(value: unknown): DailyLoopTurnStateData {
  return expectEnumString(value, 'daily loop turn state', [
    'in_progress',
    'waiting_for_input',
    'completed',
  ]);
}

export function decodeDailyLoopPromptKindData(value: unknown): DailyLoopPromptKindData {
  return expectEnumString(value, 'daily loop prompt kind', [
    'intent_question',
    'commitment_reduction',
    'constraint_check',
  ]);
}

export function decodeDailyStandupBucketData(value: unknown): DailyStandupBucketData {
  return expectEnumString(value, 'daily standup bucket', ['must', 'should', 'stretch']);
}

export function decodeDailyLoopStartMetadataData(value: unknown): DailyLoopStartMetadataData {
  const record = expectRecord(value, 'daily loop start metadata');
  return {
    source: decodeDailyLoopStartSourceData(record.source),
    surface: decodeDailyLoopSurfaceData(record.surface),
  };
}

export function decodeDailyLoopPromptData(value: unknown): DailyLoopPromptData {
  const record = expectRecord(value, 'daily loop prompt');
  return {
    prompt_id: expectString(record.prompt_id, 'daily loop prompt.prompt_id'),
    kind: decodeDailyLoopPromptKindData(record.kind),
    text: expectString(record.text, 'daily loop prompt.text'),
    ordinal: expectNumber(record.ordinal, 'daily loop prompt.ordinal'),
    allow_skip: expectBoolean(record.allow_skip, 'daily loop prompt.allow_skip'),
  };
}

export function decodeDailyLoopCommitmentActionData(
  value: unknown,
): DailyLoopCommitmentActionData {
  return expectEnumString(value, 'daily loop commitment action', [
    'accept',
    'defer',
    'choose',
    'close',
  ]);
}

export function decodeMorningFrictionCalloutData(value: unknown): MorningFrictionCalloutData {
  const record = expectRecord(value, 'morning friction callout');
  return {
    label: expectString(record.label, 'morning friction callout.label'),
    detail: expectString(record.detail, 'morning friction callout.detail'),
  };
}

export function decodeMorningIntentSignalData(value: unknown): MorningIntentSignalData {
  const record = expectRecord(value, 'morning intent signal');
  const kind = expectEnumString(record.kind, 'morning intent signal.kind', [
    'must_do_hint',
    'focus_intent',
    'meeting_doubt',
  ]);
  const text = expectString(record.text, `morning intent signal.${kind}.text`);
  switch (kind) {
    case 'must_do_hint':
      return { kind, text };
    case 'focus_intent':
      return { kind, text };
    case 'meeting_doubt':
      return { kind, text };
  }
}

export function decodeDailyLoopCheckInResolutionKindData(
  value: unknown,
): DailyLoopCheckInResolutionKindData {
  return expectEnumString(value, 'daily loop check-in resolution kind', [
    'submitted',
    'bypassed',
  ]);
}

export function decodeDailyLoopCheckInResolutionData(
  value: unknown,
): DailyLoopCheckInResolutionData {
  const record = expectRecord(value, 'daily loop check-in resolution');
  return {
    prompt_id: expectString(record.prompt_id, 'daily loop check-in resolution.prompt_id'),
    ordinal: expectNumber(record.ordinal, 'daily loop check-in resolution.ordinal'),
    kind: decodeDailyLoopCheckInResolutionKindData(record.kind),
    response_text: expectNullableString(
      record.response_text ?? null,
      'daily loop check-in resolution.response_text',
    ),
    note_text: expectNullableString(
      record.note_text ?? null,
      'daily loop check-in resolution.note_text',
    ),
  };
}

export function decodeDailyLoopCheckInEventData(value: unknown): DailyLoopCheckInEventData {
  const record = expectRecord(value, 'daily loop check-in event');
  return {
    event_id: expectString(record.event_id, 'daily loop check-in event.event_id'),
    session_id: expectString(record.session_id, 'daily loop check-in event.session_id'),
    prompt_id: expectString(record.prompt_id, 'daily loop check-in event.prompt_id'),
    check_in_type: expectString(record.check_in_type, 'daily loop check-in event.check_in_type'),
    session_phase: expectString(record.session_phase, 'daily loop check-in event.session_phase'),
    source: expectString(record.source, 'daily loop check-in event.source'),
    answered_at: expectNullableNumber(
      record.answered_at ?? null,
      'daily loop check-in event.answered_at',
    ),
    text: expectNullableString(record.text ?? null, 'daily loop check-in event.text'),
    scale: expectNullableNumber(record.scale ?? null, 'daily loop check-in event.scale'),
    scale_min: expectNumber(record.scale_min, 'daily loop check-in event.scale_min'),
    scale_max: expectNumber(record.scale_max, 'daily loop check-in event.scale_max'),
    keywords_json: decodeJsonValue(record.keywords_json),
    confidence: expectNullableNumber(record.confidence ?? null, 'daily loop check-in event.confidence'),
    schema_version: expectNumber(record.schema_version, 'daily loop check-in event.schema_version'),
    skipped: expectBoolean(record.skipped, 'daily loop check-in event.skipped'),
    skip_reason_code: expectNullableString(
      record.skip_reason_code ?? null,
      'daily loop check-in event.skip_reason_code',
    ),
    skip_reason_text: expectNullableString(
      record.skip_reason_text ?? null,
      'daily loop check-in event.skip_reason_text',
    ),
    replaced_by_event_id: expectNullableString(
      record.replaced_by_event_id ?? null,
      'daily loop check-in event.replaced_by_event_id',
    ),
    meta_json: decodeJsonValue(record.meta_json),
    created_at: expectNumber(record.created_at, 'daily loop check-in event.created_at'),
    updated_at: expectNumber(record.updated_at, 'daily loop check-in event.updated_at'),
    run_id: expectNullableString(record.run_id ?? null, 'daily loop check-in event.run_id'),
  };
}

export function decodeDailyLoopCheckInSkipResponseData(
  value: unknown,
): DailyLoopCheckInSkipResponseData {
  const record = expectRecord(value, 'daily loop check-in skip response');
  return {
    check_in_event_id: expectString(
      record.check_in_event_id,
      'daily loop check-in skip.response.check_in_event_id',
    ),
    session_id: expectString(record.session_id, 'daily loop check-in skip.response.session_id'),
    status: expectString(record.status, 'daily loop check-in skip.response.status'),
    supersedes_event_id: expectNullableString(
      record.supersedes_event_id ?? null,
      'daily loop check-in skip.response.supersedes_event_id',
    ),
  };
}

export function decodeMorningOverviewStateData(value: unknown): MorningOverviewStateData {
  const record = expectRecord(value, 'morning overview state');
  return {
    snapshot: expectString(record.snapshot, 'morning overview state.snapshot'),
    friction_callouts: decodeArray(
      record.friction_callouts ?? [],
      decodeMorningFrictionCalloutData,
    ),
    signals: decodeArray(record.signals ?? [], decodeMorningIntentSignalData),
    check_in_history: decodeArray(
      record.check_in_history ?? [],
      decodeDailyLoopCheckInResolutionData,
    ),
  };
}

export function decodeDailyCommitmentDraftData(value: unknown): DailyCommitmentDraftData {
  const record = expectRecord(value, 'daily commitment draft');
  return {
    title: expectString(record.title, 'daily commitment draft.title'),
    bucket: decodeDailyStandupBucketData(record.bucket),
    source_ref: expectNullableString(record.source_ref, 'daily commitment draft.source_ref'),
  };
}

export function decodeDailyDeferredTaskData(value: unknown): DailyDeferredTaskData {
  const record = expectRecord(value, 'daily deferred task');
  return {
    title: expectString(record.title, 'daily deferred task.title'),
    source_ref: expectNullableString(record.source_ref, 'daily deferred task.source_ref'),
    reason: expectString(record.reason, 'daily deferred task.reason'),
  };
}

export function decodeDailyFocusBlockProposalData(value: unknown): DailyFocusBlockProposalData {
  const record = expectRecord(value, 'daily focus block proposal');
  return {
    label: expectString(record.label, 'daily focus block proposal.label'),
    start_at: expectRfc3339Timestamp(record.start_at, 'daily focus block proposal.start_at'),
    end_at: expectRfc3339Timestamp(record.end_at, 'daily focus block proposal.end_at'),
    reason: expectString(record.reason, 'daily focus block proposal.reason'),
  };
}

export function decodeDailyStandupOutcomeData(value: unknown): DailyStandupOutcomeData {
  const record = expectRecord(value, 'daily standup outcome');
  return {
    commitments: decodeArray(record.commitments ?? [], decodeDailyCommitmentDraftData),
    deferred_tasks: decodeArray(record.deferred_tasks ?? [], decodeDailyDeferredTaskData),
    confirmed_calendar: decodeArray(record.confirmed_calendar ?? [], (item) =>
      expectString(item, 'daily standup outcome.confirmed_calendar'),
    ),
    focus_blocks: decodeArray(record.focus_blocks ?? [], decodeDailyFocusBlockProposalData),
    check_in_history: decodeArray(
      record.check_in_history ?? [],
      decodeDailyLoopCheckInResolutionData,
    ),
  };
}

export function decodeDailyLoopSessionOutcomeData(value: unknown): DailyLoopSessionOutcomeData {
  const record = expectRecord(value, 'daily loop session outcome');
  const phase = decodeDailyLoopPhaseData(record.phase);
  if (phase === 'morning_overview') {
    return {
      phase,
      signals: decodeArray(record.signals ?? [], decodeMorningIntentSignalData),
      check_in_history: decodeArray(
        record.check_in_history ?? [],
        decodeDailyLoopCheckInResolutionData,
      ),
    };
  }
  return {
    phase,
    ...decodeDailyStandupOutcomeData(record),
  };
}

export function decodeDailyLoopSessionStateData(value: unknown): DailyLoopSessionStateData {
  const record = expectRecord(value, 'daily loop session state');
  const phase = decodeDailyLoopPhaseData(record.phase);
  if (phase === 'morning_overview') {
    return {
      phase,
      ...decodeMorningOverviewStateData(record),
    };
  }
  return {
    phase,
    ...decodeDailyStandupOutcomeData(record),
  };
}

export function decodeDailyLoopSessionData(value: unknown): DailyLoopSessionData {
  const record = expectRecord(value, 'daily loop session');
  return {
    id: expectString(record.id, 'daily loop session.id'),
    session_date: expectString(record.session_date, 'daily loop session.session_date'),
    phase: decodeDailyLoopPhaseData(record.phase),
    status: decodeDailyLoopStatusData(record.status),
    start: decodeDailyLoopStartMetadataData(record.start),
    turn_state: decodeDailyLoopTurnStateData(record.turn_state),
    current_prompt: decodeNullable(record.current_prompt, decodeDailyLoopPromptData),
    continuity_summary: expectString(
      record.continuity_summary,
      'daily loop session.continuity_summary',
    ),
    allowed_actions: decodeArray(
      record.allowed_actions ?? [],
      decodeDailyLoopCommitmentActionData,
    ),
    state: decodeDailyLoopSessionStateData(record.state),
    outcome: decodeNullable(record.outcome, decodeDailyLoopSessionOutcomeData),
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

export function decodeBackupCoverageData(value: unknown): BackupCoverageData {
  if (Array.isArray(value)) {
    return {
      included: decodeArray(value, (item) => expectString(item, 'backup coverage')),
      omitted: [],
      notes: [],
    };
  }
  if (typeof value === 'string') {
    return {
      included: [],
      omitted: [],
      notes: value.trim() === '' ? [] : [value.trim()],
    };
  }
  if (typeof value !== 'object' || value === null) {
    return {
      included: [],
      omitted: [],
      notes: [],
    };
  }
  const record = expectRecord(value, 'backup coverage');
  return {
    included: decodeArray(record.included ?? [], (item) => expectString(item, 'backup coverage.included')),
    omitted: decodeArray(record.omitted ?? [], (item) => expectString(item, 'backup coverage.omitted')),
    notes: decodeArray(record.notes ?? [], (item) => expectString(item, 'backup coverage.notes')),
  };
}

export function decodeBackupVerificationData(value: unknown): BackupVerificationData {
  const record = expectRecord(value, 'backup verification');
  return {
    verified: expectBoolean(record.verified, 'backup verification.verified'),
    checksum_algorithm: expectString(
      record.checksum_algorithm,
      'backup verification.checksum_algorithm',
    ),
    checksum: expectString(record.checksum, 'backup verification.checksum'),
    checked_paths: decodeArray(
      record.checked_paths ?? [],
      (item) => expectString(item, 'backup verification.checked_paths'),
    ),
    notes: decodeArray(record.notes ?? [], (item) => expectString(item, 'backup verification.notes')),
  };
}

export function decodeBackupStatusData(value: unknown): BackupStatusData {
  const record = expectRecord(value, 'backup status');
  return {
    state: expectString(record.state, 'backup status.state') as BackupStatusStateData,
    last_backup_id: expectNullableString(record.last_backup_id, 'backup status.last_backup_id'),
    last_backup_at: expectNullableString(record.last_backup_at, 'backup status.last_backup_at'),
    output_root: expectNullableString(record.output_root, 'backup status.output_root'),
    artifact_coverage: decodeNullable(record.artifact_coverage, decodeBackupCoverageData),
    config_coverage: decodeNullable(record.config_coverage, decodeBackupCoverageData),
    verification_summary: decodeNullable(
      record.verification_summary,
      decodeBackupVerificationData,
    ),
    warnings: decodeArray(record.warnings ?? [], (item) => expectString(item, 'backup status.warnings')),
  };
}

export function decodeBackupFreshnessData(value: unknown): BackupFreshnessData {
  const record = expectRecord(value, 'backup freshness');
  return {
    state: expectString(record.state, 'backup freshness.state') as BackupFreshnessStateData,
    age_seconds: expectNullableNumber(record.age_seconds, 'backup freshness.age_seconds'),
    stale_after_seconds: expectNumber(
      record.stale_after_seconds,
      'backup freshness.stale_after_seconds',
    ),
  };
}

export function decodeBackupTrustData(value: unknown): BackupTrustData {
  const record = expectRecord(value, 'backup trust');
  return {
    level: expectString(record.level, 'backup trust.level') as BackupTrustLevelData,
    status: decodeBackupStatusData(record.status),
    freshness: decodeBackupFreshnessData(record.freshness),
    guidance: decodeArray(record.guidance ?? [], (item) => expectString(item, 'backup trust.guidance')),
  };
}

export function decodeBackupSettingsData(value: unknown): BackupSettingsData {
  const record = expectRecord(value, 'backup settings');
  return {
    default_output_root: expectString(
      record.default_output_root,
      'backup settings.default_output_root',
    ),
    trust: decodeBackupTrustData(record.trust),
  };
}

export function decodeLlmProfileSettingsData(value: unknown): LlmProfileSettingsData {
  const record = expectRecord(value, 'llm profile settings');
  return {
    id: expectString(record.id, 'llm profile settings.id'),
    provider: expectString(record.provider, 'llm profile settings.provider'),
    base_url: expectString(record.base_url, 'llm profile settings.base_url'),
    model: expectString(record.model, 'llm profile settings.model'),
    context_window:
      record.context_window === undefined
        ? null
        : expectNullableNumber(record.context_window, 'llm profile settings.context_window'),
    enabled: expectBoolean(record.enabled, 'llm profile settings.enabled'),
    editable: expectBoolean(record.editable, 'llm profile settings.editable'),
    has_api_key:
      record.has_api_key === undefined
        ? undefined
        : expectBoolean(record.has_api_key, 'llm profile settings.has_api_key'),
  };
}

export function decodeLlmProfileHealthData(value: unknown): LlmProfileHealthData {
  const record = expectRecord(value, 'llm profile health');
  return {
    profile_id: expectString(record.profile_id, 'llm profile health.profile_id'),
    healthy: expectBoolean(record.healthy, 'llm profile health.healthy'),
    message: expectString(record.message, 'llm profile health.message'),
  };
}

export function decodeLlmSettingsData(value: unknown): LlmSettingsData {
  const record = expectRecord(value, 'llm settings');
  return {
    models_dir: expectString(record.models_dir, 'llm settings.models_dir'),
    default_chat_profile_id: expectNullableString(
      record.default_chat_profile_id ?? null,
      'llm settings.default_chat_profile_id',
    ),
    fallback_chat_profile_id: expectNullableString(
      record.fallback_chat_profile_id ?? null,
      'llm settings.fallback_chat_profile_id',
    ),
    profiles: decodeArray(record.profiles ?? [], decodeLlmProfileSettingsData),
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
    writeback_enabled:
      record.writeback_enabled === undefined
        ? undefined
        : expectBoolean(record.writeback_enabled, 'settings.writeback_enabled'),
    tailscale_preferred:
      record.tailscale_preferred === undefined
        ? undefined
        : expectBoolean(record.tailscale_preferred, 'settings.tailscale_preferred'),
    tailscale_base_url:
      record.tailscale_base_url === undefined
        ? undefined
        : expectNullableString(record.tailscale_base_url, 'settings.tailscale_base_url'),
    tailscale_base_url_auto_discovered:
      record.tailscale_base_url_auto_discovered === undefined
        ? undefined
        : expectBoolean(
            record.tailscale_base_url_auto_discovered,
            'settings.tailscale_base_url_auto_discovered',
          ),
    lan_base_url:
      record.lan_base_url === undefined
        ? undefined
        : expectNullableString(record.lan_base_url, 'settings.lan_base_url'),
    lan_base_url_auto_discovered:
      record.lan_base_url_auto_discovered === undefined
        ? undefined
        : expectBoolean(
            record.lan_base_url_auto_discovered,
            'settings.lan_base_url_auto_discovered',
          ),
    llm:
      record.llm === undefined
        ? undefined
        : decodeLlmSettingsData(record.llm),
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
    backup:
      record.backup === undefined
        ? undefined
        : decodeBackupSettingsData(record.backup),
    web_settings:
      record.web_settings === undefined
        ? undefined
        : decodeWebSettingsData(record.web_settings),
    core_settings:
      record.core_settings === undefined
        ? undefined
        : decodeCoreSettingsData(record.core_settings),
  };
}

export function decodeIntegrationCalendarData(value: unknown): IntegrationCalendarData {
  const record = expectRecord(value, 'integration calendar');
  const legacySelected = record.selected === undefined
    ? undefined
    : expectBoolean(record.selected, 'integration calendar.selected');
  const syncEnabled = record.sync_enabled === undefined
    ? legacySelected
    : expectBoolean(record.sync_enabled, 'integration calendar.sync_enabled');
  return {
    id: expectString(record.id, 'integration calendar.id'),
    summary: expectString(record.summary, 'integration calendar.summary'),
    primary: expectBoolean(record.primary, 'integration calendar.primary'),
    sync_enabled: syncEnabled ?? true,
    display_enabled:
      record.display_enabled === undefined
        ? (syncEnabled ?? true)
        : expectBoolean(record.display_enabled, 'integration calendar.display_enabled'),
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

export function decodeIntegrationSourceRefData(value: unknown): IntegrationSourceRefData {
  const record = expectRecord(value, 'integration source ref');
  return {
    family: expectString(record.family, 'integration source ref.family'),
    provider_key: expectString(record.provider_key, 'integration source ref.provider_key'),
    connection_id: expectString(record.connection_id, 'integration source ref.connection_id'),
    external_id: expectString(record.external_id, 'integration source ref.external_id'),
  };
}

export function decodeWritebackTargetRefData(value: unknown): WritebackTargetRefData {
  const record = expectRecord(value, 'writeback target');
  return {
    family: expectString(record.family, 'writeback target.family'),
    provider_key: expectString(record.provider_key, 'writeback target.provider_key'),
    project_id: expectNullableString(record.project_id, 'writeback target.project_id'),
    connection_id: expectNullableString(record.connection_id, 'writeback target.connection_id'),
    external_id: expectNullableString(record.external_id, 'writeback target.external_id'),
  };
}

export function decodeWritebackOperationData(value: unknown): WritebackOperationData {
  const record = expectRecord(value, 'writeback operation');
  return {
    id: expectString(record.id, 'writeback operation.id'),
    kind: expectString(record.kind, 'writeback operation.kind'),
    risk: expectString(record.risk, 'writeback operation.risk'),
    status: expectString(record.status, 'writeback operation.status'),
    target: decodeWritebackTargetRefData(record.target),
    requested_payload: decodeJsonValue(record.requested_payload),
    result_payload:
      record.result_payload === null || record.result_payload === undefined
        ? null
        : decodeJsonValue(record.result_payload),
    provenance: decodeArray(record.provenance ?? [], decodeIntegrationSourceRefData),
    conflict_case_id: expectNullableString(record.conflict_case_id, 'writeback operation.conflict_case_id'),
    requested_by_node_id: expectString(
      record.requested_by_node_id,
      'writeback operation.requested_by_node_id',
    ),
    requested_at: expectRfc3339Timestamp(record.requested_at, 'writeback operation.requested_at'),
    applied_at: expectNullableRfc3339Timestamp(record.applied_at, 'writeback operation.applied_at'),
    updated_at: expectRfc3339Timestamp(record.updated_at, 'writeback operation.updated_at'),
  };
}

export function decodeConflictCaseData(value: unknown): ConflictCaseData {
  const record = expectRecord(value, 'conflict case');
  return {
    id: expectString(record.id, 'conflict case.id'),
    kind: expectString(record.kind, 'conflict case.kind'),
    status: expectString(record.status, 'conflict case.status'),
    target: decodeWritebackTargetRefData(record.target),
    summary: expectString(record.summary, 'conflict case.summary'),
    local_payload: decodeJsonValue(record.local_payload),
    upstream_payload:
      record.upstream_payload === null || record.upstream_payload === undefined
        ? null
        : decodeJsonValue(record.upstream_payload),
    resolution_payload:
      record.resolution_payload === null || record.resolution_payload === undefined
        ? null
        : decodeJsonValue(record.resolution_payload),
    opened_at: expectRfc3339Timestamp(record.opened_at, 'conflict case.opened_at'),
    resolved_at: expectNullableRfc3339Timestamp(record.resolved_at, 'conflict case.resolved_at'),
    updated_at: expectRfc3339Timestamp(record.updated_at, 'conflict case.updated_at'),
  };
}

export function decodePersonAliasData(value: unknown): PersonAliasData {
  const record = expectRecord(value, 'person alias');
  return {
    platform: expectString(record.platform, 'person alias.platform'),
    handle: expectString(record.handle, 'person alias.handle'),
    display: expectString(record.display, 'person alias.display'),
    source_ref: decodeNullable(record.source_ref, decodeIntegrationSourceRefData),
  };
}

export function decodePersonLinkRefData(value: unknown): PersonLinkRefData {
  const record = expectRecord(value, 'person link');
  return {
    kind: expectString(record.kind, 'person link.kind'),
    id: expectString(record.id, 'person link.id'),
    label: expectString(record.label, 'person link.label'),
  };
}

export function decodePersonRecordData(value: unknown): PersonRecordData {
  const record = expectRecord(value, 'person record');
  return {
    id: expectString(record.id, 'person record.id'),
    display_name: expectString(record.display_name, 'person record.display_name'),
    given_name: expectNullableString(record.given_name, 'person record.given_name'),
    family_name: expectNullableString(record.family_name, 'person record.family_name'),
    relationship_context: expectNullableString(
      record.relationship_context,
      'person record.relationship_context',
    ),
    birthday: expectNullableString(record.birthday, 'person record.birthday'),
    last_contacted_at: expectNullableRfc3339Timestamp(
      record.last_contacted_at,
      'person record.last_contacted_at',
    ),
    aliases: decodeArray(record.aliases ?? [], decodePersonAliasData),
    links: decodeArray(record.links ?? [], decodePersonLinkRefData),
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

export function decodeWorkerCapacityData(value: unknown): WorkerCapacityData {
  const record = expectRecord(value, 'worker capacity');
  return {
    max_concurrency: expectNumber(record.max_concurrency, 'worker capacity.max_concurrency'),
    current_load: expectNumber(record.current_load, 'worker capacity.current_load'),
    available_concurrency: expectNumber(
      record.available_concurrency,
      'worker capacity.available_concurrency',
    ),
  };
}

export function decodeWorkerPresenceData(value: unknown): WorkerPresenceData {
  const record = expectRecord(value, 'worker presence');
  return {
    worker_id: expectString(record.worker_id, 'worker presence.worker_id'),
    node_id: expectString(record.node_id, 'worker presence.node_id'),
    node_display_name: expectString(record.node_display_name, 'worker presence.node_display_name'),
    client_kind: expectNullableString(record.client_kind, 'worker presence.client_kind'),
    client_version: expectNullableString(record.client_version, 'worker presence.client_version'),
    protocol_version: expectNullableString(record.protocol_version, 'worker presence.protocol_version'),
    build_id: expectNullableString(record.build_id, 'worker presence.build_id'),
    worker_classes: decodeArray(record.worker_classes ?? [], (item) =>
      expectString(item, 'worker presence.worker_classes[]')),
    capabilities: decodeArray(record.capabilities ?? [], (item) =>
      expectString(item, 'worker presence.capabilities[]')),
    status: expectString(record.status, 'worker presence.status'),
    queue_depth: expectNumber(record.queue_depth, 'worker presence.queue_depth'),
    reachability: expectString(record.reachability, 'worker presence.reachability'),
    latency_class: expectString(record.latency_class, 'worker presence.latency_class'),
    compute_class: expectString(record.compute_class, 'worker presence.compute_class'),
    power_class: expectString(record.power_class, 'worker presence.power_class'),
    recent_failure_rate: expectNumber(record.recent_failure_rate, 'worker presence.recent_failure_rate'),
    tailscale_preferred: expectBoolean(record.tailscale_preferred, 'worker presence.tailscale_preferred'),
    last_heartbeat_at: expectUnixSeconds(record.last_heartbeat_at, 'worker presence.last_heartbeat_at'),
    started_at: expectUnixSeconds(record.started_at, 'worker presence.started_at'),
    sync_base_url: expectString(record.sync_base_url, 'worker presence.sync_base_url'),
    sync_transport: expectString(record.sync_transport, 'worker presence.sync_transport'),
    tailscale_base_url: expectNullableString(record.tailscale_base_url, 'worker presence.tailscale_base_url'),
    preferred_tailnet_endpoint: expectNullableString(
      record.preferred_tailnet_endpoint,
      'worker presence.preferred_tailnet_endpoint',
    ),
    tailscale_reachable: expectBoolean(record.tailscale_reachable, 'worker presence.tailscale_reachable'),
    lan_base_url: expectNullableString(record.lan_base_url, 'worker presence.lan_base_url'),
    localhost_base_url: expectNullableString(record.localhost_base_url, 'worker presence.localhost_base_url'),
    ping_ms: expectNullableNumber(record.ping_ms, 'worker presence.ping_ms'),
    sync_status: expectNullableString(record.sync_status, 'worker presence.sync_status'),
    last_upstream_sync_at: expectNullableUnixSeconds(
      record.last_upstream_sync_at,
      'worker presence.last_upstream_sync_at',
    ),
    last_downstream_sync_at: expectNullableUnixSeconds(
      record.last_downstream_sync_at,
      'worker presence.last_downstream_sync_at',
    ),
    last_sync_error: expectNullableString(record.last_sync_error, 'worker presence.last_sync_error'),
    incoming_linking_prompt: decodeNullable(
      record.incoming_linking_prompt ?? null,
      decodeLinkingPromptData,
    ),
    capacity: decodeWorkerCapacityData(record.capacity),
  };
}

export function decodeClusterWorkersData(value: unknown): ClusterWorkersData {
  const record = expectRecord(value, 'cluster workers');
  return {
    active_authority_node_id: expectString(
      record.active_authority_node_id,
      'cluster workers.active_authority_node_id',
    ),
    active_authority_epoch: expectNumber(
      record.active_authority_epoch,
      'cluster workers.active_authority_epoch',
    ),
    generated_at: expectUnixSeconds(record.generated_at, 'cluster workers.generated_at'),
    workers: decodeArray(record.workers ?? [], decodeWorkerPresenceData),
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
    write_capabilities: decodeTodoistWriteCapabilitiesData(record.write_capabilities ?? {}),
  };
}

export function decodeTodoistWriteCapabilitiesData(value: unknown): TodoistWriteCapabilitiesData {
  const record = expectRecord(value, 'todoist write capabilities');
  return {
    completion_status:
      record.completion_status === undefined
        ? true
        : expectBoolean(record.completion_status, 'todoist write capabilities.completion_status'),
    due_date:
      record.due_date === undefined
        ? true
        : expectBoolean(record.due_date, 'todoist write capabilities.due_date'),
    tags:
      record.tags === undefined
        ? false
        : expectBoolean(record.tags, 'todoist write capabilities.tags'),
  };
}

export function decodeLocalIntegrationData(value: unknown): LocalIntegrationData {
  const record = expectRecord(value, 'local integration');
  return {
    configured: expectBoolean(record.configured, 'local integration.configured'),
    source_path: expectNullableString(record.source_path, 'local integration.source_path'),
    selected_paths: decodeArray(record.selected_paths ?? [], (item) =>
      expectString(item, 'local integration.selected_paths[]')),
    available_paths: decodeArray(record.available_paths ?? [], (item) =>
      expectString(item, 'local integration.available_paths[]')),
    internal_paths: decodeArray(record.internal_paths ?? [], (item) =>
      expectString(item, 'local integration.internal_paths[]')),
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
  const createdAt = decodeRunSummaryCreatedAt(record);
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
    created_at: createdAt,
    started_at: decodeNullableFlexibleDateTimeString(record.started_at, 'run summary.started_at'),
    finished_at: decodeNullableFlexibleDateTimeString(record.finished_at, 'run summary.finished_at'),
    duration_ms: expectNullableNumber(record.duration_ms, 'run summary.duration_ms'),
    retry_scheduled_at: decodeNullableFlexibleDateTimeString(
      record.retry_scheduled_at,
      'run summary.retry_scheduled_at',
    ),
    retry_reason: expectNullableString(record.retry_reason, 'run summary.retry_reason'),
    blocked_reason: expectNullableString(record.blocked_reason, 'run summary.blocked_reason'),
  };
}

function decodeRunSummaryCreatedAt(record: Record<string, unknown>): string {
  const explicitCreatedAt = record.created_at;
  if (explicitCreatedAt !== null && explicitCreatedAt !== undefined) {
    return decodeFlexibleDateTimeString(explicitCreatedAt, 'run summary.created_at');
  }

  const startedAt = decodeNullableFlexibleDateTimeString(
    record.started_at,
    'run summary.started_at',
  );
  if (startedAt) {
    return startedAt;
  }

  const finishedAt = decodeNullableFlexibleDateTimeString(
    record.finished_at,
    'run summary.finished_at',
  );
  if (finishedAt) {
    return finishedAt;
  }

  return new Date(0).toISOString();
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
    scheduler_rules: decodeCanonicalScheduleRulesData(record.scheduler_rules ?? {}),
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

function decodeFlexibleDateTimeString(value: unknown, label: string): string {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return new Date(value * 1000).toISOString();
  }
  return expectRfc3339Timestamp(value, label);
}

function decodeNullableFlexibleDateTimeString(value: unknown, label: string): string | null {
  if (value === null || value === undefined) {
    return null;
  }
  return decodeFlexibleDateTimeString(value, label);
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
    case 'linking:updated':
      return {
        type,
        timestamp,
        payload: decodeJsonValue(record.payload),
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
    attachments:
      Array.isArray(record.attachments)
        ? decodeArray(record.attachments, decodeAssistantEntryAttachmentData)
        : undefined,
  };
}

export function decodeAssistantEntryAttachmentData(value: unknown): AssistantEntryAttachmentData {
  const record = expectRecord(value, 'assistant entry attachment');
  const kind = expectString(record.kind, 'assistant entry attachment.kind');
  if (
    ![
      'file',
      'image',
      'person',
      'event',
      'task',
      'video',
      'audio',
      'link',
      'markdown',
    ].includes(kind)
  ) {
    throw new Error(`Unsupported assistant entry attachment kind: ${kind}`);
  }
  return {
    kind: kind as AssistantEntryAttachmentKindData,
    label:
      record.label === undefined ? undefined : expectNullableString(record.label, 'assistant entry attachment.label'),
    object_id:
      record.object_id === undefined ? undefined : expectNullableString(record.object_id, 'assistant entry attachment.object_id'),
    mime_type:
      record.mime_type === undefined ? undefined : expectNullableString(record.mime_type, 'assistant entry attachment.mime_type'),
    metadata:
      record.metadata === undefined ? undefined : (record.metadata as JsonValue | null),
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
