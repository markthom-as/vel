export type IntegrationPrimitiveKey =
  | 'calendar'
  | 'tasks'
  | 'messages'
  | 'calls'
  | 'media'
  | 'storage'
  | 'notes'
  | 'reminders'
  | 'transcripts'
  | 'git'
  | 'home'
  | 'user_activity'
  | 'models'
  | 'sources';

export type IntegrationPrimitiveDescriptor = {
  key: IntegrationPrimitiveKey;
  label: string;
  description: string;
  families: string[];
  providerKeys: string[];
};

export const SYSTEM_INTEGRATION_PRIMITIVES: IntegrationPrimitiveDescriptor[] = [
  {
    key: 'calendar',
    label: 'Calendar',
    description: 'Calendar sources, scopes, and sync visibility.',
    families: ['calendar'],
    providerKeys: ['google_calendar', 'icloud_calendar', 'caldav', 'exchange_calendar'],
  },
  {
    key: 'tasks',
    label: 'Tasks',
    description: 'Task sources and writeback policy.',
    families: ['tasks', 'todoist'],
    providerKeys: ['todoist', 'apple_reminders', 'asana', 'linear', 'jira'],
  },
  {
    key: 'messages',
    label: 'Messages',
    description: 'Messaging sources and linked accounts.',
    families: ['messages', 'messaging'],
    providerKeys: ['imessage', 'discord', 'slack', 'email', 'signal', 'whatsapp', 'messaging'],
  },
  {
    key: 'calls',
    label: 'Calls',
    description: 'Call and meeting sources across voice platforms.',
    families: ['calls', 'call', 'meetings'],
    providerKeys: ['zoom', 'discord_calls', 'phone', 'facetime', 'google_meet'],
  },
  {
    key: 'media',
    label: 'Media',
    description: 'Listening, playback, and media history sources.',
    families: ['media', 'music'],
    providerKeys: ['spotify', 'last_fm', 'mpd', 'apple_music', 'plex'],
  },
  {
    key: 'storage',
    label: 'Storage',
    description: 'File and object-storage sources available to Vel.',
    families: ['storage', 'files', 'file_storage'],
    providerKeys: ['nas', 's3', 'google_drive', 'dropbox', 'icloud_drive', 'local_folders'],
  },
  {
    key: 'notes',
    label: 'Notes',
    description: 'Notes sources and linked accounts.',
    families: ['notes'],
    providerKeys: ['apple_notes', 'obsidian', 'google_docs', 'notion', 'notes'],
  },
  {
    key: 'reminders',
    label: 'Reminders',
    description: 'Reminder sources and linked accounts.',
    families: ['reminders'],
    providerKeys: ['apple_reminders', 'reminders'],
  },
  {
    key: 'transcripts',
    label: 'Transcripts',
    description: 'Transcript sources and linked accounts.',
    families: ['transcripts'],
    providerKeys: ['meeting_transcripts', 'call_transcripts', 'transcripts'],
  },
  {
    key: 'git',
    label: 'Git',
    description: 'Repository-aware source settings and linked accounts.',
    families: ['git', 'development'],
    providerKeys: ['git', 'github', 'gitlab'],
  },
  {
    key: 'home',
    label: 'Home',
    description: 'Home and environmental automation sources.',
    families: ['home', 'environment'],
    providerKeys: ['home_assistant', 'presence', 'sensors'],
  },
  {
    key: 'user_activity',
    label: 'User activity',
    description: 'Behavioral and presence sources about what the user is doing.',
    families: ['activity', 'user_activity', 'presence'],
    providerKeys: ['activity', 'browser_activity', 'app_usage', 'presence', 'home_assistant'],
  },
  {
    key: 'models',
    label: 'Models',
    description: 'LLM routing and model profile configuration.',
    families: [],
    providerKeys: [],
  },
  {
    key: 'sources',
    label: 'Sources',
    description: 'All linked source accounts across primitives.',
    families: [],
    providerKeys: [],
  },
];

export function normalizeIntegrationPrimitiveValue(value: string): string {
  return value.trim().toLowerCase().replace(/[\s-]+/g, '_');
}

export function integrationPrimitiveDescriptor(
  key: IntegrationPrimitiveKey,
): IntegrationPrimitiveDescriptor {
  return SYSTEM_INTEGRATION_PRIMITIVES.find((primitive) => primitive.key === key)
    ?? SYSTEM_INTEGRATION_PRIMITIVES[0];
}
