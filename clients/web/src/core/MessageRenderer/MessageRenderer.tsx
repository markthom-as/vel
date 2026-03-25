import { useState } from 'react';
import {
  decodeReminderCardContent,
  decodeRiskCardContent,
  decodeSuggestionCardContent,
  decodeSummaryCardContent,
  decodeTextMessageContent,
  type JsonObject,
  type MessageActionContent,
  type MessageData,
} from '../../types';
import {
  ReminderCardView,
  RiskCardView,
  SuggestionCardView,
  SummaryCardView,
} from '../Cards';
import { ObjectCard } from '../ObjectCard';
import { MarkdownMessage } from '../MarkdownMessage';
import { cn } from '../cn';
import { ActionChipButton, ActionChipLink, MessageTypeTag } from '../FilterToggleTag';
import { ChatBubbleChrome } from '../MessageBubble';
import { CopyIcon } from '../Icons';
import { NowItemRowLayout } from '../NowItemRow';
import { PortableAudioPlayer } from '../PortableAudio';
import { PortableVideoPlayer } from '../PortableVideo';

interface MessageRendererProps {
  message: MessageData;
  interventionId?: string | null;
  compact?: boolean;
  tuiMode?: boolean;
  onSnooze?: (interventionId: string) => void;
  onResolve?: (interventionId: string) => void;
  onDismiss?: (interventionId: string) => void;
  onShowWhy?: (messageId: string) => void;
}

interface ParsedMarkdownContent {
  body: string;
  frontMatter: Array<{ key: string; value: string }>;
}

interface AttachmentCardData {
  kind: string;
  label?: string | null;
  mime_type?: string | null;
  metadata?: unknown;
  object_id?: string | null;
}

interface LightboxState {
  kind: 'image' | 'video';
  src: string;
  title: string;
}

function formatMessageTime(createdAt: number): string {
  try {
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
    }).format(new Date(createdAt * 1000));
  } catch {
    return '';
  }
}

function asTrimmedString(value: unknown): string | null {
  if (typeof value !== 'string') {
    return null;
  }
  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : null;
}

function attachmentRecord(value: unknown): JsonObject | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return null;
  }
  return value as JsonObject;
}

function isLikelyExtension(value: string, extensions: string[]): boolean {
  const lower = value.toLowerCase();
  return extensions.some((extension) => lower.endsWith(extension));
}

function isLikelyLinkSource(value: string): boolean {
  return (
    value.startsWith('http://')
    || value.startsWith('https://')
    || value.startsWith('mailto:')
    || value.startsWith('tel:')
    || value.startsWith('www.')
    || value.startsWith('file://')
    || value.startsWith('blob:')
    || value.startsWith('data:')
  );
}

