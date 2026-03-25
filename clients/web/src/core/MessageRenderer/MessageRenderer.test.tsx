import { describe, it, expect, vi } from 'vitest'
import { render, screen, fireEvent, within } from '@testing-library/react'
import { MessageRenderer } from './MessageRenderer'
import type { MessageData } from '../../types'

vi.mock('video.js', () => ({
  default: vi.fn((_element, _options, ready) => {
    ready?.()
    return {
      dispose: vi.fn(),
    }
  }),
}))

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
    expect(screen.getByText('YOU')).toBeInTheDocument()
    expect(screen.getByText('USER TEXT')).toBeInTheDocument()
    expect(screen.getByText('Hello world').closest('[data-chat-bubble-variant="user"]')).toBeTruthy()
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

  it('renders file and image message attachments as cards', () => {
    const message: MessageData = {
      id: 'msg_attached',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: {
        text: 'See attached files',
        attachments: [
          { kind: 'file', label: 'brief.txt', mime_type: 'text/plain', metadata: { size_bytes: 5 } },
          { kind: 'image', label: 'diagram.png', mime_type: 'image/png', metadata: { size_bytes: 42 } },
        ],
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    const { container } = render(<MessageRenderer message={message} />)
    const messageScope = within(container)

    expect(screen.getByText('brief.txt · text/plain')).toBeInTheDocument()
    expect(messageScope.getByAltText('diagram.png')).toBeInTheDocument()
    expect(messageScope.getByRole('button', { name: 'Full View' })).toBeInTheDocument()
  })

  it('renders a video attachment as a portable video player when URL is present', () => {
    const message: MessageData = {
      id: 'msg_video_attachment',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: {
        text: 'Check this clip',
        attachments: [
          {
            kind: 'video',
            label: 'briefing.mp4',
            mime_type: 'video/mp4',
            metadata: { url: 'https://example.com/briefing.mp4' },
          },
        ],
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByTestId('portable-video-player')).toBeInTheDocument()
  })

  it('falls back to an attachment chip when video attachment has no source', () => {
    const message: MessageData = {
      id: 'msg_video_attachment_fallback',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: {
        text: 'Missing source',
        attachments: [
          {
            kind: 'video',
            label: 'missing',
            mime_type: 'video/mp4',
          },
        ],
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('missing · video/mp4')).toBeInTheDocument()
  })

  it('renders an image attachment with full-view and popout actions', () => {
    const message: MessageData = {
      id: 'msg_image_attachment',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: {
        text: 'See this image',
        attachments: [
          {
            kind: 'image',
            label: 'diagram.png',
            mime_type: 'image/png',
            metadata: { url: 'https://example.com/diagram.png' },
          },
        ],
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    const { container } = render(<MessageRenderer message={message} />)
    const messageScope = within(container)

    expect(messageScope.getByAltText('diagram.png')).toBeInTheDocument()
    expect(messageScope.getByRole('link', { name: 'Popout' })).toHaveAttribute(
      'href',
      'https://example.com/diagram.png',
    )
    fireEvent.click(messageScope.getByRole('button', { name: 'Full View' }))
    expect(messageScope.getByLabelText('Close media preview')).toBeInTheDocument()
    fireEvent.click(messageScope.getByLabelText('Close media preview'))
    expect(messageScope.queryByLabelText('Close media preview')).not.toBeInTheDocument()
  })

  it('renders an audio attachment with scrubber and transcript action', () => {
    const message: MessageData = {
      id: 'msg_audio_attachment',
      conversation_id: 'conv_1',
      role: 'user',
      kind: 'text',
      content: {
        text: 'Listen',
        attachments: [
          {
            kind: 'audio',
            label: 'voice-note.m4a',
            mime_type: 'audio/mp4',
            metadata: { url: 'https://example.com/voice-note.m4a' },
          },
        ],
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByLabelText('Audio scrubber for voice-note.m4a')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /stt/i })).toBeInTheDocument()
  })

  it('renders markdown attachments with frontmatter and markdown body', () => {
    const message: MessageData = {
      id: 'msg_markdown_attachment',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'text',
      content: {
        text: 'Attached notes',
        attachments: [
          {
            kind: 'markdown',
            label: 'notes.md',
            metadata: {
              content:
                '---\nlang: en\nstatus: draft\n---\n# Notes\n\n- alpha\n- beta',
            },
          },
        ],
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('Markdown')).toBeInTheDocument()
    expect(screen.getByText('Frontmatter')).toBeInTheDocument()
    expect(screen.getByText('lang')).toBeInTheDocument()
    expect(screen.getByText('draft')).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: 'Notes' })).toBeInTheDocument()
  })

  it('renders top-level object messages as a card with action chips', () => {
    const writeText = vi.fn().mockResolvedValue(undefined)
    Object.assign(navigator, {
      clipboard: { writeText },
    })

    const message: MessageData = {
      id: 'msg_top_level_object',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'thread',
      content: { title: 'Design thread', source: 'https://example.com/thread/42', extra: { note: 'follow-up' } },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('Thread')).toBeInTheDocument()
    expect(screen.getByText('Open source')).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: /copy payload/i }))
    expect(writeText).toHaveBeenCalledWith(
      JSON.stringify({ title: 'Design thread', source: 'https://example.com/thread/42', extra: { note: 'follow-up' } }, null, 2),
    )
  })

  it('renders top-level image object messages as image cards', () => {
    const message: MessageData = {
      id: 'msg_top_level_image',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'thread',
      content: {
        kind: 'image',
        label: 'diagram.png',
        metadata: { url: 'https://example.com/diagram.png' },
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    };

    const { container } = render(<MessageRenderer message={message} />);
    expect(within(container).getByAltText('diagram.png')).toBeInTheDocument()
    expect(screen.getByText('Image')).toBeInTheDocument()
  })

  it('renders top-level video object messages with media actions', () => {
    const message: MessageData = {
      id: 'msg_top_level_video',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'thread',
      content: {
        kind: 'video',
        label: 'briefing.mp4',
        metadata: { url: 'https://example.com/briefing.mp4', mime_type: 'video/mp4' },
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    };

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('briefing.mp4')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Full View' })).toBeInTheDocument()
    expect(screen.getByRole('link', { name: 'Popout' })).toHaveAttribute('href', 'https://example.com/briefing.mp4')
  })

  it('renders top-level audio object messages with scrubber controls', () => {
    const message: MessageData = {
      id: 'msg_top_level_audio',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'audio',
      content: {
        kind: 'audio',
        label: 'voice-note.m4a',
        metadata: { url: 'https://example.com/voice-note.m4a' },
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    };

    render(<MessageRenderer message={message} />)

    expect(screen.getByLabelText('Audio scrubber for voice-note.m4a')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /stt/i })).toBeInTheDocument()
  })

  it('renders top-level markdown objects with frontmatter and body preview', () => {
    const message: MessageData = {
      id: 'msg_top_level_markdown',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'markdown',
      content: {
        kind: 'markdown',
        content: '---\nlang: en\nstatus: draft\n---\n# Heading\n\nPreview',
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    };

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('Frontmatter')).toBeInTheDocument()
    expect(screen.getByText('lang')).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: 'Heading' })).toBeInTheDocument()
  })

  it('renders top-level link object messages with open actions', () => {
    const message: MessageData = {
      id: 'msg_top_level_link',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'link',
      content: {
        kind: 'link',
        label: 'Project link',
        source: 'https://example.com/projects/1',
      },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
      };

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('Web Link')).toBeInTheDocument()
    expect(screen.getByRole('link', { name: 'Open link' })).toHaveAttribute(
      'href',
      'https://example.com/projects/1',
    )
  })

  it('uses the tail-less chat bubble chrome for assistant messages', () => {
    const message: MessageData = {
      id: 'msg_tail_less',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'text',
      content: { text: 'Assistant response' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    render(<MessageRenderer message={message} />)

    expect(screen.getByText('Assistant response').closest('[data-chat-bubble-variant="assistant"]')).toBeTruthy()
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
