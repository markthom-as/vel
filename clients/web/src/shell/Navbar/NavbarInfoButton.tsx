import { InfoCircleIcon } from '../../core/Icons';
import { uiTheme } from '../../core/Theme';

const ACCENT = uiTheme.brandText;

interface NavbarInfoButtonProps {
  infoPanelOpen: boolean;
  onOpenDocumentation: () => void;
}

export function NavbarInfoButton({ infoPanelOpen, onOpenDocumentation }: NavbarInfoButtonProps) {
  return (
    <button
      type="button"
      onClick={onOpenDocumentation}
      aria-label="Open info"
      className={`inline-flex h-7 w-7 shrink-0 items-center justify-center rounded-full transition ${
        infoPanelOpen ? `${ACCENT} bg-zinc-900` : 'text-zinc-500 hover:bg-zinc-900 hover:text-zinc-200'
      }`}
    >
      <InfoCircleIcon size={16} />
    </button>
  );
}
