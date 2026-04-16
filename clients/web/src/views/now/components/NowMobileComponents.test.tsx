import { cleanup, fireEvent, render, screen, within } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { CompactTaskLaneRow } from './CompactTaskLaneRow'
import { NowNudgeStrip } from './NowNudgeStrip'
import { NowTasksSection } from './NowTasksSection'

const baseTask = {
  id: 'task_1',
  task_kind: 'task',
  text: 'Reply to Dimitri about the launch plan before lunch',
  title: 'Reply to Dimitri about the launch plan before lunch',
  description: 'Confirm the narrow next step and keep the thread attached.',
  state: 'active',
  primary_thread_id: 'conv_task_1',
  tags: ['launch'],
  project: 'Vel',
  due_label: 'Today',
  is_overdue: false,
  deadline_label: null,
  deadline_passed: false,
}

describe('Now mobile components', () => {
  afterEach(() => {
    cleanup()
  })

  it('keeps compact task rows thumb-sized and routes thread actions', () => {
    const onOpenThread = vi.fn()
    const onComplete = vi.fn()

    const { container } = render(
      <CompactTaskLaneRow
        item={baseTask as never}
        surface="mobile"
        onOpenThread={onOpenThread}
        onComplete={onComplete}
      />,
    )

    const row = container.firstElementChild as HTMLElement
    expect(row).toHaveClass('min-h-14')

    const complete = screen.getByRole('button', { name: /complete reply to dimitri/i })
    expect(complete).toHaveClass('!h-10')
    fireEvent.click(complete)
    expect(onComplete).toHaveBeenCalledTimes(1)

    const openThread = screen.getByRole('button', { name: /open thread/i })
    expect(openThread).toHaveClass('!min-h-10')
    fireEvent.click(openThread)
    expect(onOpenThread).toHaveBeenCalledTimes(1)
  })

  it('keeps nudge strip actions tappable on mobile and preserves action routing', () => {
    const onBarAction = vi.fn()
    const bar = {
      id: 'todoist_overdue_backlog',
      kind: 'todoist_overdue_backlog_with_long_tail',
      title: 'A very long mobile nudge title that should stay constrained beside actions',
      summary: 'Several overdue items need a small decision before the day can settle.',
      urgent: true,
      timestamp: 1710000000,
      primary_thread_id: 'conv_nudge_1',
      actions: [
        { kind: 'open_thread', label: 'Open thread' },
        { kind: 'reschedule_today:task_1', label: 'Reschedule all to today' },
      ],
    }

    const { container } = render(
      <NowNudgeStrip
        bars={[bar] as never}
        nowTs={1710000600}
        actionItems={[]}
        surface="mobile"
        onBarAction={onBarAction}
      />,
    )

    const row = container.querySelector('.min-h-14') as HTMLElement
    expect(row).toBeInTheDocument()
    expect(screen.getByText('Todoist Overdue')).toBeInTheDocument()

    const openButton = screen.getByRole('button', { name: /Open \(A very long mobile nudge title/i })
    expect(openButton).toHaveClass('!min-h-10')
    fireEvent.click(openButton)

    expect(onBarAction).toHaveBeenCalledWith(bar, bar.actions[0])
  })

  it('passes mobile row sizing through the task section', () => {
    render(
      <NowTasksSection
        taskLane={{
          active: baseTask,
          pending: [],
          recent_completed: [],
          next_up: [],
          inbox: [],
          if_time_allows: [],
          completed: [],
        } as never}
        surface="mobile"
        riskItems={[]}
        allTaskMetadata={[]}
        commitmentIds={new Set(['task_1'])}
        completedCount={0}
        remainingCount={1}
        backlogCount={0}
        groupedTaskCount={1}
        pendingCommitments={{}}
        commitmentMessages={{}}
        onCompleteCommitment={vi.fn()}
        onOpenThread={vi.fn()}
      />,
    )

    const nowGroup = screen.getByText('NOW').closest('section') as HTMLElement
    expect(within(nowGroup).getByText(baseTask.text).closest('[class*="min-h-14"]')).toBeInTheDocument()
  })
})
