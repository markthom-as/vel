import type { ActionItemData, NowData } from '../../../types';
import { OpenThreadIcon, SparkIcon, TagIcon } from '../../../core/Icons';
import { SurfaceActionChip, SurfaceTagChip } from '../../../core/SurfaceChips';
import { uiTheme } from '../../../core/Theme';
import { findBarProjectTags, formatNowBarKind, formatRelativeMinutes } from '../nowModel';
import { nudgeBadgeTone, nudgeIcon } from '../nowNudgePresentation';

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
      {bars.map((bar, index) => (
        <div
          key={bar.id}
          className={`relative rounded-[20px] border px-4 py-2.5 shadow-[0_10px_30px_rgba(0,0,0,0.2)] ${
            index === 0
              ? `${uiTheme.brandBorder} bg-zinc-900/95 ${uiTheme.brandShadow}`
              : bar.urgent
                ? 'border-amber-700/50 bg-amber-950/18'
                : 'border-zinc-800 bg-zinc-900/55'
          }`}
        >
          <div className="pointer-events-none absolute -left-7 top-1/2 -translate-y-1/2 text-zinc-400">
            <span
              className={`flex items-center justify-center ${index === 0 || bar.urgent ? 'scale-125 animate-pulse' : 'scale-110'} ${uiTheme.brandGlow} ${nudgeBadgeTone(bar.kind, bar.urgent)}`}
            >
              {nudgeIcon(bar.kind)}
            </span>
          </div>
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div className="min-w-0 flex-1 space-y-1">
              <div className="flex min-w-0 items-center gap-2">
                <span className="text-[10px] uppercase tracking-[0.16em] text-zinc-500">{formatRelativeMinutes(nowTs)}</span>
                <p className="truncate text-[11px] text-zinc-400">{bar.summary}</p>
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <p className="text-sm font-medium text-zinc-100">{bar.title}</p>
                <SurfaceTagChip tone={bar.kind === 'trust_warning' ? 'warning' : 'accent'} square>
                  {formatNowBarKind(bar.kind)}
                </SurfaceTagChip>
                {findBarProjectTags(bar, actionItems).map((tag) => (
                  <SurfaceTagChip key={`${bar.id}-${tag}`} tone="project" square>
                    <TagIcon size={11} />
                    {tag}
                  </SurfaceTagChip>
                ))}
              </div>
            </div>
            <div className="flex flex-wrap gap-2">
              {bar.actions.map((action) => (
                <SurfaceActionChip
                  key={`${bar.id}-${action.kind}-${action.label}`}
                  onClick={() => onBarAction(bar, action)}
                >
                  {action.kind === 'open_thread' || action.kind === 'expand' ? (
                    <OpenThreadIcon size={13} />
                  ) : (
                    <SparkIcon size={13} />
                  )}
                  {action.label}
                </SurfaceActionChip>
              ))}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
