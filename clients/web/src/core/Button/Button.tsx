import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { cn } from '../cn';
import {
  buttonClasses,
  isIconButtonSize,
  type ButtonSize,
  type ButtonVariant,
} from './buttonClasses';

export type ButtonProps = {
  variant?: ButtonVariant;
  size?: ButtonSize;
  fullWidth?: boolean;
  /** Shows a spinner and sets aria-busy; combines with disabled. */
  loading?: boolean;
  children: ReactNode;
} & Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'className' | 'disabled'> & {
    className?: string;
    disabled?: boolean;
  };

function ButtonSpinner({ className }: { className?: string }) {
  return (
    <svg
      className={cn('shrink-0 animate-spin', className)}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden
    >
      <circle
        className="opacity-30"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        strokeWidth="3"
      />
      <path
        className="opacity-90"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
  );
}

export function Button({
  variant = 'primary',
  size = 'md',
  fullWidth,
  loading = false,
  className,
  type = 'button',
  disabled,
  children,
  ...props
}: ButtonProps) {
  const isIcon = isIconButtonSize(size);
  const isDisabled = Boolean(disabled || loading);

  return (
    <button
      type={type}
      className={buttonClasses({ variant, size, fullWidth, loading, className })}
      disabled={isDisabled}
      aria-busy={loading || undefined}
      {...props}
    >
      {loading && isIcon ? (
        <ButtonSpinner className={size === 'icon-sm' ? 'h-3.5 w-3.5' : size === 'icon-lg' ? 'h-5 w-5' : 'h-[18px] w-[18px]'} />
      ) : loading ? (
        <>
          <ButtonSpinner className="h-4 w-4" />
          {children}
        </>
      ) : (
        children
      )}
    </button>
  );
}

export type IconButtonProps = Omit<ButtonProps, 'size'> & {
  /** Defaults to `icon` (36px). */
  size?: 'icon-sm' | 'icon' | 'icon-lg';
};

/** Square icon-only control; always set `aria-label` (or `aria-labelledby`). */
export function IconButton({ size = 'icon', children, ...props }: IconButtonProps) {
  return (
    <Button size={size} {...props}>
      {children}
    </Button>
  );
}
