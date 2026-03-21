/**
 * Layout tokens for the global app navbar. Used by {@link Navbar} and by shell
 * chrome (e.g. mobile overlays) that must align with the header footprint.
 */
export const NAVBAR_HEADER_CLASSNAME =
  'shrink-0 border-b border-zinc-800/90 bg-zinc-950/95 px-3 py-2 backdrop-blur sm:px-4';

/**
 * Tailwind `top-*` for fixed layers that sit below the navbar. Keep in sync
 * with {@link NAVBAR_HEADER_CLASSNAME} and the navbar’s inner content height.
 */
export const APP_SHELL_BELOW_NAVBAR_TOP_CLASS = 'top-[5.75rem]';
