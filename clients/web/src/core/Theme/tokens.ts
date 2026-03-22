/**
 * Canonical main-column layout for primary operator views.
 * Source of truth: `views/now/NowView.tsx` (Now). Inbox and other main surfaces reuse these so max-width,
 * padding, and vertical rhythm match.
 */
export const surfaceShell = {
  /** Full-height column under `MainPanel` (dark chrome). */
  mainColumn: 'flex min-h-0 flex-1 flex-col bg-zinc-950',
  /** Scroll region above optional bottom fade / composer. */
  scrollColumn: 'relative min-h-0 flex-1 overflow-y-auto',
  /**
   * Inner content: centered column, responsive horizontal padding, room for bottom composer (`pb-36`).
   * Matches Now hero + task stack width.
   */
  mainContent: 'mx-auto w-full max-w-5xl px-4 pb-36 pt-10 sm:px-6 sm:pt-12',
  /** Default vertical gap between major blocks (Now uses `<section className={cn(surfaceShell.sectionStack)}>`). */
  sectionStack: 'space-y-5',
} as const;

/** Tailwind font-family utilities — pair with `@theme` in `src/index.css`. */
export const uiFonts = {
  sans: 'font-sans',
  serif: 'font-serif',
  mono: 'font-mono',
  /** Headlines, wordmark, hero-style UI (Space Grotesk). */
  display: 'font-display',
  /** Secondary display — slightly rounder, good for large subheads (Outfit). */
  displayAlt: 'font-display-alt',
} as const;

export const uiTheme = {
  brandHex: '#ff6b00',
  brandText: 'text-[#ff6b00]',
  brandSoftText: 'text-[#ffb27a]',
  brandBorder: 'border-[#ff6b00]/55',
  brandHoverBorder: 'hover:border-[#ff8f40]/75',
  brandPanel: 'bg-[#4a2412]',
  brandShadow: 'shadow-[0_14px_36px_rgba(255,107,0,0.18)]',
  brandGlow: 'drop-shadow-[0_0_18px_rgba(255,107,0,0.42)]',
  /** Dense kind tag on nudge rows (paired with `nudgeKindTagClasses`). */
  brandNudgeKindTag: 'border-[#ff6b00]/45 bg-[#4a2412]/75 text-[#ffb27a]',
  /** Thread bubbles, inline assistant replies, and other “message from Vel” containers. */
  brandAssistantBubble:
    'border border-[#ff6b00]/50 bg-[#4a2412] text-zinc-100 shadow-[inset_0_1px_0_rgba(255,255,255,0.05)]',
  /** Meta line (role · kind) on assistant bubbles. */
  brandAssistantBubbleMeta: 'text-[#c9a082]',
};

/**
 * Hashed project/tag labels: distinct slots in the same orange–copper family as `uiTheme`.
 * Keeps per-label stability via `projectTagHash` in `nowModel`.
 */
export const brandTagPalette = [
  'border-[#ff6b00]/40 bg-[#2d1608]/90 text-[#ffd4b8]',
  'border-[#ff8f40]/45 bg-[#3d1d0a]/85 text-[#ffc49a]',
  'border-[#ea580c]/42 bg-[#431407]/88 text-[#fdba74]',
  'border-[#ff6b00]/35 bg-[#4a2412]/80 text-[#ffb27a]',
  'border-[#c2410c]/48 bg-[#361007]/85 text-[#fed7aa]',
  'border-[#f97316]/40 bg-[#3f240f]/82 text-[#ffedd5]',
  'border-[#fb923c]/38 bg-[#422006]/80 text-[#fde68a]',
  'border-[#ff6b00]/50 bg-[#331809]/88 text-[#ffe4c4]',
  'border-[#9a3412]/55 bg-[#2a1106]/92 text-[#fdba74]',
  'border-[#ff6b00]/32 bg-[#451a03]/78 text-[#fed7aa]',
  'border-[#ea580c]/36 bg-[#4a2412]/75 text-[#ffb27a]',
  'border-[#ff8f40]/42 bg-[#38180b]/86 text-[#ffc49a]',
] as const;
