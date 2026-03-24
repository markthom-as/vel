import type { ReactNode } from 'react';
import { cn } from '../cn';
import { itemPillCard } from '../itemPill';

export type ChatBubbleVariant = 'user' | 'assistant';

const shellByVariant: Record<ChatBubbleVariant, string> = {
  user: cn(
    itemPillCard('queue', 'laneRow'),
    'max-w-full border-emerald-800/55 bg-emerald-950/50 text-emerald-100 shadow-[0_14px_34px_rgba(6,78,59,0.16)]',
  ),
  assistant: cn(
    itemPillCard('queue', 'laneRow'),
    'vel-chat-assistant-bubble-fill max-w-full !border-[#ff6b00]/30 !bg-[rgba(74,36,18,0.2)] text-zinc-100 shadow-[0_16px_38px_rgba(74,36,18,0.18),inset_0_1px_0_rgba(255,255,255,0.03)]',
  ),
};

/**
 * Chat message shell: same `itemPill` **laneRow** treatment as tasks / inbox rows, but with a tail-less
 * modern bubble treatment tuned for readable thread transcripts.
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
        'relative w-full max-w-[min(62%,34rem)] overflow-hidden rounded-[1.55rem]',
        shellByVariant[variant],
        className,
      )}
      data-chat-bubble-variant={variant}
    >
      <div className="min-w-0">{children}</div>
    </div>
  );
}
