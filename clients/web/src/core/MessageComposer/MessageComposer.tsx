import { useRef, useState, useCallback, useMemo, useEffect } from 'react';
import { submitAssistantEntry } from '../../data/chat';
import { useSpeechRecognition } from '../../hooks/useSpeechRecognition';
import {
  type AssistantEntryAttachmentData,
  type AssistantEntryResponse,
  type NowDockedInputIntentData,
  type AssistantEntryVoiceProvenanceData,
} from '../../types';
import { cn } from '../cn';
import { ClipboardCheckIcon, CloseIcon, FileIcon, ImageIcon, MicIcon, PlusIcon, SendArrowIcon } from '../Icons';
import { uiTheme } from '../Theme';

/** Browser STT session cap — pie + auto-stop use this as the full ring. */
const FLOATING_VOICE_MAX_MS = 120_000;

function trimIntentToken(token: string): string {
  return token.trim().replace(/^[('"`[{<]+|[)'"`}\].,;:!?]+$/g, '');
}

function looksLikeLocalPathToken(token: string): boolean {
  if (!token || token.includes('://')) {
    return false;
  }
  if (token.startsWith('/') || token.startsWith('./') || token.startsWith('../') || token.startsWith('~/')) {
    return true;
  }
  if (!token.includes('/')) {
    return false;
  }
  return /[A-Za-z]/.test(token);
}

function inferImplicitIntent(text: string): NowDockedInputIntentData | null {
  const tokens = text
    .split(/\s+/)
    .map(trimIntentToken)
    .filter(Boolean);
  const hasUrlOrPath = tokens.some((token) =>
    token.startsWith('http://')
    || token.startsWith('https://')
    || token.startsWith('www.')
    || token.startsWith('file://')
    || looksLikeLocalPathToken(token)
  );
  return hasUrlOrPath ? 'url' : null;
}

function formatVoiceClock(ms: number): string {
  const s = Math.max(0, Math.floor(ms / 1000));
  const m = Math.floor(s / 60);
  const r = s % 60;
  return `${m}:${r.toString().padStart(2, '0')}`;
}

function FloatingMicPieRing({ progress, warn }: { progress: number; warn: boolean }) {
  const p = Math.min(1, Math.max(0, progress));
  const r = 17;
  const c = 2 * Math.PI * r;
  const dashOffset = c * (1 - p);
  const stroke = warn ? 'rgb(248 113 113)' : 'rgb(255 107 0)';
  return (
    <svg
      className="pointer-events-none absolute inset-0 h-full w-full"
      viewBox="0 0 44 44"
      aria-hidden
    >
      <circle
        cx="22"
        cy="22"
        r={r}
        fill="none"
        stroke="rgba(255,255,255,0.1)"
        strokeWidth="3"
      />
      <g transform="rotate(-90 22 22)">
        <circle
          cx="22"
          cy="22"
          r={r}
          fill="none"
          stroke={stroke}
          strokeWidth="3"
          strokeLinecap="round"
          strokeDasharray={c}
          strokeDashoffset={dashOffset}
          className="transition-[stroke-dashoffset] duration-100 ease-linear"
        />
      </g>
    </svg>
  );
}

interface MessageComposerProps {
  conversationId?: string | null;
  onOptimisticSend?: (text: string) => string | undefined;
  onSent: (
    clientMessageId: string | undefined,
    response: AssistantEntryResponse,
    submitted: SubmittedAssistantEntryPayload,
  ) => void;
  onSendFailed?: (clientMessageId: string | undefined) => void;
  onVoiceUnavailable?: () => void;
  compact?: boolean;
  hideHelperText?: boolean;
  floating?: boolean;
  floatingOffsetClassName?: string;
  disabled?: boolean;
  disabledReason?: string | null;
  onDisabledInteract?: () => void;
}

type AttachmentDraft = {
  id: string;
  name: string;
  kind: 'file' | 'image';
  mimeType: string | null;
  sizeBytes: number | null;
  lastModifiedMs: number | null;
};

export interface SubmittedAssistantEntryPayload {
  text: string;
  conversationId: string | null;
  intent: NowDockedInputIntentData | null;
  voice: AssistantEntryVoiceProvenanceData | null;
  attachments: AssistantEntryAttachmentData[] | null;
}

export function MessageComposer({
  conversationId,
  onOptimisticSend,
  onSent,
  onSendFailed,
  onVoiceUnavailable,
  compact: _compact = false,
  hideHelperText = false,
  floating = false,
  floatingOffsetClassName = 'bottom-5',
  disabled = false,
  disabledReason: _disabledReason = null,
  onDisabledInteract,
}: MessageComposerProps) {
  const [text, setText] = useState('');
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pendingVoice, setPendingVoice] = useState<AssistantEntryVoiceProvenanceData | null>(null);
  const pendingVoiceRef = useRef<AssistantEntryVoiceProvenanceData | null>(null);
  const voicePressActiveRef = useRef(false);
  const voiceLatchedRef = useRef(false);
  /** Counts browser STT `onend` auto-restarts per press/latch (avoids tight loops if start fails). */
  const voiceSttResumeCountRef = useRef(0);
  const voicePrevIsListeningRef = useRef(false);
  const [voiceElapsedMs, setVoiceElapsedMs] = useState(0);
  const [voiceLatched, setVoiceLatched] = useState(false);
  const [attachmentMenuOpen, setAttachmentMenuOpen] = useState(false);
  const [queuedAttachments, setQueuedAttachments] = useState<AttachmentDraft[]>([]);
  const [selectedIntent, setSelectedIntent] = useState<NowDockedInputIntentData | null>(null);
  const attachmentMenuRef = useRef<HTMLDivElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const imageInputRef = useRef<HTMLInputElement | null>(null);
  const textareaRef = useRef<HTMLTextAreaElement | null>(null);

  const appendVoiceTranscript = useCallback((transcript: string) => {
    setText((prev) => (prev ? `${prev} ${transcript}` : transcript));
    const voiceDraft = {
      surface: 'web',
      source_device: 'browser',
      locale: typeof navigator === 'undefined' ? 'en-US' : navigator.language ?? 'en-US',
      transcript_origin: 'local_browser_stt',
      recorded_at: new Date().toISOString(),
    };
    pendingVoiceRef.current = voiceDraft;
    setPendingVoice(voiceDraft);
  }, []);

  const voiceRecognition = useSpeechRecognition({
    onResult: appendVoiceTranscript,
    continuous: true,
  }) ?? {
    isSupported: false,
    isListening: false,
    error: null,
    start: () => undefined,
    stop: () => undefined,
    interimTranscript: '',
  };

  const {
    isSupported: voiceSupported,
    isListening,
    error: voiceError,
    start: startVoice,
    stop: stopVoice,
    interimTranscript,
  } = voiceRecognition;

  const stopVoiceSession = useCallback(() => {
    voicePressActiveRef.current = false;
    voiceLatchedRef.current = false;
    voiceSttResumeCountRef.current = 0;
    setVoiceLatched(false);
    stopVoice();
  }, [stopVoice]);

  /**
   * Chrome/WebKit often end the STT session (`onend`) while the mic is still held or latched.
   * If the user is still recording, start a new recognition instance.
   * Only runs when `isListening` goes from true → false (not while waiting for first `onstart`).
   */
  useEffect(() => {
    if (isListening) {
      voiceSttResumeCountRef.current = 0;
      voicePrevIsListeningRef.current = true;
      return;
    }
    const sessionJustEnded = voicePrevIsListeningRef.current;
    voicePrevIsListeningRef.current = false;
    if (!sessionJustEnded) return;

    const keep = voicePressActiveRef.current || voiceLatchedRef.current;
    if (!keep) return;
    if (voiceSttResumeCountRef.current >= 40) {
      setError('Voice input stopped unexpectedly. Release the mic and try again.');
      stopVoiceSession();
      return;
    }
    voiceSttResumeCountRef.current += 1;
    const id = window.setTimeout(() => {
      if (!voicePressActiveRef.current && !voiceLatchedRef.current) return;
      startVoice();
    }, 0);
    return () => window.clearTimeout(id);
  }, [isListening, startVoice, stopVoiceSession]);

  /** Elapsed time + auto-stop at max while the recognizer is running. */
  useEffect(() => {
    if (!isListening) {
      setVoiceElapsedMs(0);
      return;
    }
    const started = Date.now();
    setVoiceElapsedMs(0);
    const id = window.setInterval(() => {
      const elapsed = Date.now() - started;
      setVoiceElapsedMs(elapsed);
      if (elapsed >= FLOATING_VOICE_MAX_MS) {
        window.clearInterval(id);
        setError(`Maximum recording length (${formatVoiceClock(FLOATING_VOICE_MAX_MS)}) reached.`);
        stopVoiceSession();
      }
    }, 100);
    return () => window.clearInterval(id);
  }, [isListening, stopVoiceSession]);

  useEffect(() => {
    if (!isListening || !voiceLatched) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        stopVoiceSession();
      }
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [isListening, voiceLatched, stopVoiceSession]);

  useEffect(() => {
    if (!attachmentMenuOpen) return;
    const onPointerDown = (event: PointerEvent) => {
      const target = event.target as Node | null;
      if (target && attachmentMenuRef.current?.contains(target)) {
        return;
      }
      setAttachmentMenuOpen(false);
    };
    window.addEventListener('pointerdown', onPointerDown);
    return () => window.removeEventListener('pointerdown', onPointerDown);
  }, [attachmentMenuOpen]);

  /** Text committed in the textarea plus any in-flight STT (not yet appended to `text`). */
  const mergedMessage = useMemo(() => {
    const t = text.trim();
    const i = interimTranscript.trim();
    if (t && i) return `${t} ${i}`;
    return t || i;
  }, [text, interimTranscript]);

  const hasSendablePayload = mergedMessage.trim().length > 0;
  const attachmentPayload = useMemo<AssistantEntryAttachmentData[] | null>(
    () => queuedAttachments.length
      ? queuedAttachments.map((attachment) => ({
          kind: attachment.kind,
          label: attachment.name,
          mime_type: attachment.mimeType,
          metadata: {
            size_bytes: attachment.sizeBytes,
            last_modified_ms: attachment.lastModifiedMs,
          },
        }))
      : null,
    [queuedAttachments],
  );

  const send = useCallback(async () => {
    const trimmed = mergedMessage.trim();
    if (!trimmed || sending || disabled) return;
    setError(null);
    setSending(true);
    const intent = selectedIntent ?? inferImplicitIntent(trimmed);
    const clientMessageId = onOptimisticSend?.(trimmed);
    const submittedPayload: SubmittedAssistantEntryPayload = {
      text: trimmed,
      conversationId: conversationId ?? null,
      intent,
      voice: pendingVoiceRef.current,
      attachments: attachmentPayload,
    };
    try {
      const res = await submitAssistantEntry(
        trimmed,
        conversationId,
        pendingVoiceRef.current,
        intent,
        attachmentPayload,
      );
      if (res.ok && res.data) {
        onSent(clientMessageId, res.data, submittedPayload);
        setText('');
        setQueuedAttachments([]);
        setSelectedIntent(null);
        pendingVoiceRef.current = null;
        setPendingVoice(null);
        if (res.data.assistant_error) {
          setError(res.data.assistant_error);
        }
      } else if (res.ok && !res.data) {
        onSendFailed?.(clientMessageId);
        setError("Server didn't return the message. Try again.");
      } else {
        onSendFailed?.(clientMessageId);
        setError(res.error?.message ?? 'Send failed');
      }
    } catch (err) {
      onSendFailed?.(clientMessageId);
      setError(err instanceof Error ? err.message : 'Send failed');
    } finally {
      setSending(false);
    }
  }, [mergedMessage, conversationId, sending, attachmentPayload, onOptimisticSend, onSent, onSendFailed, disabled, selectedIntent]);

  const enqueueAttachments = useCallback((files: FileList | null, kind: AttachmentDraft['kind']) => {
    if (!files || files.length === 0) return;
    const next = Array.from(files).map((file, index) => ({
      id: `${kind}_${file.name}_${file.lastModified}_${index}`,
      name: file.name,
      kind,
      mimeType: file.type || null,
      sizeBytes: Number.isFinite(file.size) ? file.size : null,
      lastModifiedMs: Number.isFinite(file.lastModified) ? file.lastModified : null,
    }));
    setQueuedAttachments((current) => [...current, ...next]);
    setAttachmentMenuOpen(false);
  }, []);

  const removeQueuedAttachment = useCallback((id: string) => {
    setQueuedAttachments((current) => current.filter((attachment) => attachment.id !== id));
  }, []);

  const selectIntent = useCallback((intent: NowDockedInputIntentData) => {
    setSelectedIntent(intent);
    setAttachmentMenuOpen(false);
    window.setTimeout(() => {
      textareaRef.current?.focus();
    }, 0);
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  const beginVoiceCapture = (): boolean => {
    if (sending || disabled || !voiceSupported || voicePressActiveRef.current) {
      return false;
    }
    if (voiceError) setError(null);
    voicePressActiveRef.current = true;
    startVoice();
    return true;
  };

  const endVoiceCapture = () => {
    if (voiceLatchedRef.current) {
      return;
    }
    if (!voicePressActiveRef.current) {
      return;
    }
    voicePressActiveRef.current = false;
    stopVoice();
  };

  const handleMicDoubleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    e.stopPropagation();
    if (sending || disabled) return;
    if (!voiceSupported) {
      onVoiceUnavailable?.();
      return;
    }
    if (voiceError) setError(null);
    if (voiceLatchedRef.current) {
      stopVoiceSession();
      return;
    }
    if (isListening && !voiceLatchedRef.current) {
      voiceLatchedRef.current = true;
      setVoiceLatched(true);
      return;
    }
    voiceLatchedRef.current = true;
    setVoiceLatched(true);
    beginVoiceCapture();
  };

  const handleMicPointerDown = (e: React.PointerEvent<HTMLButtonElement>) => {
    if (e.pointerType === 'mouse' && e.button !== 0) return;
    if (sending || disabled) return;
    if (!voiceSupported) {
      onVoiceUnavailable?.();
      return;
    }
    if (voiceError) setError(null);
    if (voiceLatchedRef.current) {
      stopVoiceSession();
      return;
    }
    if (!beginVoiceCapture()) return;
    try {
      e.currentTarget.setPointerCapture(e.pointerId);
    } catch {
      /* setPointerCapture unsupported or already captured */
    }
  };

  const handleMicPointerUp = (e: React.PointerEvent<HTMLButtonElement>) => {
    try {
      if (typeof e.currentTarget.hasPointerCapture === 'function' && e.currentTarget.hasPointerCapture(e.pointerId)) {
        e.currentTarget.releasePointerCapture(e.pointerId);
      }
    } catch {
      /* ignore */
    }
    endVoiceCapture();
  };

  const handleMicKeyboardDown = () => {
    if (sending || disabled) return;
    if (!voiceSupported) {
      onVoiceUnavailable?.();
      return;
    }
    if (voiceError) setError(null);
    if (voiceLatchedRef.current) {
      stopVoiceSession();
      return;
    }
    beginVoiceCapture();
  };

  const handleVoiceKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.repeat) {
      return;
    }
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      handleMicKeyboardDown();
    }
  };

  const handleVoiceKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (voiceLatchedRef.current) {
      return;
    }
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      endVoiceCapture();
    }
  };

  const displayError = error ?? voiceError ?? null;
  const showSendButton = !floating || sending || hasSendablePayload;
  const interactionDisabled = sending || disabled;

  const voiceHint = voiceSupported
    ? isListening
      ? voiceLatched
        ? 'Hands-free recording — tap the mic or press Esc to stop. Transcript stays local until you send.'
        : 'Release to stop. Transcript stays local until you send it.'
      : floating
        ? `Hold the mic to talk, or double-click for hands-free (up to ${formatVoiceClock(FLOATING_VOICE_MAX_MS)}). Vel routes the transcript into Now, Inbox, or Threads.`
        : 'Hold the mic to talk locally. Vel routes the transcript into Now, Inbox, or Threads.'
    : 'Local speech-to-text is not available in this browser yet. Type your message instead.';

  const micAriaLabel = isListening
    ? voiceLatched
      ? 'Hands-free recording — tap to stop'
      : 'Release to stop local voice input'
    : floating
      ? 'Hold to talk locally; double-click for hands-free recording'
      : 'Hold to talk locally';

  const micTitle = isListening
    ? voiceLatched
      ? 'Hands-free — tap mic or Esc to stop'
      : 'Release to stop local voice input'
    : floating
      ? `Hold for push-to-talk, or double-click for hands-free (max ${formatVoiceClock(FLOATING_VOICE_MAX_MS)})`
      : 'Hold to talk locally with browser speech-to-text';

  const voiceProgress = voiceElapsedMs / FLOATING_VOICE_MAX_MS;
  const voiceStatusLabel = isListening
    ? interimTranscript
      ? 'Transcribing'
      : voiceLatched
        ? 'Recording'
        : 'Listening'
    : pendingVoice
      ? 'Recorded'
      : null;

  const micButtonEl = floating ? (
      <div className={`flex items-center ${isListening ? 'gap-1' : ''}`}>
        {isListening ? (
          <span
            className="font-mono text-[10px] leading-none tabular-nums text-zinc-400"
            aria-live="polite"
            aria-label={`Recording ${formatVoiceClock(voiceElapsedMs)} of ${formatVoiceClock(FLOATING_VOICE_MAX_MS)}`}
          >
            {formatVoiceClock(voiceElapsedMs)}
          </span>
        ) : null}
        <button
          type="button"
          onPointerDown={handleMicPointerDown}
          onPointerUp={handleMicPointerUp}
          onPointerCancel={handleMicPointerUp}
          onDoubleClick={handleMicDoubleClick}
          onKeyDown={handleVoiceKeyDown}
          onKeyUp={handleVoiceKeyUp}
          disabled={interactionDisabled}
          aria-pressed={voiceSupported ? isListening : undefined}
          aria-label={micAriaLabel}
          title={micTitle}
          data-vel-voice-trigger={floating ? 'true' : undefined}
          className={cn(
            'relative flex h-9 w-9 shrink-0 items-center justify-center rounded-full border transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-[#ff6b00]/45 disabled:pointer-events-none disabled:opacity-50',
            isListening
              ? 'border-red-700/60 bg-red-950/60 text-red-300'
              : voiceSupported
                ? 'border-transparent text-zinc-500 hover:text-zinc-300'
                : 'border-zinc-800/85 bg-zinc-900/60 text-zinc-500 hover:border-zinc-700 hover:text-zinc-300',
          )}
        >
          {isListening ? (
            <FloatingMicPieRing progress={voiceProgress} warn={voiceProgress >= 0.85} />
          ) : null}
          <span className="relative z-10 flex items-center justify-center">
            <MicIcon listening={isListening} size={14} />
          </span>
        </button>
      </div>
    ) : (
      <button
        type="button"
        onPointerDown={handleMicPointerDown}
        onPointerUp={handleMicPointerUp}
        onPointerCancel={handleMicPointerUp}
        onDoubleClick={handleMicDoubleClick}
        onKeyDown={handleVoiceKeyDown}
        onKeyUp={handleVoiceKeyUp}
        disabled={interactionDisabled}
        aria-pressed={isListening}
        aria-label={micAriaLabel}
        title={micTitle}
        className={`shrink-0 rounded-full border px-3 py-2 text-xs uppercase tracking-[0.18em] transition-colors focus:outline-none focus:ring-2 focus:ring-[#ff6b00]/35 disabled:pointer-events-none disabled:opacity-50 ${
          isListening
            ? 'bg-red-900/40 border-red-600/60 text-red-200'
            : voiceSupported
              ? 'bg-zinc-800/50 border-zinc-700 text-zinc-300 hover:bg-zinc-700/50 hover:text-zinc-100'
              : 'bg-zinc-900/55 border-zinc-800 text-zinc-500 hover:border-zinc-700 hover:text-zinc-300'
        }`}
      >
        <MicIcon listening={isListening} />
      </button>
    );

  const sendButtonEl = showSendButton ? (
    <button
      type="button"
      onClick={send}
      disabled={interactionDisabled || !mergedMessage.trim()}
      aria-label="Send"
      className={
        floating
          ? 'flex h-9 w-9 shrink-0 items-center justify-center rounded-full p-0 text-zinc-950 transition hover:brightness-110 focus:outline-none focus-visible:ring-2 focus-visible:ring-[#ff6b00]/45 disabled:pointer-events-none disabled:opacity-40'
          : `shrink-0 rounded-full bg-[#ff6b00] px-3 py-2 text-sm font-medium text-zinc-950 hover:bg-[#ff8f40] disabled:pointer-events-none disabled:opacity-50 ${
              mergedMessage.trim() ? 'opacity-100' : 'opacity-0'
            }`
      }
      style={floating ? { backgroundColor: uiTheme.brandHex } : undefined}
    >
      {sending ? (
        <span className="text-xs leading-none" aria-hidden>
          …
        </span>
      ) : (
        <SendArrowIcon size={floating ? 14 : 16} strokeWidth={2.2} className={floating ? 'text-white' : undefined} />
      )}
    </button>
  ) : null;

  const attachmentButtonEl = floating ? (
    <div ref={attachmentMenuRef} className="relative flex shrink-0 items-center">
      <button
        type="button"
        aria-label="Add attachment"
        aria-haspopup="menu"
        aria-expanded={attachmentMenuOpen}
        onClick={() => {
          if (disabled) {
            return;
          }
          setAttachmentMenuOpen((open) => !open);
        }}
        disabled={disabled}
        className="relative flex h-9 w-9 shrink-0 items-center justify-center rounded-full border border-[var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel)]/90 text-[var(--vel-color-accent-soft)] transition hover:border-[var(--vel-color-accent)] hover:text-[var(--vel-color-text)]"
      >
        <PlusIcon size={15} className="block" />
      </button>
      <input
        ref={fileInputRef}
        type="file"
        className="hidden"
        multiple
        onChange={(event) => {
          enqueueAttachments(event.target.files, 'file');
          event.currentTarget.value = '';
        }}
      />
      <input
        ref={imageInputRef}
        type="file"
        className="hidden"
        multiple
        accept="image/*"
        onChange={(event) => {
          enqueueAttachments(event.target.files, 'image');
          event.currentTarget.value = '';
        }}
      />
      {attachmentMenuOpen ? (
        <div
          role="menu"
          aria-label="Attachment types"
          className="absolute bottom-full left-0 z-20 mb-2 min-w-[8rem] rounded-2xl border border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)]/95 p-1.5 shadow-[0_16px_50px_rgba(0,0,0,0.45)] backdrop-blur"
        >
          <button
            type="button"
            role="menuitem"
            onClick={() => selectIntent('task')}
            className="flex w-full items-center gap-2 rounded-xl px-3 py-2 text-left text-xs uppercase tracking-[0.14em] text-[var(--vel-color-text)] transition hover:bg-white/5"
          >
            <ClipboardCheckIcon size={13} />
            <span>Task</span>
          </button>
          <button
            type="button"
            role="menuitem"
            onClick={() => fileInputRef.current?.click()}
            className="flex w-full items-center gap-2 rounded-xl px-3 py-2 text-left text-xs uppercase tracking-[0.14em] text-[var(--vel-color-text)] transition hover:bg-white/5"
          >
            <FileIcon size={13} />
            <span>File</span>
          </button>
          <button
            type="button"
            role="menuitem"
            onClick={() => imageInputRef.current?.click()}
            className="flex w-full items-center gap-2 rounded-xl px-3 py-2 text-left text-xs uppercase tracking-[0.14em] text-[var(--vel-color-text)] transition hover:bg-white/5"
          >
            <ImageIcon size={13} />
            <span>Image</span>
          </button>
        </div>
      ) : null}
    </div>
  ) : null;

  return (
    <div className={`${floating ? `fixed inset-x-0 z-30 px-4 ${floatingOffsetClassName}` : 'shrink-0 border-t border-zinc-800 p-3'}`}>
      {displayError && (
        <p className="mx-auto mb-1.5 max-w-2xl text-xs text-red-400" role="alert">
          {displayError}
        </p>
      )}
      <div className={cn('relative mx-auto flex max-w-2xl gap-2', floating ? 'group w-full items-center' : 'items-end')}>
        {disabled ? (
          <button
            type="button"
            aria-label={_disabledReason ?? 'Composer disabled'}
            className={cn(
              'absolute inset-0 z-20 cursor-not-allowed rounded-full',
              floating ? null : 'rounded-[1.75rem]',
            )}
            onClick={() => onDisabledInteract?.()}
          />
        ) : null}
        {floating ? (
          <div className={cn('vel-composer-gradient-border min-w-0 w-full flex-1 rounded-full p-px transition-[opacity,filter] duration-150', disabled ? 'opacity-45 saturate-0 grayscale' : null)}>
          <div className={cn('flex items-center gap-2 rounded-full bg-zinc-950/95 py-2 pl-2 pr-2 backdrop-blur transition-[opacity,filter] duration-150', disabled ? 'opacity-80 saturate-0 grayscale' : null)}>
            {attachmentButtonEl}
            {queuedAttachments.length ? (
              <div className="flex max-w-[14rem] shrink-0 items-center gap-1 overflow-x-auto py-0.5">
                {queuedAttachments.map((attachment) => (
                  <span
                    key={attachment.id}
                    className="inline-flex shrink-0 items-center gap-1 rounded-full border border-[var(--vel-color-border)] bg-white/5 px-2 py-1 text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]"
                  >
                    {attachment.kind === 'image' ? <ImageIcon size={10} /> : <FileIcon size={10} />}
                    <span className="max-w-[6rem] truncate">{attachment.name}</span>
                    <button
                      type="button"
                      aria-label={`Remove ${attachment.name}`}
                      onClick={() => removeQueuedAttachment(attachment.id)}
                    className="inline-flex h-3.5 w-3.5 items-center justify-center rounded-full text-current/80 transition hover:bg-white/10 hover:text-[var(--vel-color-text)]"
                    >
                      <CloseIcon size={10} />
                    </button>
                  </span>
                ))}
              </div>
            ) : null}
            {selectedIntent ? (
              <span
                className="inline-flex shrink-0 items-center gap-1 rounded-full border border-[var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel)]/88 px-2 py-1 text-[10px] font-medium uppercase tracking-[0.14em] text-[var(--vel-color-accent-soft)]"
              >
                <ClipboardCheckIcon size={10} />
                <span>{selectedIntent}</span>
                <button
                  type="button"
                  aria-label={`Clear ${selectedIntent} intent`}
                  onClick={() => setSelectedIntent(null)}
                  className="inline-flex h-3.5 w-3.5 items-center justify-center rounded-full text-current/80 transition hover:bg-white/10 hover:text-[var(--vel-color-text)]"
                >
                  <CloseIcon size={10} />
                </button>
              </span>
            ) : null}
            <textarea
              ref={textareaRef}
              value={text}
              onChange={(e) => setText(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Ask, capture, or talk to Vel…"
              data-vel-composer-input="true"
              rows={1}
              className="vel-composer-floating-textarea min-w-0 flex-1 resize-none border-0 bg-transparent py-0 text-[13px] leading-6 focus:outline-none focus:ring-0 disabled:opacity-60"
              disabled={interactionDisabled}
              style={{ scrollbarWidth: 'none', height: '1.5rem', maxHeight: '1.5rem' }}
            />
            {voiceStatusLabel ? (
              <span
                className={cn(
                  'inline-flex shrink-0 items-center gap-1 rounded-full border px-2 py-1 text-[10px] font-medium uppercase tracking-[0.14em]',
                  isListening
                    ? 'border-red-700/55 bg-red-950/35 text-red-200'
                    : 'border-[var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel)]/88 text-[var(--vel-color-accent-soft)]',
                )}
              >
                <MicIcon size={11} />
                <span>{voiceStatusLabel}</span>
                {isListening ? (
                  <span className="font-mono tabular-nums text-[9px] text-current/80">
                    {formatVoiceClock(voiceElapsedMs)}
                  </span>
                ) : null}
                {pendingVoice && !isListening ? (
                  <button
                    type="button"
                    aria-label="Clear recorded voice draft"
                    onClick={() => {
                      pendingVoiceRef.current = null;
                      setPendingVoice(null);
                    }}
                    className="inline-flex h-3.5 w-3.5 items-center justify-center rounded-full text-current/80 transition hover:bg-white/10 hover:text-[var(--vel-color-text)]"
                  >
                    <CloseIcon size={10} />
                  </button>
                ) : null}
              </span>
            ) : null}
            {interimTranscript ? (
              <span className="max-w-[10rem] truncate text-[10px] uppercase tracking-[0.12em] text-[var(--vel-color-muted)]">
                {interimTranscript}
              </span>
            ) : null}
            <div className="flex shrink-0 items-center gap-1">
              {micButtonEl}
              {sendButtonEl}
            </div>
          </div>
          </div>
        ) : (
          <>
            <div className={cn('flex min-w-0 flex-1 flex-col gap-1 transition-[opacity,filter] duration-150', disabled ? 'opacity-45 saturate-0 grayscale' : null)}>
              <textarea
                value={text}
                onChange={(e) => setText(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Ask, capture, or talk to Vel… (Enter to send, Shift+Enter for newline)"
                rows={1}
                className="w-full resize-none rounded-full border-0 bg-zinc-800/30 px-4 py-2 text-zinc-100 placeholder-zinc-500 transition-colors duration-150 focus:bg-zinc-700/55 focus:outline-none"
              disabled={interactionDisabled}
                style={{ scrollbarWidth: 'none' }}
              />
              {interimTranscript && !hideHelperText ? (
                <p className="text-zinc-500 text-xs" aria-live="polite">
                  Listening locally… {interimTranscript}
                </p>
              ) : null}
              {!interimTranscript && !hideHelperText && !interactionDisabled ? (
                <p className="text-zinc-500 text-xs" aria-live="polite">
                  {voiceHint}
                </p>
              ) : null}
            </div>
          </>
        )}
        {!floating && micButtonEl}
        {!floating && sendButtonEl}
      </div>
    </div>
  );
}
