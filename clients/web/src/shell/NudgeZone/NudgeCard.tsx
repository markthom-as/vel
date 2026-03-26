import type { ReactNode } from 'react';
import { ClockIcon } from '../../core/Icons';
import { cn } from '../../core/cn';
import { FloatingPill } from '../../core/FloatingPill';
import { uiFonts } from '../../core/Theme';
import type { NowNudgeBarData } from '../../types';
import { NudgeLeadOrb } from '../../views/now/nowNudgePresentation';
import type { NudgeViewModel } from '../../views/now/nudgeViewModel';
import { NudgeActionRail } from './NudgeActionRail';

export function NudgeCard({
  bar,
  viewModel,
  isExpanded,
  isFlashing,
  timestampLabel,
  actionButtons,
  checklistContent,
  onToggle,
}: {
  bar: NowNudgeBarData;
  viewModel: NudgeViewModel;
  isExpanded: boolean;
  isFlashing: boolean;
  timestampLabel: string | null;
  actionButtons: ReactNode;
  checklistContent?: ReactNode;
  onToggle: () => void;
}) {
  return (
    <FloatingPill
      decoration={
        <NudgeLeadOrb
          kind={bar.kind}
          iconKind={viewModel.leadKind}
          urgent={bar.urgent}
          warmSurface={viewModel.warmSurface}
          isPrimary={bar.urgent}
        />
      }
      decorationClassName="h-[1.875rem] w-[1.875rem] rounded-none border-0 bg-transparent shadow-none"
      decorationOffsetClassName="-translate-x-[114%]"
      onPress={onToggle}
      contentClassName={cn(
        isExpanded ? 'items-stretch gap-3 py-3' : 'items-stretch',
        viewModel.surfaceTone.shell,
        isExpanded ? viewModel.surfaceTone.activeOutline : null,
        isFlashing
          ? 'ring-2 ring-[var(--vel-color-accent-strong)] ring-offset-2 ring-offset-[var(--vel-color-bg)] shadow-[0_0_0_1px_rgba(255,107,0,0.42),0_0_32px_rgba(255,107,0,0.28)] animate-[pulse_0.38s_ease-in-out_4]'
          : null,
      )}
    >
      {isExpanded ? (
        <div className="-my-3 flex min-w-0 flex-1 self-stretch gap-3 py-3">
          <div className="flex min-w-0 flex-1 flex-col gap-3">
            <button
              type="button"
              className="min-w-0 flex-1 overflow-hidden pt-0.5 text-left"
              onClick={onToggle}
              data-testid={`nudge-toggle-${bar.id}`}
            >
              <div className="flex min-w-0 flex-col gap-1">
                {timestampLabel ? (
                  <span className={`inline-flex items-center gap-1 text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)] ${uiFonts.mono}`}>
                    <ClockIcon size={10} />
                    {timestampLabel}
                  </span>
                ) : null}
                <p className="text-sm font-medium whitespace-normal">{bar.title}</p>
              </div>
            </button>
            <div className="flex w-full flex-col">
              <p className="w-full whitespace-normal text-xs leading-5 text-[var(--vel-color-muted)]">
                {bar.summary}
              </p>
              {checklistContent}
            </div>
          </div>
          <NudgeActionRail isExpanded>
            {actionButtons}
          </NudgeActionRail>
        </div>
      ) : (
        <>
          <button
            type="button"
            className="-my-2.5 min-w-0 flex-1 self-stretch overflow-hidden py-2.5 text-left"
            onClick={onToggle}
            data-testid={`nudge-toggle-${bar.id}`}
          >
            <div className="flex min-w-0 flex-col gap-1">
              {timestampLabel ? (
                <span className={`inline-flex items-center gap-1 text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)] ${uiFonts.mono}`}>
                  <ClockIcon size={10} />
                  {timestampLabel}
                </span>
              ) : null}
              <p className="text-sm font-medium whitespace-normal break-words leading-5">{bar.title}</p>
            </div>
            <p className="truncate text-xs text-[var(--vel-color-muted)]">
              {bar.summary}
            </p>
          </button>
          <NudgeActionRail isExpanded={false}>
            {actionButtons}
          </NudgeActionRail>
        </>
      )}
    </FloatingPill>
  );
}
