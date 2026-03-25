import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { cleanup, render, fireEvent, waitFor, within } from '@testing-library/react'
import { MessageComposer } from './MessageComposer'
import * as api from '../../api/client'
import * as speech from '../../hooks/useSpeechRecognition'

vi.mock('../../api/client', () => ({
  apiPost: vi.fn(),
}))

vi.mock('../../hooks/useSpeechRecognition', () => ({
  useSpeechRecognition: vi.fn(),
}))

describe('MessageComposer', () => {
  const onSent = vi.fn()
  const onOptimisticSend = vi.fn()
  const onSendFailed = vi.fn()

  function requireHtmlElement<T extends HTMLElement>(element: T | null): T {
    expect(element).not.toBeNull()
    return element as T
  }

  beforeEach(() => {
    onSent.mockClear()
    onOptimisticSend.mockReset()
    onSendFailed.mockReset()
    vi.mocked(api.apiPost).mockReset()
    vi.mocked(speech.useSpeechRecognition).mockReturnValue({
      isSupported: true,
      isListening: false,
      error: null,
      start: vi.fn(),
      stop: vi.fn(),
      interimTranscript: '',
    })
  })

  afterEach(() => {
    cleanup()
  })

  it('renders textarea and Send button', () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    expect(composer.querySelector('textarea')).toBeInTheDocument()
    expect(
      within(composer).getByPlaceholderText(/ask, capture, or talk to vel/i),
    ).toBeInTheDocument()
    expect(within(composer).getByRole('button', { name: /send/i })).toBeInTheDocument()
  })

  it('floating mode shows elapsed timer and max cap while recording', () => {
    vi.mocked(speech.useSpeechRecognition).mockReturnValue({
      isSupported: true,
      isListening: true,
      error: null,
      start: vi.fn(),
      stop: vi.fn(),
      interimTranscript: '',
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    expect(container.textContent).toMatch(/0:00/)
    expect(container.querySelector('svg[aria-hidden="true"]')).toBeTruthy()
  })

  it('hides Send in floating mode until there is text to send', () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    expect(within(container).queryByRole('button', { name: /send/i })).not.toBeInTheDocument()
    const textarea = requireHtmlElement(container.querySelector('textarea'))
    fireEvent.change(textarea, { target: { value: 'Hi' } })
    expect(within(container).getByRole('button', { name: /send/i })).toBeInTheDocument()
  })

  it('shows attachment menu and queued chip in floating mode', async () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)

    fireEvent.click(within(container).getByRole('button', { name: /add attachment/i }))

    expect(within(container).getByRole('menu', { name: /attachment types/i })).toBeInTheDocument()
    expect(within(container).getByRole('menuitem', { name: /task/i })).toBeInTheDocument()

    const file = new File(['hello'], 'brief.txt', { type: 'text/plain' })
    const fileInputs = container.querySelectorAll('input[type="file"]')
    const generalFileInput = fileInputs[0] as HTMLInputElement
    fireEvent.change(generalFileInput, { target: { files: [file] } })

    await waitFor(() => {
      expect(within(container).getByText('brief.txt')).toBeInTheDocument()
    })

    fireEvent.click(within(container).getByRole('button', { name: /remove brief.txt/i }))

    await waitFor(() => {
      expect(within(container).queryByText('brief.txt')).not.toBeInTheDocument()
    })
  })

  it('sends attachment mime type and basic file metadata through the assistant-entry seam', async () => {
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'threads',
        user_message: {
          id: 'msg_attach_1',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: 'See attached' },
          status: null,
          importance: null,
          created_at: 0,
          updated_at: null,
        },
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_1',
          title: 'Conversation',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 0,
          updated_at: 0,
        },
      },
      meta: { request_id: 'req_attach_1' },
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    fireEvent.click(within(container).getByRole('button', { name: /add attachment/i }))

    const file = new File(['hello'], 'brief.txt', { type: 'text/plain' })
    const fileInputs = container.querySelectorAll('input[type="file"]')
    fireEvent.change(fileInputs[0] as HTMLInputElement, { target: { files: [file] } })
    fireEvent.change(requireHtmlElement(container.querySelector('textarea')), { target: { value: 'See attached' } })
    fireEvent.click(within(container).getByRole('button', { name: /send/i }))

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        {
          text: 'See attached',
          conversation_id: 'conv_1',
          attachments: [
            {
              kind: 'file',
              label: 'brief.txt',
              mime_type: 'text/plain',
              metadata: {
                size_bytes: 5,
                last_modified_ms: file.lastModified,
              },
            },
          ],
        },
        expect.any(Function),
      )
    })
  })

  it('disables Send when text is empty', () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const sendBtn = within(composer).getByRole('button', { name: /send/i })
    expect(sendBtn).toBeDisabled()
  })

  it('supports an explicit setup gate disabled state', () => {
    const onDisabledInteract = vi.fn()
    const { container } = render(
      <MessageComposer
        conversationId="conv_1"
        onSent={onSent}
        disabled
        disabledReason="Finish Core setup to enable the composer."
        onDisabledInteract={onDisabledInteract}
        floating
      />,
    )
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const textarea = requireHtmlElement(composer.querySelector('textarea'))
    const micBtn = within(composer).getByRole('button', { name: /hold to talk locally/i })
    const overlayButton = within(composer).getByRole('button', { name: /finish core setup to enable the composer\./i })
    const disabledShell = composer.querySelector('.vel-composer-gradient-border') as HTMLElement | null

    expect(textarea).toBeDisabled()
    expect(micBtn).toBeDisabled()
    expect(disabledShell).not.toBeNull()
    expect(disabledShell?.className ?? '').toContain('opacity-45')
    expect(within(composer).queryByText(/finish core setup to enable the composer/i)).not.toBeInTheDocument()

    fireEvent.click(overlayButton)
    expect(onDisabledInteract).toHaveBeenCalled()
  })

  it('calls apiPost and onSent when Send is clicked with text', async () => {
    const mockUserMessage = {
      id: 'msg_1',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: { text: 'Hi' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'threads',
        user_message: mockUserMessage,
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_1',
          title: 'Conversation',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 0,
          updated_at: 0,
        },
      },
      meta: { request_id: 'req_1' },
    })
    onOptimisticSend.mockReturnValue('tmp_1')

    const { container } = render(
      <MessageComposer
        conversationId="conv_1"
        onOptimisticSend={onOptimisticSend}
        onSent={onSent}
      />
    )
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const textarea = requireHtmlElement(composer.querySelector('textarea'))
    const sendBtn = within(composer).getByRole('button', { name: /send/i })
    fireEvent.change(textarea, { target: { value: 'Hi' } })
    fireEvent.click(sendBtn)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        { text: 'Hi', conversation_id: 'conv_1' },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(onOptimisticSend).toHaveBeenCalledWith('Hi')
      expect(onSent).toHaveBeenCalledWith(
        'tmp_1',
        expect.objectContaining({
          route_target: 'threads',
          user_message: mockUserMessage,
        }),
        expect.objectContaining({
          text: 'Hi',
          conversationId: 'conv_1',
          intent: null,
          voice: null,
          attachments: null,
        }),
      )
    })
  })

  it('shows error when apiPost rejects', async () => {
    vi.mocked(api.apiPost).mockRejectedValue(new Error('Network error'))
    onOptimisticSend.mockReturnValue('tmp_2')

    const { container } = render(
      <MessageComposer
        conversationId="conv_1"
        onOptimisticSend={onOptimisticSend}
        onSent={onSent}
        onSendFailed={onSendFailed}
      />
    )
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const textarea = requireHtmlElement(composer.querySelector('textarea'))
    const sendBtn = within(composer).getByRole('button', { name: /send/i })
    fireEvent.change(textarea, { target: { value: 'Hi' } })
    fireEvent.click(sendBtn)

    await waitFor(() => {
      expect(within(composer).getByRole('alert')).toHaveTextContent(/network error/i)
    })
    expect(onSendFailed).toHaveBeenCalledWith('tmp_2')
  })

  it('uses local voice transcript and sends it through the normal assistant route', async () => {
    const mockUserMessage = {
      id: 'msg_voice_1',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: { text: 'voice drafted note' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'threads',
        user_message: mockUserMessage,
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_1',
          title: 'Conversation',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 0,
          updated_at: 0,
        },
      },
      meta: { request_id: 'req_voice_1' },
    })

    vi.mocked(speech.useSpeechRecognition).mockImplementation((options = {}) => ({
      isSupported: true,
      isListening: false,
      error: null,
      start: () => options.onResult?.('voice drafted note'),
      stop: vi.fn(),
      interimTranscript: '',
    }))

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const voiceButton = within(container).getByRole('button', { name: /hold to talk locally/i })

    fireEvent.pointerDown(voiceButton)
    fireEvent.pointerUp(voiceButton)

    await waitFor(() => {
      expect(requireHtmlElement(composer.querySelector('textarea'))).toHaveValue('voice drafted note')
    })

    const sendBtn = within(container).getByRole('button', { name: /send/i })
    fireEvent.click(sendBtn)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        expect.objectContaining({
          text: 'voice drafted note',
          conversation_id: 'conv_1',
        }),
        expect.any(Function),
      )
    })
      expect(onSent).toHaveBeenCalledWith(
      undefined,
      expect.objectContaining({
        route_target: 'threads',
        user_message: mockUserMessage,
      }),
      expect.objectContaining({
        text: 'voice drafted note',
        conversationId: 'conv_1',
        intent: null,
      }),
    )
  })

  it('detects a url in composer text and forwards url intent', async () => {
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'inbox',
        entry_intent: 'url',
        user_message: {
          id: 'msg_url_1',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: 'https://example.com/spec' },
          status: null,
          importance: null,
          created_at: 0,
          updated_at: null,
        },
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_1',
          title: 'Conversation',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 0,
          updated_at: 0,
        },
      },
      meta: { request_id: 'req_url_1' },
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    fireEvent.change(requireHtmlElement(container.querySelector('textarea')), { target: { value: 'https://example.com/spec' } })
    fireEvent.click(within(container).getByRole('button', { name: /send/i }))

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        {
          text: 'https://example.com/spec',
          conversation_id: 'conv_1',
          intent: 'url',
        },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(onSent).toHaveBeenCalledWith(
        undefined,
        expect.objectContaining({ entry_intent: 'url' }),
        expect.objectContaining({
          text: 'https://example.com/spec',
          conversationId: 'conv_1',
          intent: 'url',
        }),
      )
    })
  })

  it('allows explicit task intent from the add menu and prefers it over inferred url intent', async () => {
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'inbox',
        entry_intent: 'task',
        user_message: {
          id: 'msg_task_1',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: 'Review https://example.com/spec' },
          status: null,
          importance: null,
          created_at: 0,
          updated_at: null,
        },
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_1',
          title: 'Conversation',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 0,
          updated_at: 0,
        },
      },
      meta: { request_id: 'req_task_1' },
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    fireEvent.click(within(container).getByRole('button', { name: /add attachment/i }))
    fireEvent.click(within(container).getByRole('menuitem', { name: /task/i }))
    expect(within(container).getByText('task')).toBeInTheDocument()

    fireEvent.change(requireHtmlElement(container.querySelector('textarea')), { target: { value: 'Review https://example.com/spec' } })
    fireEvent.click(within(container).getByRole('button', { name: /send/i }))

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        {
          text: 'Review https://example.com/spec',
          conversation_id: 'conv_1',
          intent: 'task',
        },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(onSent).toHaveBeenCalledWith(
        undefined,
        expect.objectContaining({ entry_intent: 'task' }),
        expect.objectContaining({
          text: 'Review https://example.com/spec',
          conversationId: 'conv_1',
          intent: 'task',
        }),
      )
    })
    await waitFor(() => {
      expect(within(container).queryByText('task')).not.toBeInTheDocument()
    })
  })

  it('allows explicit check-in intent from the add menu and sends reflection intent', async () => {
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'threads',
        entry_intent: 'reflection',
        user_message: {
          id: 'msg_checkin_1',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: 'Mood is steady today.' },
          status: null,
          importance: null,
          created_at: 0,
          updated_at: null,
        },
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_1',
          title: 'Conversation',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 0,
          updated_at: 0,
        },
      },
      meta: { request_id: 'req_checkin_1' },
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    fireEvent.click(within(container).getByRole('button', { name: /add attachment/i }))
    fireEvent.click(within(container).getByRole('menuitem', { name: /check-in/i }))
    expect(within(container).getByText('check-in')).toBeInTheDocument()

    fireEvent.change(requireHtmlElement(container.querySelector('textarea')), { target: { value: 'Mood is steady today.' } })
    fireEvent.click(within(container).getByRole('button', { name: /send/i }))

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        {
          text: 'Mood is steady today.',
          conversation_id: 'conv_1',
          intent: 'reflection',
        },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(onSent).toHaveBeenCalledWith(
        undefined,
        expect.objectContaining({ entry_intent: 'reflection' }),
        expect.objectContaining({
          text: 'Mood is steady today.',
          conversationId: 'conv_1',
          intent: 'reflection',
        }),
      )
    })
    await waitFor(() => {
      expect(within(container).queryByText('check-in')).not.toBeInTheDocument()
    })
  })

  it('detects a file path in composer text and forwards url intent', async () => {
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: {
        route_target: 'inbox',
        entry_intent: 'url',
        user_message: {
          id: 'msg_path_1',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: '/home/jove/code/vel/README.md' },
          status: null,
          importance: null,
          created_at: 0,
          updated_at: null,
        },
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_1',
          title: 'Conversation',
          kind: 'general',
          pinned: false,
          archived: false,
          call_mode_active: false,
          created_at: 0,
          updated_at: 0,
        },
      },
      meta: { request_id: 'req_path_1' },
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    fireEvent.change(requireHtmlElement(container.querySelector('textarea')), { target: { value: '/home/jove/code/vel/README.md' } })
    fireEvent.click(within(container).getByRole('button', { name: /send/i }))

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        {
          text: '/home/jove/code/vel/README.md',
          conversation_id: 'conv_1',
          intent: 'url',
        },
        expect.any(Function),
      )
    })
  })

  it('allows clearing the recorded voice draft chip before send', async () => {
    vi.mocked(speech.useSpeechRecognition).mockImplementation((options = {}) => ({
      isSupported: true,
      isListening: false,
      error: null,
      start: () => options.onResult?.('voice drafted note'),
      stop: vi.fn(),
      interimTranscript: '',
    }))

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} floating />)
    const voiceButton = within(container).getByRole('button', { name: /hold to talk locally/i })

    fireEvent.pointerDown(voiceButton)
    fireEvent.pointerUp(voiceButton)

    await waitFor(() => {
      expect(within(container).getByRole('button', { name: /clear recorded voice draft/i })).toBeInTheDocument()
    })

    fireEvent.click(within(container).getByRole('button', { name: /clear recorded voice draft/i }))

    await waitFor(() => {
      expect(within(container).queryByRole('button', { name: /clear recorded voice draft/i })).not.toBeInTheDocument()
    })
  })

  it('shows an explicit typed fallback when local speech-to-text is unavailable', () => {
    vi.mocked(speech.useSpeechRecognition).mockReturnValue({
      isSupported: false,
      isListening: false,
      error: null,
      start: vi.fn(),
      stop: vi.fn(),
      interimTranscript: '',
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)

    expect(
      within(composer).getByText(/local speech-to-text is not available in this browser yet/i),
    ).toBeInTheDocument()
    expect(
      within(composer).getByRole('button', { name: /hold to talk locally/i }),
    ).toBeInTheDocument()
  })

  it('raises the voice-unavailable callback when the mic is pressed without browser support', () => {
    const onVoiceUnavailable = vi.fn()
    vi.mocked(speech.useSpeechRecognition).mockReturnValue({
      isSupported: false,
      isListening: false,
      error: null,
      start: vi.fn(),
      stop: vi.fn(),
      interimTranscript: '',
    })

    const { container } = render(
      <MessageComposer conversationId="conv_1" onSent={onSent} onVoiceUnavailable={onVoiceUnavailable} />,
    )
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const voiceButton = within(composer).getByRole('button', { name: /hold to talk locally/i })

    fireEvent.pointerDown(voiceButton)

    expect(onVoiceUnavailable).toHaveBeenCalledTimes(1)
  })

  it('uses press-and-release controls for local voice capture', () => {
    const start = vi.fn()
    const stop = vi.fn()
    vi.mocked(speech.useSpeechRecognition).mockReturnValue({
      isSupported: true,
      isListening: false,
      error: null,
      start,
      stop,
      interimTranscript: '',
    })

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const voiceButton = within(composer).getByRole('button', { name: 'Hold to talk locally' })

    fireEvent.pointerDown(voiceButton)
    fireEvent.pointerUp(voiceButton)

    expect(start).toHaveBeenCalledTimes(1)
    expect(stop).toHaveBeenCalledTimes(1)
  })
})
