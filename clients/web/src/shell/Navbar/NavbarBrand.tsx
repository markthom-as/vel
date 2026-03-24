import { uiFonts } from '../../core/Theme';

/** Shared typography; gradient + clip live on each `.vel-brand-wordmark-shimmer` glyph span. */
const WORDMARK_TYPE = `${uiFonts.display} text-2xl font-semibold leading-none tracking-[0.08em] antialiased`;

interface NavbarBrandProps {
  onSelectNow: () => void;
}

export function NavbarBrand({ onSelectNow }: NavbarBrandProps) {
  return (
    <button
      type="button"
      onClick={onSelectNow}
      className="shrink-0 cursor-pointer rounded-md border-0 bg-transparent p-0 text-left focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--vel-color-accent)]"
    >
      <span className={`vel-brand-wordmark-glow inline-flex items-baseline ${WORDMARK_TYPE}`}>
        <span className="vel-brand-wordmark-shimmer">V</span>
        <span className="vel-brand-wordmark-shimmer relative top-[0.08em] inline-block">e</span>
        <span className="vel-brand-wordmark-shimmer">l</span>
      </span>
    </button>
  );
}
