import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor, within } from '@testing-library/react'
import { SettingsPage } from './SettingsPage'
import * as client from '../api/client'
import { clearQueryCache } from '../data/query'
import { resetWsQuerySyncForTests } from '../data/ws-sync'
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

async function openComponentsTab(container: HTMLElement) {
  const root = getSettingsRoot(container)
  await waitFor(() => {
    expect(within(root).getByRole('button', { name: /^components$/i })).toBeInTheDocument()
  })
  fireEvent.click(within(root).getByRole('button', { name: /^components$/i }))
  await waitFor(() => {
    expect(within(root).getByRole('heading', { name: /google calendar/i })).toBeInTheDocument()
  })
  return root
}

async function openLoopsTab(container: HTMLElement) {
  const root = getSettingsRoot(container)
  await waitFor(() => {
    expect(within(root).getByRole('button', { name: /^loops$/i })).toBeInTheDocument()
  })
  fireEvent.click(within(root).getByRole('button', { name: /^loops$/i }))
  await waitFor(() => {
    expect(within(root).getByRole('heading', { name: /runtime loops/i })).toBeInTheDocument()
  })
  return root
}

describe('SettingsPage', () => {
  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.useRealTimers()
    subscribeWs.mockReset()
    vi.mocked(client.apiGet).mockReset()
    vi.mocked(client.apiPatch).mockReset()
    vi.mocked(client.apiPost).mockReset()
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/settings') {
        return {
          ok: true,
          data: {
            disable_proactive: false,
            toggle_risks: true,
            toggle_reminders: true,
            timezone: 'America/Denver',
            adaptive_policy_overrides: {
              commute_buffer_minutes: 30,
              default_prep_minutes: 45,
            },
          },
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
              guidance: {
                title: 'Calendar has not synced yet',
                detail: 'Run a calendar sync to populate upcoming events and prep/commute context.',
                action: 'Sync now',
              },
            },
            todoist: {
              configured: true,
              connected: true,
              has_api_token: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            activity: {
              configured: true,
              source_path: '/tmp/activity.json',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            git: {
              configured: true,
              source_path: '/tmp/git.json',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            messaging: {
              configured: true,
              source_path: '/tmp/messaging.json',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            notes: {
              configured: true,
              source_path: '/tmp/notes',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: {
                title: 'Local sync failed',
                detail: 'The notes source could not be read.',
                action: 'Fix the source and retry sync',
              },
            },
            transcripts: {
              configured: true,
              source_path: '/tmp/transcripts.json',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
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
      if (path === '/api/components') {
        return {
          ok: true,
          data: [
            {
              id: 'google-calendar',
              name: 'Google Calendar',
              description: 'Calendar ingest',
              status: 'ok',
              last_restarted_at: 1_700_000_000,
              last_error: null,
              restart_count: 0,
            },
            {
              id: 'todoist',
              name: 'Todoist',
              description: 'Task ingest',
              status: 'ok',
              last_restarted_at: 1_700_000_000,
              last_error: null,
              restart_count: 2,
            },
            {
              id: 'evaluate',
              name: 'Evaluate',
              description: 'Evaluate all pipelines',
              status: 'idle',
              last_restarted_at: null,
              last_error: null,
              restart_count: 0,
            },
          ],
          meta: { request_id: 'req_components' },
        } as never
      }
      if (path === '/v1/loops') {
        return {
          ok: true,
          data: [
            {
              kind: 'evaluate_current_state',
              enabled: true,
              interval_seconds: 300,
              last_started_at: 1_710_000_000,
              last_finished_at: 1_710_000_030,
              last_status: 'success',
              last_error: null,
              next_due_at: 1_710_000_300,
            },
          ],
          meta: { request_id: 'req_loops' },
        } as never
      }
      if (path === '/api/components/evaluate/logs?limit=50') {
        return {
          ok: true,
          data: [
            {
              id: 'log_eval_1',
              component_id: 'evaluate',
              event_name: 'component.restart.requested',
              status: 'running',
              message: 'component restart requested',
              payload: {},
              created_at: 1_700_000_100,
            },
            {
              id: 'log_eval_2',
              component_id: 'evaluate',
              event_name: 'component.restart.completed',
              status: 'success',
              message: 'Evaluate complete',
              payload: {},
              created_at: 1_700_000_200,
            },
          ],
          meta: { request_id: 'req_component_logs' },
        } as never
      }
      if (path === '/api/integrations/notes/logs?limit=10') {
        return {
          ok: true,
          data: [
            {
              id: 'int_notes_1',
              integration_id: 'notes',
              event_name: 'integration.sync.succeeded',
              status: 'ok',
              message: 'notes sync completed with 4 items.',
              payload: { item_count: 4 },
              created_at: 1_700_000_400,
            },
          ],
          meta: { request_id: 'req_notes_logs' },
        } as never
      }
      if (path === '/api/integrations/todoist/logs?limit=10') {
        return {
          ok: true,
          data: [
            {
              id: 'int_todoist_1',
              integration_id: 'todoist',
              event_name: 'integration.sync.failed',
              status: 'error',
              message: 'todoist sync failed: upstream 500',
              payload: { error: 'upstream 500' },
              created_at: 1_700_000_500,
            },
          ],
          meta: { request_id: 'req_todoist_logs' },
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
    expect(within(root).getByDisplayValue('America/Denver')).toBeInTheDocument()
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
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/settings',
        { toggle_risks: false },
        expect.any(Function),
      )
    })
  })

  it('saves timezone changes through settings api', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByDisplayValue('America/Denver')).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    const input = within(root).getByPlaceholderText('America/Denver')
    fireEvent.change(input, { target: { value: 'America/Los_Angeles' } })
    fireEvent.click(within(root).getByRole('button', { name: /save timezone/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/settings',
        { timezone: 'America/Los_Angeles' },
        expect.any(Function),
      )
    })
  })

  it('renders active adaptive policy overrides in general settings', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/adaptive policy overrides/i)).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    expect(within(root).getByText('30 min')).toBeInTheDocument()
    expect(within(root).getByText('45 min')).toBeInTheDocument()
  })

  it('keeps todoist sync active while google credential save is pending', async () => {
    const googleSave = createDeferred<unknown>()
    vi.mocked(client.apiPatch).mockImplementationOnce(() => googleSave.promise as never)
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'todoist', signals_ingested: 0 },
    } as never)

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
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/todoist', {}, expect.any(Function))
    })

    googleSave.resolve({ ok: true })
  })

  it('keeps unrelated integration feedback when actions finish out of order', async () => {
    const googleSave = createDeferred<unknown>()
    vi.mocked(client.apiPatch).mockImplementationOnce(() => googleSave.promise as never)
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'todoist', signals_ingested: 0 },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    fireEvent.click(within(root).getByRole('button', { name: /save credentials/i }))
    await waitFor(() => {
      expect(within(root).getByRole('button', { name: /saving…/i })).toBeInTheDocument()
    })

    const todoistSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Todoist'),
    )
    expect(todoistSyncButton).toBeDefined()
    fireEvent.click(todoistSyncButton as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
    })

    googleSave.resolve({ ok: true })

    await waitFor(() => {
      expect(within(root).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    })

    const googleCard = within(root).getByRole('heading', { name: /google calendar/i }).closest('.rounded-lg')
    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(googleCard).not.toBeNull()
    expect(todoistCard).not.toBeNull()
    expect(within(googleCard as HTMLElement).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    expect(within(todoistCard as HTMLElement).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
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

  it('keeps multiple todoist action messages on the todoist card', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({ ok: true } as never)
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'todoist', signals_ingested: 0 },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    fireEvent.click(within(root).getByRole('button', { name: /save token/i }))
    await waitFor(() => {
      expect(within(root).getByText('Todoist token saved.')).toBeInTheDocument()
    })

    const todoistSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Todoist'),
    )
    expect(todoistSyncButton).toBeDefined()
    fireEvent.click(todoistSyncButton as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
    })

    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(todoistCard).not.toBeNull()
    expect(within(todoistCard as HTMLElement).getByText('Todoist token saved.')).toBeInTheDocument()
    expect(within(todoistCard as HTMLElement).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
  })

  it('renders local integration cards and syncs the selected local adapter', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    expect(within(root).getByRole('heading', { name: /computer activity/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /git activity/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /^messaging$/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /^notes$/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /transcripts/i })).toBeInTheDocument()
    expect(within(root).getByText('Source: /tmp/activity.json')).toBeInTheDocument()

    const notesSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Notes'),
    )
    expect(notesSyncButton).toBeDefined()
    fireEvent.click(notesSyncButton as HTMLElement)

    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/notes', {}, expect.any(Function))
    })
    expect(within(root).getByText('Notes synced (0 signals).')).toBeInTheDocument()
  })

  it('shows integration sync history for the selected card', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    const notesCard = within(root).getByRole('heading', { name: /^notes$/i }).closest('.rounded-lg')
    expect(notesCard).not.toBeNull()
    fireEvent.click(within(notesCard as HTMLElement).getByRole('button', { name: /^show history$/i }))

    await waitFor(() => {
      expect(within(notesCard as HTMLElement).getByText('Recent sync history')).toBeInTheDocument()
    })
    expect(within(notesCard as HTMLElement).getByText('notes sync completed with 4 items.')).toBeInTheDocument()
    expect(within(notesCard as HTMLElement).getByText('integration.sync.succeeded')).toBeInTheDocument()
    expect(within(notesCard as HTMLElement).getByText('Items: 4')).toBeInTheDocument()

    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(todoistCard).not.toBeNull()
    fireEvent.click(within(todoistCard as HTMLElement).getByRole('button', { name: /show history/i }))

    await waitFor(() => {
      expect(within(todoistCard as HTMLElement).getByText('todoist sync failed: upstream 500')).toBeInTheDocument()
    })
    expect(within(todoistCard as HTMLElement).getByText('Error: upstream 500')).toBeInTheDocument()

    fireEvent.click(within(notesCard as HTMLElement).getByRole('checkbox', { name: /failures only/i }))
    await waitFor(() => {
      expect(within(notesCard as HTMLElement).getByText('No failed syncs in recent history.')).toBeInTheDocument()
    })

    fireEvent.click(within(todoistCard as HTMLElement).getByRole('checkbox', { name: /failures only/i }))
    expect(within(todoistCard as HTMLElement).queryByText('No failed syncs in recent history.')).toBeNull()
    expect(within(todoistCard as HTMLElement).getByText('todoist sync failed: upstream 500')).toBeInTheDocument()
  })

  it('renders guidance callouts for degraded integrations', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    expect(within(root).getByText('Calendar has not synced yet')).toBeInTheDocument()
    expect(within(root).getByText(/Run a calendar sync to populate upcoming events/i)).toBeInTheDocument()
    expect(within(root).getByText(/Recommended action: Sync now/i)).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /run: sync now/i })).toBeInTheDocument()
    expect(within(root).getByText('Local sync failed')).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /retry sync/i })).toBeInTheDocument()
  })

  it('runs the recommended google guidance action from the guidance card', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'calendar', signals_ingested: 3 },
      meta: { request_id: 'req_sync_calendar' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const googleCard = within(root).getByRole('heading', { name: /google calendar/i }).closest('.rounded-lg')
    expect(googleCard).not.toBeNull()

    fireEvent.click(within(googleCard as HTMLElement).getByRole('button', { name: /run: sync now/i }))

    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/calendar', {}, expect.any(Function))
    })
    expect(within(googleCard as HTMLElement).getByText('Calendar synced (3 signals).')).toBeInTheDocument()
  })

  it('opens history and retries local sync from guidance actions', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'notes', signals_ingested: 4 },
      meta: { request_id: 'req_sync_notes' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const notesCard = within(root).getByRole('heading', { name: /^notes$/i }).closest('.rounded-lg')
    expect(notesCard).not.toBeNull()

    fireEvent.click(within(notesCard as HTMLElement).getByRole('button', { name: /open history/i }))
    await waitFor(() => {
      expect(within(notesCard as HTMLElement).getByText('Recent sync history')).toBeInTheDocument()
    })

    fireEvent.click(within(notesCard as HTMLElement).getByRole('button', { name: /retry sync/i }))

    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/notes', {}, expect.any(Function))
    })
    expect(within(notesCard as HTMLElement).getByText('Notes synced (4 signals).')).toBeInTheDocument()
  })

  it('saves a local integration source path from the settings card', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
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
          guidance: null,
        },
        todoist: {
          configured: true,
          connected: true,
          has_api_token: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        activity: {
          configured: true,
          source_path: '/tmp/fresh-activity.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        git: {
          configured: true,
          source_path: '/tmp/git.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        messaging: {
          configured: true,
          source_path: '/tmp/messaging.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        notes: {
          configured: true,
          source_path: '/tmp/notes',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        transcripts: {
          configured: true,
          source_path: '/tmp/transcripts.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
      },
      meta: { request_id: 'req_local_source_save' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const activityCard = within(root).getByRole('heading', { name: /computer activity/i }).closest('.rounded-lg')
    expect(activityCard).not.toBeNull()

    fireEvent.change(
      within(activityCard as HTMLElement).getByPlaceholderText(/path to local snapshot file or directory/i),
      { target: { value: '/tmp/fresh-activity.json' } },
    )
    fireEvent.click(within(activityCard as HTMLElement).getByRole('button', { name: /^save path$/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/integrations/activity/source',
        { source_path: '/tmp/fresh-activity.json' },
        expect.any(Function),
      )
    })
    expect(within(activityCard as HTMLElement).getByText('Source path saved.')).toBeInTheDocument()
    expect(within(activityCard as HTMLElement).getByDisplayValue('/tmp/fresh-activity.json')).toBeInTheDocument()
  })

  it('uses guidance actions to focus and save a missing local source path', async () => {
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/settings') {
        return {
          ok: true,
          data: {
            disable_proactive: false,
            toggle_risks: true,
            toggle_reminders: true,
            timezone: 'America/Denver',
          },
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
              guidance: null,
            },
            todoist: {
              configured: true,
              connected: true,
              has_api_token: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            activity: {
              configured: false,
              source_path: null,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: {
                title: 'Local source missing',
                detail: 'Configure a source path for this local adapter before syncing it.',
                action: 'Set source path',
              },
            },
            git: {
              configured: true,
              source_path: '/tmp/git.json',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            messaging: {
              configured: true,
              source_path: '/tmp/messaging.json',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            notes: {
              configured: true,
              source_path: '/tmp/notes',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            transcripts: {
              configured: true,
              source_path: '/tmp/transcripts.json',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
          },
          meta: { request_id: 'req_integrations_local_missing' },
        } as never
      }
      if (path === '/v1/runs?limit=6') {
        return { ok: true, data: [], meta: { request_id: 'req_runs' } } as never
      }
      return { ok: true, data: [], meta: { request_id: 'req_default' } } as never
    })
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
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
          guidance: null,
        },
        todoist: {
          configured: true,
          connected: true,
          has_api_token: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        activity: {
          configured: true,
          source_path: '/tmp/activity.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        git: {
          configured: true,
          source_path: '/tmp/git.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        messaging: {
          configured: true,
          source_path: '/tmp/messaging.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        notes: {
          configured: true,
          source_path: '/tmp/notes',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        transcripts: {
          configured: true,
          source_path: '/tmp/transcripts.json',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
      },
      meta: { request_id: 'req_local_source_guidance_save' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const activityCard = within(root).getByRole('heading', { name: /computer activity/i }).closest('.rounded-lg')
    expect(activityCard).not.toBeNull()

    fireEvent.click(within(activityCard as HTMLElement).getByRole('button', { name: /edit source path/i }))
    const input = within(activityCard as HTMLElement).getByPlaceholderText(/path to local snapshot file or directory/i)
    expect(input).toHaveFocus()

    fireEvent.change(input, { target: { value: '/tmp/activity.json' } })
    fireEvent.click(within(activityCard as HTMLElement).getByRole('button', { name: /^save path$/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/integrations/activity/source',
        { source_path: '/tmp/activity.json' },
        expect.any(Function),
      )
    })
    expect(within(activityCard as HTMLElement).getByText('Source path saved.')).toBeInTheDocument()
  })

  it('renders component cards in the components tab', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openComponentsTab(container)

    expect(within(root).getByRole('heading', { name: /google calendar/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /todoist/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /evaluate/i })).toBeInTheDocument()
    expect(within(root).getByText('Calendar ingest')).toBeInTheDocument()
    expect(within(root).getAllByText(/Restarts: 0/)).toHaveLength(2)
  })

  it('expands component logs and shows restart event history', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openComponentsTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    fireEvent.click(within(evaluateCard as HTMLElement).getByRole('button', { name: /show logs/i }))

    await waitFor(() => {
      expect(within(evaluateCard as HTMLElement).getByText('Recent logs')).toBeInTheDocument()
    })
    expect(within(evaluateCard as HTMLElement).getByText('component.restart.requested')).toBeInTheDocument()
    expect(within(evaluateCard as HTMLElement).getByText('component.restart.completed')).toBeInTheDocument()
  })

  it('restarts a component and shows success feedback', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'evaluate',
        name: 'Evaluate',
        description: 'Evaluate all pipelines',
        status: 'ok',
        last_restarted_at: 1_700_000_300,
        last_error: null,
        restart_count: 3,
      },
      meta: { request_id: 'req_restart_eval' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openComponentsTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    fireEvent.click(within(evaluateCard as HTMLElement).getByRole('button', { name: /restart now/i }))

    await waitFor(() => {
      expect(within(evaluateCard as HTMLElement).getByText(/evaluate restarted \(ok\)\.?/i)).toBeInTheDocument()
    })
    expect(client.apiPost).toHaveBeenCalledWith('/api/components/evaluate/restart', {}, expect.any(Function))
  })

  it('shows an error message when component restart fails', async () => {
    vi.mocked(client.apiPost).mockRejectedValueOnce(new Error('component panic'))

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openComponentsTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    fireEvent.click(within(evaluateCard as HTMLElement).getByRole('button', { name: /restart now/i }))

    await waitFor(() => {
      expect(within(evaluateCard as HTMLElement).getByText('component panic')).toBeInTheDocument()
    })
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

    expect(client.apiPatch).not.toHaveBeenCalledWith('/v1/runs/run_122', expect.anything(), expect.any(Function))
    expect(within(root).getByText(/automatic retry is unsupported for search/i)).toBeInTheDocument()
    fireEvent.click(within(root).getByRole('button', { name: /confirm override retry/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/v1/runs/run_122',
        {
          status: 'retry_scheduled',
          reason: 'manual_backoff',
          retry_after_seconds: 90,
          allow_unsupported_retry: true,
        },
        expect.any(Function),
      )
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
    expect(client.apiPatch).not.toHaveBeenCalledWith('/v1/runs/run_122', expect.anything(), expect.any(Function))
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
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/v1/runs/run_123',
        {
          status: 'blocked',
          blocked_reason: 'operator_ui_blocked',
        },
        expect.any(Function),
      )
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
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/v1/runs/run_122',
        {
          status: 'blocked',
          blocked_reason: 'waiting_on_dependency',
        },
        expect.any(Function),
      )
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

  it('updates components from websocket payloads without refetching', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openComponentsTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    expect(within(evaluateCard as HTMLElement).getByText(/Restarts: 0/)).toBeInTheDocument()

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }

    const componentUpdateListener: (event: WsEnvelope) => void = wsListener
    componentUpdateListener({
      type: 'components:updated',
      timestamp: '2026-03-16T22:40:00Z',
      payload: {
        id: 'evaluate',
        name: 'Evaluate',
        description: 'Evaluate all pipelines',
        status: 'error',
        last_restarted_at: 1_700_000_400,
        last_error: 'restart failed',
        restart_count: 3,
      },
    })
    await Promise.resolve()

    expect(within(evaluateCard as HTMLElement).getByText('Restarts: 3')).toBeInTheDocument()
    expect(within(evaluateCard as HTMLElement).getByText('Last error: restart failed')).toBeInTheDocument()
    expect(within(evaluateCard as HTMLElement).getByText('error')).toBeInTheDocument()
  })

  it('updates runtime loops from the loops tab', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        kind: 'evaluate_current_state',
        enabled: false,
        interval_seconds: 600,
        last_started_at: 1_710_000_000,
        last_finished_at: 1_710_000_030,
        last_status: 'success',
        last_error: null,
        next_due_at: 1_710_000_600,
      },
      meta: { request_id: 'req_loop_patch' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openLoopsTab(container)

    fireEvent.change(within(root).getByDisplayValue('300'), { target: { value: '600' } })
    fireEvent.click(within(root).getByLabelText(/enabled/i))

    await waitFor(() => {
      expect(vi.mocked(client.apiPatch)).toHaveBeenCalledWith(
        '/v1/loops/evaluate_current_state',
        { enabled: false, interval_seconds: 600 },
        expect.any(Function),
      )
    })

    expect(within(root).getByText('Loop updated.')).toBeInTheDocument()
  })

  it('subscribes to websocket updates for runs', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await openRunsTab(container)

    expect(subscribeWs.mock.calls.length).toBeGreaterThanOrEqual(1)
  })

  it('opens directly to a targeted integration card', async () => {
    const scrollIntoView = vi.fn()
    Object.defineProperty(HTMLElement.prototype, 'scrollIntoView', {
      configurable: true,
      value: scrollIntoView,
    })

    const { container } = render(
      <SettingsPage
        onBack={() => {}}
        initialTab="integrations"
        initialIntegrationId="activity"
      />,
    )

    const root = getSettingsRoot(container)
    await waitFor(() => {
      expect(within(root).getByRole('heading', { name: /computer activity/i })).toBeInTheDocument()
    })

    expect(scrollIntoView).toHaveBeenCalled()
  })

})
