import { uiTheme } from '../../core/Theme';

const ACCENT = uiTheme.brandText;

/** Wordmark only — typography and brand color; interactive chrome stays on the parent `<button>`. */
const NAVBAR_BRAND_MARK_CLASSNAME = `inline-block text-3xl font-black leading-[0.92] tracking-[-0.03em] antialiased min-[480px]:text-3xl ${ACCENT}`;

interface NavbarBrandProps {
  onSelectNow: () => void;
}

export function NavbarBrand({ onSelectNow }: NavbarBrandProps) {
  return (
    <button
      type="button"
      onClick={onSelectNow}
      className="shrink-0 cursor-pointer rounded-sm border-0 bg-transparent p-0 text-left focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-zinc-500"
    >
      <span className={NAVBAR_BRAND_MARK_CLASSNAME}>Vel</span>
    </button>
  );
}
