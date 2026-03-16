import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor, within } from '@testing-library/react'
import { SettingsPage } from './SettingsPage'
import * as client from '../api/client'
import { clearQueryCache } from '../data/query'
import type { WsEnvelope } from '../types'

const subscribeWs = vi.fn()

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPatch: vi.fn(),
}))

vi.mock('../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

function getSettingsRoot(container: HTMLElement) {
  return container.firstElementChild as HTMLElement
}

function setDocumentVisibilityState(state: DocumentVisibilityState) {
  Object.defineProperty(document, 'visibilityState', {
    configurable: true,
    get: () => state,
  })
}

describe('SettingsPage', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.useRealTimers()
    vi.stubGlobal('confirm', vi.fn(() => true))
    subscribeWs.mockReset()
    setDocumentVisibilityState('visible')
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/settings') {
        return {
          ok: true,
          data: { disable_proactive: false, toggle_risks: true, toggle_reminders: true },
          meta: { request_id: 'req_1' },
        } as never
      }
      if (path === '/v1/runs?limit=6') {
        return {
          ok: true,
          data: [
            {
              id: 'run_122',
              kind: 'search',
              status: 'failed',
              automatic_retry_supported: false,
              automatic_retry_reason: 'search runs do not have an automatic retry executor',
              unsupported_retry_override: false,
              unsupported_retry_override_reason: null,
              created_at: '2026-03-16T21:55:00Z',
              started_at: null,
              finished_at: '2026-03-16T21:56:00Z',
              duration_ms: 60000,
              retry_scheduled_at: null,
              retry_reason: null,
              blocked_reason: null,
            },
            {
              id: 'run_123',
              kind: 'search',
              status: 'retry_scheduled',
              automatic_retry_supported: false,
              automatic_retry_reason: 'search runs do not have an automatic retry executor',
              unsupported_retry_override: true,
              unsupported_retry_override_reason: 'manual operator override',
              created_at: '2026-03-16T22:00:00Z',
              started_at: null,
              finished_at: null,
              duration_ms: null,
              retry_scheduled_at: '2026-03-16T22:05:00Z',
              retry_reason: 'operator_override',
              blocked_reason: null,
            },
          ],
          meta: { request_id: 'req_runs' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })
    vi.mocked(client.apiPatch).mockResolvedValue({} as never)
  })

  it('shows Back button and Settings heading when loaded', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    expect(screen.getByText(/loading settings/i)).toBeInTheDocument()
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('button', { name: /back/i })).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    expect(within(root).getByRole('heading', { name: /settings/i })).toBeInTheDocument()
  })

  it('renders checkboxes for disable_proactive, toggle_risks, toggle_reminders', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/disable proactive/i)).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    expect(within(root).getByText(/show risks/i)).toBeInTheDocument()
    expect(within(root).getByText(/show reminders/i)).toBeInTheDocument()
  })

  it('calls onBack when Back is clicked', async () => {
    const onBack = vi.fn()
    const { container } = render(<SettingsPage onBack={onBack} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('button', { name: /back/i })).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    fireEvent.click(within(root).getByRole('button', { name: /back/i }))
    expect(onBack).toHaveBeenCalledTimes(1)
  })

  it('calls apiPatch when a checkbox is toggled', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/show risks/i)).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    const risksCheckbox = within(root).getByRole('checkbox', { name: /show risks/i })
    fireEvent.click(risksCheckbox)
    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith('/api/settings', { toggle_risks: false })
    })
  })

  it('renders recent run policy and override metadata', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: /recent runs/i })).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    expect(within(root).getByText('run_123')).toBeInTheDocument()
    expect(within(root).getByText('run_122')).toBeInTheDocument()
    expect(within(root).getAllByText(/auto retry:/i)).toHaveLength(2)
    expect(within(root).getAllByText(/search runs do not have an automatic retry executor/i)).toHaveLength(2)
    expect(within(root).getByText(/manual override active: manual operator override/i)).toBeInTheDocument()
    expect(within(root).getAllByRole('button', { name: /schedule retry/i })).toHaveLength(2)
    expect(within(root).getAllByRole('button', { name: /block run/i })).toHaveLength(2)
    expect(within(root).getAllByLabelText(/retry reason/i)).toHaveLength(2)
    expect(within(root).getAllByLabelText(/delay seconds/i)).toHaveLength(2)
    expect(within(root).getAllByLabelText(/blocked reason/i)).toHaveLength(2)
    expect(within(root).getAllByText(/requires override/i)).toHaveLength(2)
  })

  it('schedules unsupported retry with explicit override after confirmation', async () => {
    const confirmSpy = vi.mocked(window.confirm)
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getAllByRole('button', { name: /schedule retry/i })).toHaveLength(2)
    })

    const root = getSettingsRoot(container)
    const reasonInputs = within(root).getAllByLabelText(/retry reason/i)
    const delayInputs = within(root).getAllByLabelText(/delay seconds/i)
    fireEvent.change(reasonInputs[0] as HTMLElement, { target: { value: 'manual_backoff' } })
    fireEvent.change(delayInputs[0] as HTMLElement, { target: { value: '90' } })
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(confirmSpy).toHaveBeenCalledWith(
        'Automatic retry is unsupported for search. Schedule a manual override retry anyway?',
      )
    })
    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith('/v1/runs/run_122', {
        status: 'retry_scheduled',
        reason: 'manual_backoff',
        retry_after_seconds: 90,
        allow_unsupported_retry: true,
      })
    })
  })

  it('blocks a run with an inline blocked reason', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getAllByRole('button', { name: /block run/i })).toHaveLength(2)
    })

    const root = getSettingsRoot(container)
    const blockedReasonInputs = within(root).getAllByLabelText(/blocked reason/i)
    fireEvent.change(blockedReasonInputs[0] as HTMLElement, { target: { value: 'waiting_on_dependency' } })
    fireEvent.click(within(root).getAllByRole('button', { name: /block run/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith('/v1/runs/run_122', {
        status: 'blocked',
        blocked_reason: 'waiting_on_dependency',
      })
    })
    await waitFor(() => {
      expect(within(root).getByText('Run blocked.')).toBeInTheDocument()
    })
  })

  it('renders run action errors inline on the relevant card', async () => {
    vi.mocked(client.apiPatch).mockRejectedValueOnce(new Error('API 500: /v1/runs/run_122'))

    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getAllByRole('button', { name: /block run/i })).toHaveLength(2)
    })

    const root = getSettingsRoot(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /block run/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('API 500: /v1/runs/run_122')).toBeInTheDocument()
    })
  })

  it('updates rendered runs from websocket payloads without refetching', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: /recent runs/i })).toBeInTheDocument()
    })

    const runsCallsBefore = vi.mocked(client.apiGet).mock.calls.filter(
      ([path]) => path === '/v1/runs?limit=6',
    ).length

    const runUpdateListener = wsListener as ((event: WsEnvelope) => void) | null
    expect(runUpdateListener).not.toBeNull()
    runUpdateListener?.({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:10:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T21:56:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'ws_blocked_reason',
      },
    })
    await Promise.resolve()

    const runsCallsAfter = vi.mocked(client.apiGet).mock.calls.filter(
      ([path]) => path === '/v1/runs?limit=6',
    ).length
    expect(runsCallsAfter).toBe(runsCallsBefore)
    expect(within(root).getByText('Blocked reason: ws_blocked_reason')).toBeInTheDocument()
  })

  it('subscribes to websocket updates for runs', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: /recent runs/i })).toBeInTheDocument()
    })

    expect(subscribeWs.mock.calls.length).toBeGreaterThanOrEqual(1)
  })

  it('refetches runs when the document becomes visible again', async () => {
    setDocumentVisibilityState('hidden')

    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: /recent runs/i })).toBeInTheDocument()
    })

    const runsCallsBefore = vi.mocked(client.apiGet).mock.calls.filter(
      ([path]) => path === '/v1/runs?limit=6',
    ).length

    setDocumentVisibilityState('visible')
    document.dispatchEvent(new Event('visibilitychange'))
    await Promise.resolve()

    const runsCallsAfter = vi.mocked(client.apiGet).mock.calls.filter(
      ([path]) => path === '/v1/runs?limit=6',
    ).length
    expect(runsCallsAfter).toBeGreaterThan(runsCallsBefore)
  })
})
