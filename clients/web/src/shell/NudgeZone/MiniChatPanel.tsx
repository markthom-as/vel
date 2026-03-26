import { ChevronLeftIcon, CloseIcon, MinimizeIcon, ThreadsIcon } from '../../core/Icons';
import { ThreadView } from '../../views/threads';

export function MiniChatPanel({
  miniChatThreadId,
  onMiniChatClose,
  onMiniChatThreadSelect,
}: {
  miniChatThreadId?: string | null;
  onMiniChatClose?: () => void;
  onMiniChatThreadSelect?: (conversationId: string) => void;
}) {
  return (
    <section
      aria-label="Mini chat panel"
      className="absolute inset-x-0 bottom-2 z-40 flex max-h-[calc(100%-1rem)] w-full flex-col overflow-hidden border-b-[7px] border-b-[#2a160c] bg-[color:var(--vel-color-bg)]/95 py-1 font-mono ring-1 ring-[var(--vel-color-border)]/85 shadow-[0_2px_8px_rgba(0,0,0,0.26)]"
    >
      <div className="flex items-center justify-between gap-1 border-b border-[var(--vel-color-border)] px-2 py-1">
        <p className="inline-flex min-w-0 items-center gap-1.5 whitespace-nowrap text-[10px] uppercase leading-none tracking-[0.14em] text-[var(--vel-color-accent-soft)]">
          <ThreadsIcon size={14} />
          TERMINAL CHAT
        </p>
        <div className="flex shrink-0 items-center gap-1">
          <button
            type="button"
            onClick={() => onMiniChatClose?.()}
            aria-label="Return to GUI mode"
            className="inline-flex h-5 min-w-[2.35rem] shrink-0 items-center justify-center gap-1 whitespace-nowrap rounded border border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/70 px-1 !text-[8px] uppercase leading-none tracking-[0.1em] text-[var(--vel-color-accent-soft)] transition hover:border-[var(--vel-color-accent-border)] hover:text-[var(--vel-color-text)]"
          >
            <ChevronLeftIcon size={9} />
            <span>GUI</span>
          </button>
          <button
            type="button"
            onClick={() => onMiniChatClose?.()}
            aria-label="Minimize mini chat"
            className="mt-0.5 inline-flex h-6 w-6 items-center justify-center text-[var(--vel-color-accent-soft)] transition hover:text-[var(--vel-color-text)]"
          >
            <MinimizeIcon size={11} />
          </button>
          <button
            type="button"
            onClick={() => onMiniChatClose?.()}
            aria-label="Close mini chat"
            className="inline-flex h-6 w-6 items-center justify-center text-[var(--vel-color-accent-soft)] transition hover:text-[var(--vel-color-text)]"
          >
            <CloseIcon size={11} />
          </button>
        </div>
      </div>
      <ThreadView
        miniMode
        className="min-h-0 flex-1 px-1 pb-1"
        conversationId={miniChatThreadId ?? null}
        onMiniChatClose={onMiniChatClose}
        onSelectConversation={(conversationId) => {
          onMiniChatThreadSelect?.(conversationId);
        }}
      />
    </section>
  );
}
