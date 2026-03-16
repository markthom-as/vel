export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: { code: string; message: string };
  meta: { request_id: string };
}

export interface WsEnvelope {
  type: string;
  timestamp: string;
  payload: unknown;
}

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
  content: unknown;
  status: string | null;
  importance: string | null;
  created_at: number;
  updated_at: number | null;
}

/** Response from POST /api/conversations/:id/messages: user message + optional assistant reply. */
export interface CreateMessageResponse {
  user_message: MessageData;
  assistant_message?: MessageData | null;
  /** When assistant reply was requested but failed (e.g. LLM unreachable). */
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

// Assumption for websocket `interventions:new`: payload matches the inbox item shape
// because both inbox and thread UI key intervention state by { id, message_id }.
export interface InterventionEventData extends InboxItemData {}

export interface ProvenanceData {
  message_id: string;
  events: ProvenanceEvent[];
  signals: unknown[];
  policy_decisions: unknown[];
  linked_objects: unknown[];
}

export interface ProvenanceEvent {
  id: string;
  event_name: string;
  created_at: number;
  payload: unknown;
}

export interface SettingsData {
  quiet_hours?: unknown;
  disable_proactive?: boolean;
  toggle_risks?: boolean;
  toggle_reminders?: boolean;
}
