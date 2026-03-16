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

export interface CurrentContextData {
  computed_at: number;
  context: JsonValue;
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

export type WsEvent =
  | WsMessageNewEvent
  | WsInterventionsNewEvent
  | WsInterventionsUpdatedEvent;

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

export function decodeCurrentContextData(value: unknown): CurrentContextData {
  const record = expectRecord(value, 'current context');
  return {
    computed_at: expectNumber(record.computed_at, 'current context.computed_at'),
    context: decodeJsonValue(record.context),
  };
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
