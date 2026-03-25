/**
 * Canonical main-column layout for primary operator views.
 * Source of truth: `views/now/NowView.tsx` (Now). Inbox and other main surfaces reuse these so max-width,
 * padding, and vertical rhythm match.
 */
export const surfaceShell = {
  /** Full-height column under the shared shell regions. */
  mainColumn: 'flex min-h-0 flex-1 flex-col bg-transparent',
  /** Scroll region above optional bottom fade / composer. */
  scrollColumn: 'relative min-h-0 flex-1 overflow-y-auto',
  /** Natural-flow region when the parent surface owns scrolling. */
  flowColumn: 'relative flex-1',
  /**
   * Inner content: centered column, responsive horizontal padding, room for bottom composer (`pb-36`).
   * Matches Now hero + task stack width.
   */
  mainContent: 'mx-auto w-full max-w-5xl px-4 pb-40 pt-8 sm:px-6 sm:pt-10',
  /** Default vertical gap between major blocks (Now uses `<section className={cn(surfaceShell.sectionStack)}>`). */
  sectionStack: 'space-y-5',
} as const;

export const shellChrome = {
  app: 'flex min-h-screen flex-col overflow-visible bg-[radial-gradient(circle_at_top,_rgba(44,31,22,0.78)_0%,_var(--vel-color-bg)_36%)] text-[var(--vel-color-text)]',
  workspace:
    'mx-auto grid w-full max-w-[1540px] flex-1 gap-5 overflow-visible px-4 pb-36 pt-4 sm:px-6 lg:grid-cols-[minmax(0,1fr)_27rem] lg:items-start',
  workspaceMain: 'flex min-h-0 min-w-0 flex-col overflow-visible',
  workspaceAside: 'sticky top-[5.25rem] hidden overflow-visible self-start lg:block',
  workspaceAsideInner: 'overflow-visible',
  workspaceAsideScroll: 'overflow-visible px-8 py-3',
  topBand:
    'sticky top-0 z-40 shrink-0 border-b border-[var(--vel-color-border-subtle)] bg-[color:var(--vel-color-bg-overlay)] backdrop-blur-[18px]',
  topBandInner: 'mx-auto flex w-full max-w-[1440px] min-w-0 items-center gap-4 px-4 py-3 sm:px-6',
  actionBarDock:
    'fixed inset-x-0 bottom-0 z-30 border-t border-[var(--vel-color-border-subtle)] bg-[color:var(--vel-color-bg-elevated)]/95 px-3 py-3 backdrop-blur-[18px] sm:bottom-5 sm:left-1/2 sm:right-auto sm:w-auto sm:min-w-[42rem] sm:max-w-[calc(100vw-2rem)] sm:-translate-x-1/2 sm:rounded-full sm:border',
  actionBarInner: 'flex items-center justify-between gap-2 sm:justify-center',
} as const;

/** Tailwind font-family utilities — pair with `@theme` in `src/index.css`. */
export const uiFonts = {
  sans: 'font-sans',
  serif: 'font-serif',
  mono: 'font-mono',
  /** Headlines, wordmark, hero-style UI (Geist with Space Grotesk fallback). */
  display: 'font-display',
  /** Temporary compatibility alias until older display accents are retired. */
  displayAlt: 'font-display-alt',
  tabular: 'tabular-nums',
} as const;

export const uiTheme = {
  brandHex: '#c8742b',
  brandText: 'text-[var(--vel-color-accent)]',
  brandSoftText: 'text-[var(--vel-color-accent-soft)]',
  brandBorder: 'border-[color:var(--vel-color-accent-border)]',
  brandHoverBorder: 'hover:border-[color:var(--vel-color-accent-strong)]',
  brandPanel: 'bg-[color:var(--vel-color-panel-2)]',
  brandShadow: 'shadow-[0_18px_40px_rgba(0,0,0,0.26)]',
  brandGlow: 'drop-shadow-[0_0_18px_rgba(200,116,43,0.34)]',
  /** Thread bubbles, inline assistant replies, and other “message from Vel” containers. */
  brandAssistantBubble:
    'border border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] text-zinc-100 shadow-[inset_0_1px_0_rgba(255,255,255,0.05)]',
  /** Meta line (role · kind) on assistant bubbles. */
  brandAssistantBubbleMeta: 'text-[var(--vel-color-muted)]',
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
