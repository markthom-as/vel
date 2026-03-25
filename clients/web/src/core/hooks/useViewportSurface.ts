import { useCallback, useEffect, useMemo, useState } from 'react';

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
  if (width < 768) {
    return 'mobile';
  }
  if (width < 1200) {
    return 'tablet';
  }
  return 'desktop';
}

export function buildViewportSnapshot(): ViewportSurfaceSnapshot {
  if (typeof window === 'undefined') {
    return DEFAULT_SURFACE;
  }
  const width = window.innerWidth;
  const height = window.innerHeight;
  const surface = getSurfaceFromWidth(width);
  const isLandscape = width >= height;
  const supportsKeyboardInset =
    typeof window.visualViewport !== 'undefined'
      && window.visualViewport.height < window.innerHeight;
  return {
    surface,
    width,
    height,
    isMobile: surface === 'mobile',
    isTablet: surface === 'tablet',
    isDesktop: surface === 'desktop',
    isLandscape,
    supportsKeyboardInset,
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
    const mediaQueries = [window.matchMedia('(max-width: 767px)'), window.matchMedia('(max-width: 1199px)')];
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
