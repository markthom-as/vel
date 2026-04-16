import { CheckCircleIcon, OpenThreadIcon } from '../../core/Icons';
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
  pendingItemId,
  onAcceptSuggestion,
  onOpenSystemAction,
}: {
  bar: Pick<NowNudgeBarData, 'id' | 'title'>;
  items: Array<{ action: NowNudgeBarData['actions'][number]; checklist: CoreChecklistItem }>;
  pendingItemId?: string | null;
  onAcceptSuggestion: (checklist: CoreChecklistItem) => void;
  onOpenSystemAction: (action: NowNudgeBarData['actions'][number]) => void;
}) {
  if (items.length === 0) {
    return null;
  }

  return (
    <div className="mt-1 flex w-full flex-col gap-1">
      {items.map(({ action, checklist }, index) => (
        <div
          key={`${bar.id}-check-${checklist.id}-${index}`}
          className="flex w-full items-center gap-2 rounded-lg px-1 py-1 transition hover:bg-[var(--vel-color-panel)]/30"
        >
          <button
            type="button"
            onClick={(event) => {
              event.stopPropagation();
              onOpenSystemAction(action);
            }}
            aria-label={nudgeActionAriaLabel(bar, action, index, items.length)}
            data-testid={`core-setup-open-icon-${checklist.id}`}
            className="flex min-w-0 flex-1 items-center gap-2 text-left focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--vel-color-accent-strong)]/40"
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
          </button>
          {checklist.value ? (
            checklist.state === 'required' ? (
              <button
                type="button"
                onClick={(event) => {
                  event.stopPropagation();
                  onAcceptSuggestion(checklist);
                }}
                aria-label={`Use ${checklist.value} for ${checklist.label}`}
                disabled={pendingItemId === checklist.id}
                className="ml-auto inline-flex min-w-0 max-w-[55%] shrink items-center justify-end gap-1 rounded-full px-1.5 py-0.5 text-[11px] leading-5 text-[var(--vel-color-muted)] transition hover:bg-[var(--vel-color-panel)]/60 hover:text-[var(--vel-color-text)] disabled:opacity-55 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--vel-color-accent-strong)]/40"
              >
                <span className="min-w-0 truncate">{checklist.value}</span>
                <CheckCircleIcon size={13} className="shrink-0 text-emerald-200" />
              </button>
            ) : (
              <span className="ml-auto inline-flex min-w-0 max-w-[55%] shrink items-center justify-end gap-1 px-1.5 text-[11px] leading-5 text-[var(--vel-color-muted)]">
                <span className="min-w-0 truncate">{checklist.value}</span>
                <CheckCircleIcon size={13} className="shrink-0 text-emerald-200" />
              </span>
            )
          ) : (
            <button
              type="button"
              onClick={(event) => {
                event.stopPropagation();
                onOpenSystemAction(action);
              }}
              aria-label={`Open ${checklist.label}`}
              className="ml-auto inline-flex shrink-0 items-center rounded-full p-1 text-[var(--vel-color-muted)] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--vel-color-accent-strong)]/40"
            >
              <OpenThreadIcon size={11} />
            </button>
          )}
        </div>
      ))}
    </div>
  );
}
