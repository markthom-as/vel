/**
 * Layout tokens for the global app navbar. Used by {@link Navbar} and by shell
 * chrome (e.g. mobile overlays) that must align with the header footprint.
 */
export const NAVBAR_HEADER_CLASSNAME =
  'sticky top-0 z-40 shrink-0 border-b border-[var(--vel-color-border-subtle)] bg-[color:var(--vel-color-bg-overlay)] backdrop-blur-[18px]';

export const NAVBAR_INNER_CLASSNAME =
  'mx-auto flex w-full max-w-[1440px] min-w-0 items-center gap-4 px-4 py-3 sm:px-6';

export const NAVBAR_MOBILE_BAR_CLASSNAME =
  'fixed inset-x-0 bottom-0 z-40 border-t border-[var(--vel-color-border-subtle)] bg-[color:var(--vel-color-bg-overlay)] backdrop-blur-[18px] pb-[env(safe-area-inset-bottom)]';

export const NAVBAR_MOBILE_BAR_INNER_CLASSNAME =
  'mx-auto flex h-[3.85rem] w-full max-w-[560px] min-w-0 items-stretch gap-1.5 px-3 py-1.5';
