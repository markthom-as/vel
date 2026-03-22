import { cn } from '../cn';

export type ButtonVariant =
  | 'primary'
  | 'secondary'
  | 'outline'
  | 'ghost'
  | 'danger'
  /** Soft success tone for confirmations / positive status affordances */
  | 'success'
  /** Soft warning tone for caution / attention without destructive intent */
  | 'warning';
export type ButtonSize =
  | 'sm'
  | 'md'
  | 'lg'
  /** Square icon-only targets (pair with aria-label). */
  | 'icon-sm'
  | 'icon'
  | 'icon-lg';

const variantClasses: Record<ButtonVariant, string> = {
  primary:
    'border border-[#ff8f40]/45 bg-gradient-to-br from-[#ffc49a] via-[#ff6b00] to-[#9a3412] text-white ' +
    'shadow-[inset_0_1px_0_rgba(255,255,255,0.28),0_4px_18px_-2px_rgba(255,107,0,0.42),0_1px_0_rgba(0,0,0,0.2)] ' +
    'hover:from-[#ffd4b8] hover:via-[#ff7a1a] hover:to-[#b45309] hover:shadow-[inset_0_1px_0_rgba(255,255,255,0.32),0_6px_22px_-2px_rgba(255,107,0,0.5)] ' +
    'active:translate-y-px active:brightness-[0.98] ' +
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#ff8f40]/60 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ' +
    'disabled:pointer-events-none disabled:opacity-40',
  secondary:
    'border border-zinc-500/50 bg-gradient-to-br from-zinc-500/95 via-zinc-700 to-zinc-950 text-zinc-50 ' +
    'shadow-[inset_0_1px_0_rgba(255,255,255,0.12),0_2px_8px_rgba(0,0,0,0.35)] ' +
    'hover:from-zinc-500 hover:via-zinc-600 hover:to-zinc-950 hover:border-zinc-400/40 ' +
    'active:translate-y-px ' +
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-400/40 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ' +
    'disabled:pointer-events-none disabled:opacity-40',
  outline:
    'border border-zinc-600 bg-zinc-950/40 text-zinc-100 shadow-none ' +
    'hover:bg-zinc-800/80 hover:border-zinc-500 active:translate-y-px ' +
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-500/40 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ' +
    'disabled:pointer-events-none disabled:opacity-40',
  ghost:
    'border border-transparent bg-transparent text-zinc-400 shadow-none ' +
    'hover:bg-zinc-900 hover:text-zinc-100 ' +
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-500/35 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ' +
    'disabled:pointer-events-none disabled:opacity-40',
  danger:
    'border border-rose-400/40 bg-gradient-to-br from-rose-400 via-rose-600 to-rose-950 text-white ' +
    'shadow-[inset_0_1px_0_rgba(255,255,255,0.18),0_4px_14px_-2px_rgba(225,29,72,0.35)] ' +
    'hover:from-rose-300 hover:via-rose-500 hover:to-rose-900 ' +
    'active:translate-y-px ' +
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-rose-400/50 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ' +
    'disabled:pointer-events-none disabled:opacity-40',
  success:
    'border border-emerald-500/45 bg-emerald-950/55 text-emerald-50 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)] ' +
    'hover:bg-emerald-900/70 hover:border-emerald-400/50 active:translate-y-px ' +
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-500/45 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ' +
    'disabled:pointer-events-none disabled:opacity-40',
  warning:
    'border border-amber-500/45 bg-amber-950/45 text-amber-50 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] ' +
    'hover:bg-amber-900/55 hover:border-amber-400/50 active:translate-y-px ' +
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-500/45 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ' +
    'disabled:pointer-events-none disabled:opacity-40',
};

const sizeClasses: Record<ButtonSize, string> = {
  sm: 'min-h-8 px-2.5 py-1.5 text-xs',
  md: 'min-h-9 px-3.5 py-2 text-sm',
  lg: 'min-h-10 px-4 py-2.5 text-base',
  'icon-sm': 'h-8 w-8 min-h-8 min-w-8 shrink-0 p-0 text-xs [&_svg]:h-3.5 [&_svg]:w-3.5',
  icon: 'h-9 w-9 min-h-9 min-w-9 shrink-0 p-0 text-sm [&_svg]:h-[18px] [&_svg]:w-[18px]',
  'icon-lg': 'h-10 w-10 min-h-10 min-w-10 shrink-0 p-0 text-base [&_svg]:h-5 [&_svg]:w-5',
};

const base =
  'inline-flex shrink-0 items-center justify-center gap-2 rounded-md font-medium normal-case tracking-normal transition-[color,background-color,box-shadow,transform,filter] duration-150 select-none';

/** Settings/configuration surfaces: align action buttons to the trailing edge. */
export const settingsFormActionsClass =
  'flex flex-wrap items-center justify-end gap-3';

export function isIconButtonSize(size: ButtonSize): boolean {
  return size === 'icon-sm' || size === 'icon' || size === 'icon-lg';
}

/** Class string for default app buttons (settings, forms, configuration). Not for nudge SurfaceActionChip. */
export function buttonClasses(options: {
  variant?: ButtonVariant;
  size?: ButtonSize;
  className?: string;
  fullWidth?: boolean;
  loading?: boolean;
}): string {
  const { variant = 'primary', size = 'md', className, fullWidth, loading } = options;
  return cn(
    base,
    variantClasses[variant],
    sizeClasses[size],
    fullWidth && 'w-full',
    loading && 'cursor-wait',
    className,
  );
}
