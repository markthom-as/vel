/** Keys that map to a product mark in Settings integrations (matches `IntegrationProductKey` in Settings). */
export type BrandIntegrationKey =
  | 'google'
  | 'todoist'
  | 'activity'
  | 'health'
  | 'git'
  | 'messaging'
  | 'reminders'
  | 'notes'
  | 'transcripts';

/** Maps Stats “Source health” row keys to brand keys. */
export function statsRowKeyToBrand(key: string): BrandIntegrationKey | null {
  switch (key) {
    case 'google_calendar':
      return 'google';
    case 'todoist':
      return 'todoist';
    case 'activity':
      return 'activity';
    case 'health':
      return 'health';
    case 'git':
      return 'git';
    case 'messaging':
      return 'messaging';
    case 'reminders':
      return 'reminders';
    case 'notes':
      return 'notes';
    case 'transcripts':
      return 'transcripts';
    default:
      return null;
  }
}
