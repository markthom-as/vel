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
    <div className="rounded-[24px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-4 py-4">
      <SearchField
        aria-label="Filter system sections"
        value={sidebarFilter}
        onChange={(event) => onSidebarFilterChange(event.target.value)}
        placeholder="Filter system"
      />
      {groupedNav.length === 0 ? (
        <p className="mt-4 text-sm leading-6 text-[var(--vel-color-muted)]">
          No system sections match that filter.
        </p>
      ) : (
        <nav className="mt-4 space-y-4" aria-label="System sections">
          {groupedNav.map((group) => (
            <div key={group.key} className="space-y-2">
              <div className="border-b border-[var(--vel-color-border)] pb-2">
                <p className="text-[10px] uppercase tracking-[0.18em] text-[var(--vel-color-muted)]">
                  {group.label}
                </p>
              </div>
              <div className="space-y-1">
                {group.items.map((item) => {
                  const itemActive = activeSubsection === item.key;
                  const children = subsectionChildren[item.key] ?? [];
                  const showChildren = itemActive || children.some((child) => child.id === activeChildAnchor);
                  return (
                    <div key={item.key} className="space-y-1.5">
                      <button
                        type="button"
                        onClick={() => onSelectSubsection(item.key)}
                        aria-pressed={itemActive}
                        className={cn(
                          'block w-full rounded-[18px] border px-3 py-2 text-left transition',
                          itemActive
                            ? 'border-[var(--vel-color-accent-border)] bg-[rgba(255,255,255,0.045)]'
                            : 'border-[var(--vel-color-border)] bg-transparent hover:border-[var(--vel-color-accent-border)]/70 hover:bg-[rgba(255,255,255,0.025)]',
                        )}
                      >
                        <p
                          className={cn(
                            'text-[13px] font-medium leading-5',
                            itemActive ? 'text-[var(--vel-color-text)]' : 'text-[var(--vel-color-muted)]',
                          )}
                        >
                          {item.label}
                        </p>
                        <p className="mt-1 text-[12px] leading-5 text-[var(--vel-color-dim)]">
                          {item.description}
                        </p>
                      </button>
                      {showChildren && children.length > 0 ? (
                        <div className="space-y-1 pl-3">
                          {children.map((child) => {
                            const childActive = child.id === activeChildAnchor;
                            return (
                              <button
                                key={child.id}
                                type="button"
                                onClick={() => onSelectSubsection(item.key, child.id)}
                                aria-pressed={childActive}
                                className={cn(
                                  'block w-full rounded-full border px-3 py-1.5 text-left text-[11px] uppercase tracking-[0.14em] transition',
                                  childActive
                                    ? 'border-[var(--vel-color-accent-border)] bg-[rgba(255,255,255,0.045)] text-[var(--vel-color-text)]'
                                    : 'border-[var(--vel-color-border)] text-[var(--vel-color-muted)] hover:border-[var(--vel-color-accent-border)]/70 hover:text-[var(--vel-color-text)]',
                                )}
                              >
                                {child.label}
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
