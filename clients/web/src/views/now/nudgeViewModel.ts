import type { NowNudgeBarData } from '../../types';
import {
  nudgeFamilySurfaceTone,
  type NudgeFamily,
  type NudgeSurfaceTone,
} from '../../core/Theme/semanticAppearance';

type NudgeLike = Pick<NowNudgeBarData, 'id' | 'kind' | 'urgent' | 'actions'>;

export type NudgeViewModel = {
  family: NudgeFamily;
  leadKind: string;
  surfaceTone: NudgeSurfaceTone;
  warmSurface: boolean;
  isSystem: boolean;
};

export function nudgeUsesSystemPresentation(bar: Pick<NowNudgeBarData, 'id' | 'actions'>): boolean {
  if (
    bar.id === 'core_setup_required'
    || bar.id === 'backup_trust_warning'
    || bar.id === 'mesh_summary_warning'
  ) {
    return true;
  }
  return bar.actions.some((action) => action.kind.startsWith('open_settings'));
}

export function nudgeFamilyForBar(bar: NudgeLike): NudgeFamily {
  if (nudgeUsesSystemPresentation(bar)) {
    return 'system';
  }

  switch (bar.kind) {
    case 'trust_warning':
      return 'warning';
    case 'freshness_warning':
      return 'freshness';
    case 'review_request':
      return 'review';
    case 'reflow_proposal':
      return 'reflow';
    case 'needs_input':
    case 'thread_continuation':
      return 'thread';
    default:
      return 'default';
  }
}

export function nudgeLeadKindForBar(bar: NudgeLike): string {
  return nudgeFamilyForBar(bar) === 'system' ? 'system_settings' : bar.kind;
}

export function nudgeSurfaceToneForFamily(family: NudgeFamily): NudgeSurfaceTone {
  return nudgeFamilySurfaceTone[family] ?? nudgeFamilySurfaceTone.default;
}

export function buildNudgeViewModel(bar: NudgeLike): NudgeViewModel {
  const family = nudgeFamilyForBar(bar);
  const surfaceTone = nudgeSurfaceToneForFamily(family);
  return {
    family,
    leadKind: nudgeLeadKindForBar(bar),
    surfaceTone,
    warmSurface: surfaceTone.warmSurface,
    isSystem: family === 'system',
  };
}
