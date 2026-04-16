import { resolveNudgeSemantic } from '../../core/Theme/semanticRegistry';

const MOBILE_NUDGE_LABEL_MAX = 18;

function normalizeNudgeKind(kind: string): string {
  return kind.trim().toLowerCase().replace(/\s+/g, '_');
}

function compactHumanNudgeLabel(kind: string): string {
  const normalized = normalizeNudgeKind(kind);
  if (!normalized || normalized === 'default' || normalized === 'nudge') {
    return 'Nudge';
  }
  const label = normalized
    .split('_')
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
  return label.length > MOBILE_NUDGE_LABEL_MAX ? `${label.slice(0, MOBILE_NUDGE_LABEL_MAX - 3)}...` : label;
}

export type NudgePresentationPriority = 'normal' | 'warning' | 'urgent';

export function nudgePresentationPriority(kind: string, urgent: boolean): NudgePresentationPriority {
  if (urgent) {
    return 'urgent';
  }
  switch (normalizeNudgeKind(kind)) {
    case 'trust_warning':
    case 'freshness_warning':
      return 'warning';
    default:
      return 'normal';
  }
}

export function nudgeKindMobileLabel(kind: string): string {
  const semantic = resolveNudgeSemantic(kind);
  if (semantic.label !== 'Nudge') {
    return semantic.label;
  }
  return compactHumanNudgeLabel(kind);
}
