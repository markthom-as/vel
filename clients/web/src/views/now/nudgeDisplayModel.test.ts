import { describe, expect, it } from 'vitest'
import { buildNudgeDisplayModel } from './nudgeDisplayModel'

describe('buildNudgeDisplayModel', () => {
  it('uses compact human fallback labels for unknown nudge kinds', () => {
    const model = buildNudgeDisplayModel(
      {
        id: 'unknown_mobile_bar',
        kind: 'todoist_overdue_backlog_with_long_tail',
        title: 'Backlog needs a decision',
        summary: 'Several items need routing.',
        urgent: false,
        timestamp: 1710000000,
        primary_thread_id: null,
        actions: [],
      },
      [],
    )

    expect(model.kindLabel).toBe('Todoist Overdue')
    expect(model.kindUrgent).toBe(false)
  })

  it('keeps warning-priority nudges distinct from normal nudges without requiring urgent', () => {
    const model = buildNudgeDisplayModel(
      {
        id: 'freshness_warning',
        kind: 'freshness_warning',
        title: 'Calendar is stale',
        summary: 'Refresh before trusting the day plan.',
        urgent: false,
        timestamp: 1710000000,
        primary_thread_id: null,
        actions: [],
      },
      [],
    )

    expect(model.kindLabel).toBe('Freshness warning')
    expect(model.kindUrgent).toBe(true)
  })
})
