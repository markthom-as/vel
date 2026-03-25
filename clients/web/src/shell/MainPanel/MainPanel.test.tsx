import { render, screen, waitFor } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { MainPanel } from './MainPanel'
import { clearQueryCache, setQueryData } from '../../data/query'
import {
  operatorSurfaces,
  primarySurfaces,
  supportSurfaces,
} from '../../data/operatorSurfaces'
import type { AssistantEntryResponse } from '../../types'

const loadSettings = vi.fn()
const loadIntegrations = vi.fn()
let lastComposerProps: {
  onSent?: (clientMessageId: string | undefined, response: AssistantEntryResponse, submitted: { text: string, conversationId: string | null, intent: null, voice: null, attachments: null }) => void
  disabled?: boolean
  disabledReason?: string | null
  onDisabledInteract?: () => void
} | null = null

vi.mock('../../data/operator', async () => {
  const actual = await vi.importActual<typeof import('../../data/operator')>('../../data/operator')
  return {
    ...actual,
    loadSettings: (...args: unknown[]) => loadSettings(...args),
    loadIntegrations: (...args: unknown[]) => loadIntegrations(...args),
  }
})

vi.mock('../../core/MessageComposer', () => ({
  MessageComposer: (props: {
    onSent?: (clientMessageId: string | undefined, response: AssistantEntryResponse, submitted: { text: string, conversationId: string | null, intent: null, voice: null, attachments: null }) => void
    disabled?: boolean
    disabledReason?: string | null
    onDisabledInteract?: () => void
  }) => {
    lastComposerProps = props
    return (
      <div>
        <div>Composer {props.disabled ? 'disabled' : 'enabled'}</div>
        {props.disabledReason ? <div>{props.disabledReason}</div> : null}
        {props.onDisabledInteract ? <button type="button" onClick={props.onDisabledInteract}>Disabled interact</button> : null}
      </div>
    )
  },
}))

vi.mock('../../views/now', () => ({
  NowView: () => <div>Now view</div>,
}))

vi.mock('../../views/threads', () => ({
  ThreadView: ({ conversationId }: { conversationId: string | null }) => (
    <div>{conversationId ? `Thread ${conversationId}` : 'Thread empty'}</div>
  ),
}))

vi.mock('../../views/threads/useResolvedThreadConversationId', () => ({
  useResolvedThreadConversationId: (conversationId: string | null) => conversationId,
}))

vi.mock('../../views/system', () => ({
  SystemView: () => <div>System view</div>,
}))

