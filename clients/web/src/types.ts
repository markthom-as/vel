export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: { code: string; message: string };
  meta: { request_id: string };
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

export interface InboxItemData {
  id: string;
  message_id: string;
  kind: string;
  state: string;
  surfaced_at: number;
  snoozed_until: number | null;
  confidence: number | null;
}

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
