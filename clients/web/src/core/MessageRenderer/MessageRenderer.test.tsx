import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { MessageRenderer } from './MessageRenderer'
import type { MessageData } from '../../types'

describe('MessageRenderer', () => {
  it('renders text message content', () => {
    const message: MessageData = {
      id: 'msg_1',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: { text: 'Hello world' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    render(<MessageRenderer message={message} />)
    expect(screen.getByText('Hello world')).toBeInTheDocument()
    expect(screen.getByText(/user · text/)).toBeInTheDocument()
  })

  it('renders reminder card with title', () => {
    const message: MessageData = {
      id: 'msg_2',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'reminder_card',
      content: { title: 'Call mom', reason: 'Birthday' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    render(<MessageRenderer message={message} />)
    expect(screen.getByText('Call mom')).toBeInTheDocument()
  })

  it('renders risk card with commitment title and level', () => {
    const message: MessageData = {
      id: 'msg_3',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'risk_card',
      content: { commitment_title: 'Ship feature', risk_level: 'high' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    render(<MessageRenderer message={message} />)
    expect(screen.getByText('Ship feature')).toBeInTheDocument()
    expect(screen.getByText(/high/)).toBeInTheDocument()
  })

  it('renders canonical risk card payloads with score and dependencies', () => {
    const message: MessageData = {
      id: 'msg_3b',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'risk_card',
      content: {
        commitment_id: 'commit_42',
        risk_level: 'danger',
        risk_score: 0.82,
        factors: {
          reasons: ['long-stale open commitment', 'externally anchored commitment'],
          dependency_ids: ['dep_1', 'dep_2'],
        },
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    render(<MessageRenderer message={message} />)
    expect(screen.getByText('commit_42')).toBeInTheDocument()
    expect(screen.getByText('Score: 0.82')).toBeInTheDocument()
    expect(screen.getByText(/long-stale open commitment/)).toBeInTheDocument()
    expect(screen.getByText('Dependencies: dep_1, dep_2')).toBeInTheDocument()
  })

  it('calls onSnooze when Snooze is clicked', () => {
    const onSnooze = vi.fn()
    const message: MessageData = {
      id: 'msg_4',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'text',
      content: { text: 'Nudge' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    render(
      <MessageRenderer
        message={message}
        interventionId="intv_1"
        onSnooze={onSnooze}
      />
    )
    fireEvent.click(screen.getByRole('button', { name: /snooze/i }))
    expect(onSnooze).toHaveBeenCalledWith('intv_1')
  })

  it('calls onShowWhy when Show why is clicked', () => {
    const onShowWhy = vi.fn()
    const message: MessageData = {
      id: 'msg_5',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'text',
      content: { text: 'Nudge' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }
    render(
      <MessageRenderer
        message={message}
        interventionId="intv_1"
        onShowWhy={onShowWhy}
      />
    )
    fireEvent.click(screen.getByRole('button', { name: /show why/i }))
    expect(onShowWhy).toHaveBeenCalledWith('msg_5')
  })

  it('renders markdown headings, lists, links, and inline code', () => {
    const message: MessageData = {
      id: 'msg_md',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'text',
      content: {
        text: '# Plan\n\n- Review `worker.rs`\n- Open [status](https://example.com/status)\n\nUse *careful* retries.',
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByRole('heading', { name: 'Plan' })).toBeInTheDocument()
    expect(screen.getByText('Review')).toBeInTheDocument()
    expect(screen.getByText('worker.rs')).toBeInTheDocument()
    expect(screen.getByRole('link', { name: 'status' })).toHaveAttribute(
      'href',
      'https://example.com/status',
    )
    expect(screen.getByText('careful')).toBeInTheDocument()
  })

  it('renders fenced code blocks with copy affordance', async () => {
    const writeText = vi.fn().mockResolvedValue(undefined)
    Object.assign(navigator, {
      clipboard: { writeText },
    })

    const message: MessageData = {
      id: 'msg_code',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'text',
      content: {
        text: '```bash\ncargo test -p veld\n```',
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('bash')).toBeInTheDocument()
    expect(screen.getByText('cargo test -p veld')).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: /copy code/i }))
    expect(writeText).toHaveBeenCalledWith('cargo test -p veld')
  })
})
