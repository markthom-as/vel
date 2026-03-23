import { fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { clearQueryCache } from '../../data/query'
import { SystemView } from './SystemView'

const loadAgentInspect = vi.fn()
const disconnectGoogleCalendar = vi.fn()
const disconnectTodoist = vi.fn()
const loadIntegrationConnections = vi.fn()
const loadIntegrations = vi.fn()
const syncSource = vi.fn()

vi.mock('../../data/agent-grounding', () => ({
  loadAgentInspect: (...args: unknown[]) => loadAgentInspect(...args),
}))

vi.mock('../../data/operator', () => ({
  disconnectGoogleCalendar: (...args: unknown[]) => disconnectGoogleCalendar(...args),
  disconnectTodoist: (...args: unknown[]) => disconnectTodoist(...args),
  loadIntegrationConnections: (...args: unknown[]) => loadIntegrationConnections(...args),
  loadIntegrations: (...args: unknown[]) => loadIntegrations(...args),
  operatorQueryKeys: {
    agentInspect: () => ['agent', 'inspect'],
    integrations: () => ['integrations'],
    integrationConnections: () => ['integrations', 'connections', 'all', 'all'],
  },
  syncSource: (...args: unknown[]) => syncSource(...args),
}))

describe('SystemView', () => {
  beforeEach(() => {
    clearQueryCache()
    loadAgentInspect.mockReset()
    disconnectGoogleCalendar.mockReset()
    disconnectTodoist.mockReset()
    loadIntegrationConnections.mockReset()
    loadIntegrations.mockReset()
    syncSource.mockReset()

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
    syncSource.mockResolvedValue({ ok: true, data: { source: 'calendar', signals_ingested: 3 }, meta: { request_id: 'req_sync' } })
    disconnectGoogleCalendar.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_disconnect' } })
    disconnectTodoist.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_disconnect_todoist' } })
  })

  it('renders the fixed canonical system sections under one surface', async () => {
    render(<SystemView target={{ section: 'domain' }} />)

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: /Canonical object and capability truth/i })).toBeInTheDocument()
    })

    expect(screen.getByRole('button', { name: 'Domain' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Capabilities' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Configuration' })).toBeInTheDocument()
    expect(screen.getByText('Avery')).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'Capabilities' }))
    expect(await screen.findByText('Read objects')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'Configuration' }))
    expect((await screen.findAllByText('Google Calendar')).length).toBeGreaterThan(0)
    expect(screen.getByText('Accounts')).toBeInTheDocument()
    expect(screen.getByText('Scopes')).toBeInTheDocument()
  })

  it('uses only named canonical integration actions', async () => {
    render(<SystemView target={{ section: 'configuration' }} />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: 'Refresh' }).length).toBeGreaterThan(0)
    })

    const integrationsSection = screen.getAllByRole('heading', { name: 'Integrations' })[0].closest('section') as HTMLElement
    const refreshButtons = within(integrationsSection).getAllByRole('button', { name: 'Refresh' })
    fireEvent.click(refreshButtons[0])
    await waitFor(() => expect(syncSource).toHaveBeenCalledWith('calendar'))

    const disconnectButtons = within(integrationsSection).getAllByRole('button', { name: 'Disconnect' })
    fireEvent.click(disconnectButtons[0])
    await waitFor(() => expect(disconnectGoogleCalendar).toHaveBeenCalled())

    expect(screen.queryByRole('button', { name: /Reconnect/i })).not.toBeInTheDocument()
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
