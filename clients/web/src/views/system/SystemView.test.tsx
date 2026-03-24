import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { clearQueryCache } from '../../data/query'
import { SystemView } from './SystemView'

const loadAgentInspect = vi.fn()
const disconnectGoogleCalendar = vi.fn()
const disconnectTodoist = vi.fn()
const loadIntegrationConnections = vi.fn()
const loadIntegrations = vi.fn()
const loadSettings = vi.fn()
const syncSource = vi.fn()
const updateSettings = vi.fn()
const updateWebSettings = vi.fn()

vi.mock('../../data/agent-grounding', () => ({
  loadAgentInspect: (...args: unknown[]) => loadAgentInspect(...args),
}))

vi.mock('../../data/operator', () => ({
  disconnectGoogleCalendar: (...args: unknown[]) => disconnectGoogleCalendar(...args),
  disconnectTodoist: (...args: unknown[]) => disconnectTodoist(...args),
  loadIntegrationConnections: (...args: unknown[]) => loadIntegrationConnections(...args),
  loadIntegrations: (...args: unknown[]) => loadIntegrations(...args),
  loadSettings: (...args: unknown[]) => loadSettings(...args),
  operatorQueryKeys: {
    agentInspect: () => ['agent', 'inspect'],
    integrations: () => ['integrations'],
    integrationConnections: () => ['integrations', 'connections', 'all', 'all'],
    settings: () => ['settings'],
  },
  syncSource: (...args: unknown[]) => syncSource(...args),
  updateSettings: (...args: unknown[]) => updateSettings(...args),
  updateWebSettings: (...args: unknown[]) => updateWebSettings(...args),
}))

