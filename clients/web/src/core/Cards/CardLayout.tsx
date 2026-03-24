import type { ReactNode } from 'react';
import { ObjectCard, type ObjectCardKind } from '../ObjectCard';

interface CardLayoutProps {
  kind: string;
  children: ReactNode;
  className?: string;
}

export function CardLayout({ kind, children, className = '' }: CardLayoutProps) {
  const cardKind: ObjectCardKind =
    kind === 'risk_card'
      ? 'run'
      : kind === 'reminder_card'
        ? 'config'
        : kind === 'summary_card'
          ? 'artifact'
          : 'default';

  return (
    <ObjectCard kind={cardKind} className={className}>
      <div className="text-xs text-zinc-500 mb-1.5">{kind.replace(/_/g, ' ')}</div>
      {children}
    </ObjectCard>
  );
}
