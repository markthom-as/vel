import type { ReactNode } from 'react';
import { cn } from '../../core/cn';

type SystemGroupTone = 'domain' | 'capabilities' | 'configuration';

const groupToneClasses: Record<SystemGroupTone, { card: string; badge: string; line: string }> = {
  domain: {
    card: 'border-[rgba(222,179,123,0.22)] bg-[rgba(38,30,18,0.48)]',
    badge: 'text-[#f3d7b2]',
    line: 'border-[rgba(222,179,123,0.3)]',
  },
  capabilities: {
    card: 'border-[rgba(145,170,216,0.22)] bg-[rgba(20,28,40,0.48)]',
    badge: 'text-[#d2ddff]',
    line: 'border-[rgba(145,170,216,0.3)]',
  },
  configuration: {
    card: 'border-[rgba(174,191,146,0.22)] bg-[rgba(24,33,22,0.48)]',
    badge: 'text-[#dceac8]',
    line: 'border-[rgba(174,191,146,0.3)]',
  },
};

export function SystemGroupCard({
  title,
  description,
  summary,
  tone,
  active,
  onClick,
}: {
  title: string;
  description: string;
  summary: string;
  tone: SystemGroupTone;
  active?: boolean;
  onClick?: () => void;
}) {
  const classes = groupToneClasses[tone];
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        'rounded-[22px] border px-4 py-4 text-left transition',
        classes.card,
        active
          ? 'shadow-[0_0_0_1px_rgba(255,255,255,0.04)]'
          : 'opacity-82 hover:opacity-100',
      )}
    >
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className={cn('text-[10px] uppercase tracking-[0.18em]', classes.badge)}>{title}</p>
          <p className="mt-2 text-sm font-medium text-[var(--vel-color-text)]">{summary}</p>
        </div>
      </div>
      <p className="mt-3 text-[13px] leading-5 text-[var(--vel-color-muted)]">{description}</p>
      <div className={cn('mt-4 border-t', classes.line)} />
    </button>
  );
}

export function SystemSubsectionHero({
  eyebrow,
  title,
  description,
  children,
}: {
  eyebrow: string;
  title: string;
  description: string;
  children?: ReactNode;
}) {
  return (
    <div className="rounded-[24px] border border-[var(--vel-color-border)] bg-[linear-gradient(180deg,rgba(255,255,255,0.035),rgba(255,255,255,0.015))] px-5 py-5">
      <div className="flex flex-wrap items-start justify-between gap-4">
        <div className="min-w-0 flex-1">
          <p className="text-[10px] uppercase tracking-[0.18em] text-[var(--vel-color-muted)]">{eyebrow}</p>
          <h2 className="mt-2 text-[22px] font-medium tracking-[-0.02em] text-[var(--vel-color-text)]">{title}</h2>
          <p className="mt-2 max-w-3xl text-[14px] leading-6 text-[var(--vel-color-muted)]">{description}</p>
        </div>
        {children ? <div className="shrink-0">{children}</div> : null}
      </div>
    </div>
  );
}

export function SystemAnchorStrip({
  items,
  activeId,
  onSelect,
}: {
  items: Array<{ id: string; label: string }>;
  activeId: string | null;
  onSelect: (id: string) => void;
}) {
  if (items.length === 0) {
    return null;
  }
  return (
    <div className="rounded-[20px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-3 py-3">
      <p className="mb-2 text-[10px] uppercase tracking-[0.18em] text-[var(--vel-color-muted)]">Within this section</p>
      <div className="flex flex-wrap gap-2">
        {items.map((item) => {
          const active = item.id === activeId;
          return (
            <button
              key={item.id}
              type="button"
              onClick={() => onSelect(item.id)}
              className={cn(
                'rounded-full border px-3 py-1.5 text-[11px] uppercase tracking-[0.14em] transition',
                active
                  ? 'border-[var(--vel-color-accent-border)] bg-[rgba(255,255,255,0.05)] text-[var(--vel-color-text)]'
                  : 'border-[var(--vel-color-border)] text-[var(--vel-color-muted)] hover:text-[var(--vel-color-text)]',
              )}
            >
              {item.label}
            </button>
          );
        })}
      </div>
    </div>
  );
}
