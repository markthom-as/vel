export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: { code: string; message: string };
  warnings?: string[];
  meta: { request_id: string; degraded?: boolean };
}

export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonValue[] | { [key: string]: JsonValue };
export type JsonObject = { [key: string]: JsonValue };

export interface ConversationData {
  id: string;
  title: string | null;
  kind: string;
  pinned: boolean;
  archived: boolean;
  created_at: number;
  updated_at: number;
}

export interface MessageData {
  id: string;
  conversation_id: string;
  role: string;
  kind: string;
  content: JsonValue;
  status: string | null;
  importance: string | null;
  created_at: number;
  updated_at: number | null;
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
  surfaced_at: number;
  snoozed_until: number | null;
  confidence: number | null;
}

export interface InterventionActionData {
  id: string;
  state: string;
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
  created_at: number;
  payload: JsonValue;
}

export interface SettingsData {
  quiet_hours?: JsonValue;
  disable_proactive?: boolean;
  toggle_risks?: boolean;
  toggle_reminders?: boolean;
}

export interface IntegrationCalendarData {
  id: string;
  summary: string;
  primary: boolean;
  selected: boolean;
}

export interface GoogleCalendarIntegrationData {
  configured: boolean;
  connected: boolean;
  has_client_id: boolean;
  has_client_secret: boolean;
  calendars: IntegrationCalendarData[];
  all_calendars_selected: boolean;
  last_sync_at: number | null;
  last_sync_status: string | null;
  last_error: string | null;
  last_item_count: number | null;
}

export interface TodoistIntegrationData {
  configured: boolean;
  connected: boolean;
  has_api_token: boolean;
  last_sync_at: number | null;
  last_sync_status: string | null;
  last_error: string | null;
  last_item_count: number | null;
}

export interface LocalIntegrationData {
  configured: boolean;
  source_path: string | null;
  last_sync_at: number | null;
  last_sync_status: string | null;
  last_error: string | null;
  last_item_count: number | null;
}

export interface IntegrationsData {
  google_calendar: GoogleCalendarIntegrationData;
  todoist: TodoistIntegrationData;
  activity: LocalIntegrationData;
  git: LocalIntegrationData;
  messaging: LocalIntegrationData;
  notes: LocalIntegrationData;
  transcripts: LocalIntegrationData;
}

export interface ComponentData {
  id: string;
  name: string;
  description: string;
  status: string;
  last_restarted_at: number | null;
  last_error: string | null;
  restart_count: number;
}

export interface ComponentLogEventData {
  id: string;
  component_id: string;
  event_name: string;
  status: string;
  message: string;
  payload: JsonValue;
  created_at: number;
}

export interface GoogleCalendarAuthStartData {
  auth_url: string;
}

