import { describe, expect, it } from 'vitest'

import { resolveDevProxyTarget } from './devProxyTarget'

describe('resolveDevProxyTarget', () => {
  it('defaults to localhost veld', () => {
    expect(resolveDevProxyTarget({})).toBe('http://127.0.0.1:4130')
  })

  it('prefers the explicit dev proxy target', () => {
    expect(
      resolveDevProxyTarget({
        VELD_URL: 'http://127.0.0.1:4242',
        VITE_API_URL: 'http://127.0.0.1:4343',
      }),
    ).toBe('http://127.0.0.1:4242')
  })

  it('falls back to VITE_API_URL when no proxy target override is set', () => {
    expect(
      resolveDevProxyTarget({
        VITE_API_URL: 'http://127.0.0.1:4343',
      }),
    ).toBe('http://127.0.0.1:4343')
  })
})
