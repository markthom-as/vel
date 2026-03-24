import { cn } from '../cn';

/** Outer shape shared by filter toggles, panel metrics (Inbox/Now), and compact action pills. */
export const filterPillFrame =
  'inline-flex min-h-[1.45rem] max-w-full items-center justify-center gap-1.25 align-middle rounded-full border px-2.5 py-[0.32rem] leading-none transition [font-variant-numeric:tabular-nums] [line-height:1] [&_svg]:my-0 [&_svg]:block [&_svg]:shrink-0 [&_svg]:self-center [&_svg]:align-middle [&>*]:self-center [&>span]:inline-flex [&>span]:items-center [&>span]:justify-center [&>span]:leading-none';

/** Inbox/Threads filter toggles — unselected (interactive). */
export const filterPillToggleIdle =
  'border-zinc-800 bg-zinc-950/40 text-zinc-500 hover:border-zinc-600 hover:text-zinc-300';

/** Inbox/Threads filter toggles — selected. */
export const filterPillToggleSelected =
  'border-amber-300/55 bg-amber-400/10 text-amber-100 shadow-[0_0_0_1px_rgba(251,191,36,0.12)]';

/** Typography for filter toggle rows. */
export const filterPillToggleTypography = 'text-left text-[11px] font-medium uppercase tracking-[0.12em]';

/**
 * Read-only metrics in header strips (see `PanelMetricStrip` / `PanelMetricChip`).
 * Canonical usage: Now header `NowMetricStrip` — Inbox and other surfaces reuse the same tokens.
 */
export const filterPillMetricIdle = 'border-zinc-800 bg-zinc-950/40 text-zinc-500';
export const filterPillMetricActive = 'border-zinc-700/85 bg-zinc-950/55 text-zinc-200';

export const filterPillCountMuted = 'shrink-0 tabular-nums text-zinc-600 leading-none';
export const filterPillCountMutedStrong = 'shrink-0 tabular-nums text-zinc-400 leading-none';
export const filterPillCountSelected = 'shrink-0 tabular-nums text-amber-200/90 leading-none';

/** Secondary row actions (e.g. Now nudge) — interactive, sentence case. */
export const filterPillActionIdle =
  'border-zinc-800 bg-zinc-950/40 text-zinc-300 hover:border-zinc-600 hover:bg-zinc-900/55 hover:text-zinc-100';

export function filterDenseTagToneClass(tone: 'neutral' | 'muted' | 'brand' | 'ghost') {
  switch (tone) {
    case 'muted':
      return 'border-[var(--vel-color-border)] bg-transparent text-[var(--vel-color-muted)]';
    case 'brand':
      return 'border-[color:var(--vel-color-accent-border)] bg-transparent text-[var(--vel-color-accent-soft)]';
    case 'ghost':
      return 'border-transparent bg-transparent text-zinc-600';
    default:
      return 'border-zinc-800/90 bg-zinc-900/92 text-zinc-400';
  }
}

export function filterPillActionToneClass(tone: 'neutral' | 'muted' | 'brand' | 'success' | 'ghost') {
  switch (tone) {
    case 'muted':
      return 'border-[var(--vel-color-border)] bg-transparent text-[var(--vel-color-muted)] hover:text-[var(--vel-color-text)]';
    case 'brand':
      return 'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)]/55 text-[var(--vel-color-accent-soft)] hover:border-[var(--vel-color-accent-soft)] hover:text-[var(--vel-color-text)]';
    case 'success':
      return 'border-emerald-800/80 bg-emerald-950/50 text-emerald-200 hover:border-emerald-600';
    case 'ghost':
      return 'border-transparent bg-transparent text-zinc-500 hover:text-zinc-200';
    default:
      return filterPillActionIdle;
  }
}

export function filterPillToggleClassNames(selected: boolean, className?: string) {
  return cn(filterPillFrame, filterPillToggleTypography, selected ? filterPillToggleSelected : filterPillToggleIdle, className);
}

export function filterPillMetricClassNames(active: boolean, className?: string) {
  return cn(
    filterPillFrame,
    filterPillToggleTypography,
    active ? filterPillMetricActive : filterPillMetricIdle,
    className,
  );
}
