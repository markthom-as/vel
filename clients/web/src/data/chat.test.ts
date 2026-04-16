import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as client from '../api/client'
import {
  acknowledgeInboxItem,
  submitAssistantEntry,
  dismissInboxItem,
  getInboxThreadPath,
  getInterventionApiId,
  reactivateInboxItem,
  resolveInboxItem,
  snoozeInboxItem,
  updateConversationArchive,
  updateConversationCallMode,
  updateConversationPinned,
} from './chat'
import type { InboxItemData } from '../types'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
  apiPatch: vi.fn(),
}))

describe('chat data helpers', () => {
  beforeEach(() => {
    vi.mocked(client.apiPost).mockReset()
    vi.mocked(client.apiPatch).mockReset()
  })

  it('posts acknowledge inbox mutations to /api/interventions/:id/acknowledge', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_1', state: 'acknowledged' },
      meta: { request_id: 'req_ack' },
    } as never)

    const response = await acknowledgeInboxItem('intv_1')

    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_1/acknowledge',
      {},
      expect.any(Function),
    )
    expect(response.data?.state).toBe('acknowledged')
  })

  it('posts snooze inbox mutations to /api/interventions/:id/snooze', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_2', state: 'snoozed' },
      meta: { request_id: 'req_snooze' },
    } as never)

    const response = await snoozeInboxItem('intv_2', 15)

    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_2/snooze',
      { minutes: 15 },
      expect.any(Function),
    )
    expect(response.data?.state).toBe('snoozed')
  })

  it('posts dismiss inbox mutations to /api/interventions/:id/dismiss', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_3', state: 'dismissed' },
      meta: { request_id: 'req_dismiss' },
    } as never)

    const response = await dismissInboxItem('intv_3')

    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_3/dismiss',
      {},
      expect.any(Function),
    )
    expect(response.data?.state).toBe('dismissed')
  })

  it('posts resolve and reactivate inbox mutations', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_4', state: 'resolved' },
      meta: { request_id: 'req_res' },
    } as never)

    await resolveInboxItem('intv_4')
    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_4/resolve',
      {},
      expect.any(Function),
    )

    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_4', state: 'active' },
      meta: { request_id: 'req_react' },
    } as never)

    await reactivateInboxItem('intv_4')
    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_4/reactivate',
      {},
      expect.any(Function),
    )
  })

  it('includes explicit assistant-entry intent and attachments in the request body', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'inbox',
        user_message: {
          id: 'msg_1',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: 'hello' },
          status: null,
          importance: null,
          created_at: 1,
          updated_at: null,
        },
        conversation: {
          id: 'conv_1',
          title: 'Hello',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 1,
          updated_at: 1,
          continuation: null,
        },
      },
      meta: { request_id: 'req_entry' },
    } as never)

    await submitAssistantEntry(
      'hello',
      'conv_1',
      null,
      'question',
      [{ kind: 'file', label: 'notes.md', object_id: 'art_1' }],
    )

    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/assistant/entry',
      {
        text: 'hello',
        conversation_id: 'conv_1',
        intent: 'question',
        attachments: [{ kind: 'file', label: 'notes.md', object_id: 'art_1' }],
      },
      expect.any(Function),
    )
  })

  it('patches thread call mode through the conversation update seam', async () => {
    vi.mocked(client.apiPatch).mockResolvedValue({
      ok: true,
      data: {
        id: 'conv_1',
        title: 'Hello',
        kind: 'general',
        pinned: false,
        archived: false,
        call_mode_active: true,
        created_at: 1,
        updated_at: 2,
        continuation: null,
      },
      meta: { request_id: 'req_call_mode' },
    } as never)

    const response = await updateConversationCallMode('conv_1', true)

    expect(client.apiPatch).toHaveBeenCalledWith(
      '/api/conversations/conv_1',
      { call_mode_active: true },
      expect.any(Function),
    )
    expect(response.data?.call_mode_active).toBe(true)
  })

  it('patches pinned and archived thread state through the conversation update seam', async () => {
    vi.mocked(client.apiPatch).mockResolvedValue({
      ok: true,
      data: {
        id: 'conv_1',
        title: 'Hello',
        kind: 'general',
        pinned: true,
        archived: false,
        call_mode_active: false,
        created_at: 1,
        updated_at: 2,
        continuation: null,
      },
      meta: { request_id: 'req_thread_state' },
    } as never)

    await updateConversationPinned('conv_1', true)
    expect(client.apiPatch).toHaveBeenCalledWith(
      '/api/conversations/conv_1',
      { pinned: true },
      expect.any(Function),
    )

    await updateConversationArchive('conv_1', true)
    expect(client.apiPatch).toHaveBeenLastCalledWith(
      '/api/conversations/conv_1',
      { archived: true },
      expect.any(Function),
    )
  })

  it('derives intervention API id from synthetic action ids and evidence', () => {
    expect(
      getInterventionApiId({
        id: 'intv_9',
        message_id: 'm',
        kind: 'k',
        state: 'active',
        surfaced_at: 1,
        snoozed_until: null,
        confidence: null,
        conversation_id: null,
        title: '',
        summary: '',
        project_id: null,
        project_label: null,
        available_actions: [],
        evidence: [],
      }),
    ).toBe('intv_9')
    expect(
      getInterventionApiId({
        id: 'act_intervention_abc123',
        message_id: 'm',
        kind: 'k',
        state: 'active',
        surfaced_at: 1,
        snoozed_until: null,
        confidence: null,
        conversation_id: null,
        title: '',
        summary: '',
        project_id: null,
        project_label: null,
        available_actions: [],
        evidence: [],
      }),
    ).toBe('abc123')
  })

  it('reuses /api/conversations/:id only when open_thread is available', () => {
    const threadable: InboxItemData = {
      id: 'intv_4',
      message_id: 'msg_4',
      kind: 'risk',
      state: 'active',
      surfaced_at: 1710000000,
      snoozed_until: null,
      confidence: null,
      conversation_id: 'conv_4',
      title: 'Open the thread',
      summary: 'Continue in the existing thread',
      project_id: null,
      project_label: null,
      available_actions: ['acknowledge', 'open_thread'],
      evidence: [],
    }
    const nonThreadable: InboxItemData = {
      ...threadable,
      id: 'intv_5',
      conversation_id: null,
      available_actions: ['acknowledge', 'dismiss'],
    }

    expect(getInboxThreadPath(threadable)).toBe('/api/conversations/conv_4')
    expect(getInboxThreadPath(nonThreadable)).toBeNull()
  })
})
