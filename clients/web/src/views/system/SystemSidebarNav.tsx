import { cn } from '../../core/cn';
import { SearchField } from '../../core/SearchField/SearchField';
import type { SystemSidebarChild } from './SystemNavigationModel';
import type { SystemSubsectionKey } from './systemNavigation';

type GroupedNavItem = {
  key: SystemSubsectionKey;
  label: string;
  description: string;
};

type GroupedNavGroup = {
  key: string;
  label: string;
  items: GroupedNavItem[];
};

export function SystemSidebarNav({
  sidebarFilter,
  onSidebarFilterChange,
  groupedNav,
  activeSubsection,
  activeChildAnchor,
  subsectionChildren,
  onSelectSubsection,
}: {
  sidebarFilter: string;
  onSidebarFilterChange: (value: string) => void;
  groupedNav: GroupedNavGroup[];
  activeSubsection: SystemSubsectionKey;
  activeChildAnchor: string | null;
  subsectionChildren: Record<SystemSubsectionKey, SystemSidebarChild[]>;
  onSelectSubsection: (subsection: SystemSubsectionKey, anchor?: string | null) => void;
}) {
  return (
    <div className="px-1 py-1">
      <SearchField
        aria-label="Filter system sections"
        value={sidebarFilter}
        onChange={(event) => onSidebarFilterChange(event.target.value)}
        placeholder="Filter system"
      />
      {groupedNav.length === 0 ? (
        <p className="mt-2 px-1 text-[11px] leading-5 text-[var(--vel-color-muted)]">
          No system sections match that filter.
        </p>
      ) : (
        <nav className="mt-2 space-y-2" aria-label="System sections">
          {groupedNav.map((group) => (
            <div key={group.key} className="space-y-0.5">
              <p className="px-1 text-[11px] font-medium uppercase tracking-[0.1em] text-[var(--vel-color-muted)]">
                {group.label}
              </p>
              <div className="space-y-0.5">
                {group.items.map((item) => {
                  const itemActive = activeSubsection === item.key;
                  const children = subsectionChildren[item.key] ?? [];
                  const showChildren = children.length > 1 && (itemActive || children.some((child) => child.id === activeChildAnchor));
                  return (
                    <div key={item.key} className="space-y-0.5">
                      <button
                        type="button"
                        onClick={() => onSelectSubsection(item.key)}
                        aria-current={itemActive ? 'page' : undefined}
                        aria-label={`${item.label}. ${item.description}`}
                        className={cn(
                          'block w-full rounded-[9px] px-2 py-1 text-left transition',
                          itemActive
                            ? 'bg-[rgba(255,255,255,0.045)] text-[var(--vel-color-text)] ring-1 ring-[var(--vel-color-accent-border)]'
                            : 'bg-transparent text-[var(--vel-color-muted)] hover:bg-[rgba(255,255,255,0.025)] hover:text-[var(--vel-color-text)]',
                        )}
                      >
                        <p className="text-[12px] font-medium leading-4">
                          {item.label}
                        </p>
                      </button>
                      {showChildren && children.length > 0 ? (
                        <div className="space-y-0.5 border-l border-[var(--vel-color-border-subtle)] pl-3.5 ml-2">
                          {children.map((child) => {
                            const childActive = child.id === activeChildAnchor;
                            return (
                              <button
                                key={child.id}
                                type="button"
                                onClick={() => onSelectSubsection(item.key, child.id)}
                                aria-current={childActive ? 'location' : undefined}
                                className={cn(
                                  'flex w-full items-center gap-1 rounded-[8px] px-1.5 py-0.5 text-left text-[8px] leading-[1.1] tracking-normal transition',
                                  childActive
                                    ? 'bg-[rgba(255,255,255,0.03)] text-[var(--vel-color-muted)]'
                                    : 'text-[var(--vel-color-dim)] hover:bg-[rgba(255,255,255,0.015)] hover:text-[var(--vel-color-muted)]',
                                )}
                              >
                                <span aria-hidden="true" className="text-[var(--vel-color-border)]">·</span>
                                <span className="text-[8px] leading-[1.1] tracking-normal">
                                  {child.label}
                                </span>
                              </button>
                            );
                          })}
                        </div>
                      ) : null}
                    </div>
                  );
                })}
              </div>
            </div>
          ))}
        </nav>
      )}
    </div>
  );
}
