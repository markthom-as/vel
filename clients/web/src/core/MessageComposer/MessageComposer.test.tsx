import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, fireEvent, waitFor, within } from '@testing-library/react'
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

  it('renders textarea and Send button', () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    expect(composer.querySelector('textarea')).toBeInTheDocument()
    expect(
      within(composer).getByPlaceholderText(/ask, capture, or talk to vel/i),
    ).toBeInTheDocument()
    expect(within(composer).getByRole('button', { name: /send/i })).toBeInTheDocument()
  })

  it('disables Send when text is empty', () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const sendBtn = within(composer).getByRole('button', { name: /send/i })
    expect(sendBtn).toBeDisabled()
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
      expect(onSent).toHaveBeenCalledWith('tmp_1', expect.objectContaining({
        route_target: 'threads',
        user_message: mockUserMessage,
      }))
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

    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    const voiceButton = within(composer).getByRole('button', { name: /hold to talk locally/i })
    const sendBtn = within(composer).getByRole('button', { name: /send/i })

    fireEvent.mouseDown(voiceButton)
    fireEvent.mouseUp(voiceButton)

    await waitFor(() => {
      expect(requireHtmlElement(composer.querySelector('textarea'))).toHaveValue('voice drafted note')
    })

    fireEvent.click(sendBtn)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        expect.objectContaining({
          text: 'voice drafted note',
          conversation_id: 'conv_1',
          voice: expect.objectContaining({
            surface: 'web',
            source_device: 'browser',
            transcript_origin: 'local_browser_stt',
          }),
        }),
        expect.any(Function),
      )
    })
    expect(onSent).toHaveBeenCalledWith(undefined, expect.objectContaining({
      route_target: 'threads',
      user_message: mockUserMessage,
    }))
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
      within(composer).queryByRole('button', { name: /hold to talk locally/i }),
    ).not.toBeInTheDocument()
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
    const voiceButton = within(composer).getByRole('button', { name: /hold to talk locally/i })

    fireEvent.mouseDown(voiceButton)
    fireEvent.mouseUp(voiceButton)

    expect(start).toHaveBeenCalledTimes(1)
    expect(stop).toHaveBeenCalledTimes(1)
  })
})
