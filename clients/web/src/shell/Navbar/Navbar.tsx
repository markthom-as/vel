import type { MainView } from '../../data/operatorSurfaces';
import { NAVBAR_HEADER_CLASSNAME, NAVBAR_INNER_CLASSNAME } from './navbarChrome';
import { NavbarBrand } from './NavbarBrand';
import { NavbarNavLinks } from './NavbarNavLinks';

export interface NavbarProps {
  activeView: MainView;
  onSelectView: (view: MainView) => void;
}

export function Navbar({ activeView, onSelectView }: NavbarProps) {
  return (
    <header className={NAVBAR_HEADER_CLASSNAME} role="banner">
      <div className={NAVBAR_INNER_CLASSNAME}>
        <NavbarBrand onSelectNow={() => onSelectView('now')} />
        <div className="ml-auto flex min-w-0 items-center gap-3">
          <NavbarNavLinks activeView={activeView} onSelectView={onSelectView} />
        </div>
      </div>
    </header>
  );
}
