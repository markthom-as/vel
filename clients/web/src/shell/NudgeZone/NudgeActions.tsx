import type { MainView } from '../../data/operatorSurfaces';
import { rescheduleNowTasksToToday } from '../../data/context';
import { acknowledgeInboxItem, snoozeInboxItem } from '../../data/chat';
import { cn } from '../../core/cn';
import type { NowNudgeBarData } from '../../types';
import { NudgeActionButton } from '../../views/now/NudgeActionButton';
import { nudgeOpenSystemTarget } from '../../views/now/nowModel';
import {
  nudgeActionAriaLabel,
  nudgeActionButtonLabel,
  nudgeActionToneClass,
} from '../../views/now/nowNudgePresentation';
import type { SystemNavigationTarget } from '../../views/system';

const actionChipClass =
  '!min-h-[1.1rem] !gap-1.5 !rounded-full !px-2 !py-[0.2rem] !text-[9px] !tracking-[0.1em] opacity-90';
const nudgeActionChipClass =
  `${actionChipClass} w-full justify-center`;

function parseRescheduleCommitmentIds(kind: string): string[] {
  const [prefix, encodedIds] = kind.split(':', 2);
  if (prefix !== 'reschedule_today' || !encodedIds) {
    return [];
  }
  return encodedIds
    .split(',')
    .map((id) => id.trim())
    .filter((id) => id.length > 0);
}

function parseJumpAnchor(kind: string): string | null {
  const [prefix, anchor] = kind.split(':', 2);
  if (prefix !== 'jump_backlog' || !anchor?.trim()) {
    return null;
  }
  return anchor.trim();
}

export function NudgeActions({
  bar,
  activeView,
  interventionId,
  pendingActionKey,
  onOpenThread,
  onOpenSystem,
  runMutation,
}: {
  bar: NowNudgeBarData;
  activeView: MainView;
  interventionId: string | null;
  pendingActionKey: string | null;
  onOpenThread?: (conversationId: string) => void;
  onOpenSystem?: (target?: SystemNavigationTarget) => void;
  runMutation: (actionKey: string, callback: () => Promise<unknown>) => Promise<void>;
}) {
  const visibleActions = bar.id === 'core_setup_required' ? [] : bar.actions;

  return (
    <>
      {visibleActions.map((action, index) => {
        const actionKey = `${bar.id}-${action.kind}-${index}`;
        const label = nudgeActionButtonLabel(action, bar);
        const ariaLabel = nudgeActionAriaLabel(bar, action, index, bar.actions.length);
        const actionTone = nudgeActionToneClass(action.kind);
        if (action.kind.startsWith('open_settings')) {
          return (
            <NudgeActionButton
              key={actionKey}
              kind={action.kind}
              label={label}
              onClick={() => onOpenSystem?.(nudgeOpenSystemTarget(bar, action))}
              aria-label={ariaLabel}
              className={cn(nudgeActionChipClass, actionTone)}
            />
          );
        }
        if (action.kind.startsWith('reschedule_today')) {
          const commitmentIds = parseRescheduleCommitmentIds(action.kind);
          return (
            <NudgeActionButton
              key={actionKey}
              kind={action.kind}
              label={label}
              onClick={() => {
                if (commitmentIds.length === 0) {
                  return;
                }
                void runMutation(actionKey, () => rescheduleNowTasksToToday(commitmentIds));
              }}
              disabled={pendingActionKey === actionKey || commitmentIds.length === 0}
              aria-label={ariaLabel}
              className={cn(nudgeActionChipClass, actionTone)}
            />
          );
        }
        if (action.kind.startsWith('jump_backlog')) {
          const anchor = parseJumpAnchor(action.kind);
          return (
            <NudgeActionButton
              key={actionKey}
              kind={action.kind}
              label={label}
              onClick={() => {
                if (activeView !== 'now' || !anchor) {
                  return;
                }
                document.getElementById(anchor)?.scrollIntoView({ behavior: 'smooth', block: 'start' });
              }}
              disabled={activeView !== 'now' || !anchor}
              aria-label={ariaLabel}
              className={cn(nudgeActionChipClass, actionTone)}
            />
          );
        }
        if (interventionId && (action.kind === 'accept' || action.kind === 'acknowledge')) {
          return (
            <NudgeActionButton
              key={actionKey}
              kind={action.kind}
              label={label}
              onClick={() => {
                void runMutation(actionKey, () => acknowledgeInboxItem(interventionId));
              }}
              disabled={pendingActionKey === actionKey}
              className={cn(nudgeActionChipClass, actionTone)}
              aria-label={ariaLabel}
            />
          );
        }
        if (bar.primary_thread_id && (action.kind === 'expand' || action.kind === 'escalate' || action.kind === 'edit' || action.kind === 'open_thread')) {
          return (
            <NudgeActionButton
              key={actionKey}
              kind={action.kind}
              label={label}
              onClick={() => onOpenThread?.(bar.primary_thread_id!)}
              className={cn(nudgeActionChipClass, actionTone)}
              aria-label={ariaLabel}
            />
          );
        }
        return (
          <NudgeActionButton
            key={actionKey}
            kind={action.kind}
            label={label}
            aria-label={ariaLabel}
            disabled
            className={cn(nudgeActionChipClass, actionTone)}
          />
        );
      })}
      <NudgeActionButton
        kind="snooze"
        label="Defer"
        aria-label={`Defer (${bar.title}) · ${bar.id}`}
        className={cn(nudgeActionChipClass, nudgeActionToneClass('snooze'))}
        disabled={!interventionId || pendingActionKey === `${bar.id}-defer`}
        onClick={() => {
          if (!interventionId) return;
          void runMutation(`${bar.id}-defer`, () => snoozeInboxItem(interventionId, 10));
        }}
      />
    </>
  );
}
