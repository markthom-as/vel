import { uiFonts } from '../../core/Theme';

/** Shared typography; gradient + clip live on each `.vel-brand-wordmark-shimmer` glyph span. */
const WORDMARK_TYPE = `${uiFonts.display} text-3xl font-black leading-[0.92] tracking-[0.04em] antialiased min-[480px]:text-3xl`;

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
      <span className={`vel-brand-wordmark-glow inline-flex items-baseline ${WORDMARK_TYPE}`}>
        <span className="vel-brand-wordmark-shimmer">V</span>
        <span className="vel-brand-wordmark-shimmer relative top-[0.12em] inline-block">e</span>
        <span className="vel-brand-wordmark-shimmer">l</span>
      </span>
    </button>
  );
}
