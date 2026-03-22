import { ClipboardCheckIcon, InboxIcon, LayoutGridIcon, OpenThreadIcon } from '../../core/Icons';
import { FilterMetricToggleTag } from '../../core/FilterToggleTag';
import { uiTheme } from '../../core/Theme';

const SEGMENTS = [
  { value: 'new' as const, label: 'New' },
  { value: 'opened' as const, label: 'Opened' },
  { value: 'archived' as const, label: 'Archived' },
  { value: 'all' as const, label: 'All' },
] as const;

export function InboxQueueSegmentedControl({
  queueFilter,
  onQueueFilterChange,
  newCount,
  openedCount,
  archivedCount,
  totalQueueCount,
}: {
  queueFilter: 'all' | 'new' | 'opened' | 'archived';
  onQueueFilterChange: (value: 'all' | 'new' | 'opened' | 'archived') => void;
  newCount: number;
  openedCount: number;
  archivedCount: number;
  totalQueueCount: number;
}) {
  const countFor = (value: (typeof SEGMENTS)[number]['value']) => {
    if (value === 'all') return totalQueueCount;
    if (value === 'new') return newCount;
    if (value === 'opened') return openedCount;
    return archivedCount;
  };

  const iconFor = (value: (typeof SEGMENTS)[number]['value'], selected: boolean) => {
    const dim = 'text-zinc-600';
    switch (value) {
      case 'new':
        return <InboxIcon size={12} className={selected ? uiTheme.brandText : dim} />;
      case 'opened':
        return <OpenThreadIcon size={12} className={selected ? uiTheme.brandSoftText : dim} />;
      case 'archived':
        return <ClipboardCheckIcon size={12} className={selected ? 'text-emerald-400/90' : dim} />;
      case 'all':
        return <LayoutGridIcon size={12} className={selected ? 'text-amber-200/90' : dim} />;
    }
  };

  return (
    <div
      className="flex max-w-full flex-wrap items-center justify-end gap-1.5 opacity-90"
      role="group"
      aria-label="Queue state"
    >
      {SEGMENTS.map((seg) => {
        const selected = queueFilter === seg.value;
        const count = countFor(seg.value);
        return (
          <FilterMetricToggleTag
            key={seg.value}
            label={seg.label}
            count={count}
            selected={selected}
            onClick={() => onQueueFilterChange(seg.value)}
            icon={iconFor(seg.value, selected)}
          />
        );
      })}
    </div>
  );
}
