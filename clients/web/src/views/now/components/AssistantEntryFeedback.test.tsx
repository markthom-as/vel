import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { AssistantEntryFeedback } from './AssistantEntryFeedback'

describe('AssistantEntryFeedback', () => {
  it('shows a retry affordance for retryable assistant failures', () => {
    const onRetry = vi.fn()

    render(
      <AssistantEntryFeedback
        message={{ status: 'error', message: 'Temporary provider failure' }}
        inlineResponse={null}
        assistantEntryThreadId={null}
        canRetry
        onRetry={onRetry}
      />,
    )

    fireEvent.click(screen.getByRole('button', { name: 'Retry' }))

    expect(onRetry).toHaveBeenCalledTimes(1)
  })
})
