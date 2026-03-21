import type { ActionItemData } from '../../../types';
import { projectTagTone } from '../nowModel';

export function ActionRow({
  item,
  onOpenInbox,
  onOpenThread,
}: {
  item: ActionItemData;
  onOpenInbox?: () => void;
  onOpenThread?: (conversationId: string) => void;
}) {
  return (
    <div className="rounded-xl border border-amber-700/30 bg-amber-950/10 px-4 py-3">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0 flex-1">
          <div className="flex flex-wrap items-center gap-2">
            <p className="text-sm font-medium text-zinc-100">{item.title}</p>
            {item.project_label ? (
              <span
                className={`rounded-full border px-2 py-0.5 text-[10px] uppercase tracking-[0.16em] ${projectTagTone(item.project_label)}`}
              >
                {item.project_label}
              </span>
            ) : null}
          </div>
          <p className="mt-1 text-sm text-zinc-300">{item.summary}</p>
        </div>
        <div className="flex flex-wrap gap-2">
          {item.thread_route?.thread_id ? (
            <button
              type="button"
              onClick={() => onOpenThread?.(item.thread_route?.thread_id as string)}
              className="rounded-full border border-zinc-700 bg-zinc-950/80 px-3 py-2 text-[10px] uppercase tracking-[0.18em] text-zinc-300 transition hover:border-zinc-500 hover:text-zinc-100"
            >
              {item.thread_route.label}
            </button>
          ) : null}
          <button
            type="button"
            onClick={() => onOpenInbox?.()}
            className="rounded-full border border-zinc-700 bg-zinc-950/80 px-3 py-2 text-[10px] uppercase tracking-[0.18em] text-zinc-300 transition hover:border-zinc-500 hover:text-zinc-100"
          >
            Open Inbox
          </button>
        </div>
      </div>
    </div>
  );
}
