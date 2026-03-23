import type { ReactNode } from 'react'

interface SurfaceDrawerProps {
  title: string
  onClose: () => void
  children: ReactNode
  className?: string
}

export function SurfaceDrawer({ title, onClose, children, className = '' }: SurfaceDrawerProps) {
  return (
    <aside
      className={`absolute inset-y-0 right-0 z-10 flex w-full max-w-[24rem] flex-col border-l border-zinc-700 bg-zinc-900 shadow-xl ${className}`}
    >
      <div className="flex shrink-0 items-center justify-between border-b border-zinc-700 px-4 py-3">
        <h3 className="font-medium text-zinc-200">{title}</h3>
        <button
          type="button"
          onClick={onClose}
          className="text-zinc-500 hover:text-zinc-300"
          aria-label={`Close ${title}`}
        >
          ✕
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-4 text-sm">
        {children}
      </div>
    </aside>
  )
}
