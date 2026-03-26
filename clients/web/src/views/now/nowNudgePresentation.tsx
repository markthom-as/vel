import type { ReactNode } from 'react';
import { cn } from '../../core/cn';
import { SemanticIcon } from '../../core/Icons/SemanticIcon';
import {
  CalendarSyncIcon,
  ClockPlusIcon,
  ClipboardCheckIcon,
  EyeIcon,
  ClockIcon,
  DotIcon,
  InboxIcon,
  SettingsIcon,
  SparkIcon,
  ThreadsIcon,
  WarningIcon,
} from '../../core/Icons';
import type { IconProps } from '../../core/Icons/IconGlyphs';
import { uiTheme } from '../../core/Theme';
import { resolveNudgeSemantic } from '../../core/Theme/semanticRegistry';
import { nudgeFamilyForBar } from './nudgeViewModel';

const NUDGE_TAG_ICON = 10;
/** Lead glyph (floating left of the nudge card). */
const NUDGE_LEAD_ICON = 14;

export function nudgeIcon(kind: string, urgent = false): ReactNode {
  if (urgent && kind === 'nudge') {
    return <WarningIcon size={NUDGE_LEAD_ICON} />;
  }
  const semantic = resolveNudgeSemantic(kind);
  return <SemanticIcon icon={semantic.icon} size={NUDGE_LEAD_ICON} />;
}

/** Default icon for nudge kind tags (dense; matches `nudgeIcon` semantics). */
export function nudgeKindTagIcon(kind: string): ReactNode {
  const semantic = resolveNudgeSemantic(kind);
  return <SemanticIcon icon={semantic.icon} size={NUDGE_TAG_ICON} />;
}

function nudgeOrbColorFamily(kind: string): 'brand' | 'warm' | 'sky' | 'emerald' | 'orange' | 'indigo' {
  if (kind === 'system_settings') {
    return 'indigo';
  }
  const family = nudgeFamilyForBar({ id: kind, kind, urgent: false, actions: [] });
  switch (family) {
    case 'system':
      return 'indigo';
    case 'warning':
      return 'warm';
    case 'freshness':
      return 'sky';
    case 'review':
      return 'emerald';
    case 'reflow':
      return 'orange';
    case 'thread':
    case 'default':
    default:
      return 'brand';
  }
}

export function nudgeBadgeTone(kind: string, urgent: boolean): string {
  switch (nudgeOrbColorFamily(kind)) {
    case 'indigo':
      return 'text-indigo-300';
    case 'warm':
      return 'text-amber-300';
    case 'sky':
      return 'text-sky-300';
    case 'emerald':
      return 'text-emerald-300';
    case 'orange':
      return 'text-orange-300';
    case 'brand':
    default:
      return urgent ? 'text-[var(--vel-color-accent-strong)]' : uiTheme.brandSoftText;
  }
}

/**
 * Floating lead marker: slightly inset gradient ring (shimmer + glow) and a soft glint on the glyph.
 * Styles: `index.css` — `.vel-nudge-orb-ring--*`, `.vel-nudge-orb-icon`.
 */
export function NudgeLeadOrb({
  kind,
  urgent,
  warmSurface,
  isPrimary,
  iconKind,
}: {
  kind: string;
  urgent: boolean;
  warmSurface: boolean;
  isPrimary: boolean;
  iconKind?: string;
}) {
  const resolvedKind = iconKind ?? kind;
  const family = warmSurface && nudgeOrbColorFamily(resolvedKind) === 'brand' ? 'warm' : nudgeOrbColorFamily(resolvedKind);
  const ringClass = `vel-nudge-orb-ring--${family}`;
  const iconClass = `vel-nudge-orb-icon--${family}`;

  return (
    <div
      className={cn(
        'relative size-[1.875rem] shrink-0',
        isPrimary ? 'scale-100' : 'scale-[0.98]',
      )}
    >
      <div
        className={cn(
          'pointer-events-none absolute inset-[2px] rounded-full p-[1.5px]',
          ringClass,
        )}
        aria-hidden
      >
        <div className="flex h-full w-full items-center justify-center rounded-full bg-zinc-950">
          <span
            className={cn(
              'vel-nudge-orb-icon flex items-center justify-center',
              iconClass,
              nudgeBadgeTone(resolvedKind, urgent),
            )}
          >
            {nudgeIcon(resolvedKind, urgent)}
          </span>
        </div>
      </div>
    </div>
  );
}

