import type { ReactNode } from 'react';
import { cn } from '../cn';

export type ObjectCardKind = 'default' | 'nudge' | 'run' | 'artifact' | 'config' | 'subtle';

const cardKindClasses: Record<ObjectCardKind, string> = {
  default: 'border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/92 shadow-[0_18px_40px_rgba(0,0,0,0.22)]',
  nudge: 'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] shadow-[0_18px_40px_rgba(0,0,0,0.24)]',
  run: 'border-[color:var(--vel-color-offline)]/45 bg-[color:var(--vel-color-panel)]/94 shadow-[0_18px_40px_rgba(0,0,0,0.2)]',
  artifact: 'border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel-2)]/92 shadow-[0_18px_40px_rgba(0,0,0,0.22)]',
  config: 'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel)]/96 shadow-[0_18px_40px_rgba(0,0,0,0.22)]',
  subtle: 'border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/78 shadow-none',
};

export function objectCardClass(kind: ObjectCardKind = 'default') {
  return cn('rounded-[1.1rem] border p-4', cardKindClasses[kind]);
}

export function ObjectCard({
  kind = 'default',
  as: Comp = 'div',
  className,
  children,
}: {
  kind?: ObjectCardKind;
  as?: 'div' | 'article' | 'section';
  className?: string;
  children: ReactNode;
}) {
  return <Comp className={cn(objectCardClass(kind), className)}>{children}</Comp>;
}
