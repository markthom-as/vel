import { brandTagPalette } from './tokens';

export type NudgeFamily = 'system' | 'warning' | 'freshness' | 'review' | 'reflow' | 'thread' | 'default';

export type NudgeSurfaceTone = {
  shell: string;
  activeOutline: string;
  warmSurface: boolean;
};

export type PanelStatusTone = 'ok' | 'warn' | 'bad' | 'neutral';

export type SystemStatusTone =
  | 'active'
  | 'warning'
  | 'degraded'
  | 'offline'
  | 'done'
  | 'neutral';

export const nudgeFamilySurfaceTone: Record<NudgeFamily, NudgeSurfaceTone> = {
  system: {
    shell: '!border-indigo-400/38 bg-indigo-950/18 text-indigo-100 shadow-[0_0_0_1px_rgba(99,102,241,0.12)]',
    activeOutline: 'ring-1 ring-indigo-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(129,140,248,0.28),0_0_24px_rgba(99,102,241,0.16)]',
    warmSurface: false,
  },
  warning: {
    shell: '!border-amber-400/45 bg-amber-950/30 text-amber-100 shadow-[0_0_0_1px_rgba(245,158,11,0.12)]',
    activeOutline: 'ring-1 ring-amber-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(251,191,36,0.3),0_0_24px_rgba(245,158,11,0.18)]',
    warmSurface: true,
  },
  freshness: {
    shell: '!border-sky-400/38 bg-sky-950/25 text-sky-100 shadow-[0_0_0_1px_rgba(14,165,233,0.1)]',
    activeOutline: 'ring-1 ring-sky-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(56,189,248,0.28),0_0_24px_rgba(14,165,233,0.16)]',
    warmSurface: false,
  },
  review: {
    shell: '!border-emerald-400/34 bg-emerald-950/20 text-emerald-100 shadow-[0_0_0_1px_rgba(16,185,129,0.1)]',
    activeOutline: 'ring-1 ring-emerald-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(52,211,153,0.28),0_0_24px_rgba(16,185,129,0.16)]',
    warmSurface: false,
  },
  reflow: {
    shell: '!border-orange-400/38 bg-orange-950/20 text-orange-100 shadow-[0_0_0_1px_rgba(249,115,22,0.1)]',
    activeOutline: 'ring-1 ring-orange-400/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(251,146,60,0.28),0_0_24px_rgba(249,115,22,0.16)]',
    warmSurface: true,
  },
  thread: {
    shell: '!border-[color:rgba(255,107,0,0.34)] bg-[color:var(--vel-color-panel)]/82 text-[var(--vel-color-text)] shadow-[0_0_0_1px_rgba(255,107,0,0.1)]',
    activeOutline: 'ring-1 ring-[var(--vel-color-accent-strong)]/80 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(255,107,0,0.34),0_0_24px_rgba(255,107,0,0.18)]',
    warmSurface: false,
  },
  default: {
    shell: '!border-[color:rgba(255,107,0,0.24)] bg-[color:var(--vel-color-panel)]/78 text-[var(--vel-color-text)] shadow-[0_0_0_1px_rgba(255,107,0,0.08)]',
    activeOutline: 'ring-1 ring-[var(--vel-color-accent-border)]/70 ring-offset-1 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(255,107,0,0.22),0_0_20px_rgba(255,107,0,0.12)]',
    warmSurface: false,
  },
};

function projectTagHash(label: string): number {
  const normalized = label.trim().toLowerCase();
  if (normalized.length === 0) {
    return 0;
  }
  return Array.from(normalized).reduce((sum, char) => sum + char.charCodeAt(0), 0);
}

export function projectTagAppearance(label: string): string {
  return brandTagPalette[projectTagHash(label) % brandTagPalette.length] ?? brandTagPalette[0] ?? '';
}

export function nudgeKindTagAppearance(urgent: boolean): string {
  return urgent
    ? 'border-amber-600/45 bg-amber-950/70 text-amber-100'
    : 'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)]/80 text-[var(--vel-color-accent-soft)]';
}

export function calendarFilterChipAppearance(active: boolean): string {
  return active
    ? '!border-[var(--vel-color-accent-border)] !bg-[color:var(--vel-color-panel-2)]'
    : '';
}

export function panelStatusChipAppearance(tone: PanelStatusTone): string {
  switch (tone) {
    case 'ok':
      return 'bg-emerald-500/20 text-emerald-200 border-emerald-500/30';
    case 'warn':
      return 'bg-amber-500/20 text-amber-200 border-amber-500/30';
    case 'bad':
      return 'bg-rose-500/20 text-rose-200 border-rose-500/30';
    case 'neutral':
    default:
      return 'bg-zinc-700/40 text-zinc-300 border-zinc-600/50';
  }
}

export function panelStatusToneForSync(status: string | null): PanelStatusTone {
  if (status === 'ok' || status === 'success') return 'ok';
  if (status === 'error' || status === 'failed') return 'bad';
  if (status === 'stale' || status === 'warning') return 'warn';
  return 'neutral';
}

export function panelStatusToneForComponent(status: string): PanelStatusTone {
  if (status === 'healthy' || status === 'running') return 'ok';
  if (status === 'degraded') return 'warn';
  if (status === 'failed' || status === 'error') return 'bad';
  return 'neutral';
}

export function systemStatusChipAppearance(tone: SystemStatusTone): string {
  switch (tone) {
    case 'active':
      return 'border-[#b96e3a]/50 bg-[#2d1608] text-[#ffd4b8]';
    case 'warning':
      return 'border-amber-500/40 bg-amber-950/50 text-amber-100';
    case 'degraded':
      return 'border-orange-500/40 bg-orange-950/50 text-orange-100';
    case 'offline':
      return 'border-slate-500/40 bg-slate-950/60 text-slate-200';
    case 'done':
      return 'border-emerald-500/35 bg-emerald-950/40 text-emerald-100';
    case 'neutral':
    default:
      return 'border-zinc-700 bg-zinc-900/80 text-zinc-300';
  }
}
