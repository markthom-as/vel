import type { ReactNode } from 'react';

interface CardLayoutProps {
  kind: string;
  children: ReactNode;
  className?: string;
}

export function CardLayout({ kind, children, className = '' }: CardLayoutProps) {
  return (
    <div className={`rounded-lg border border-zinc-700 bg-zinc-800/80 px-3 py-2 ${className}`}>
      <div className="text-xs text-zinc-500 mb-1.5">{kind.replace(/_/g, ' ')}</div>
      {children}
    </div>
  );
}