export function taskKindIcon(kind: string): ReactNode {
  switch (kind) {
    case 'commitment':
      return <ClockIcon size={12} />;
    case 'task':
      return <ClipboardCheckIcon size={12} />;
    default:
      return <DotIcon size={12} />;
  }
}

export function formatTaskKindLabel(kind: string): string {
  return kind.replaceAll('_', ' ');
}

/** Icon-only nudge actions: thread-style actions use the open / external-link glyph. */
export function NudgeActionIcon({ kind, ...props }: IconProps & { kind: string }) {
  switch (kind) {
    case 'open_thread':
    case 'expand':
    case 'accept':
      return <ThreadsIcon {...props} />;
    case 'snooze':
      return <ClockPlusIcon {...props} />;
    case 'open_settings':
      return <SettingsIcon {...props} />;
    case 'open_inbox':
      return <InboxIcon {...props} />;
    default:
      if (kind.startsWith('reschedule_today')) {
        return <CalendarSyncIcon {...props} />;
      }
      if (kind.startsWith('jump_backlog')) {
        return <EyeIcon {...props} />;
      }
      if (kind.startsWith('open_settings')) {
        return <SettingsIcon {...props} />;
      }
      return <SparkIcon {...props} />;
  }
}

/** Muted color per action kind so each button is visually distinct. */
export function nudgeActionToneClass(kind: string): string {
  const base = 'whitespace-nowrap';
  switch (kind) {
    case 'open_thread':
    case 'expand':
    case 'accept':
      return `${base} !border-emerald-900/40 !text-emerald-400/55 shadow-[0_0_6px_rgba(52,211,153,0.08)] hover:!text-emerald-200/80`;
    case 'snooze':
      return `${base} !border-amber-900/40 !text-amber-400/55 shadow-[0_0_6px_rgba(251,191,36,0.08)] hover:!text-amber-200/80`;
    case 'open_inbox':
      return `${base} !border-orange-900/40 !text-orange-400/55 shadow-[0_0_6px_rgba(251,146,60,0.08)] hover:!text-orange-200/80`;
    case 'open_settings':
      return `${base} !border-indigo-900/40 !text-indigo-400/60 shadow-[0_0_6px_rgba(129,140,248,0.08)] hover:!text-indigo-200/85`;
    default:
      if (kind.startsWith('reschedule_today')) {
        return `${base} !border-sky-900/40 !text-sky-400/55 shadow-[0_0_6px_rgba(56,189,248,0.08)] hover:!text-sky-200/80`;
      }
      if (kind.startsWith('jump_backlog')) {
        return `${base} !border-violet-900/40 !text-violet-400/55 shadow-[0_0_6px_rgba(167,139,250,0.08)] hover:!text-violet-200/80`;
      }
      if (kind.startsWith('open_settings')) {
        return `${base} !border-indigo-900/40 !text-indigo-400/60 shadow-[0_0_6px_rgba(129,140,248,0.08)] hover:!text-indigo-200/85`;
      }
      return base;
  }
}

/** Visible label next to the action icon (short; deep-link context is encoded in navigation, not API copy). */
export function nudgeActionButtonLabel(action: { kind: string; label: string }, bar: { id: string }): string {
  if (action.kind.startsWith('open_settings:core_settings:')) {
    return action.label;
  }
  switch (action.kind) {
    case 'open_thread':
    case 'expand':
    case 'accept':
      return 'Open';
    case 'open_inbox':
      return 'Inbox';
    case 'open_settings':
      if (bar.id === 'core_setup_required') return 'Core settings';
      if (bar.id === 'backup_trust_warning') return 'Backups';
      if (bar.id === 'mesh_summary_warning') return 'Sync & clients';
      return 'Settings';
    default:
      if (action.kind.startsWith('reschedule_today')) {
        return 'To Today';
      }
      if (action.kind.startsWith('jump_backlog')) {
        return 'Backlog';
      }
      if (action.kind.startsWith('open_settings')) {
        return action.label;
      }
      return action.label.length > 28 ? `${action.label.slice(0, 25)}…` : action.label;
  }
}

/** Accessible name: primary action label + nudge context (for duplicate-safe queries). */
export function nudgeActionAriaLabel(
  bar: { id: string; title: string },
  action: { kind: string; label: string },
  actionIndex: number,
  actionCount: number,
): string {
  const primary = nudgeActionButtonLabel(action, bar);
  const context = ` (${bar.title}) · ${bar.id}`;
  const indexPart = actionCount > 1 ? ` · ${actionIndex + 1} of ${actionCount}` : '';
  return `${primary}${context}${indexPart}`;
}