describe('SystemView', () => {
  beforeEach(() => {
    clearQueryCache()
    loadAgentInspect.mockReset()
    disconnectGoogleCalendar.mockReset()
    disconnectTodoist.mockReset()
    loadIntegrationConnections.mockReset()
    loadIntegrations.mockReset()
    loadSettings.mockReset()
    syncSource.mockReset()
    updateSettings.mockReset()
    updateWebSettings.mockReset()

    loadAgentInspect.mockResolvedValue({
      ok: true,
      data: {
        grounding: {
          generated_at: 1710000000,
          now: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            schedule: {
              upcoming_events: [
                { id: 'event_1', title: 'Design review', kind: 'event', start_ts: 1710003600, end_ts: 1710007200 },
              ],
            },
          },
          current_context: {
            computed_at: 1710000000,
            mode: 'work',
            morning_state: 'ready',
            current_context_path: 'var/context/current.json',
            explain_context_path: 'var/context/explain.json',
            explain_drift_path: 'var/context/drift.json',
          },
          projects: [{ id: 'project_1', slug: 'vel', name: 'Vel', family: 'work', status: 'active', primary_repo: { path: '/repo', label: 'repo', kind: 'repo' }, primary_notes_root: { path: '/notes', label: 'notes', kind: 'notes_root' }, secondary_repos: [], secondary_notes_roots: [], upstream_ids: {}, pending_provision: { create_repo: false, create_notes_root: false }, created_at: '2026-03-22T00:00:00Z', updated_at: '2026-03-22T00:00:00Z', archived_at: null }],
          people: [{ id: 'person_1', display_name: 'Avery', given_name: null, family_name: null, relationship_context: 'teammate', birthday: null, last_contacted_at: null, aliases: [], links: [] }],
          commitments: [],
          review: { review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 }, pending_writebacks: [], conflicts: [], pending_execution_handoffs: [] },
        },
        capabilities: {
          groups: [
            {
              kind: 'read_context',
              label: 'Read context',
              entries: [
                {
                  key: 'object.read',
                  label: 'Read objects',
                  summary: 'Read canonical objects safely.',
                  available: true,
                  blocked_reason: null,
                  requires_review_gate: null,
                  requires_writeback_enabled: false,
                },
              ],
            },
          ],
        },
        blockers: [],
        explainability: {
          persisted_record_kinds: ['task', 'event'],
          supporting_paths: ['docs/MASTER_PLAN.md'],
          raw_context_json_supporting_only: true,
        },
      },
      meta: { request_id: 'req_inspect' },
    })
    loadIntegrations.mockResolvedValue({
      ok: true,
      data: {
        google_calendar: {
          configured: true,
          connected: true,
          has_client_id: true,
          has_client_secret: true,
          calendars: [{ id: 'cal_1', summary: 'Primary', primary: true, selected: true }],
          all_calendars_selected: false,
          last_sync_at: 1710000000,
          last_sync_status: 'ok',
          last_error: null,
          last_item_count: 12,
          guidance: { title: 'Healthy', detail: 'Calendar connection is healthy.', action: 'refresh' },
        },
        todoist: {
          configured: true,
          connected: true,
          has_api_token: true,
          last_sync_at: 1710000000,
          last_sync_status: 'ok',
          last_error: null,
          last_item_count: 8,
          guidance: { title: 'Healthy', detail: 'Todoist connection is healthy.', action: 'refresh' },
        },
        activity: buildLocalIntegration(),
        health: buildLocalIntegration(),
        git: buildLocalIntegration(),
        messaging: buildLocalIntegration(),
        reminders: buildLocalIntegration(),
        notes: buildLocalIntegration(),
        transcripts: buildLocalIntegration(),
      },
      meta: { request_id: 'req_integrations' },
    })
    loadIntegrationConnections.mockResolvedValue({
      ok: true,
      data: [
        {
          id: 'conn_google',
          family: 'calendar',
          provider_key: 'google_calendar',
          status: 'connected',
          display_name: 'Google Calendar',
          account_ref: 'acct_google',
          metadata: {},
          created_at: 1710000000,
          updated_at: 1710000000,
          setting_refs: [],
        },
      ],
      meta: { request_id: 'req_connections' },
    })
    loadSettings.mockResolvedValue({
      ok: true,
      data: {
        writeback_enabled: false,
        backup: null,
        web_settings: {
          dense_rows: true,
          tabular_numbers: true,
          reduced_motion: false,
          strong_focus: true,
          docked_action_bar: true,
        },
      },
      meta: { request_id: 'req_settings' },
    })
    syncSource.mockResolvedValue({ ok: true, data: { source: 'calendar', signals_ingested: 3 }, meta: { request_id: 'req_sync' } })
    disconnectGoogleCalendar.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_disconnect' } })
    disconnectTodoist.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_disconnect_todoist' } })
    updateSettings.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_settings_patch' } })
    updateWebSettings.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_web_settings' } })
  })

  it('renders the compact system rail and scroll document without the rejected helper panels', async () => {
    render(<SystemView target={{ section: 'overview', subsection: 'trust' }} />)

    await waitFor(() => {
      expect(screen.getByText('Status and activity')).toBeInTheDocument()
    })

    expect(screen.getByRole('button', { name: /Overview/i })).toHaveAttribute('aria-pressed', 'true')
    expect(screen.getByRole('button', { name: /Operations/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Integrations/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Control/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Preferences/i })).toBeInTheDocument()
    expect(screen.getByText('Status and activity')).toBeInTheDocument()
    expect(screen.queryByText('One surface, five sections. Trust stays legible. Operations stay bounded.')).not.toBeInTheDocument()
    expect(screen.queryByRole('heading', { name: /Structural truth, trust, and repair/i })).not.toBeInTheDocument()

    expect(await screen.findByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('Avery')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: /Control/i }))
    expect((await screen.findAllByText('Vel')).length).toBeGreaterThan(0)
    expect(screen.getByDisplayValue('/repo')).toBeInTheDocument()

    fireEvent.click(screen.getAllByRole('button', { name: /Capabilities/i })[0])
    expect(await screen.findByText('Read objects')).toBeInTheDocument()
  })

  it('keeps integration actions available without the old browse-detail shell', async () => {
    render(<SystemView target={{ section: 'integrations', subsection: 'providers' }} />)

    await waitFor(() => {
      expect(screen.getByText('Providers')).toBeInTheDocument()
    })

    expect(screen.getAllByText('Google Calendar').length).toBeGreaterThan(0)
    expect(screen.queryByText('Browse / detail')).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Google Calendar' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Todoist' })).toBeInTheDocument()

    const refreshButton = screen.getAllByRole('button', { name: 'Refresh' })[0]
    fireEvent.click(refreshButton)
    await waitFor(() => expect(syncSource).toHaveBeenCalledWith('calendar'))

    const disconnectButton = screen.getAllByRole('button', { name: 'Disconnect' })[0]
    fireEvent.click(disconnectButton)
    await waitFor(() => expect(disconnectGoogleCalendar).toHaveBeenCalled())

    fireEvent.click(screen.getAllByRole('button', { name: /Accounts/i })[0])
    expect((await screen.findAllByDisplayValue('acct_google')).length).toBeGreaterThan(0)
    expect(screen.getAllByText('Google Calendar').length).toBeGreaterThan(0)

    expect(screen.queryByRole('button', { name: /Reconnect/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /Deactivate/i })).not.toBeInTheDocument()
  })

  it('exposes truthful persisted operator settings fields', async () => {
    render(<SystemView target={{ section: 'preferences', subsection: 'accessibility' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Operator settings').length).toBeGreaterThan(0)
    })

    const timezoneField = screen.getAllByLabelText('Timezone')[0]
    fireEvent.change(timezoneField, { target: { value: 'America/Chicago' } })
    fireEvent.blur(timezoneField)

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ timezone: 'America/Chicago' })
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Writeback enabled' })[0])
    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ writeback_enabled: true })
    })
  })
})

function buildLocalIntegration() {
  return {
    configured: false,
    source_path: null,
    selected_paths: [],
    available_paths: [],
    internal_paths: [],
    suggested_paths: [],
    source_kind: 'directory',
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  }
}
