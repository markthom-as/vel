import type { ActionItemData, NowData } from '../../../types';
import { ActionChipButton, FilterDenseTag, NudgeKindTag, ProjectTag } from '../../../core/FilterToggleTag';
import { TagIcon } from '../../../core/Icons';
import { NowItemRowShell } from '../../../core/NowItemRow';
import { findBarProjectTags, formatNowBarKind, formatRelativeMinutes } from '../nowModel';
import {
  NudgeActionIcon,
  nudgeActionAriaLabel,
  nudgeActionButtonLabel,
  nudgeLeadKindForBar,
  NudgeLeadOrb,
  nudgeKindTagIcon,
} from '../nowNudgePresentation';

export function NowNudgeStrip({
  bars,
  nowTs,
  actionItems,
  onBarAction,
}: {
  bars: NowData['nudge_bars'];
  nowTs: number;
  actionItems: ActionItemData[];
  onBarAction: (
    bar: NowData['nudge_bars'][number],
    action: NowData['nudge_bars'][number]['actions'][number],
  ) => void;
}) {
  if (bars.length === 0) {
    return null;
  }

  return (
    <div className="space-y-2">
      {bars.map((bar) => {
        const warmUrgent = bar.urgent || bar.kind === 'trust_warning' || bar.kind === 'freshness_warning';
        /** Outside the rounded card; `compact` shell uses `overflow-hidden` unless overridden. */
        const isPrimary = bars[0]?.id === bar.id || bar.urgent;

        return (
        <NowItemRowShell
          key={bar.id}
          surface={warmUrgent ? 'warm' : 'brand'}
          shell="compact"
          className="!overflow-visible"
        >
          <div
            className="pointer-events-none absolute -left-11 top-1/2 z-10 -translate-y-1/2"
            aria-hidden
          >
            <NudgeLeadOrb
              kind={bar.kind}
              iconKind={nudgeLeadKindForBar(bar)}
              urgent={bar.urgent}
              warmSurface={warmUrgent}
              isPrimary={isPrimary}
            />
          </div>
          <div className="relative z-10 flex min-w-0 flex-row items-stretch gap-2">
            <div className="min-w-0 flex-1 flex flex-col justify-center gap-1">
              <div className="min-w-0 overflow-x-auto overflow-y-hidden [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
                <div className="flex min-w-0 flex-nowrap items-center justify-start gap-x-1.5">
                  <p className="min-w-0 max-w-[min(100%,15rem)] shrink truncate text-sm font-medium leading-none tracking-tight text-white sm:max-w-[min(100%,20rem)]">
                    {bar.title}
                  </p>
                  <FilterDenseTag tone="ghost">
                    {formatRelativeMinutes(nowTs)}
                  </FilterDenseTag>
                  <span className="shrink-0">
                    <NudgeKindTag urgent={bar.urgent || bar.kind === 'trust_warning' || bar.kind === 'freshness_warning'}>
                      <span aria-hidden className="inline-flex shrink-0 items-center">
                        {nudgeKindTagIcon(bar.kind)}
                      </span>
                      {formatNowBarKind(bar.kind)}
                    </NudgeKindTag>
                  </span>
                  {findBarProjectTags(bar, actionItems).map((tag) => (
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
            <div className="flex shrink-0 flex-col items-end justify-center gap-1.5 self-stretch">
              {bar.actions.map((action, actionIndex) => (
                <ActionChipButton
                  key={`${bar.id}-${actionIndex}-${action.kind}-${action.label}`}
                  onClick={() => onBarAction(bar, action)}
                  aria-label={nudgeActionAriaLabel(bar, action, actionIndex, bar.actions.length)}
                >
                  <NudgeActionIcon kind={action.kind} size={16} className="shrink-0" aria-hidden />
                  <span className="capitalize">{nudgeActionButtonLabel(action, bar)}</span>
                </ActionChipButton>
              ))}
            </div>
          </div>
        </NowItemRowShell>
        );
      })}
    </div>
  );
}
