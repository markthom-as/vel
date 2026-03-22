import type { ReactNode } from 'react';
import { cn } from '../cn';
import { itemPillCard } from '../itemPill';

export type ChatBubbleVariant = 'user' | 'assistant';

const shellByVariant: Record<ChatBubbleVariant, string> = {
  user: cn(
    itemPillCard('queue', 'laneRow'),
    'max-w-full border-emerald-800/55 bg-emerald-950/50 text-emerald-100',
  ),
  assistant: cn(
    itemPillCard('queue', 'laneRow'),
    'vel-chat-assistant-bubble-fill max-w-full !border-[#ff6b00]/50 !bg-transparent text-zinc-100 shadow-[inset_0_1px_0_rgba(255,255,255,0.05)]',
  ),
};

const tailByVariant: Record<ChatBubbleVariant, string> = {
  user:
    'pointer-events-none absolute bottom-3 z-0 h-3 w-3 rotate-45 rounded-[3px] border border-emerald-800/55 bg-emerald-950/50 shadow-[0_1px_0_rgba(0,0,0,0.08)] -right-2',
  assistant:
    'pointer-events-none absolute bottom-3 z-0 h-3 w-3 rotate-45 rounded-[3px] border border-[#ff6b00]/50 bg-[#4a2412] shadow-[0_1px_0_rgba(0,0,0,0.08)] -left-2',
};

/**
 * Chat message shell: same `itemPill` **laneRow** treatment as tasks / inbox rows, plus a small corner tail.
 * Inner content should use {@link NowItemRowLayout} + {@link ItemRowTitleMetaBand} for header alignment.
 */
export function ChatBubbleChrome({
  variant,
  className,
  children,
}: {
  variant: ChatBubbleVariant;
  className?: string;
  children: ReactNode;
}) {
  return (
    <div
      className={cn(
        'relative w-full max-w-[min(85%,42rem)]',
        shellByVariant[variant],
        '!overflow-visible',
        className,
      )}
    >
      <span className={tailByVariant[variant]} aria-hidden />
      <div className="relative z-[1] min-w-0">{children}</div>
    </div>
  );
}