export interface RunSummaryData {
  id: string;
  kind: string;
  status: string;
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

export interface CurrentContextData {
  computed_at: number;
  context: JsonValue;
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
  timestamp: number;
  summary: JsonValue;
}

export interface ContextExplainData {
  computed_at: number;
  mode: string | null;
  morning_state: string | null;
  context: JsonValue;
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
  top_drivers?: string[];
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
  timestamp: string;
  payload: MessageData;
}

export interface WsInterventionsNewEvent {
  type: 'interventions:new';
  timestamp: string;
  payload: InboxItemData;
}

export interface WsInterventionsUpdatedEvent {
  type: 'interventions:updated';
  timestamp: string;
  payload: InterventionActionData;
}

export type RunUpdateEventData = RunSummaryData;

export interface WsRunsUpdatedEvent {
  type: 'runs:updated';
  timestamp: string;
  payload: RunUpdateEventData;
}

export type WsEvent =
  | WsMessageNewEvent
  | WsInterventionsNewEvent
  | WsInterventionsUpdatedEvent
  | WsRunsUpdatedEvent;

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
    created_at: expectNumber(record.created_at, 'conversation.created_at'),
    updated_at: expectNumber(record.updated_at, 'conversation.updated_at'),
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
    created_at: expectNumber(record.created_at, 'message.created_at'),
    updated_at: expectNullableNumber(record.updated_at, 'message.updated_at'),
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

export function decodeInboxItemData(value: unknown): InboxItemData {
  const record = expectRecord(value, 'inbox item');
  return {
    id: expectString(record.id, 'inbox item.id'),
    message_id: expectString(record.message_id, 'inbox item.message_id'),
    kind: expectString(record.kind, 'inbox item.kind'),
    state: expectString(record.state, 'inbox item.state'),
    surfaced_at: expectNumber(record.surfaced_at, 'inbox item.surfaced_at'),
    snoozed_until: expectNullableNumber(record.snoozed_until, 'inbox item.snoozed_until'),
    confidence: expectNullableNumber(record.confidence, 'inbox item.confidence'),
  };
}

export function decodeInterventionActionData(value: unknown): InterventionActionData {
  const record = expectRecord(value, 'intervention action');
  return {
    id: expectString(record.id, 'intervention action.id'),
    state: expectString(record.state, 'intervention action.state'),
  };
}

export function decodeRunUpdateEventData(value: unknown): RunUpdateEventData {
  return decodeRunSummaryData(value);
}

export function decodeCurrentContextData(value: unknown): CurrentContextData {
  const record = expectRecord(value, 'current context');
  return {
    computed_at: expectNumber(record.computed_at, 'current context.computed_at'),
    context: decodeJsonValue(record.context),
  };
}

export function decodeSignalExplainSummary(value: unknown): SignalExplainSummary {
  const record = expectRecord(value, 'signal explain summary');
  return {
    signal_id: expectString(record.signal_id, 'signal explain summary.signal_id'),
    signal_type: expectString(record.signal_type, 'signal explain summary.signal_type'),
    source: expectString(record.source, 'signal explain summary.source'),
    timestamp: expectNumber(record.timestamp, 'signal explain summary.timestamp'),
    summary: decodeJsonValue(record.summary),
  };
}

export function decodeContextExplainData(value: unknown): ContextExplainData {
  const record = expectRecord(value, 'context explain');
  return {
    computed_at: expectNumber(record.computed_at, 'context explain.computed_at'),
    mode: expectNullableString(record.mode, 'context explain.mode'),
    morning_state: expectNullableString(record.morning_state, 'context explain.morning_state'),
    context: decodeJsonValue(record.context),
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
    last_sync_at: expectNullableNumber(record.last_sync_at, 'google calendar integration.last_sync_at'),
    last_sync_status: expectNullableString(
      record.last_sync_status,
      'google calendar integration.last_sync_status',
    ),
    last_error: expectNullableString(record.last_error, 'google calendar integration.last_error'),
    last_item_count: expectNullableNumber(
      record.last_item_count,
      'google calendar integration.last_item_count',
    ),
  };
}

export function decodeTodoistIntegrationData(value: unknown): TodoistIntegrationData {
  const record = expectRecord(value, 'todoist integration');
  return {
    configured: expectBoolean(record.configured, 'todoist integration.configured'),
    connected: expectBoolean(record.connected, 'todoist integration.connected'),
    has_api_token: expectBoolean(record.has_api_token, 'todoist integration.has_api_token'),
    last_sync_at: expectNullableNumber(record.last_sync_at, 'todoist integration.last_sync_at'),
    last_sync_status: expectNullableString(record.last_sync_status, 'todoist integration.last_sync_status'),
    last_error: expectNullableString(record.last_error, 'todoist integration.last_error'),
    last_item_count: expectNullableNumber(record.last_item_count, 'todoist integration.last_item_count'),
  };
}

export function decodeLocalIntegrationData(value: unknown): LocalIntegrationData {
  const record = expectRecord(value, 'local integration');
  return {
    configured: expectBoolean(record.configured, 'local integration.configured'),
    source_path: expectNullableString(record.source_path, 'local integration.source_path'),
    last_sync_at: expectNullableNumber(record.last_sync_at, 'local integration.last_sync_at'),
    last_sync_status: expectNullableString(record.last_sync_status, 'local integration.last_sync_status'),
    last_error: expectNullableString(record.last_error, 'local integration.last_error'),
    last_item_count: expectNullableNumber(record.last_item_count, 'local integration.last_item_count'),
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
    git: decodeLocalIntegrationData(record.git ?? {}),
    messaging: decodeLocalIntegrationData(record.messaging ?? {}),
    notes: decodeLocalIntegrationData(record.notes ?? {}),
    transcripts: decodeLocalIntegrationData(record.transcripts ?? {}),
  };
}

export function decodeComponentData(value: unknown): ComponentData {
  const record = expectRecord(value, 'component');
  return {
    id: expectString(record.id, 'component.id'),
    name: expectString(record.name, 'component.name'),
    description: expectString(record.description, 'component.description'),
    status: expectString(record.status, 'component.status'),
    last_restarted_at: expectNullableNumber(
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
    created_at: expectNumber(record.created_at, 'component log event.created_at'),
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
  return {
    id: expectString(record.id, 'run summary.id'),
    kind: expectString(record.kind, 'run summary.kind'),
    status: expectString(record.status, 'run summary.status'),
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
    created_at: expectString(record.created_at, 'run summary.created_at'),
    started_at: expectNullableString(record.started_at, 'run summary.started_at'),
    finished_at: expectNullableString(record.finished_at, 'run summary.finished_at'),
    duration_ms: expectNullableNumber(record.duration_ms, 'run summary.duration_ms'),
    retry_scheduled_at: expectNullableString(
      record.retry_scheduled_at,
      'run summary.retry_scheduled_at',
    ),
    retry_reason: expectNullableString(record.retry_reason, 'run summary.retry_reason'),
    blocked_reason: expectNullableString(record.blocked_reason, 'run summary.blocked_reason'),
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
  if (typeof value === 'string') {
    return value;
  }
  if (Array.isArray(value)) {
    return decodeTimeTuple(value, label);
  }
  throw new Error(`Expected ${label} to be a string`);
}

function decodeNullableDateTimeString(value: unknown, label: string): string | null {
  if (value === null || value === undefined) {
    return null;
  }
  return decodeDateTimeString(value, label);
}

function decodeTimeTuple(value: unknown[], label: string): string {
  if (value.length < 6) {
    throw new Error(`Expected ${label} tuple to have at least 6 items`);
  }
  const [year, ordinal, hour, minute, second] = value;
  if (
    typeof year !== 'number'
    || typeof ordinal !== 'number'
    || typeof hour !== 'number'
    || typeof minute !== 'number'
    || typeof second !== 'number'
  ) {
    throw new Error(`Expected ${label} tuple numbers`);
  }
  const date = new Date(Date.UTC(year, 0, ordinal, hour, minute, second));
  return date.toISOString();
}

export function decodeProvenanceEvent(value: unknown): ProvenanceEvent {
  const record = expectRecord(value, 'provenance event');
  return {
    id: expectString(record.id, 'provenance event.id'),
    event_name: expectString(record.event_name, 'provenance event.event_name'),
    created_at: expectNumber(record.created_at, 'provenance event.created_at'),
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
  const timestamp = expectString(record.timestamp, 'websocket event.timestamp');

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
    case 'runs:updated':
      return {
        type,
        timestamp,
        payload: decodeRunUpdateEventData(record.payload),
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
  return { text: record.text };
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
  if (!record || typeof record.commitment_title !== 'string' || typeof record.risk_level !== 'string') {
    return null;
  }
  return {
    commitment_title: record.commitment_title,
    risk_level: record.risk_level,
    top_drivers: optionalStringArray(record.top_drivers),
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

function expectNullableNumber(value: unknown, label: string): number | null {
  if (value === null || value === undefined) {
    return null;
  }
  return expectNumber(value, label);
}

function expectBoolean(value: unknown, label: string): boolean {
  if (typeof value !== 'boolean') {
    throw new Error(`Expected ${label} to be a boolean`);
  }
  return value;
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
