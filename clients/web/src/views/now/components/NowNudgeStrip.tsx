import type { ActionItemData, NowData } from '../../../types';
import { FilterDenseTag, NudgeKindTag, ProjectTag } from '../../../core/FilterToggleTag';
import { cn } from '../../../core/cn';
import { TagIcon } from '../../../core/Icons';
import { NowItemRowShell } from '../../../core/NowItemRow';
import { formatRelativeMinutes } from '../nowModel';
import { buildNudgeDisplayModel } from '../nudgeDisplayModel';
import {
  nudgeActionAriaLabel,
  nudgeActionButtonLabel,
  NudgeLeadOrb,
  nudgeKindTagIcon,
} from '../nowNudgePresentation';
import { NudgeActionButton } from '../NudgeActionButton';

export function NowNudgeStrip({
  bars,
  nowTs,
  actionItems,
  onBarAction,
  surface = 'default',
}: {
  bars: NowData['nudge_bars'];
  nowTs: number;
  actionItems: ActionItemData[];
  onBarAction: (
    bar: NowData['nudge_bars'][number],
    action: NowData['nudge_bars'][number]['actions'][number],
  ) => void;
  surface?: 'default' | 'mobile';
}) {
  if (bars.length === 0) {
    return null;
  }

  return (
    <div className="space-y-2">
      {bars.map((bar) => {
        const displayModel = buildNudgeDisplayModel(bar, actionItems);
        const { viewModel } = displayModel;
        /** Outside the rounded card; `compact` shell uses `overflow-hidden` unless overridden. */
        const isPrimary = bars[0]?.id === bar.id || bar.urgent;

        return (
        <NowItemRowShell
          key={bar.id}
          surface={viewModel.warmSurface ? 'warm' : 'brand'}
          shell="compact"
          className={cn('!overflow-visible', surface === 'mobile' ? 'min-h-14 px-3 py-3' : null)}
        >
          <div
            className="pointer-events-none absolute -left-11 top-1/2 z-10 -translate-y-1/2"
            aria-hidden
          >
            <NudgeLeadOrb
              kind={bar.kind}
              iconKind={viewModel.leadKind}
              urgent={bar.urgent}
              warmSurface={viewModel.warmSurface}
              isPrimary={isPrimary}
            />
          </div>
          <div className={cn('relative z-10 flex min-w-0 flex-row items-stretch gap-2', surface === 'mobile' ? 'gap-3' : null)}>
            <div className="min-w-0 flex-1 flex flex-col justify-center gap-1">
              <div className="min-w-0 overflow-x-auto overflow-y-hidden [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
                <div className="flex min-w-0 flex-nowrap items-center justify-start gap-x-1.5">
                  <p className={cn('min-w-0 shrink truncate text-sm font-medium leading-none tracking-tight text-white', surface === 'mobile' ? 'max-w-[min(100%,11rem)]' : 'max-w-[min(100%,15rem)] sm:max-w-[min(100%,20rem)]')}>
                    {bar.title}
                  </p>
                  <FilterDenseTag tone="ghost">
                    {formatRelativeMinutes(nowTs)}
                  </FilterDenseTag>
                  <span className="shrink-0">
                    <NudgeKindTag urgent={displayModel.kindUrgent}>
                      <span aria-hidden className="inline-flex shrink-0 items-center">
                        {nudgeKindTagIcon(displayModel.kindIconKind)}
                      </span>
                      {displayModel.kindLabel}
                    </NudgeKindTag>
                  </span>
                  {displayModel.projectTags.map((tag) => (
                    <ProjectTag key={`${bar.id}-${tag}`} label={tag}>
                      <span aria-hidden className="inline-flex shrink-0 items-center opacity-80">
                        <TagIcon size={10} />
                      </span>
                      {tag}
                    </ProjectTag>
                  ))}
                </div>
              </div>
              <p className="line-clamp-2 text-xs leading-snug text-zinc-500">{bar.summary}</p>
            </div>
            <div className={cn('flex shrink-0 flex-col items-end justify-center gap-1.5 self-stretch', surface === 'mobile' ? 'min-w-[6.5rem]' : null)}>
              {bar.actions.map((action, actionIndex) => (
                <NudgeActionButton
                  key={`${bar.id}-${actionIndex}-${action.kind}-${action.label}`}
                  kind={action.kind}
                  label={nudgeActionButtonLabel(action, bar)}
                  onClick={() => onBarAction(bar, action)}
                  aria-label={nudgeActionAriaLabel(bar, action, actionIndex, bar.actions.length)}
                  className={surface === 'mobile' ? '!min-h-10 w-full justify-center !px-3' : undefined}
                />
              ))}
            </div>
          </div>
        </NowItemRowShell>
        );
      })}
    </div>
  );
}
