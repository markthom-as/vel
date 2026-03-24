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
    'vel-chat-assistant-bubble-fill max-w-full !border-[#ff6b00]/30 !bg-[rgba(74,36,18,0.2)] text-zinc-100 shadow-[inset_0_1px_0_rgba(255,255,255,0.03)]',
  ),
};

const tailWrapByVariant: Record<ChatBubbleVariant, string> = {
  user: 'pointer-events-none absolute bottom-[-0.28rem] right-3 z-[0] h-3.5 w-3.5 rotate-45 rounded-[0.18rem] border-r border-b',
  assistant: 'pointer-events-none absolute bottom-[-0.28rem] left-3 z-[0] h-3.5 w-3.5 rotate-45 rounded-[0.18rem] border-l border-b',
};

const tailToneByVariant: Record<ChatBubbleVariant, string> = {
  user: 'border-emerald-800/55 bg-emerald-950/50',
  assistant: 'border-[#ff6b00]/30 bg-[rgba(74,36,18,0.2)]',
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
        'relative w-full max-w-[min(56%,31rem)]',
        shellByVariant[variant],
        '!overflow-visible',
        className,
      )}
    >
      <span className={cn(tailWrapByVariant[variant], tailToneByVariant[variant])} aria-hidden />
      <div className="relative z-[1] min-w-0">{children}</div>
    </div>
  );
}
