import { describe, expect, it } from 'vitest'
import { nudgeKindMobileLabel, nudgePresentationPriority } from './nudgePresentationMapping'

describe('nudgePresentationMapping', () => {
  it('keeps unknown nudge kind labels compact and normalized for mobile tags', () => {
    expect(nudgeKindMobileLabel('  TODOIST   OVERDUE   BACKLOG_WITH_LONG_TAIL  ')).toBe('Todoist Overdue')
    expect(nudgeKindMobileLabel('')).toBe('Nudge')
    expect(nudgeKindMobileLabel('exceptionally_verbose_custom_provider')).toBe('Exceptionally V...')
  })

  it('keeps known nudge labels and warning priority explicit', () => {
    expect(nudgeKindMobileLabel('freshness_warning')).toBe('Freshness warning')
    expect(nudgePresentationPriority('freshness_warning', false)).toBe('warning')
    expect(nudgePresentationPriority('trust_warning', false)).toBe('warning')
    expect(nudgePresentationPriority('nudge', true)).toBe('urgent')
    expect(nudgePresentationPriority('nudge', false)).toBe('normal')
  })
})
