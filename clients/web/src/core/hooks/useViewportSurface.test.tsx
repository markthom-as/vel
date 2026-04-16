import { act, renderHook } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  buildViewportSnapshot,
  getSurfaceFromWidth,
  supportsKeyboardInset,
  useViewportSurface,
} from './useViewportSurface';

function setViewport(width: number, height: number) {
  Object.defineProperty(window, 'innerWidth', {
    configurable: true,
    value: width,
  });
  Object.defineProperty(window, 'innerHeight', {
    configurable: true,
    value: height,
  });
}

function installMatchMedia() {
  const listeners: Array<(event: MediaQueryListEvent) => void> = [];
  const addEventListener = vi.fn((event: string, listener: (event: MediaQueryListEvent) => void) => {
    if (event === 'change') {
      listeners.push(listener);
    }
  });
  const removeEventListener = vi.fn((event: string, listener: (event: MediaQueryListEvent) => void) => {
    if (event !== 'change') {
      return;
    }
    const index = listeners.indexOf(listener);
    if (index >= 0) {
      listeners.splice(index, 1);
    }
  });
  Object.defineProperty(window, 'matchMedia', {
    configurable: true,
    value: vi.fn((query: string) => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener,
      removeEventListener,
      dispatchEvent: vi.fn(),
    })),
  });
  return { addEventListener, removeEventListener };
}

function installVisualViewport(height: number) {
  const addEventListener = vi.fn();
  const removeEventListener = vi.fn();
  Object.defineProperty(window, 'visualViewport', {
    configurable: true,
    value: {
      height,
      addEventListener,
      removeEventListener,
    },
  });
  return { addEventListener, removeEventListener };
}

afterEach(() => {
  vi.restoreAllMocks();
  Object.defineProperty(window, 'visualViewport', {
    configurable: true,
    value: undefined,
  });
});

describe('viewport surface contract', () => {
  it('maps widths to canonical mobile tablet and desktop surfaces', () => {
    expect(getSurfaceFromWidth(390)).toBe('mobile');
    expect(getSurfaceFromWidth(767)).toBe('mobile');
    expect(getSurfaceFromWidth(768)).toBe('tablet');
    expect(getSurfaceFromWidth(1199)).toBe('tablet');
    expect(getSurfaceFromWidth(1200)).toBe('desktop');
  });

  it('builds derived flags and keyboard-inset state from a viewport window', () => {
    const snapshot = buildViewportSnapshot({
      innerWidth: 834,
      innerHeight: 1112,
      visualViewport: { height: 760 },
    });

    expect(snapshot.surface).toBe('tablet');
    expect(snapshot.isTablet).toBe(true);
    expect(snapshot.isLandscape).toBe(false);
    expect(snapshot.supportsKeyboardInset).toBe(true);
    expect(supportsKeyboardInset({ innerWidth: 390, innerHeight: 844, visualViewport: null })).toBe(false);
  });

  it('updates on resize and cleans up global listeners', () => {
    setViewport(1280, 900);
    const matchMedia = installMatchMedia();
    const visualViewport = installVisualViewport(900);
    const addWindowListener = vi.spyOn(window, 'addEventListener');
    const removeWindowListener = vi.spyOn(window, 'removeEventListener');

    const { result, unmount } = renderHook(() => useViewportSurface());

    expect(result.current.surface).toBe('desktop');
    act(() => {
      setViewport(390, 844);
      window.dispatchEvent(new Event('resize'));
    });
    expect(result.current.surface).toBe('mobile');
    expect(result.current.isMobile).toBe(true);

    unmount();

    expect(addWindowListener).toHaveBeenCalledWith('resize', expect.any(Function));
    expect(removeWindowListener).toHaveBeenCalledWith('resize', expect.any(Function));
    expect(matchMedia.addEventListener).toHaveBeenCalled();
    expect(matchMedia.removeEventListener).toHaveBeenCalled();
    expect(visualViewport.addEventListener).toHaveBeenCalledWith('resize', expect.any(Function));
    expect(visualViewport.removeEventListener).toHaveBeenCalledWith('resize', expect.any(Function));
  });
});
