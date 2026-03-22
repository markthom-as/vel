import type { ReactNode } from 'react';
import { cn } from '../../core/cn';
import {
  ClipboardCheckIcon,
  ClockIcon,
  DotIcon,
  InboxIcon,
  OpenThreadIcon,
  SettingsIcon,
  SparkIcon,
  ThreadsIcon,
  WarningIcon,
} from '../../core/Icons';
import type { IconProps } from '../../core/Icons/IconGlyphs';
import { uiTheme } from '../../core/Theme';

const NUDGE_TAG_ICON = 10;
/** Lead glyph (floating left of the nudge card). */
const NUDGE_LEAD_ICON = 16;

export function nudgeIcon(kind: string): ReactNode {
  switch (kind) {
    case 'needs_input':
      return <ThreadsIcon size={NUDGE_LEAD_ICON} />;
    case 'trust_warning':
    case 'freshness_warning':
      return <WarningIcon size={NUDGE_LEAD_ICON} />;
    case 'review_request':
      return <OpenThreadIcon size={NUDGE_LEAD_ICON} />;
    default:
      return <SparkIcon size={NUDGE_LEAD_ICON} />;
  }
}

/** Default icon for nudge kind tags (dense; matches `nudgeIcon` semantics). */
export function nudgeKindTagIcon(kind: string): ReactNode {
  switch (kind) {
    case 'needs_input':
      return <ThreadsIcon size={NUDGE_TAG_ICON} />;
    case 'trust_warning':
    case 'freshness_warning':
      return <WarningIcon size={NUDGE_TAG_ICON} />;
    case 'review_request':
      return <OpenThreadIcon size={NUDGE_TAG_ICON} />;
    case 'reflow_proposal':
      return <SparkIcon size={NUDGE_TAG_ICON} />;
    case 'thread_continuation':
      return <ThreadsIcon size={NUDGE_TAG_ICON} />;
    case 'nudge':
    default:
      return <SparkIcon size={NUDGE_TAG_ICON} />;
  }
}

export function nudgeBadgeTone(kind: string, urgent: boolean): string {
  if (kind === 'trust_warning' || kind === 'freshness_warning' || urgent) {
    return 'text-amber-300';
  }
  return uiTheme.brandSoftText;
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
}: {
  kind: string;
  urgent: boolean;
  warmSurface: boolean;
  isPrimary: boolean;
}) {
  const ringClass = warmSurface ? 'vel-nudge-orb-ring--warm' : 'vel-nudge-orb-ring--brand';

  return (
    <div
      className={cn(
        'relative size-9 shrink-0',
        isPrimary ? 'scale-105' : 'scale-100',
      )}
    >
      <div
        className={cn(
          'pointer-events-none absolute inset-[3px] rounded-full p-[2px]',
          ringClass,
        )}
        aria-hidden
      >
        <div className="flex h-full w-full items-center justify-center rounded-full bg-zinc-950">
          <span
            className={cn(
              'vel-nudge-orb-icon flex items-center justify-center',
              nudgeBadgeTone(kind, urgent),
            )}
          >
            {nudgeIcon(kind)}
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

/** Overrides on `FilterPillButton` for nudge row actions (same shell as inbox/thread filter pills). */
export const surfaceActionChipNudgeClass =
  '!gap-2 !px-3 !py-1.5 !text-[11px] !font-medium !normal-case leading-tight tracking-normal [&_svg]:!h-4 [&_svg]:!w-4 [&_svg]:!max-h-4 [&_svg]:!max-w-4';

/** Icon-only nudge actions: thread-style actions use the open / external-link glyph. */
export function NudgeActionIcon({ kind, ...props }: IconProps & { kind: string }) {
  switch (kind) {
    case 'open_thread':
    case 'expand':
    case 'accept':
      return <OpenThreadIcon {...props} />;
    case 'open_settings':
      return <SettingsIcon {...props} />;
    case 'open_inbox':
      return <InboxIcon {...props} />;
    default:
      return <SparkIcon {...props} />;
  }
}

/** Visible label next to the action icon (short; deep-link context is encoded in navigation, not API copy). */
export function nudgeActionButtonLabel(action: { kind: string; label: string }, bar: { id: string }): string {
  switch (action.kind) {
    case 'open_thread':
    case 'expand':
    case 'accept':
      return 'Open thread';
    case 'open_inbox':
      return 'Inbox';
    case 'open_settings':
      if (bar.id === 'backup_trust_warning') return 'Backups';
      if (bar.id === 'mesh_summary_warning') return 'Sync & clients';
      return 'Settings';
    default:
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
