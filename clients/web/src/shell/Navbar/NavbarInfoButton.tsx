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
      aria-label={infoPanelOpen ? 'Close info' : 'Open info'}
      aria-pressed={infoPanelOpen}
      className={`inline-flex h-7 w-7 shrink-0 items-center justify-center border-0 bg-transparent p-0 transition outline-none focus-visible:ring-2 focus-visible:ring-[#ff6b00]/45 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 ${
        infoPanelOpen ? ACCENT : 'text-zinc-500 hover:text-zinc-200'
      }`}
    >
      <InfoCircleIcon size={16} />
    </button>
  );
}
