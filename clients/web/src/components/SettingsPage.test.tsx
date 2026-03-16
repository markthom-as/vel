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
  apiPost: vi.fn(),
}))

vi.mock('../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

function getSettingsRoot(container: HTMLElement) {
  return container.firstElementChild as HTMLElement
}

function createDeferred<T>() {
  let resolve!: (value: T | PromiseLike<T>) => void
  let reject!: (reason?: unknown) => void
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    resolve = resolvePromise
    reject = rejectPromise
  })
  return { promise, resolve, reject }
}

async function openRunsTab(container: HTMLElement) {
  const root = getSettingsRoot(container)
  await waitFor(() => {
    expect(within(root).getByRole('button', { name: /^runs$/i })).toBeInTheDocument()
  })
  fireEvent.click(within(root).getByRole('button', { name: /^runs$/i }))
  await waitFor(() => {
    expect(within(root).getByRole('heading', { name: /recent runs/i })).toBeInTheDocument()
  })
  return root
}

async function openIntegrationsTab(container: HTMLElement) {
  const root = getSettingsRoot(container)
  await waitFor(() => {
    expect(within(root).getByRole('button', { name: /^integrations$/i })).toBeInTheDocument()
  })
  fireEvent.click(within(root).getByRole('button', { name: /^integrations$/i }))
  await waitFor(() => {
    expect(within(root).getByRole('heading', { name: /google calendar/i })).toBeInTheDocument()
  })
  return root
}

