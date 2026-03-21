import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { SettingsPage } from './SettingsPage'
import * as client from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { resetWsQuerySyncForTests } from '../../data/ws-sync'

vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
  apiPatch: vi.fn(),
  apiPost: vi.fn(),
}))

vi.mock('../../realtime/ws', () => ({
  subscribeWs: () => () => {},
}))

describe('SettingsPage', () => {
  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.mocked(client.apiGet).mockReset()
    vi.mocked(client.apiPatch).mockReset()
    vi.mocked(client.apiPost).mockReset()
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/settings') {
        return {
          ok: true,
          data: {
            timezone: 'America/Denver',
            node_display_name: 'Vel Desktop',
            writeback_enabled: true,
            tailscale_preferred: true,
            tailscale_base_url: 'http://vel.tail',
            lan_base_url: 'http://192.168.1.2:4130',
            llm: {
              models_dir: 'configs/models',
              default_chat_profile_id: 'oauth-openai',
              fallback_chat_profile_id: 'local-qwen3-coder',
              profiles: [],
            },
            backup: {
              default_output_root: 'var/backups',
              trust: {
                level: 'warn',
                status: { state: 'stale', warnings: ['late'] },
                freshness: { state: 'stale', age_seconds: 1, stale_after_seconds: 1 },
                guidance: ['Create backup'],
              },
            },
          },
          meta: { request_id: 'req_settings' },
        } as never
      }
      if (path === '/v1/planning-profile') {
        return {
          ok: true,
          data: {
            profile: { routine_blocks: [], planning_constraints: [] },
          },
          meta: { request_id: 'req_profile' },
        } as never
      }
      if (path === '/v1/cluster/workers') {
        return {
          ok: true,
          data: { active_authority_node_id: 'vel-desktop', active_authority_epoch: 1, generated_at: 1, workers: [] },
          meta: { request_id: 'req_workers' },
        } as never
      }
      if (path === '/v1/linking/status') {
        return {
          ok: true,
          data: [],
          meta: { request_id: 'req_linking' },
        } as never
      }
      if (path === '/v1/cluster/bootstrap') {
        return {
          ok: true,
          data: {
            node_id: 'vel-desktop',
            node_display_name: 'Vel Desktop',
            active_authority_node_id: 'vel-desktop',
            active_authority_epoch: 1,
            sync_base_url: 'http://127.0.0.1:4130',
            sync_transport: 'tailscale',
            tailscale_base_url: 'http://vel.tail',
            lan_base_url: 'http://192.168.1.2:4130',
            localhost_base_url: 'http://127.0.0.1:4130',
            capabilities: [],
            linked_nodes: [],
            projects: [],
            action_items: [],
          },
          meta: { request_id: 'req_bootstrap' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: {
            google_calendar: { status: 'ready', connected: true, calendars: [], guidance: [] },
            todoist: { status: 'ready', connected: false, guidance: [] },
            activity: { status: 'idle', source_path: '/tmp/activity', selected_paths: [], suggested_paths: [] },
            health: { status: 'idle', source_path: '/tmp/health', selected_paths: [], suggested_paths: [] },
            git: { status: 'idle', source_path: '/tmp/git', selected_paths: [], suggested_paths: [] },
            messaging: { status: 'idle', source_path: '/tmp/messages', selected_paths: [], suggested_paths: [] },
            reminders: { status: 'idle', source_path: '/tmp/reminders', selected_paths: [], suggested_paths: [] },
            notes: { status: 'idle', source_path: '/tmp/notes', selected_paths: [], suggested_paths: [] },
            transcripts: { status: 'idle', source_path: '/tmp/transcripts', selected_paths: [], suggested_paths: [] },
          },
          meta: { request_id: 'req_integrations' },
        } as never
      }
      if (path === '/v1/runs?limit=6') {
        return { ok: true, data: [], meta: { request_id: 'req_runs' } } as never
      }
      if (path === '/v1/execution/handoffs?state=pending_review') {
        return { ok: true, data: [], meta: { request_id: 'req_handoffs' } } as never
      }
      if (path === '/v1/agent/inspect') {
        return {
          ok: true,
          data: {
            grounding: {
              generated_at: 1,
              now: {
                computed_at: 1710000000,
                timezone: 'America/Denver',
                summary: { mode: { key: 'focus', label: 'Focus' }, phase: { key: 'engaged', label: 'Engaged' }, meds: { key: 'ok', label: 'OK' }, risk: { level: 'low', score: 0.2, label: 'low' } },
                schedule: { empty_message: null, next_event: null, upcoming_events: [] },
                tasks: { todoist: [], other_open: [], next_commitment: null },
                attention: { state: { key: 'on_task', label: 'On task' }, drift: { key: 'none', label: 'None' }, severity: { key: 'none', label: 'None' }, confidence: null, reasons: [] },
                sources: { git_activity: null, health: null, mood: null, pain: null, note_document: null, assistant_message: null },
                freshness: { overall_status: 'fresh', sources: [] },
                day_plan: null,
                action_items: [],
                review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 },
                pending_writebacks: [],
                conflicts: [],
                people: [],
                reasons: [],
                debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
              },
              current_context: { computed_at: 1, mode: 'focus', morning_state: null, current_context_path: '/tmp/a', explain_context_path: '/tmp/b', explain_drift_path: '/tmp/c' },
              projects: [],
              people: [],
              commitments: [],
              review: { review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 }, pending_writebacks: [], conflicts: [], pending_execution_handoffs: [] },
            },
            capabilities: { groups: [] },
            blockers: [],
            explainability: { persisted_record_kinds: [], supporting_paths: [], raw_context_json_supporting_only: true },
          },
          meta: { request_id: 'req_agent' },
        } as never
      }
      if (path === '/v1/now') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            summary: { mode: { key: 'focus', label: 'Focus' }, phase: { key: 'engaged', label: 'Engaged' }, meds: { key: 'ok', label: 'OK' }, risk: { level: 'low', score: 0.2, label: 'low' } },
            schedule: { empty_message: null, next_event: null, upcoming_events: [] },
            tasks: { todoist: [], other_open: [], next_commitment: null },
            attention: { state: { key: 'on_task', label: 'On task' }, drift: { key: 'none', label: 'None' }, severity: { key: 'none', label: 'None' }, confidence: null, reasons: [] },
            sources: { git_activity: null, health: null, mood: null, pain: null, note_document: null, assistant_message: null },
            freshness: { overall_status: 'fresh', sources: [] },
            day_plan: null,
            action_items: [],
            review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 },
            pending_writebacks: [],
            conflicts: [],
            people: [],
            reasons: [],
            debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
          },
          meta: { request_id: 'req_now' },
        } as never
      }
      if (path === '/api/components' || path === '/api/loops') {
        return { ok: true, data: [], meta: { request_id: `req_${path}` } } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })
  })

  it('renders a compact settings shell with a left tab rail', async () => {
    render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Profile/i })).toBeInTheDocument()
    })
    expect(screen.queryByText(/Daily use/i)).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Clients & Sync/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Permissions & Policies/i })).toBeInTheDocument()
    expect(screen.queryByText(/Documentation/i)).not.toBeInTheDocument()
  })

  it('shows integration controls in the integrations section', async () => {
    render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /^Integrations$/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /^Integrations$/i })[0])

    await waitFor(() => {
      expect(screen.getAllByText('Google Calendar').length).toBeGreaterThan(0)
    })
    expect(screen.getAllByText('Google Calendar').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Todoist').length).toBeGreaterThan(0)
  })
})
