import { cn } from '../cn';
import { uiTheme } from '../Theme';

/**
 * Shared “item pill” surfaces: nudge-origin cards (border + zinc base + optional ::before tint).
 * Use for nudge bars, task rows, inbox entries, thread sidebar rows, and similar list chrome.
 */
export const itemPillSurface = {
  brand: cn(
    uiTheme.brandBorder,
    'bg-zinc-950/78 shadow-[0_10px_32px_rgba(255,107,0,0.05)]',
    "before:pointer-events-none before:absolute before:inset-0 before:rounded-[20px] before:bg-[#ff6b00]/4 before:content-['']",
  ),
  warm: cn(
    'border-amber-700/40 bg-zinc-950/78 shadow-[0_8px_28px_rgba(245,158,11,0.045)]',
    "before:pointer-events-none before:absolute before:inset-0 before:rounded-[20px] before:bg-amber-500/5 before:content-['']",
  ),
  /** Default list row on dark chrome (threads sidebar, inbox, idle tasks). */
  muted: cn(
    'border-zinc-800/90 bg-zinc-950/78 shadow-[0_8px_24px_rgba(0,0,0,0.18)]',
    "before:pointer-events-none before:absolute before:inset-0 before:rounded-[20px] before:bg-zinc-950/25 before:content-['']",
  ),
  /** Lane “active” task emphasis — same family as brand nudges, slightly stronger tint. */
  emphasis: cn(
    uiTheme.brandBorder,
    'bg-zinc-950/78 shadow-[0_10px_28px_rgba(255,107,0,0.07)]',
    "before:pointer-events-none before:absolute before:inset-0 before:rounded-[20px] before:bg-[#ff6b00]/6 before:content-['']",
  ),
  /** At-risk / warning rows (aligns with warm nudge). */
  risk: cn(
    'border-amber-700/40 bg-zinc-950/78 shadow-[0_8px_28px_rgba(245,158,11,0.045)]',
    "before:pointer-events-none before:absolute before:inset-0 before:rounded-[20px] before:bg-amber-500/5 before:content-['']",
  ),
  /** Completed / de-emphasized lane items (no tint overlay). */
  ghost: 'border-zinc-900/80 bg-transparent',
  /** TODAY / NEXT task rows — flat border + fill only; no shadow or ::before wash. */
  queue: 'border-zinc-800/90 bg-zinc-950/78',
} as const;

export const itemPillShell = {
  /** Nudge bar / compact task density */
  compact: 'relative flex flex-col gap-2 overflow-hidden rounded-[20px] border px-4 py-2.5',
  /**
   * Section title row + trailing metrics/chips on one horizontal band (no outer flex-col).
   * Use with `PanelSectionHeaderBand` + lead/trail helpers in `PanelChrome`.
   */
  sectionHeader:
    'relative flex min-w-0 flex-row flex-wrap items-start justify-between gap-x-3 gap-y-2 overflow-hidden rounded-[20px] border px-4 py-3',
  /** Task lane row without nudge-style inner stack */
  laneRow: 'relative overflow-hidden rounded-[20px] border px-4 py-2.5',
  /** Inbox-style blocks */
  comfortable: 'relative overflow-hidden rounded-[20px] border p-4',
  /** Full-width selectable row (thread list) */
  rowButton: 'relative w-full overflow-hidden rounded-[20px] border p-3 text-left transition',
} as const;

export const itemPillIconGlow = {
  brand: 'drop-shadow-[0_0_14px_rgba(255,107,0,0.22)]',
  warm: 'drop-shadow-[0_0_14px_rgba(245,158,11,0.2)]',
} as const;

/** Selected thread row (light surface for contrast in sidebar). */
export const itemPillRowSelected =
  'relative w-full overflow-hidden rounded-[20px] border border-zinc-100 bg-zinc-100 p-3 text-left text-zinc-950 shadow-none transition';

/**
 * Small meta pills (kind, continuation, inbox labels) — pair with `onDark` / `onLight` text utilities at call site.
 */
export const itemPillMeta = {
  onDark: 'rounded-full border border-zinc-700 px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-zinc-400',
  onLight: 'rounded-full border border-zinc-300 px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-zinc-800',
} as const;

export function itemPillCard(
  surface: keyof typeof itemPillSurface,
  shell: keyof typeof itemPillShell = 'compact',
): string {
  return cn(itemPillShell[shell], itemPillSurface[surface]);
}

