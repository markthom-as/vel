interface SurfaceStateProps {
  message: string
  layout?: 'panel' | 'centered' | 'drawer'
  tone?: 'muted' | 'danger' | 'warning'
  title?: string
}

export function SurfaceSpinner({
  className,
  variant = 'default',
}: {
  className?: string
  variant?: 'default' | 'brand'
}) {
  if (variant === 'brand') {
    return (
      <div
        className={`relative inline-flex items-center justify-center text-[var(--vel-color-accent)] ${className ?? ''}`}
        aria-hidden
      >
        <span className="absolute inset-0 rounded-full border border-[color:rgba(200,116,43,0.2)]" />
        <span className="absolute inset-0 animate-spin rounded-full border-2 border-transparent border-t-[var(--vel-color-accent)] border-r-[color:rgba(255,214,170,0.85)] shadow-[0_0_26px_rgba(200,116,43,0.24)]" />
        <span className="absolute inset-[7px] rounded-full border border-[color:rgba(255,214,170,0.18)]" />
        <span className="absolute inset-[7px] rounded-full border-2 border-transparent border-b-[color:rgba(255,214,170,0.78)] animate-[spin_1.35s_linear_infinite_reverse]" />
        <span className="font-display text-[10px] uppercase tracking-[0.34em] text-[var(--vel-color-accent-soft)]">
          VEL
        </span>
      </div>
    )
  }

  return (
    <svg
      className={`mx-auto mb-3 h-5 w-5 animate-spin ${className ?? ''}`}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden
    >
      <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="2.5" className="opacity-25" />
      <path
        fill="currentColor"
        className="opacity-90"
        d="M12 3a9 9 0 0 1 9 9h-3a6 6 0 0 0-6-6V3Z"
      />
    </svg>
  )
}

export function SurfaceState({
  message,
  layout = 'panel',
  tone = 'muted',
  title,
}: SurfaceStateProps) {
  const showSpinner = /^loading\b/i.test(message)
  const toneClass =
    tone === 'danger'
      ? 'text-red-400'
      : tone === 'warning'
        ? 'text-amber-400'
        : 'text-zinc-500'
  const layoutClass =
    layout === 'centered'
      ? showSpinner
        ? 'flex min-h-[calc(100vh-9rem)] w-full items-center justify-center px-6 text-center'
        : 'flex-1 flex items-center justify-center px-6 text-center'
      : layout === 'drawer'
        ? 'text-sm'
        : 'p-4 text-sm'

  return (
    <div className={`${layoutClass} ${toneClass}`}>
      <div>
        {showSpinner ? <SurfaceSpinner /> : null}
        {title ? <h3 className="mb-2 font-medium text-zinc-400">{title}</h3> : null}
        <p>{message}</p>
      </div>
    </div>
  )
}
