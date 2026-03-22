import type { ReactNode } from 'react';
import { cn } from '../cn';
import { uiTheme } from '../Theme';

export function SurfaceActionChip({
  children,
  tone = 'neutral',
  onClick,
  compact = false,
  className,
  'aria-label': ariaLabel,
}: {
  children: ReactNode;
  tone?: 'neutral' | 'accent';
  onClick?: () => void;
  compact?: boolean;
  className?: string;
  'aria-label'?: string;
}) {
  const sizeClass = compact
    ? 'gap-0.5 px-1.5 py-px text-[10px] uppercase tracking-[0.07em] leading-none [&_svg]:!h-2.5 [&_svg]:!w-2.5 [&_svg]:!max-h-2.5 [&_svg]:!max-w-2.5 [&_svg]:shrink-0 [&_svg]:block'
    : 'gap-1 px-1.5 py-0.5 text-[11px] uppercase tracking-[0.08em] leading-none [&_svg]:!h-[11px] [&_svg]:!w-[11px] [&_svg]:!max-h-[11px] [&_svg]:!max-w-[11px] [&_svg]:shrink-0 [&_svg]:block';

  const classNameMerged = `inline-flex items-center rounded-full border bg-zinc-950/88 shadow-[0_3px_10px_rgba(0,0,0,0.14)] transition ${
    tone === 'accent'
      ? `${uiTheme.brandBorder} ${uiTheme.brandSoftText} ${uiTheme.brandHoverBorder} hover:text-[#fff0c4]`
      : 'border-zinc-700/85 text-zinc-300 hover:border-zinc-500 hover:text-zinc-100'
  } ${sizeClass}${className ? ` ${className}` : ''}`;

  if (!onClick) {
    return <span className={classNameMerged}>{children}</span>;
  }

  return (
    <button type="button" onClick={onClick} className={classNameMerged} aria-label={ariaLabel}>
      {children}
    </button>
  );
}

export function SurfaceTagChip({
  children,
  tone = 'neutral',
  square = false,
  className,
}: {
  children: ReactNode;
  tone?: 'neutral' | 'project' | 'warning' | 'accent';
  square?: boolean;
  /** When set, overrides `tone` (use with `projectTagClasses` from `nowModel`). */
  className?: string;
}) {
  const toneClass =
    className
      ? ''
      : tone === 'project'
        ? cn('border', uiTheme.brandBorder, uiTheme.brandPanel, uiTheme.brandSoftText)
        : tone === 'warning'
          ? 'bg-amber-950/68 text-amber-100'
          : tone === 'accent'
            ? `${uiTheme.brandPanel} ${uiTheme.brandSoftText}`
            : 'bg-zinc-900/92 text-zinc-400';

  return (
    <span
      className={`${square ? 'rounded-[6px]' : 'rounded-md'} inline-flex items-center gap-1.5 border border-transparent px-2 py-1 text-xs uppercase tracking-[0.08em] leading-none [&_svg]:shrink-0 [&_svg]:block ${toneClass} ${className ?? ''}`.trim()}
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
      className={`inline-flex items-center gap-1.5 rounded-full px-2 py-1 text-xs uppercase tracking-[0.1em] leading-none [&_svg]:shrink-0 [&_svg]:block ${
        tone === 'accent' ? `${uiTheme.brandPanel} ${uiTheme.brandSoftText}` : 'bg-zinc-900/70 text-zinc-400'
      }`}
    >
      {children}
    </span>
  );
}
