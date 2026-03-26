import type { ActionItemData, NowNudgeBarData } from '../../types';
import { resolveNudgeSemantic } from '../../core/Theme/semanticRegistry';
import { findBarProjectTags } from './nowModel';
import { buildNudgeViewModel, type NudgeViewModel } from './nudgeViewModel';

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
  return {
    kindIconKind: viewModel.leadKind === 'system_settings' ? viewModel.leadKind : bar.kind,
    kindLabel: kindSemantic.label,
    kindUrgent: bar.urgent || bar.kind === 'trust_warning' || bar.kind === 'freshness_warning',
    projectTags: findBarProjectTags(bar, actionItems),
    viewModel,
  };
}
