import type { ReactNode } from 'react';
import {
  ClipboardCheckIcon,
  ClockIcon,
  DotIcon,
  OpenThreadIcon,
  SparkIcon,
  ThreadsIcon,
  WarningIcon,
} from '../../core/Icons';

export function nudgeIcon(kind: string): ReactNode {
  switch (kind) {
    case 'needs_input':
      return <ThreadsIcon size={20} />;
    case 'trust_warning':
    case 'freshness_warning':
      return <WarningIcon size={20} />;
    case 'review_request':
      return <OpenThreadIcon size={20} />;
    default:
      return <SparkIcon size={20} />;
  }
}

export function nudgeBadgeTone(kind: string, urgent: boolean): string {
  if (kind === 'trust_warning' || urgent) {
    return 'text-amber-300';
  }
  if (kind === 'needs_input') {
    return 'text-sky-300';
  }
  return 'text-[#ff8a63]';
}

export function taskKindIcon(kind: string): ReactNode {
  switch (kind) {
    case 'commitment':
      return <ClockIcon size={11} />;
    case 'task':
      return <ClipboardCheckIcon size={11} />;
    default:
      return <DotIcon size={11} />;
  }
}