function isLikelyImageSource(value: string): boolean {
  return (
    isLikelyLinkSource(value)
    || isLikelyExtension(value, ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.avif', '.bmp', '.svg', '.heic'])
  );
}

function isLikelyVideoSource(value: string): boolean {
  return (
    isLikelyLinkSource(value)
    || value.startsWith('rtsp://')
    || isLikelyExtension(value, ['.mp4', '.webm', '.ogv', '.mov', '.mkv', '.m3u8', '.avi'])
  );
}

function isLikelyAudioSource(value: string): boolean {
  return (
    isLikelyLinkSource(value)
    || isLikelyExtension(value, ['.mp3', '.wav', '.ogg', '.m4a', '.aac', '.flac', '.opus', '.webm'])
  );
}

const ATTACHMENT_SOURCE_KEYS = [
  'url',
  'src',
  'href',
  'uri',
  'path',
  'attachment_url',
  'storage_uri',
  'download_url',
  'public_url',
  'media_url',
  'url_raw',
  'file_url',
  'srcUrl',
];

const TOP_LEVEL_SOURCE_KEYS = [
  ...ATTACHMENT_SOURCE_KEYS,
  'id',
  'name',
  'title',
  'path',
  'identifier',
  'object_id',
  'source',
  'label',
];

function extractAttachmentValueFromUnknown(
  value: unknown,
  predicate: (candidate: string) => boolean,
  depth = 0,
): string | null {
  const record = attachmentRecord(value);
  if (!record) {
    return null;
  }

  for (const key of ATTACHMENT_SOURCE_KEYS) {
    const candidate = asTrimmedString(record[key]);
    if (candidate && predicate(candidate)) {
      return candidate;
    }
  }

  if (depth > 2) {
    return null;
  }

  for (const key of ['metadata', 'link', 'links', 'file', 'resource', 'data']) {
    const nested = extractAttachmentValueFromUnknown(record[key], predicate, depth + 1);
    if (nested) {
      return nested;
    }
  }

  return null;
}

function attachmentMediaSource(
  attachment: {
    kind: string;
    label?: string | null;
    mime_type?: string | null;
    metadata?: unknown;
    object_id?: string | null;
  },
  predicate: (value: string) => boolean,
): string | null {
  const metadataSource = extractAttachmentValueFromUnknown(attachment.metadata, predicate);
  if (metadataSource) {
    return metadataSource;
  }

  const objectId = asTrimmedString(attachment.object_id);
  if (objectId && predicate(objectId)) {
    return objectId;
  }

  const label = asTrimmedString(attachment.label);
  if (label && predicate(label)) {
    return label;
  }

  const mimeType = asTrimmedString(attachment.mime_type)?.toLowerCase();
  if (mimeType && mimeType.startsWith('video/')) {
    return attachmentMediaSource({ ...attachment, kind: attachment.kind, mime_type: undefined }, predicate);
  }

  return null;
}

function attachmentDescriptor(attachment: { kind: string; label?: string | null; mime_type?: string | null }): string {
  const label = attachment.label?.trim() || attachment.kind.replaceAll('_', ' ');
  if (!attachment.mime_type) {
    return label;
  }
  return `${label} · ${attachment.mime_type}`;
}

function buildTopLevelAttachment(message: MessageData): AttachmentCardData | null {
  const record = attachmentRecord(message.content);
  if (!record) {
    return null;
  }

  const kind =
    asTrimmedString(record.kind)
    || asTrimmedString(record.object_type)
    || asTrimmedString(record.payload_kind);
  if (!kind) {
    return null;
  }

  const label =
    asTrimmedString(record.label)
    || asTrimmedString(record.title)
    || asTrimmedString(record.name)
    || asTrimmedString(record.source)
    || message.kind;
  const objectId =
    asTrimmedString(record.object_id)
    || asTrimmedString(record.id)
    || asTrimmedString(record.thread_id)
    || asTrimmedString(record.run_id)
    || asTrimmedString(record.artifact_id)
    || asTrimmedString(record.config_id);
  const metadata = {
    ...record,
    kind,
    label,
    object_id: objectId ?? undefined,
  };

  return {
    kind,
    label,
    mime_type: asTrimmedString(record.mime_type),
    metadata,
    object_id: objectId,
  };
}

function extractTopLevelValueFromUnknown(
  value: unknown,
  predicate: (candidate: string) => boolean,
): string | null {
  const record = attachmentRecord(value);
  if (!record) {
    return null;
  }

  for (const key of TOP_LEVEL_SOURCE_KEYS) {
    const candidate = asTrimmedString(record[key]);
    if (candidate && predicate(candidate)) {
      return candidate;
    }
  }

  for (const key of ['metadata', 'content', 'payload']) {
    const nested = extractAttachmentValueFromUnknown(record[key], predicate);
    if (nested) {
      return nested;
    }
  }

  return null;
}

function parseFrontMatterMarkdown(value: string): ParsedMarkdownContent {
  const trimmed = value.trim();
  const match = trimmed.match(/^---\s*\n([\s\S]*?)\n---\s*\n?/);

  if (!match) {
    return { body: value, frontMatter: [] };
  }

  const rawFrontMatter = match[1] ?? '';
  const body = trimmed.slice(match[0].length);
  const frontMatter: Array<{ key: string; value: string }> = [];

  rawFrontMatter.split('\n').forEach((line) => {
    const trimmedLine = line.trim();
    if (!trimmedLine || trimmedLine.startsWith('#')) {
      return;
    }
    const parts = trimmedLine.split(':');
    if (parts.length < 2) {
      return;
    }
    const key = parts.shift()?.trim();
    const value = parts.join(':').trim();
    if (key) {
      frontMatter.push({ key, value });
    }
  });

  return { body, frontMatter };
}

function extractMarkdownAttachmentText(attachment: {
  label?: string | null;
  metadata?: unknown;
  object_id?: string | null;
}): string | null {
  const metadata = attachmentRecord(attachment.metadata);
  if (!metadata) {
    if (asTrimmedString(attachment.object_id)) {
      return asTrimmedString(attachment.object_id);
    }
    return asTrimmedString(attachment.label);
  }

  for (const key of ['text', 'content', 'markdown', 'body', 'snippet', 'note']) {
    const candidate = asTrimmedString(metadata[key]);
    if (candidate) {
      return candidate;
    }
  }

  return asTrimmedString(attachment.label);
}

function cardHeaderLabel(kind: string): string {
  return kind.replaceAll('_', ' ').replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function formatTopLevelPayload(value: unknown): string {
  if (typeof value === 'string') {
    return value;
  }
  if (value === null || value === undefined) {
    return JSON.stringify(value);
  }
  if (Array.isArray(value)) {
    return JSON.stringify(value, null, 2);
  }
  if (typeof value === 'object') {
    return JSON.stringify(value, null, 2);
  }
  return String(value);
}

export function MessageRenderer({
  message,
  interventionId,
  compact = false,
  tuiMode = false,
  onSnooze,
  onResolve,
  onDismiss,
  onShowWhy,
}: MessageRendererProps) {
  const [copiedAction, setCopiedAction] = useState<string | null>(null);
  const [lightbox, setLightbox] = useState<LightboxState | null>(null);
  const isUser = message.role === 'user';
  const textContent = decodeTextMessageContent(message.content);
  const reminderCardContent = decodeReminderCardContent(message.content);
  const riskCardContent = decodeRiskCardContent(message.content);
  const suggestionCardContent = decodeSuggestionCardContent(message.content);
  const summaryCardContent = decodeSummaryCardContent(message.content);
  const knownMessageKinds = ['text', 'system_notice', 'reminder_card', 'risk_card', 'suggestion_card', 'summary_card'];
  const shouldShowRawFallback =
    (message.kind === 'text' || message.kind === 'system_notice') && !textContent
    || message.kind === 'reminder_card' && !reminderCardContent
    || message.kind === 'risk_card' && !riskCardContent
    || message.kind === 'suggestion_card' && !suggestionCardContent
    || message.kind === 'summary_card' && !summaryCardContent;

  const messageText =
    textContent?.text
    || (message.kind === 'system_notice' && textContent?.text)
    || null;

  if (tuiMode) {
    const sender = isUser ? '$' : '>';
    const senderClass = isUser
      ? 'text-emerald-300'
      : 'text-[var(--vel-color-accent-strong)]';
    const roleLabel = isUser ? 'YOU' : 'VEL';
    const body = messageText
      ? messageText
      : shouldShowRawFallback
        ? JSON.stringify(message.content, null, 2)
        : '[empty]';
    const whyAction = onShowWhy && message.kind !== 'text' ? (
      <button
        type="button"
        onClick={() => onShowWhy(message.id)}
        className="mt-0.5 text-[9px] uppercase tracking-[0.14em] text-[var(--vel-color-accent-soft)] hover:text-[var(--vel-color-text)]"
      >
        show why
      </button>
    ) : null;

    return (
      <div className={cn('mb-1 flex', isUser ? 'justify-end' : 'justify-start')}>
        <div className={cn('max-w-full', isUser ? 'text-right' : 'text-left')}>
          <p className={cn('inline-flex items-center gap-2 uppercase tracking-[0.08em] text-[var(--vel-color-muted)]', compact ? 'text-[9px]' : 'text-[10px]')}>
            <span className={senderClass}>{sender}</span>
            <span className={cn('font-semibold', compact ? 'text-[10px]' : 'text-[11px]')}>{roleLabel}</span>
            <span>{formatMessageTime(message.created_at).toUpperCase()}</span>
            {message.status === 'sending' ? <span>...</span> : null}
          </p>
          {body ? (
            <pre className={cn('mt-0.5 whitespace-pre-wrap break-words py-px leading-tight text-[var(--vel-color-text)]', compact ? 'text-[9px]' : 'text-[10px]')}>
              {body}
            </pre>
          ) : null}
          {whyAction}
        </div>
      </div>
    );
  }

  const hasInterventionActions =
    interventionId && (onSnooze || onResolve || onDismiss || onShowWhy);

  const metaTimeClass = isUser ? 'text-emerald-600/80' : 'text-[#c9a082]/75';
  const metaTextClass = cn(
    'inline-flex items-center gap-1.5 font-medium uppercase leading-none',
    compact ? 'text-[8px] tracking-[0.12em]' : 'text-[10px] tracking-[0.14em]',
  );
  const cardContentClass = cn('min-w-0', compact ? 'mt-1 space-y-1' : 'mt-2 space-y-2');
  const rowClass = compact ? 'gap-1' : 'gap-3';

  function requestCopy(label: string, value: string) {
    if (typeof navigator === 'undefined' || !navigator.clipboard?.writeText) {
      return;
    }

    void navigator.clipboard.writeText(value).then(() => {
      setCopiedAction(label);
      window.setTimeout(() => setCopiedAction((current) => (current === label ? null : current)), 1200);
    });
  }

  function renderMessageActions(actions: MessageActionContent[] | undefined) {
    if (!actions || actions.length === 0) {
      return null;
    }

    return (
      <div className="mt-2 flex flex-wrap gap-1.5">
        {actions.map((action, index) => {
          const key = `${action.action_type}-${action.label}-${index}`;
          if (action.action_type === 'open_url' && action.url) {
            return (
              <ActionChipLink key={key} href={action.url} target="_blank" rel="noreferrer" variant="message">
                {action.label}
              </ActionChipLink>
            );
          }

          if (action.action_type === 'copy_text' && action.value) {
            const label = copiedAction === action.label ? 'Copied' : action.label;
            return (
              <ActionChipButton
                key={key}
                variant="message"
                onClick={() => requestCopy(action.label, action.value!)}
              >
                {label}
              </ActionChipButton>
            );
          }

          if (action.action_type === 'show_why' && onShowWhy) {
            return (
              <ActionChipButton
                key={key}
                variant="message"
                onClick={() => onShowWhy(message.id)}
              >
                {action.label}
              </ActionChipButton>
            );
          }

          return null;
        })}
      </div>
    );
  }

  function renderTopLevelObjectCard() {
    const topLevelAttachment = buildTopLevelAttachment(message);
    if (topLevelAttachment && ['image', 'video', 'audio', 'link', 'markdown', 'file', 'thread'].includes(topLevelAttachment.kind.toLowerCase())) {
      return renderAttachmentCard(topLevelAttachment, `top-level-${message.id}`);
    }

    const source = extractTopLevelValueFromUnknown(message.content, isLikelyLinkSource);
    const descriptor = formatTopLevelPayload(message.content);
    const copyLabel = `${message.id}-top-level-copy`;
    const headerLabel = cardHeaderLabel(topLevelAttachment?.kind || message.kind);
    const kindHint = topLevelAttachment?.mime_type || topLevelAttachment?.label || topLevelAttachment?.object_id;

    return (
      <ObjectCard className="space-y-2">
        <p className="text-xs font-semibold uppercase tracking-[0.16em] text-zinc-400">
          {headerLabel}
        </p>
        <pre className="overflow-x-auto whitespace-pre-wrap break-words text-sm text-zinc-300">
          {descriptor === 'null' ? 'null payload' : descriptor}
        </pre>
        {kindHint ? <p className="text-[11px] text-zinc-500">{kindHint}</p> : null}
        <div className="mt-1 flex flex-wrap gap-1.5">
          <ActionChipButton
            variant="message"
            onClick={() => requestCopy(copyLabel, descriptor)}
          >
            {copiedAction === copyLabel ? 'Copied' : 'Copy payload'}
          </ActionChipButton>
          {source ? (
            <ActionChipLink href={source} target="_blank" rel="noreferrer" variant="message">
              Open source
            </ActionChipLink>
          ) : null}
        </div>
      </ObjectCard>
    );
  }

  function renderAttachmentCard(attachment: AttachmentCardData, key: string) {
    const normalizedKind = attachment.kind.toLowerCase();
    const descriptor = attachmentDescriptor(attachment);

    if (normalizedKind === 'video') {
      const source = attachmentMediaSource(attachment, isLikelyVideoSource);
      const title = attachment.label?.trim() || 'Video';
      if (!source) {
        return (
          <ObjectCard key={key} className="space-y-2">
            <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">{title}</p>
            <p className="text-sm text-zinc-400">{descriptor}</p>
          </ObjectCard>
        );
      }

      return (
        <ObjectCard key={key} className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">{title}</p>
          <PortableVideoPlayer
            src={source}
            mimeType={attachment.mime_type}
            title={title}
            className="max-h-72"
          />
          <div className="mt-1 flex flex-wrap gap-1.5">
            <ActionChipButton variant="message" onClick={() => setLightbox({ kind: 'video', src: source, title })}>
              Full View
            </ActionChipButton>
            <ActionChipLink href={source} target="_blank" rel="noreferrer" variant="message">
              Popout
            </ActionChipLink>
          </div>
        </ObjectCard>
      );
    }

    if (normalizedKind === 'image') {
      const source = attachmentMediaSource(attachment, isLikelyImageSource);
      const title = attachment.label?.trim() || 'Image';
      if (!source) {
        return (
          <ObjectCard key={key} className="space-y-2">
            <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">{title}</p>
            <p className="text-sm text-zinc-400">{descriptor}</p>
          </ObjectCard>
        );
      }

      return (
        <ObjectCard key={key} className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">{title}</p>
          <img
            src={source}
            alt={title}
            className="w-full max-h-72 rounded border border-zinc-700 object-cover"
            loading="lazy"
          />
          <div className="mt-1 flex flex-wrap gap-1.5">
            <ActionChipButton variant="message" onClick={() => setLightbox({ kind: 'image', src: source, title })}>
              Full View
            </ActionChipButton>
            <ActionChipLink href={source} target="_blank" rel="noreferrer" variant="message">
              Popout
            </ActionChipLink>
          </div>
        </ObjectCard>
      );
    }

    if (normalizedKind === 'audio') {
      const source = attachmentMediaSource(attachment, isLikelyAudioSource);
      const title = attachment.label?.trim() || 'Audio';
      if (!source) {
        return (
          <ObjectCard key={key} className="space-y-2">
            <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">{title}</p>
            <p className="text-sm text-zinc-400">{descriptor}</p>
          </ObjectCard>
        );
      }

      return (
        <ObjectCard key={key} className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">{title}</p>
          <PortableAudioPlayer src={source} title={title} />
          <div className="mt-1 flex flex-wrap gap-1.5">
            <ActionChipLink href={source} target="_blank" rel="noreferrer" variant="message">
              Download
            </ActionChipLink>
          </div>
        </ObjectCard>
      );
    }

    if (normalizedKind === 'link') {
      const source = attachmentMediaSource(attachment, isLikelyLinkSource) ?? attachment.label ?? attachment.object_id;
      const title = attachment.label?.trim() || source?.trim() || 'Web Link';
      if (!source) {
        return (
          <ObjectCard key={key} className="space-y-2">
            <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">Web Link</p>
            <p className="text-sm text-zinc-400">{descriptor}</p>
          </ObjectCard>
        );
      }

      return (
        <ObjectCard key={key} className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">Web Link</p>
          <p className="text-sm text-zinc-300">{title}</p>
          <ActionChipLink href={source} target="_blank" rel="noreferrer" variant="message">
            Open link
          </ActionChipLink>
        </ObjectCard>
      );
    }

    if (normalizedKind === 'markdown') {
      const rawMarkdown = extractMarkdownAttachmentText(attachment);
      if (!rawMarkdown) {
        return (
          <ObjectCard key={key} className="space-y-2">
            <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">Markdown</p>
            <p className="text-sm text-zinc-400">{descriptor}</p>
          </ObjectCard>
        );
      }

      const parsed = parseFrontMatterMarkdown(rawMarkdown);
      return (
        <ObjectCard key={key} className="space-y-3">
          <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">Markdown</p>
          {parsed.frontMatter.length ? (
            <div className="rounded border border-zinc-700 bg-black/30 p-2 text-[11px] leading-6 text-zinc-300">
              <p className="mb-1 uppercase tracking-[0.16em] text-zinc-500">Frontmatter</p>
              <dl className="grid gap-1">
                {parsed.frontMatter.map((item) => (
                  <div key={`${key}-${item.key}`} className="grid grid-cols-[100px,1fr] gap-2">
                    <dt className="text-zinc-500">{item.key}</dt>
                    <dd className="text-zinc-200">{item.value}</dd>
                  </div>
                ))}
              </dl>
            </div>
          ) : null}
          {parsed.body ? <MarkdownMessage text={parsed.body} /> : <p className="text-sm text-zinc-400">No markdown body content.</p>}
        </ObjectCard>
      );
    }

    return (
      <ObjectCard key={key} className="space-y-2">
        <p className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-300">{cardHeaderLabel(normalizedKind)}</p>
        <p className="text-sm text-zinc-200">{descriptor}</p>
      </ObjectCard>
    );
  }

  const interventionHeaderActions =
    hasInterventionActions && interventionId ? (
      <>
        {onSnooze && (
          <ActionChipButton variant="message" onClick={() => onSnooze(interventionId)}>
            Snooze
          </ActionChipButton>
        )}
        {onResolve && (
          <ActionChipButton
            variant="message"
            tone="success"
            onClick={() => onResolve(interventionId)}
          >
            Resolve
          </ActionChipButton>
        )}
        {onDismiss && (
          <ActionChipButton variant="message" onClick={() => onDismiss(interventionId)}>
            Dismiss
          </ActionChipButton>
        )}
        {onShowWhy && (
          <ActionChipButton variant="message" onClick={() => onShowWhy(message.id)}>
            Show why
          </ActionChipButton>
        )}
      </>
    ) : null;

  const cardContent = (
    <>
      {message.kind === 'text' && textContent && (
        <>
          <MarkdownMessage text={textContent.text} />
          {textContent.attachments?.length ? (
            <div className="mt-2 flex flex-col gap-2">
              {textContent.attachments.map((attachment, index) =>
                renderAttachmentCard(attachment, `attachment-${message.id}-${attachment.kind}-${index}`),
              )}
            </div>
          ) : null}
          {renderMessageActions(textContent.actions)}
        </>
      )}
      {message.kind === 'reminder_card' && reminderCardContent && <ReminderCardView content={reminderCardContent} />}
      {message.kind === 'risk_card' && riskCardContent && <RiskCardView content={riskCardContent} />}
      {message.kind === 'suggestion_card' && suggestionCardContent && <SuggestionCardView content={suggestionCardContent} />}
      {message.kind === 'summary_card' && summaryCardContent && <SummaryCardView content={summaryCardContent} />}
      {message.kind === 'system_notice' && textContent && (
        <>
          <MarkdownMessage text={textContent.text} muted />
          {renderMessageActions(textContent.actions)}
        </>
      )}
      {!knownMessageKinds.includes(message.kind) && (
        renderTopLevelObjectCard()
      )}
      {shouldShowRawFallback && knownMessageKinds.includes(message.kind) && (
        <pre className="text-sm overflow-x-auto whitespace-pre-wrap break-words text-zinc-400">{JSON.stringify(message.content, null, 2)}</pre>
      )}
    </>
  );

  const titleClass = isUser ? 'text-emerald-50' : 'text-zinc-100';

  async function handleCopyMessage() {
    const plainText = textContent?.text;
    if (!plainText || typeof navigator === 'undefined' || !navigator.clipboard?.writeText) {
      return;
    }
    await navigator.clipboard.writeText(plainText);
  }

  return (
    <>
      <div
        className={cn(`flex ${isUser ? 'justify-end' : 'justify-start'}`, compact ? 'mb-1.5' : 'mb-3')}
        data-message-id={message.id}
      >
        <ChatBubbleChrome variant={isUser ? 'user' : 'assistant'}>
          <NowItemRowLayout actions={interventionHeaderActions} actionsLayout="inline">
            <div className={cn('flex items-start justify-between', rowClass)}>
              <div className="min-w-0">
                <p className={cn(metaTextClass, titleClass)}>
                  <span>{isUser ? 'YOU' : 'VEL'}</span>
                  <span className={metaTimeClass}>|</span>
                  <span className={metaTimeClass}>{formatMessageTime(message.created_at).toUpperCase()}</span>
                </p>
              </div>
              <div className={cn('flex min-w-0 flex-wrap justify-end', compact ? 'gap-1' : 'gap-1.5')}>
                <MessageTypeTag variant={isUser ? 'user' : 'assistant'} className="opacity-35">
                  {isUser ? 'USER TEXT' : 'ASSISTANT TEXT'}
                </MessageTypeTag>
                {!isUser && textContent ? (
                  <button
                    type="button"
                    aria-label="Copy assistant message"
                    onClick={() => void handleCopyMessage()}
                    className={cn(
                      'inline-flex items-center justify-center rounded-full border border-[#ff6b00]/35 bg-[rgba(74,36,18,0.22)] text-[#c9a082] transition hover:text-zinc-100',
                      compact ? 'h-5 w-5' : 'h-6 w-6',
                    )}
                  >
                    <CopyIcon size={11} />
                  </button>
                ) : null}
                {message.status === 'sending' ? <MessageTypeTag variant={isUser ? 'user' : 'assistant'}>Sending…</MessageTypeTag> : null}
              </div>
            </div>
            <div className={cardContentClass}>{cardContent}</div>
          </NowItemRowLayout>
        </ChatBubbleChrome>
      </div>
      {lightbox ? (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/90 p-4"
          onClick={() => setLightbox(null)}
        >
          <div
            className="mx-auto flex w-full max-w-5xl flex-col rounded-lg border border-zinc-700 bg-black/95 p-3"
            onClick={(event) => event.stopPropagation()}
          >
            <div className="mb-2 flex items-center justify-between">
              <p className="text-sm uppercase tracking-[0.16em] text-zinc-300">{lightbox.title}</p>
              <button
                type="button"
                onClick={() => setLightbox(null)}
                className="rounded-full border border-zinc-700 px-3 py-1 text-xs text-zinc-300"
                aria-label="Close media preview"
              >
                Close
              </button>
            </div>
            {lightbox.kind === 'image' ? (
              <img
                src={lightbox.src}
                alt={lightbox.title}
                className="max-h-[80vh] w-full rounded border border-zinc-800 object-contain"
              />
            ) : (
              <PortableVideoPlayer src={lightbox.src} title={lightbox.title} className="max-h-[80vh]" />
            )}
          </div>
        </div>
      ) : null}
    </>
  );
}
