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

export function ZoomBrandIcon({ size = 16, className }: { size?: number; className?: string }) {
  return <SiZoom size={size} className={className} aria-hidden />;
}

export function GoogleMeetBrandIcon({ size = 16, className }: { size?: number; className?: string }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      className={className}
      aria-hidden
    >
      <path d="M4.5 7.5a2 2 0 0 1 2-2h7.35l2.15 2.15V16.5a2 2 0 0 1-2 2H6.5a2 2 0 0 1-2-2v-9Z" fill="#34A853" />
      <path d="M16 9.2 20.4 6.6c.53-.31 1.2.07 1.2.69v9.42c0 .62-.67 1-1.2.69L16 14.8V9.2Z" fill="#4285F4" />
      <path d="M13.85 5.5 16 7.65V9.2l-4.3-2.7 2.15-1Z" fill="#FBBC04" />
      <path d="M11.7 6.5 16 9.2v5.6l-2.15 1-4.64-2.93a1.5 1.5 0 0 1 0-2.54L11.7 6.5Z" fill="#EA4335" />
    </svg>
  );
}
