interface SurfaceStateProps {
  message: string
  layout?: 'panel' | 'centered' | 'drawer'
  tone?: 'muted' | 'danger' | 'warning'
  title?: string
}

export function SurfaceState({
  message,
  layout = 'panel',
  tone = 'muted',
  title,
}: SurfaceStateProps) {
  const toneClass =
    tone === 'danger'
      ? 'text-red-400'
      : tone === 'warning'
        ? 'text-amber-400'
        : 'text-zinc-500'
  const layoutClass =
    layout === 'centered'
      ? 'flex-1 flex items-center justify-center px-6 text-center'
      : layout === 'drawer'
        ? 'text-sm'
        : 'p-4 text-sm'

  return (
    <div className={`${layoutClass} ${toneClass}`}>
      <div>
        {title ? <h3 className="mb-2 font-medium text-zinc-400">{title}</h3> : null}
        <p>{message}</p>
      </div>
    </div>
  )
}
