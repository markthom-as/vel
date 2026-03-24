import type { ReactNode } from 'react';
import { uiFonts } from '../Theme';
import { cn } from '../cn';
import { ObjectCard } from '../ObjectCard';
import { ObjectRowFrame } from '../ObjectRow';
import { objectRowFrameClass } from '../ObjectRow/objectRowFrameClass';
import { SurfaceTagChip } from '../SurfaceChips';

type PanelBandSurface = 'brand' | 'warm' | 'muted' | 'emphasis' | 'ghost' | 'queue';

/** Temporary compatibility wrapper over the canonical `ObjectCard`. */
export function PanelPageSection({
  children,
  className,
  as: Comp = 'section',
}: {
  children: ReactNode;
  className?: string;
  as?: 'section' | 'div';
}) {
  return (
    <ObjectCard as={Comp} kind="default" className={cn('p-5', className)}>
      {children}
    </ObjectCard>
  );
}

export function PanelSectionHeader({ title, description }: { title: string; description?: string }) {
  return (
    <>
      <h2 className={cn('text-lg font-medium text-zinc-100', uiFonts.display)}>{title}</h2>
      {description ? <p className="mt-1 text-sm text-zinc-400">{description}</p> : null}
    </>
  );
}

/** Tiny uppercase label (info panel headers, doc sections, form field labels). */
export function PanelEyebrow({
  children,
  className,
  tracking = 'standard',
  as: Comp = 'p',
}: {
  children: ReactNode;
  className?: string;
  /** `wide` matches documentation / App info chrome; `standard` matches settings form labels. */
  tracking?: 'wide' | 'standard';
  as?: 'p' | 'span';
}) {
  return (
    <Comp
      className={cn(
        'text-[10px] uppercase text-zinc-500',
        tracking === 'wide' ? 'tracking-[0.24em]' : 'tracking-[0.18em]',
        className,
      )}
    >
      {children}
    </Comp>
  );
}

export type PanelSectionHeaderBandMode = 'pill' | 'section-header';

/**
 * Title + trailing metrics on one row. Lead/trail slots see {@link PanelSectionHeaderLead} / {@link PanelSectionHeaderTrail}.
 *
 * - **`pill`** (default): full `itemPill` band — border, padding, rounded shape, surface background.
 * - **`section-header`**: layout only (flex row, gaps); no border, padding, or background — flush on page chrome (e.g. Now hero).
 */
export function PanelSectionHeaderBand({
  surface = 'muted',
  mode = 'pill',
  className,
  children,
}: {
  surface?: PanelBandSurface;
  mode?: PanelSectionHeaderBandMode;
  className?: string;
  children: ReactNode;
}) {
  return (
    <div
      className={cn(
        mode === 'pill'
          ? objectRowFrameClass(
              surface === 'brand'
                ? 'accent'
                : surface === 'warm'
                  ? 'warning'
                  : surface === 'emphasis'
                    ? 'emphasis'
                    : surface === 'ghost'
                      ? 'ghost'
                      : 'neutral',
              'sectionHeader',
            )
          : 'flex min-w-0 flex-row flex-wrap items-start justify-between gap-x-3 gap-y-2',
        className,
      )}
    >
      {children}
    </div>
  );
}

export function PanelSectionHeaderLead({ children, className }: { children: ReactNode; className?: string }) {
  return <div className={cn('min-w-0 flex-1', className)}>{children}</div>;
}

export function PanelSectionHeaderTrail({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <div className={cn('flex shrink-0 flex-wrap items-center justify-end gap-2', className)}>{children}</div>
  );
}

/** Metric / summary tiles (Stats grid, Context drift grid). */
export function PanelStatTile({
  label,
  value,
  detail,
  density = 'comfortable',
}: {
  label: string;
  value: string;
  /** Shown below the value when `density` is `"comfortable"` (default). */
  detail?: string;
  density?: 'comfortable' | 'compact';
}) {
  if (density === 'compact') {
    return (
      <ObjectCard kind="subtle" className="p-2">
        <p className="text-xs uppercase tracking-wide text-zinc-500">{label}</p>
        <p className="mt-1 text-zinc-100">{value}</p>
      </ObjectCard>
    );
  }
  return (
    <ObjectCard kind="subtle" className="px-4 py-3">
      <p className="text-xs uppercase tracking-wide text-zinc-500">{label}</p>
      <p className="mt-1 text-xl font-medium text-zinc-100">{value}</p>
      <p className="mt-1 text-xs text-zinc-400">{detail ?? ''}</p>
    </ObjectCard>
  );
}

/** Inner list rows (integrations, loops, components). */
export function PanelDenseRow({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <ObjectRowFrame tone="neutral" density="standard" className={cn('p-3', className)}>
      {children}
    </ObjectRowFrame>
  );
}

/** Nested key/value blocks (Projects detail DL, form callouts). */
export function PanelInsetCard({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <ObjectCard kind="config" className={cn('p-4', className)}>
      {children}
    </ObjectCard>
  );
}

