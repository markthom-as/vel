interface SurfaceStateProps {
  message: string
  layout?: 'panel' | 'centered' | 'drawer'
  tone?: 'muted' | 'danger' | 'warning'
  title?: string
}

export function SurfaceSpinner({ className }: { className?: string }) {
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
