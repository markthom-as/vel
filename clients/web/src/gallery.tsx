import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import '@fontsource/ibm-plex-serif/400.css';
import '@fontsource/ibm-plex-serif/600.css';
import '@fontsource-variable/space-grotesk/wght.css';
import '@fontsource-variable/outfit/wght.css';
import './index.css';
import { ActionChipButton, ActionChipLink } from './core/FilterToggleTag';
import { MessageRenderer } from './core/MessageRenderer';
import type { MessageData } from './types';

function toMessage(partial: Omit<MessageData, 'role'>): MessageData {
  return {
    role: 'assistant',
    ...partial,
    status: null,
    importance: null,
    created_at: 0,
    updated_at: null,
  };
}

const galleryTopLevelMessages: MessageData[] = [
  toMessage({
    id: 'gallery-top-level-thread',
    conversation_id: 'conv_gallery',
    kind: 'thread',
    content: {
      kind: 'thread',
      title: 'Continuity review thread',
      source: 'https://example.com/threads/continuity-review',
      source_kind: 'thread',
      status: 'open',
    },
  }),
  toMessage({
    id: 'gallery-top-level-run',
    conversation_id: 'conv_gallery',
    kind: 'run',
    content: {
      kind: 'run',
      run_id: 'run_2026_03_25',
      title: 'Reflow validation run',
      source: 'https://example.com/runs/run_2026_03_25',
    },
  }),
  toMessage({
    id: 'gallery-top-level-artifact',
    conversation_id: 'conv_gallery',
    kind: 'artifact',
    content: {
      kind: 'artifact',
      artifact_id: 'art_vega_notes',
      title: 'Design artifact',
      source: 'https://example.com/artifacts/vega_notes',
    },
  }),
  toMessage({
    id: 'gallery-top-level-config',
    conversation_id: 'conv_gallery',
    kind: 'config',
    content: {
      kind: 'config',
      config_id: 'cfg_gallery_audio',
      title: 'Capture source profile',
      path: '/var/lib/vel/config.yaml',
      source: 'file:///var/lib/vel/config.yaml',
    },
  }),
  toMessage({
    id: 'gallery-top-level-link',
    conversation_id: 'conv_gallery',
    kind: 'link',
    content: {
      kind: 'link',
      label: 'Project plan',
      source: 'https://example.com/plans/q1-2026',
    },
  }),
  toMessage({
    id: 'gallery-top-level-markdown',
    conversation_id: 'conv_gallery',
    kind: 'markdown',
    content: {
      kind: 'markdown',
      content: '---\nlang: en\nstatus: draft\n---\n# Thread Notes\n\n- check continuity\n- verify card rendering\n',
      title: 'Markdown object payload',
      source: 'artifacts/threads/top-level.md',
    },
  }),
  toMessage({
    id: 'gallery-top-level-audio',
    conversation_id: 'conv_gallery',
    kind: 'audio',
    content: {
      kind: 'audio',
      label: 'meeting_note_take',
      title: 'Meeting note take',
      content: 'https://example.com/media/meeting_note.wav',
      mime_type: 'audio/wav',
    },
  }),
  toMessage({
    id: 'gallery-text-with-attachments',
    conversation_id: 'conv_gallery',
    kind: 'text',
    content: {
      text: 'Attachment pass',
      attachments: [
        {
          kind: 'image',
          label: 'architecture-overview.png',
          mime_type: 'image/png',
          metadata: { url: 'https://picsum.photos/seed/vel-image/900/500' },
        },
        {
          kind: 'video',
          label: 'preview-loop',
          mime_type: 'video/mp4',
          metadata: { url: 'https://filesamples.com/samples/video/mp4/sample_640x360.mp4' },
        },
        {
          kind: 'audio',
          label: 'stt_sample.m4a',
          mime_type: 'audio/mp4',
          metadata: { url: 'https://www.w3schools.com/html/horse.ogg' },
        },
        {
          kind: 'markdown',
          label: 'notes.md',
          metadata: {
            content: '```ts\nconst ready = true;\nconsole.log(ready);\n```',
          },
        },
        {
          kind: 'link',
          label: 'Vel docs',
          metadata: { url: 'https://example.com/vel/docs' },
        },
        {
          kind: 'file',
          label: 'release-notes.txt',
          mime_type: 'text/plain',
          metadata: { size_bytes: 1456 },
        },
      ],
    },
  }),
  toMessage({
    id: 'gallery-inline-markdown-snippets',
    conversation_id: 'conv_gallery',
    kind: 'text',
    content: {
      text: [
        '# Markup snippet',
        '',
        'Short `inline code` sample.',
        '',
        '```bash',
        'cargo test -p vel',
        'cargo run --bin vel',
        '```',
        '',
        '```rust',
        'fn main() {',
        '    println!(\"frontmatter and sections supported\");',
        '}',
        '```',
      ].join('\n'),
    },
  }),
];

const markdownSnippetMessage = galleryTopLevelMessages.find(
  (message) => message.id === 'gallery-inline-markdown-snippets',
)!;
const topLevelMarkdownMessage = galleryTopLevelMessages.find(
  (message) => (message.content as { kind?: string }).kind === 'markdown',
)!;
const attachmentMessage = galleryTopLevelMessages.find(
  (message) => message.id === 'gallery-text-with-attachments',
)!;

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <div className="min-h-screen bg-[var(--vel-color-bg)] p-6 text-[var(--vel-color-text)]">
      <div className="mx-auto max-w-5xl space-y-10">
        <header className="space-y-2">
          <h1 className="text-2xl font-semibold text-[var(--vel-color-accent-soft)]">
            Vel Component Gallery
          </h1>
          <p className="text-sm text-zinc-300">
            Lightweight preview for media cards, top-level object rendering, markdown variants, and action chip primitives.
          </p>
        </header>

        <section className="space-y-3">
          <h2 className="text-lg font-semibold">Action chips</h2>
          <div className="flex flex-wrap gap-2">
            <ActionChipButton variant="message" onClick={() => {}}>
              Default action
            </ActionChipButton>
            <ActionChipButton variant="message" tone="brand" onClick={() => {}}>
              Brand action
            </ActionChipButton>
            <ActionChipButton variant="message" tone="success" onClick={() => {}}>
              Success action
            </ActionChipButton>
            <ActionChipLink variant="message" href="https://example.com">
              Open URL
            </ActionChipLink>
          </div>
        </section>

        <section className="space-y-3">
          <h2 className="text-lg font-semibold">Top-level object cards</h2>
          <div className="space-y-4">
            {galleryTopLevelMessages.slice(0, 7).map((message) => (
              <MessageRenderer key={message.id} message={message} compact />
            ))}
          </div>
        </section>

        <section className="space-y-3">
          <h2 className="text-lg font-semibold">Top-level markdown object</h2>
          {topLevelMarkdownMessage ? (
            <MessageRenderer key={topLevelMarkdownMessage.id} message={topLevelMarkdownMessage} compact />
          ) : null}
        </section>

        <section className="space-y-3">
          <h2 className="text-lg font-semibold">Attachment cards by kind</h2>
          {attachmentMessage ? <MessageRenderer message={attachmentMessage} compact /> : null}
        </section>

        <section className="space-y-3">
          <h2 className="text-lg font-semibold">Markdown snippet variants</h2>
          <MessageRenderer message={markdownSnippetMessage} compact />
        </section>
      </div>
    </div>
  </StrictMode>,
);
