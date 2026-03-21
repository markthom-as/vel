import type { ReactNode } from 'react';
import { uiTheme } from '../Theme';

export function SurfaceActionChip({
  children,
  tone = 'neutral',
  onClick,
  compact = false,
}: {
  children: ReactNode;
  tone?: 'neutral' | 'accent';
  onClick?: () => void;
  compact?: boolean;
}) {
  const className = `inline-flex items-center gap-1 rounded-full border bg-zinc-950/88 shadow-[0_8px_18px_rgba(0,0,0,0.18)] transition ${
    tone === 'accent'
      ? `${uiTheme.brandBorder} ${uiTheme.brandSoftText} ${uiTheme.brandHoverBorder} hover:text-[#fff0c4]`
      : 'border-zinc-700/85 text-zinc-300 hover:border-zinc-500 hover:text-zinc-100'
  } ${compact ? 'px-1.25 py-[2px] text-[6px] uppercase tracking-[0.06em]' : 'px-1.5 py-[3px] text-[6px] uppercase tracking-[0.06em]'}`;

  if (!onClick) {
    return <span className={className}>{children}</span>;
  }

  return (
    <button type="button" onClick={onClick} className={className}>
      {children}
    </button>
  );
}

export function SurfaceTagChip({
  children,
  tone = 'neutral',
  square = false,
}: {
  children: ReactNode;
  tone?: 'neutral' | 'project' | 'warning' | 'accent';
  square?: boolean;
}) {
  const toneClass =
    tone === 'project'
      ? 'bg-zinc-800/92 text-zinc-100'
      : tone === 'warning'
        ? 'bg-amber-950/68 text-amber-100'
        : tone === 'accent'
          ? `${uiTheme.brandPanel} ${uiTheme.brandSoftText}`
          : 'bg-zinc-900/92 text-zinc-400';

  return (
    <span
      className={`${square ? 'rounded-[4px]' : 'rounded-md'} inline-flex items-center gap-1 px-1.5 py-[3px] text-[6px] uppercase tracking-[0.06em] ${toneClass}`}
    >
      {children}
    </span>
  );
}

export function SurfaceMetricChip({
  children,
  tone = 'neutral',
}: {
  children: ReactNode;
  tone?: 'neutral' | 'accent';
}) {
  return (
    <span
      className={`inline-flex items-center gap-1 rounded-full px-1.75 py-[3px] text-[7px] uppercase tracking-[0.08em] ${
        tone === 'accent' ? `${uiTheme.brandPanel} ${uiTheme.brandSoftText}` : 'bg-zinc-900/70 text-zinc-400'
      }`}
    >
      {children}
    </span>
  );
}