describe('MainPanel', () => {
  beforeEach(() => {
    clearQueryCache()
    lastComposerProps = null
    loadSettings.mockReset()
    loadIntegrations.mockReset()
    loadSettings.mockResolvedValue({
      ok: true,
      data: {
        node_display_name: 'Vel',
        llm: {
          models_dir: '/models',
          default_chat_profile_id: 'local-llama',
          fallback_chat_profile_id: null,
          profiles: [
            {
              id: 'local-llama',
              provider: 'llama_cpp',
              base_url: 'http://127.0.0.1:8012/v1',
              model: 'qwen3',
              context_window: 32768,
              enabled: true,
              editable: false,
            },
          ],
        },
        core_settings: {
          user_display_name: 'Jove',
          developer_mode: false,
          bypass_setup_gate: false,
          agent_profile: {
            role: 'Operator',
            preferences: null,
            constraints: null,
            freeform: 'Keep things concise.',
          },
        },
      },
      meta: { request_id: 'req_settings' },
    })
    loadIntegrations.mockResolvedValue({
      ok: true,
      data: {
        google_calendar: {
          configured: false,
          connected: false,
          has_client_id: false,
          has_client_secret: false,
          calendars: [],
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
        activity: buildLocalIntegration(),
        health: buildLocalIntegration(),
        git: buildLocalIntegration(),
        messaging: buildLocalIntegration(),
        reminders: buildLocalIntegration(),
        notes: { ...buildLocalIntegration(), source_kind: 'directory' },
        transcripts: buildLocalIntegration(),
      },
      meta: { request_id: 'req_integrations' },
    })
  })

  afterEach(() => {
    clearQueryCache()
  })

  function renderMainPanel(mainView: 'now' | 'threads' | 'system') {
    return render(
      <MainPanel
        conversationId={mainView === 'threads' ? 'conv_1' : null}
        mainView={mainView}
        onNavigate={() => {}}
        onOpenThread={() => {}}
        onOpenSystem={() => {}}
        shellOwnsNowNudges
        systemTarget={{ section: 'integrations' }}
      />,
    )
  }

  it('shows the Now view when mainView is now', () => {
    renderMainPanel('now')
    expect(screen.getByText('Now view')).toBeInTheDocument()
    expect(screen.queryByText('Inbox view')).toBeNull()
  })

  it('shows the thread view when mainView is threads', () => {
    renderMainPanel('threads')
    expect(screen.getByText('Thread conv_1')).toBeInTheDocument()
  })

  it('shows the System surface when mainView is system', () => {
    renderMainPanel('system')
    expect(screen.getByText('System view')).toBeInTheDocument()
  })

  it('raises a stable core-setup nudge and disables the composer until minimum setup is complete', async () => {
    loadSettings.mockResolvedValueOnce({
      ok: true,
      data: {
        node_display_name: null,
        llm: {
          models_dir: '/models',
          default_chat_profile_id: null,
          fallback_chat_profile_id: null,
          profiles: [],
        },
        core_settings: {
          user_display_name: null,
          developer_mode: false,
          bypass_setup_gate: false,
          agent_profile: {
            role: null,
            preferences: null,
            constraints: null,
            freeform: null,
          },
        },
      },
      meta: { request_id: 'req_settings_incomplete' },
    })
    loadIntegrations.mockResolvedValueOnce({
      ok: true,
      data: {
        google_calendar: {
          configured: false,
          connected: false,
          has_client_id: false,
          has_client_secret: false,
          calendars: [],
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        todoist: {
          configured: false,
          connected: false,
          has_api_token: false,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        activity: buildLocalIntegration(),
        health: buildLocalIntegration(),
        git: buildLocalIntegration(),
        messaging: buildLocalIntegration(),
        reminders: buildLocalIntegration(),
        notes: { ...buildLocalIntegration(), source_kind: 'directory' },
        transcripts: buildLocalIntegration(),
      },
      meta: { request_id: 'req_integrations_incomplete' },
    })
    const onRaiseNudge = vi.fn()

    render(
      <MainPanel
        conversationId={null}
        mainView="now"
        onNavigate={() => {}}
        onOpenThread={() => {}}
        onOpenSystem={() => {}}
        onRaiseNudge={onRaiseNudge}
        shellOwnsNowNudges
        systemTarget={{ section: 'integrations' }}
      />,
    )

    await waitFor(() => {
      expect(screen.getByText('Composer disabled')).toBeInTheDocument()
    })
    expect(screen.getByText(/core setup is incomplete\. use the nudge to open core settings and finish setup\./i)).toBeInTheDocument()

    await waitFor(() => {
      expect(onRaiseNudge).toHaveBeenCalledWith(
        expect.objectContaining({
          id: 'core_setup_required',
          title: 'Finish Core setup to enable the composer',
          summary: 'Finish the checklist below to enable Vel.',
          actions: expect.arrayContaining([
            expect.objectContaining({
              kind: 'open_settings:core_settings:user_display_name:missing',
              label: 'Your name',
            }),
            expect.objectContaining({
              kind: 'open_settings:core_settings:synced_provider:missing',
              label: 'Synced provider',
            }),
          ]),
        }),
      )
    })

    screen.getAllByRole('button', { name: 'Disabled interact' }).at(-1)?.click()

    await waitFor(() => {
      expect(onRaiseNudge).toHaveBeenLastCalledWith(
        expect.objectContaining({
          id: 'core_setup_required',
          title: 'Finish Core setup to enable the composer',
        }),
      )
    })
  })

  it('includes saved checklist values in the core setup nudge when some items are already configured', async () => {
    loadSettings.mockResolvedValueOnce({
      ok: true,
      data: {
        node_display_name: null,
        llm: {
          models_dir: '/models',
          default_chat_profile_id: 'local-llama',
          fallback_chat_profile_id: null,
          profiles: [
            {
              id: 'local-llama',
              provider: 'llama_cpp',
              base_url: 'http://127.0.0.1:8012/v1',
              model: 'qwen3',
              context_window: 32768,
              enabled: true,
              editable: false,
            },
          ],
        },
        core_settings: {
          user_display_name: 'Jove',
          developer_mode: false,
          bypass_setup_gate: false,
          agent_profile: {
            role: 'Operator',
            preferences: null,
            constraints: null,
            freeform: null,
          },
        },
      },
      meta: { request_id: 'req_settings_partial' },
    })
    loadIntegrations.mockResolvedValueOnce({
      ok: true,
      data: {
        google_calendar: {
          configured: false,
          connected: false,
          has_client_id: false,
          has_client_secret: false,
          calendars: [],
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
        activity: buildLocalIntegration(),
        health: buildLocalIntegration(),
        git: buildLocalIntegration(),
        messaging: buildLocalIntegration(),
        reminders: buildLocalIntegration(),
        notes: { ...buildLocalIntegration(), source_kind: 'directory' },
        transcripts: buildLocalIntegration(),
      },
      meta: { request_id: 'req_integrations_partial' },
    })
    const onRaiseNudge = vi.fn()

    render(
      <MainPanel
        conversationId={null}
        mainView="now"
        onNavigate={() => {}}
        onOpenThread={() => {}}
        onOpenSystem={() => {}}
        onRaiseNudge={onRaiseNudge}
        shellOwnsNowNudges
        systemTarget={{ section: 'integrations' }}
      />,
    )

    await waitFor(() => {
      expect(onRaiseNudge).toHaveBeenCalledWith(
        expect.objectContaining({
          id: 'core_setup_required',
          actions: expect.arrayContaining([
            expect.objectContaining({
              kind: 'open_settings:core_settings:user_display_name:ready:Jove',
              label: 'Your name',
            }),
            expect.objectContaining({
              kind: 'open_settings:core_settings:agent_profile:ready:Operator',
              label: 'Agent profile',
            }),
            expect.objectContaining({
              kind: 'open_settings:core_settings:llm_provider:ready:qwen3%20%C2%B7%20local-llama',
              label: 'LLM integration',
            }),
            expect.objectContaining({
              kind: 'open_settings:core_settings:synced_provider:ready:Todoist',
              label: 'Synced provider',
            }),
            expect.objectContaining({
              kind: 'open_settings:core_settings:node_display_name:missing',
              label: 'Node name',
            }),
          ]),
        }),
      )
    })
  })

  it('clears the core setup nudge once setup becomes ready', async () => {
    loadSettings.mockResolvedValueOnce({
      ok: true,
      data: {
        node_display_name: null,
        llm: {
          models_dir: '/models',
          default_chat_profile_id: null,
          fallback_chat_profile_id: null,
          profiles: [],
        },
        core_settings: {
          user_display_name: null,
          developer_mode: false,
          bypass_setup_gate: false,
          agent_profile: {
            role: null,
            preferences: null,
            constraints: null,
            freeform: null,
          },
        },
      },
      meta: { request_id: 'req_settings_incomplete' },
    })
    loadIntegrations.mockResolvedValueOnce({
      ok: true,
      data: {
        google_calendar: {
          configured: false,
          connected: false,
          has_client_id: false,
          has_client_secret: false,
          calendars: [],
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        todoist: {
          configured: false,
          connected: false,
          has_api_token: false,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        activity: buildLocalIntegration(),
        health: buildLocalIntegration(),
        git: buildLocalIntegration(),
        messaging: buildLocalIntegration(),
        reminders: buildLocalIntegration(),
        notes: { ...buildLocalIntegration(), source_kind: 'directory' },
        transcripts: buildLocalIntegration(),
      },
      meta: { request_id: 'req_integrations_incomplete' },
    })

    const onRaiseNudge = vi.fn()
    const onClearNudge = vi.fn()

    render(
      <MainPanel
        conversationId={null}
        mainView="now"
        onNavigate={() => {}}
        onOpenThread={() => {}}
        onOpenSystem={() => {}}
        onRaiseNudge={onRaiseNudge}
        onClearNudge={onClearNudge}
        shellOwnsNowNudges
        systemTarget={{ section: 'integrations' }}
      />,
    )

    await waitFor(() => {
      expect(onRaiseNudge).toHaveBeenCalledWith(
        expect.objectContaining({ id: 'core_setup_required' }),
      )
    })

    setQueryData(['settings'], {
      node_display_name: 'Vel Desktop',
      llm: {
        models_dir: '/models',
        default_chat_profile_id: 'local-llama',
        fallback_chat_profile_id: null,
        profiles: [
          {
            id: 'local-llama',
            provider: 'llama_cpp',
            base_url: 'http://127.0.0.1:8012/v1',
            model: 'qwen3',
            context_window: 32768,
            enabled: true,
            editable: false,
          },
        ],
      },
      core_settings: {
        user_display_name: 'Jove',
        developer_mode: false,
        bypass_setup_gate: false,
        agent_profile: {
          role: 'Operator',
          preferences: null,
          constraints: null,
          freeform: null,
        },
      },
    })
    setQueryData(['integrations'], {
      google_calendar: {
        configured: false,
        connected: false,
        has_client_id: false,
        has_client_secret: false,
        calendars: [],
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
      activity: buildLocalIntegration(),
      health: buildLocalIntegration(),
      git: buildLocalIntegration(),
      messaging: buildLocalIntegration(),
      reminders: buildLocalIntegration(),
      notes: { ...buildLocalIntegration(), source_kind: 'directory' },
      transcripts: buildLocalIntegration(),
    })

    await waitFor(() => {
      expect(onClearNudge).toHaveBeenCalledWith('core_setup_required')
    })
  })

  it('uses the approved shell parity taxonomy as the first-class route set', () => {
    expect(primarySurfaces.map((surface) => surface.view)).toEqual(['now', 'threads', 'system'])
    expect(supportSurfaces.map((surface) => surface.view)).toEqual([])
    expect(
      operatorSurfaces.filter((surface) => surface.navVisible).map((surface) => surface.view),
    ).toEqual(['now', 'threads', 'system'])
  })

  it('speaks assistant replies locally when the thread is in call mode', async () => {
    const speak = vi.fn()
    const cancel = vi.fn()
    Object.defineProperty(window, 'speechSynthesis', {
      configurable: true,
      value: { speak, cancel },
    })
    // Minimal test double for jsdom.
    ;(globalThis as typeof globalThis & { SpeechSynthesisUtterance: typeof SpeechSynthesisUtterance }).SpeechSynthesisUtterance = class {
      text: string
      constructor(text?: string) {
        this.text = text ?? ''
      }
    } as unknown as typeof SpeechSynthesisUtterance

    renderMainPanel('threads')

    await waitFor(() => {
      expect(lastComposerProps?.onSent).toBeDefined()
    })

    lastComposerProps?.onSent?.(
      undefined,
      {
        route_target: 'threads',
        user_message: {
          id: 'msg_user',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: 'Talk to me' },
          status: null,
          importance: null,
          created_at: 1,
          updated_at: null,
        },
        assistant_message: {
          id: 'msg_assistant',
          conversation_id: 'conv_1',
          role: 'assistant',
          kind: 'text',
          content: { text: 'Spoken thread reply' },
          status: null,
          importance: null,
          created_at: 2,
          updated_at: null,
        },
        conversation: {
          id: 'conv_1',
          title: 'Thread',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: true,
          created_at: 1,
          updated_at: 2,
          message_count: 2,
          last_message_at: 2,
          project_label: null,
          continuation: null,
        },
      },
      { text: 'Talk to me', conversationId: 'conv_1', voice: null, attachments: null },
    )

    expect(cancel).toHaveBeenCalled()
    expect(speak).toHaveBeenCalledWith(expect.objectContaining({ text: 'Spoken thread reply' }))
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
    source_kind: 'file',
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  }
}
