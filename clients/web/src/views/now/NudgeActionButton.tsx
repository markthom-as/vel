import type { ButtonHTMLAttributes } from 'react';
import { ActionChipButton } from '../../core/FilterToggleTag';
import { cn } from '../../core/cn';
import { NudgeActionIcon } from './nowNudgePresentation';

export function NudgeActionButton({
  kind,
  label,
  className,
  ...props
}: {
  kind: string;
  label?: string | null;
  className?: string;
} & Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'className' | 'type' | 'children'>) {
  return (
    <ActionChipButton className={cn(className)} {...props}>
      <NudgeActionIcon kind={kind} size={16} className="shrink-0" aria-hidden />
      {label ? <span>{label}</span> : null}
    </ActionChipButton>
  );
}
