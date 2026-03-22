import type { ReactNode } from 'react';
import { cn } from '../cn';

/**
 * Single header band: **title** and **meta** (time, tags, etc.) in one left-aligned row.
 * Pair with {@link NowItemRowLayout} — put this in `children` and pass interactive **actions** into the layout’s `actions` slot so actions stay on the right (inbox / task / chat pattern).
 */
export function ItemRowTitleMetaBand({
  title,
  titleClassName,
  meta,
}: {
  title: ReactNode;
  titleClassName?: string;
  meta: ReactNode;
}) {
  return (
    <div className="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
      <span
        className={cn(
          'max-w-[min(100%,12rem)] shrink-0 truncate text-sm font-medium leading-tight tracking-tight',
          titleClassName,
        )}
      >
        {title}
      </span>
      <div className="flex min-w-0 flex-1 flex-wrap items-center gap-x-1.5 gap-y-1">{meta}</div>
    </div>
  );
}
