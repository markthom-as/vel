import type { MainView } from '../../data/operatorSurfaces';
import { NAVBAR_HEADER_CLASSNAME } from './navbarChrome';
import { NavbarBrand } from './NavbarBrand';
import { NavbarInfoButton } from './NavbarInfoButton';
import { NavbarNavLinks } from './NavbarNavLinks';

export interface NavbarProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
  onOpenDocumentation: () => void;
  infoPanelOpen: boolean;
}

export function Navbar({
  activeView,
  onSelectView,
  onOpenDocumentation,
  infoPanelOpen,
}: NavbarProps) {
  return (
    <header className={NAVBAR_HEADER_CLASSNAME} role="banner">
      <div className="flex min-w-0 items-center justify-between gap-x-4">
        <NavbarBrand onSelectNow={() => onSelectView('now')} />
        <div className="ml-auto flex min-w-0 items-center gap-3 pr-1 sm:gap-5 sm:pr-5">
          <NavbarNavLinks activeView={activeView} onSelectView={onSelectView} />
          <NavbarInfoButton infoPanelOpen={infoPanelOpen} onOpenDocumentation={onOpenDocumentation} />
        </div>
      </div>
    </header>
  );
}
