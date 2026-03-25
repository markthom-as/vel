import { OpenThreadIcon } from '../../core/Icons';
import { cn } from '../../core/cn';
import type { NowNudgeBarData } from '../../types';
import { nudgeActionAriaLabel } from '../../views/now/nowNudgePresentation';

export type CoreChecklistItem = {
  id: string;
  label: string;
  state: 'required' | 'ready';
  value: string | null;
};

export function CoreSetupChecklist({
  bar,
  items,
  onOpenSystemAction,
}: {
  bar: Pick<NowNudgeBarData, 'id' | 'title'>;
  items: Array<{ action: NowNudgeBarData['actions'][number]; checklist: CoreChecklistItem }>;
  onOpenSystemAction: (action: NowNudgeBarData['actions'][number]) => void;
}) {
  if (items.length === 0) {
    return null;
  }

  return (
    <div className="mt-1 flex w-full flex-col gap-1">
      {items.map(({ action, checklist }, index) => (
        <button
          key={`${bar.id}-check-${checklist.id}-${index}`}
          type="button"
          onClick={(event) => {
            event.stopPropagation();
            onOpenSystemAction(action);
          }}
          aria-label={nudgeActionAriaLabel(bar, action, index, items.length)}
          className="flex w-full items-center gap-2 rounded-lg px-1 py-1 text-left transition hover:bg-[var(--vel-color-panel)]/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--vel-color-accent-strong)]/40"
        >
          <span
            aria-hidden
            className={cn(
              'flex h-4 w-4 shrink-0 items-center justify-center rounded-sm border text-[10px] leading-none',
              checklist.state === 'ready'
                ? 'border-emerald-500/40 bg-emerald-950/35 text-emerald-100'
                : 'border-[var(--vel-color-border)] bg-transparent text-transparent',
            )}
          >
            {checklist.state === 'ready' ? '✓' : '·'}
          </span>
          <span className="shrink-0 text-xs leading-5 text-[var(--vel-color-text)]">
            {checklist.label}
          </span>
          {checklist.value ? (
            <span className="min-w-0 flex-1 truncate text-[11px] leading-5 text-[var(--vel-color-muted)]">
              {checklist.value}
            </span>
          ) : null}
          <span
            aria-hidden
            className="ml-auto inline-flex shrink-0 items-center text-[var(--vel-color-muted)]"
            data-testid={`core-setup-open-icon-${checklist.id}`}
          >
            <OpenThreadIcon size={11} />
          </span>
        </button>
      ))}
    </div>
  );
}
