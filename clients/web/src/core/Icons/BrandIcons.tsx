import type { IconType } from 'react-icons';
import {
  SiApple,
  SiFitbit,
  SiGit,
  SiGooglecalendar,
  SiImessage,
  SiObsidian,
  SiOpenai,
  SiRescuetime,
  SiTodoist,
  SiZoom,
} from 'react-icons/si';
import type { BrandIntegrationKey } from './integrationBrands';

export type IntegrationBrandIconProps = {
  brand: BrandIntegrationKey;
  size?: number;
  className?: string;
  /**
   * When true, all marks share one cap height; width follows each logo’s aspect ratio (Simple Icons vary).
   * Use on Settings integrations so rows align cleanly.
   */
  normalizeHeight?: boolean;
};

const HEIGHT_NORMALIZED_CLASSES = '!block !h-5 !w-auto !max-w-[1.75rem] object-contain';

function renderBrandIcon(Icon: IconType, props: IntegrationBrandIconProps) {
  const { size = 20, className, normalizeHeight = false } = props;
  const base = className ?? 'shrink-0 text-current';
  const cn = normalizeHeight ? `${base} ${HEIGHT_NORMALIZED_CLASSES}` : base;
  return (
    <Icon
      {...(normalizeHeight ? {} : { size })}
      className={cn}
      aria-hidden
    />
  );
}

/**
 * Official-style marks from Simple Icons (`react-icons/si`).
 */
export function IntegrationBrandIcon(props: IntegrationBrandIconProps) {
  const { brand } = props;
  switch (brand) {
    case 'google':
      return renderBrandIcon(SiGooglecalendar, props);
    case 'todoist':
      return renderBrandIcon(SiTodoist, props);
    case 'activity':
      return renderBrandIcon(SiRescuetime, props);
    case 'health':
      return renderBrandIcon(SiFitbit, props);
    case 'git':
      return renderBrandIcon(SiGit, props);
    case 'messaging':
      return renderBrandIcon(SiImessage, props);
    case 'reminders':
      return renderBrandIcon(SiApple, props);
    case 'notes':
      return renderBrandIcon(SiObsidian, props);
    case 'transcripts':
      return renderBrandIcon(SiZoom, props);
    default: {
      const _exhaustive: never = brand;
      return _exhaustive;
    }
  }
}

/** OpenAI mark for LLM / OpenAI-compatible settings (Simple Icons). */
export function OpenAiBrandIcon({ size = 18, className }: { size?: number; className?: string }) {
  return <SiOpenai size={size} className={className} aria-hidden />;
}
