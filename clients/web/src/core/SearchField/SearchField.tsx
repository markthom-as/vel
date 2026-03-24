import type { InputHTMLAttributes } from 'react';
import { SearchIcon } from '../Icons';
import { cn } from '../cn';

export function SearchField({
  className,
  inputClassName,
  iconClassName,
  ...props
}: Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> & {
  inputClassName?: string;
  iconClassName?: string;
}) {
  return (
    <label className={cn('relative block', className)}>
      <input
        type="text"
        {...props}
        className={cn(
          'w-full rounded-xl border border-[var(--vel-color-border)] bg-transparent px-3 py-2 pr-9 text-sm text-[var(--vel-color-text)] placeholder:text-[var(--vel-color-dim)]',
          inputClassName,
        )}
      />
      <span className={cn('pointer-events-none absolute inset-y-0 right-3 flex items-center text-[var(--vel-color-dim)]', iconClassName)}>
        <SearchIcon size={14} />
      </span>
    </label>
  );
}
