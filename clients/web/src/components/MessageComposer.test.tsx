import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, fireEvent, waitFor, within } from '@testing-library/react'
import { MessageComposer } from './MessageComposer'
import * as api from '../api/client'

vi.mock('../api/client', () => ({
  apiPost: vi.fn(),
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
  })

  it('renders textarea and Send button', () => {
    const { container } = render(<MessageComposer conversationId="conv_1" onSent={onSent} />)
    const composer = requireHtmlElement(container.firstElementChild as HTMLElement | null)
    expect(composer.querySelector('textarea')).toBeInTheDocument()
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
      data: { user_message: mockUserMessage, assistant_message: null },
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
        '/api/conversations/conv_1/messages',
        { role: 'user', kind: 'text', content: { text: 'Hi' } },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(onOptimisticSend).toHaveBeenCalledWith('Hi')
      expect(onSent).toHaveBeenCalledWith('tmp_1', mockUserMessage, null)
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
})
