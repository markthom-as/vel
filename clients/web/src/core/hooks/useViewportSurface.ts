import { useCallback, useEffect, useMemo, useState } from 'react';
import { viewportSurfaceBreakpoints } from '../Theme/tokens';

export type ViewportSurface = 'mobile' | 'tablet' | 'desktop';

export interface ViewportSurfaceSnapshot {
  surface: ViewportSurface;
  width: number;
  height: number;
  isMobile: boolean;
  isTablet: boolean;
  isDesktop: boolean;
  isLandscape: boolean;
  supportsKeyboardInset: boolean;
}

type ViewportWindow = Pick<Window, 'innerHeight' | 'innerWidth'> & {
  visualViewport?: Pick<VisualViewport, 'height'> | null;
};

const DEFAULT_SURFACE: ViewportSurfaceSnapshot = {
  surface: 'desktop',
  width: 0,
  height: 0,
  isMobile: false,
  isTablet: false,
  isDesktop: true,
  isLandscape: false,
  supportsKeyboardInset: false,
};

export function getSurfaceFromWidth(width: number): ViewportSurface {
  if (width <= viewportSurfaceBreakpoints.mobileMax) {
    return 'mobile';
  }
  if (width <= viewportSurfaceBreakpoints.tabletMax) {
    return 'tablet';
  }
  return 'desktop';
}

export function supportsKeyboardInset(viewportWindow: ViewportWindow | undefined): boolean {
  if (!viewportWindow?.visualViewport) {
    return false;
  }
  return viewportWindow.visualViewport.height < viewportWindow.innerHeight;
}

export function buildViewportSnapshot(viewportWindow?: ViewportWindow): ViewportSurfaceSnapshot {
  const activeWindow = viewportWindow ?? (typeof window === 'undefined' ? undefined : window);
  if (!activeWindow) {
    return DEFAULT_SURFACE;
  }
  const width = activeWindow.innerWidth;
  const height = activeWindow.innerHeight;
  const surface = getSurfaceFromWidth(width);
  const isLandscape = width >= height;
  return {
    surface,
    width,
    height,
    isMobile: surface === 'mobile',
    isTablet: surface === 'tablet',
    isDesktop: surface === 'desktop',
    isLandscape,
    supportsKeyboardInset: supportsKeyboardInset(activeWindow),
  };
}

export function useViewportSurface(): ViewportSurfaceSnapshot {
  const [surfaceSnapshot, setSurfaceSnapshot] = useState<ViewportSurfaceSnapshot>(buildViewportSnapshot);

  const update = useCallback(() => {
    setSurfaceSnapshot(buildViewportSnapshot());
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return;
    }
    const mediaQueries =
      typeof window.matchMedia === 'function'
        ? [
            window.matchMedia(`(max-width: ${viewportSurfaceBreakpoints.mobileMax}px)`),
            window.matchMedia(`(max-width: ${viewportSurfaceBreakpoints.tabletMax}px)`),
          ]
        : [];
    window.addEventListener('resize', update);
    window.addEventListener('orientationchange', update);
    window.addEventListener('pageshow', update);
    for (const mediaQuery of mediaQueries) {
      mediaQuery.addEventListener('change', update);
    }
    const { visualViewport } = window;
    visualViewport?.addEventListener('resize', update);
    visualViewport?.addEventListener('scroll', update);
    return () => {
      window.removeEventListener('resize', update);
      window.removeEventListener('orientationchange', update);
      window.removeEventListener('pageshow', update);
      for (const mediaQuery of mediaQueries) {
        mediaQuery.removeEventListener('change', update);
      }
      visualViewport?.removeEventListener('resize', update);
      visualViewport?.removeEventListener('scroll', update);
    };
  }, [update]);

  return useMemo(() => surfaceSnapshot, [surfaceSnapshot]);
}
