import { useEffect, useRef, useState, type ReactNode } from 'react';
import { Button } from '../Button';
import { cn } from '../cn';

export type SystemDocumentStatusTone =
  | 'active'
  | 'warning'
  | 'degraded'
  | 'offline'
  | 'done'
  | 'neutral';

function systemDocumentStatusChipClass(tone: SystemDocumentStatusTone) {
  switch (tone) {
    case 'active':
      return 'border-[#b96e3a]/50 bg-[#2d1608] text-[#ffd4b8]';
    case 'warning':
      return 'border-amber-500/40 bg-amber-950/50 text-amber-100';
    case 'degraded':
      return 'border-orange-500/40 bg-orange-950/50 text-orange-100';
    case 'offline':
      return 'border-slate-500/40 bg-slate-950/60 text-slate-200';
    case 'done':
      return 'border-emerald-500/35 bg-emerald-950/40 text-emerald-100';
    default:
      return 'border-zinc-700 bg-zinc-900/80 text-zinc-300';
  }
}

export function SystemDocumentList({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) {
  return <div className={cn('divide-y divide-[var(--vel-color-border)]', className)}>{children}</div>;
}

export function SystemDocumentStatsGrid({
  children,
  className,
  id,
}: {
  children: ReactNode;
  className?: string;
  id?: string;
}) {
  return <div id={id} className={cn('grid gap-x-4 gap-y-0 sm:grid-cols-2', className)}>{children}</div>;
}

export function SystemDocumentSectionLabel({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <p
      className={cn(
        'text-[10px] uppercase tracking-[0.18em] text-[var(--vel-color-muted)]',
        className,
      )}
    >
      {children}
    </p>
  );
}

export function SystemDocumentItem({
  leading,
  title,
  subtitle,
  trailing,
  children,
  className,
  id,
}: {
  leading?: ReactNode;
  title: ReactNode;
  subtitle?: ReactNode;
  trailing?: ReactNode;
  children?: ReactNode;
  className?: string;
  id?: string;
}) {
  return (
    <div id={id} className={cn('space-y-2.5 py-2.5 scroll-mt-24', className)}>
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="flex min-w-0 items-start gap-3">
          {leading ? <div className="shrink-0">{leading}</div> : null}
          <div className="min-w-0">
            <h3 className="text-[15px] font-medium leading-5 text-[var(--vel-color-text)]">{title}</h3>
            {subtitle ? <p className="text-[13px] leading-5 text-[var(--vel-color-muted)]">{subtitle}</p> : null}
          </div>
        </div>
        {trailing ? <div className="shrink-0">{trailing}</div> : null}
      </div>
      {children ? <div className="space-y-1">{children}</div> : null}
    </div>
  );
}

export function SystemDocumentField({
  label,
  value,
  onChange,
  onCommit,
  className,
  multiline = false,
  placeholder,
  fieldId,
}: {
  label: string;
  value: string;
  onChange?: (nextValue: string) => void;
  onCommit?: (nextValue: string) => void | Promise<void>;
  className?: string;
  multiline?: boolean;
  placeholder?: string;
  fieldId?: string;
}) {
  const [draft, setDraft] = useState(value);
  const [saving, setSaving] = useState(false);
  const lastSubmittedDraft = useRef<string | null>(null);
  const draftRef = useRef(draft);
  const valueRef = useRef(value);
  const savingRef = useRef(saving);
  const onCommitRef = useRef(onCommit);

  useEffect(() => {
    draftRef.current = draft;
  }, [draft]);

  useEffect(() => {
    valueRef.current = value;
  }, [value]);

  useEffect(() => {
    savingRef.current = saving;
  }, [saving]);

  useEffect(() => {
    onCommitRef.current = onCommit;
  }, [onCommit]);

  useEffect(() => {
    setDraft(value);
    if (lastSubmittedDraft.current === value) {
      lastSubmittedDraft.current = null;
    }
  }, [value]);

  async function commitDraft(nextDraft = draftRef.current) {
    if (
      !onCommitRef.current
      || nextDraft === valueRef.current
      || savingRef.current
      || lastSubmittedDraft.current === nextDraft
    ) {
      return;
    }
    lastSubmittedDraft.current = nextDraft;
    setSaving(true);
    try {
      await onCommitRef.current(nextDraft);
    } catch (error) {
      lastSubmittedDraft.current = null;
      throw error;
    } finally {
      setSaving(false);
    }
  }

  useEffect(() => {
    if (!onCommit || draft === value || saving) {
      return;
    }
    const timeoutId = window.setTimeout(() => {
      void commitDraft();
    }, 650);
    return () => {
      window.clearTimeout(timeoutId);
    };
  }, [draft, value, onCommit, saving]);

  return (
    <label className={cn('block border-b border-[var(--vel-color-border)] py-1.5', className)}>
      <span className="text-xs uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">{label}</span>
      {multiline ? (
        <textarea
          id={fieldId}
          name={fieldId}
          aria-label={label}
          value={draft}
          placeholder={placeholder}
          onChange={(event) => {
            const nextValue = event.target.value;
            setDraft(nextValue);
            onChange?.(nextValue);
          }}
          onBlur={() => {
            void commitDraft();
          }}
          className="min-h-[5.5rem] w-full resize-y bg-transparent px-0 py-1 text-sm leading-5 text-[var(--vel-color-text)] outline-none"
        />
      ) : (
        <input
          id={fieldId}
          name={fieldId}
          aria-label={label}
          value={draft}
          placeholder={placeholder}
          onChange={(event) => {
            const nextValue = event.target.value;
            setDraft(nextValue);
            onChange?.(nextValue);
          }}
          onBlur={() => {
            void commitDraft();
          }}
          onKeyDown={(event) => {
            if (event.key === 'Enter') {
              event.preventDefault();
              void commitDraft();
            }
          }}
          className="w-full bg-transparent px-0 py-1 text-sm text-[var(--vel-color-text)] outline-none"
        />
      )}
      {saving ? (
        <span className="block pt-0.5 text-[10px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">
          Saving…
        </span>
      ) : null}
    </label>
  );
}

export function SystemDocumentMetaRow({
  label,
  value,
  className,
  id,
}: {
  label: ReactNode;
  value: ReactNode;
  className?: string;
  id?: string;
}) {
  return (
    <div id={id} className={cn('flex items-start justify-between gap-4 border-b border-[var(--vel-color-border)] py-2 scroll-mt-24', className)}>
      <span className="min-w-0 text-[11px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">
        {label}
      </span>
      <span className="min-w-0 text-right text-[13px] leading-5 text-[var(--vel-color-text)]">
        {value}
      </span>
    </div>
  );
}

export function SystemDocumentStatusChip({
  tone,
  children,
  className,
}: {
  tone: SystemDocumentStatusTone;
  children: ReactNode;
  className?: string;
}) {
  return (
    <span
      className={cn(
        'inline-flex min-h-[1.1rem] items-center rounded-full border px-2 py-0.5 text-[9px] font-medium uppercase tracking-[0.12em]',
        systemDocumentStatusChipClass(tone),
        className,
      )}
    >
      {children}
    </span>
  );
}

export function SystemDocumentToggleRow({
  title,
  detail,
  value,
  onToggle,
  id,
}: {
  title: string;
  detail: string;
  value: boolean;
  onToggle: () => void;
  id?: string;
}) {
  return (
    <div id={id} className="border-b border-[var(--vel-color-border)] py-2.5 scroll-mt-24">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <h3 className="text-[15px] font-medium leading-5 text-[var(--vel-color-text)]">{title}</h3>
          <p className="text-[13px] leading-5 text-[var(--vel-color-muted)]">{detail}</p>
        </div>
        <Button
          variant={value ? 'success' : 'outline'}
          size="sm"
          aria-pressed={value}
          aria-label={title}
          onClick={onToggle}
        >
          {value ? 'On' : 'Off'}
        </Button>
      </div>
    </div>
  );
}
