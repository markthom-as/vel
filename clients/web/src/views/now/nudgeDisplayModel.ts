import type { ActionItemData, NowNudgeBarData } from '../../types';
import { resolveNudgeSemantic } from '../../core/Theme/semanticRegistry';
import { findBarProjectTags } from './nowModel';
import { buildNudgeViewModel, type NudgeViewModel } from './nudgeViewModel';
import { nudgeKindMobileLabel, nudgePresentationPriority } from './nudgePresentationMapping';

export type NudgeDisplayModel = {
  kindIconKind: string;
  kindLabel: string;
  kindUrgent: boolean;
  projectTags: string[];
  viewModel: NudgeViewModel;
};

export function buildNudgeDisplayModel(
  bar: NowNudgeBarData,
  actionItems: ActionItemData[],
): NudgeDisplayModel {
  const viewModel = buildNudgeViewModel(bar);
  const kindSemantic = resolveNudgeSemantic(viewModel.leadKind === 'system_settings' ? viewModel.leadKind : bar.kind);
  const presentationKind = viewModel.leadKind === 'system_settings' ? viewModel.leadKind : bar.kind;
  return {
    kindIconKind: presentationKind,
    kindLabel: kindSemantic.label === 'Nudge' ? nudgeKindMobileLabel(presentationKind) : kindSemantic.label,
    kindUrgent: nudgePresentationPriority(bar.kind, bar.urgent) !== 'normal',
    projectTags: findBarProjectTags(bar, actionItems),
    viewModel,
  };
}
