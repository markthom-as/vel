import { CardLayout } from './CardLayout';

interface ReminderContent {
  title: string;
  due_time?: number;
  reason?: string;
  confidence?: number;
}

export function ReminderCardView({ content }: { content: ReminderContent }) {
  const due = content.due_time != null ? new Date(content.due_time * 1000).toLocaleString() : null;
  return (
    <CardLayout kind="reminder_card">
      <div className="font-medium text-zinc-200">{content.title}</div>
      {due && <div className="text-xs text-zinc-500 mt-1">Due: {due}</div>}
      {content.reason && <p className="text-sm text-zinc-400 mt-1">{content.reason}</p>}
      {content.confidence != null && (
        <div className="text-xs text-zinc-500 mt-1">Confidence: {Math.round(content.confidence * 100)}%</div>
      )}
    </CardLayout>
  );
}
