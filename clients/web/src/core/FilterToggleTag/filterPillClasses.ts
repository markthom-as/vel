import { cn } from '../cn';

/** Outer shape shared by filter toggles, panel metrics (Inbox/Now), and compact action pills. */
export const filterPillFrame =
  'inline-flex max-w-full items-center gap-1.5 rounded-full border px-2.5 py-1.5 transition';

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

export const filterPillCountMuted = 'shrink-0 tabular-nums text-zinc-600';
export const filterPillCountMutedStrong = 'shrink-0 tabular-nums text-zinc-400';
export const filterPillCountSelected = 'shrink-0 tabular-nums text-amber-200/90';

/** Secondary row actions (e.g. Now nudge) — interactive, sentence case. */
export const filterPillActionIdle =
  'border-zinc-800 bg-zinc-950/40 text-zinc-300 hover:border-zinc-600 hover:bg-zinc-900/55 hover:text-zinc-100';

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