/** Dashed empty state row. */
export function PanelEmptyRow({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <p
      className={cn(
        'rounded-xl border border-dashed border-zinc-800 bg-zinc-950/60 px-4 py-3 text-sm text-zinc-500',
        className,
      )}
    >
      {children}
    </p>
  );
}

export function PanelCallout({
  tone,
  children,
  className,
}: {
  tone: 'warning' | 'danger' | 'success';
  children: ReactNode;
  className?: string;
}) {
  const toneCls =
    tone === 'warning'
      ? 'border-amber-500/40 bg-amber-500/10 text-amber-100'
      : tone === 'danger'
        ? 'border-rose-500/40 bg-rose-500/10 text-rose-200'
        : 'border-emerald-500/40 bg-emerald-500/10 text-emerald-100';
  return (
    <div className={cn('rounded-xl border px-4 py-3 text-sm', toneCls, className)}>{children}</div>
  );
}

/** Full-width selectable list row (Suggestions, Projects registry). */
export function PanelSelectableListButton({
  selected,
  selectionAccent = 'emerald',
  onClick,
  children,
  className,
}: {
  selected: boolean;
  selectionAccent?: 'emerald' | 'amber';
  onClick: () => void;
  children: ReactNode;
  className?: string;
}) {
  const ring =
    selectionAccent === 'emerald'
      ? 'border-emerald-500/80 bg-emerald-500/10 ring-1 ring-emerald-500/35'
      : 'border-amber-500/80 bg-amber-500/10 ring-1 ring-amber-500/35';
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        objectRowFrameClass('neutral', 'comfortable'),
        'w-full text-left transition',
        selected ? ring : 'hover:border-zinc-600',
        className,
      )}
    >
      {children}
    </button>
  );
}

export function PanelJsonPre({
  children,
  compact = false,
  className,
}: {
  children: ReactNode;
  compact?: boolean;
  className?: string;
}) {
  return (
    <pre
      className={cn(
        objectRowFrameClass('neutral', 'standard'),
        'mt-3 overflow-x-auto px-4 py-3 font-mono text-xs text-zinc-300',
        compact ? '' : 'whitespace-pre-wrap',
        className,
      )}
    >
      {children}
    </pre>
  );
}

export function PanelDetailShell({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <ObjectCard kind="default" className={cn('p-5', className)}>
      {children}
    </ObjectCard>
  );
}

type StatusTone = 'ok' | 'warn' | 'bad' | 'neutral';

const STATUS_CHIP: Record<StatusTone, string> = {
  ok: 'bg-emerald-500/20 text-emerald-200 border-emerald-500/30',
  warn: 'bg-amber-500/20 text-amber-200 border-amber-500/30',
  bad: 'bg-rose-500/20 text-rose-200 border-rose-500/30',
  neutral: 'bg-zinc-700/40 text-zinc-300 border-zinc-600/50',
};

export function PanelStatusChip({ tone, children }: { tone: StatusTone; children: ReactNode }) {
  return (
    <SurfaceTagChip square className={cn('rounded-full border px-2 py-1 text-[11px]', STATUS_CHIP[tone])}>
      {children}
    </SurfaceTagChip>
  );
}

export function syncStatusTone(status: string | null): StatusTone {
  if (status === 'ok' || status === 'success') return 'ok';
  if (status === 'error' || status === 'failed') return 'bad';
  if (status === 'stale' || status === 'warning') return 'warn';
  return 'neutral';
}

export function componentHealthTone(status: string): StatusTone {
  if (status === 'healthy' || status === 'running') return 'ok';
  if (status === 'degraded') return 'warn';
  if (status === 'failed' || status === 'error') return 'bad';
  return 'neutral';
}

export function PanelKeyValueRow({ label, value }: { label: string; value: string }) {
  return (
    <ObjectRowFrame tone="neutral" density="standard" className="px-3 py-2">
      <p className="text-xs text-zinc-500">{label}</p>
      <p className="mt-1 break-words text-zinc-200">{value}</p>
    </ObjectRowFrame>
  );
}

export function PanelListBullet({ children }: { children: ReactNode }) {
  return (
    <li className={cn(objectRowFrameClass('neutral', 'standard'), 'px-3 py-2 text-sm text-zinc-200')}>{children}</li>
  );
}

export function PanelListBulletMuted({ children }: { children: ReactNode }) {
  return (
    <li className={cn(objectRowFrameClass('ghost', 'standard'), 'px-3 py-2 text-sm text-zinc-300')}>
      {children}
    </li>
  );
}

export function PanelDebugBlock({ title, children }: { title: string; children: ReactNode }) {
  return (
    <ObjectCard kind="subtle" className="p-3">
      <p className="mb-2 text-xs text-zinc-400">{title}</p>
      {children}
    </ObjectCard>
  );
}

/** Top-of-page intro strip (Projects). */
export function PanelIntroStrip({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <ObjectCard as="section" kind="subtle" className={cn('mb-6 p-4', className)}>
      {children}
    </ObjectCard>
  );
}

/** Tight inset block (Context drift card, sidebar). */
export function PanelMutedInset({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <ObjectCard kind="subtle" className={cn('p-3', className)}>
      {children}
    </ObjectCard>
  );
}