describe('SettingsPage', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.useRealTimers()
    subscribeWs.mockReset()
    vi.mocked(client.apiGet).mockReset()
    vi.mocked(client.apiPatch).mockReset()
    vi.mocked(client.apiPost).mockReset()
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/settings') {
        return {
          ok: true,
          data: { disable_proactive: false, toggle_risks: true, toggle_reminders: true },
          meta: { request_id: 'req_1' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: {
            google_calendar: {
              configured: true,
              connected: true,
              has_client_id: true,
              has_client_secret: true,
              calendars: [],
              all_calendars_selected: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
            },
            todoist: {
              configured: true,
              connected: true,
              has_api_token: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
            },
          },
          meta: { request_id: 'req_integrations' },
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
    vi.mocked(client.apiPost).mockResolvedValue({ ok: true } as never)
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

  it('keeps todoist sync active while google credential save is pending', async () => {
    const googleSave = createDeferred<unknown>()
    vi.mocked(client.apiPatch).mockImplementationOnce(() => googleSave.promise as never)
    vi.mocked(client.apiPost).mockResolvedValueOnce({ ok: true } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    const googleClientIdInput = within(root).getByLabelText(/client id/i)
    const googleClientSecretInput = within(root).getByLabelText(/client secret/i)
    fireEvent.change(googleClientIdInput as HTMLElement, { target: { value: 'client-id' } })
    fireEvent.change(googleClientSecretInput as HTMLElement, { target: { value: 'client-secret' } })

    fireEvent.click(within(root).getByRole('button', { name: /save credentials/i }))

    await waitFor(() => {
      expect(within(root).getByRole('button', { name: /saving…/i })).toBeInTheDocument()
    })

    const todoistSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Todoist'),
    )
    expect(todoistSyncButton).toBeDefined()
    expect(todoistSyncButton).not.toBeDisabled()

    fireEvent.click(todoistSyncButton as HTMLElement)
    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/todoist', {})
    })

    googleSave.resolve({ ok: true })
  })

  it('renders integration feedback on the matching integration card', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({ ok: true } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    fireEvent.click(within(root).getByRole('button', { name: /save credentials/i }))

    await waitFor(() => {
      expect(within(root).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    })

    const googleCard = within(root).getByRole('heading', { name: /google calendar/i }).closest('.rounded-lg')
    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(googleCard).not.toBeNull()
    expect(todoistCard).not.toBeNull()
    expect(within(googleCard as HTMLElement).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    expect(within(todoistCard as HTMLElement).queryByText('Google Calendar credentials saved.')).toBeNull()
  })

  it('renders recent run policy and override metadata', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
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

  it('schedules unsupported retry only after inline override confirmation', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'retry_scheduled',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: true,
        unsupported_retry_override_reason: 'manual operator override',
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: null,
        duration_ms: null,
        retry_scheduled_at: '2026-03-16T22:20:00Z',
        retry_reason: 'manual_backoff',
        blocked_reason: null,
      },
      meta: { request_id: 'req_patch' },
    } as never)
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
    const reasonInputs = within(root).getAllByLabelText(/retry reason/i)
    const delayInputs = within(root).getAllByLabelText(/delay seconds/i)
    fireEvent.change(reasonInputs[0] as HTMLElement, { target: { value: 'manual_backoff' } })
    fireEvent.change(delayInputs[0] as HTMLElement, { target: { value: '90' } })
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)

    expect(client.apiPatch).not.toHaveBeenCalledWith('/v1/runs/run_122', expect.anything())
    expect(within(root).getByText(/automatic retry is unsupported for search/i)).toBeInTheDocument()
    fireEvent.click(within(root).getByRole('button', { name: /confirm override retry/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith('/v1/runs/run_122', {
        status: 'retry_scheduled',
        reason: 'manual_backoff',
        retry_after_seconds: 90,
        allow_unsupported_retry: true,
      })
    })
    await waitFor(() => {
      expect(within(root).getAllByText(/Retry at:/i)).toHaveLength(2)
    })
    expect(within(root).getAllByText('retry_scheduled')).toHaveLength(2)
    expect(within(root).getAllByText('Retry reason: manual_backoff')).toHaveLength(1)
  })

  it('cancels unsupported retry override without hitting the API', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)

    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()
    fireEvent.click(within(root).getByRole('button', { name: /cancel/i }))

    expect(within(root).queryByRole('button', { name: /confirm override retry/i })).toBeNull()
    expect(client.apiPatch).not.toHaveBeenCalledWith('/v1/runs/run_122', expect.anything())
  })

  it('keeps pending override armed across eligible websocket updates', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)
    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:15:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'failed',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:14:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'still_retryable',
      },
    })
    await Promise.resolve()

    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()
  })

  it('clears pending override when websocket update makes the run ineligible', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)
    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:16:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'retry_scheduled',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: true,
        unsupported_retry_override_reason: 'manual operator override',
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: null,
        duration_ms: null,
        retry_scheduled_at: '2026-03-16T22:16:00Z',
        retry_reason: 'operator_override',
        blocked_reason: null,
      },
    })
    await waitFor(() => {
      expect(within(root).queryByRole('button', { name: /confirm override retry/i })).toBeNull()
    })
  })

  it('clears retry success feedback when a later websocket update supersedes it', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'retry_scheduled',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: true,
        unsupported_retry_override_reason: 'manual operator override',
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: null,
        duration_ms: null,
        retry_scheduled_at: '2026-03-16T22:20:00Z',
        retry_reason: 'manual_backoff',
        blocked_reason: null,
      },
      meta: { request_id: 'req_patch' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)
    fireEvent.click(within(root).getByRole('button', { name: /confirm override retry/i }))

    await waitFor(() => {
      expect(within(root).getByText('Retry scheduled.')).toBeInTheDocument()
    })

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:25:00Z',
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
        finished_at: '2026-03-16T22:24:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'superseded_after_retry',
      },
    })

    await waitFor(() => {
      expect(within(root).queryByText('Retry scheduled.')).toBeNull()
    })
  })

  it('clears block error feedback when a later websocket update shows the run blocked anyway', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })
    vi.mocked(client.apiPatch).mockRejectedValueOnce(new Error('API 500: /v1/runs/run_122'))

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /block run/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('API 500: /v1/runs/run_122')).toBeInTheDocument()
    })

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:30:00Z',
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
        finished_at: '2026-03-16T22:29:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'blocked_elsewhere',
      },
    })

    await waitFor(() => {
      expect(within(root).queryByText('API 500: /v1/runs/run_122')).toBeNull()
    })
  })

  it('keeps other run controls active while one run action is pending', async () => {
    const firstPatch = createDeferred<unknown>()
    vi.mocked(client.apiPatch)
      .mockImplementationOnce(() => firstPatch.promise as never)
      .mockResolvedValueOnce({
        ok: true,
        data: {
          id: 'run_123',
          kind: 'search',
          status: 'blocked',
          automatic_retry_supported: false,
          automatic_retry_reason: 'search runs do not have an automatic retry executor',
          unsupported_retry_override: true,
          unsupported_retry_override_reason: 'manual operator override',
          created_at: '2026-03-16T22:00:00Z',
          started_at: null,
          finished_at: '2026-03-16T22:31:00Z',
          duration_ms: 60000,
          retry_scheduled_at: null,
          retry_reason: null,
          blocked_reason: 'parallel_block',
        },
        meta: { request_id: 'req_parallel' },
      } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
    const blockButtons = within(root).getAllByRole('button', { name: /block run/i })
    fireEvent.click(blockButtons[0] as HTMLElement)

    await waitFor(() => {
      expect(within(root).getAllByRole('button', { name: /working…/i })).toHaveLength(2)
    })

    const updatedBlockButtons = within(root).getAllByRole('button', { name: /block run/i })
    expect(updatedBlockButtons[0]).not.toBeDisabled()
    fireEvent.click(updatedBlockButtons[0] as HTMLElement)

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith('/v1/runs/run_123', {
        status: 'blocked',
        blocked_reason: 'operator_ui_blocked',
      })
    })

    firstPatch.resolve({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:32:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'first_block',
      },
      meta: { request_id: 'req_first_parallel' },
    })
  })

  it('blocks a run with an inline blocked reason', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:21:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'waiting_on_dependency',
      },
      meta: { request_id: 'req_patch_block' },
    } as never)
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
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
    expect(within(root).getAllByText('Blocked reason: waiting_on_dependency')).toHaveLength(1)
  })

  it('renders run action errors inline on the relevant card', async () => {
    vi.mocked(client.apiPatch).mockRejectedValueOnce(new Error('API 500: /v1/runs/run_122'))

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRunsTab(container)
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
    const root = await openRunsTab(container)
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
    await openRunsTab(container)

    expect(subscribeWs.mock.calls.length).toBeGreaterThanOrEqual(1)
  })

})
