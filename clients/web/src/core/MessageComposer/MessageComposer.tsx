import { useRef, useState, useCallback, useMemo, useEffect } from 'react';
import { submitAssistantEntry } from '../../data/chat';
import { useSpeechRecognition } from '../../hooks/useSpeechRecognition';
import {
  type AssistantEntryResponse,
  type AssistantEntryVoiceProvenanceData,
} from '../../types';
import { cn } from '../cn';
import { MicIcon, SendArrowIcon } from '../Icons';
import { uiTheme } from '../Theme';

/** Browser STT session cap — pie + auto-stop use this as the full ring. */
const FLOATING_VOICE_MAX_MS = 120_000;

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
  onSent: (clientMessageId: string | undefined, response: AssistantEntryResponse) => void;
  onSendFailed?: (clientMessageId: string | undefined) => void;
  compact?: boolean;
  hideHelperText?: boolean;
  floating?: boolean;
}

export function MessageComposer({
  conversationId,
  onOptimisticSend,
  onSent,
  onSendFailed,
  compact: _compact = false,
  hideHelperText = false,
  floating = false,
}: MessageComposerProps) {
  const [text, setText] = useState('');
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pendingVoice, setPendingVoice] = useState<AssistantEntryVoiceProvenanceData | null>(null);
  const voicePressActiveRef = useRef(false);
  const voiceLatchedRef = useRef(false);
  /** Counts browser STT `onend` auto-restarts per press/latch (avoids tight loops if start fails). */
  const voiceSttResumeCountRef = useRef(0);
  const voicePrevIsListeningRef = useRef(false);
  const [voiceElapsedMs, setVoiceElapsedMs] = useState(0);
  const [voiceLatched, setVoiceLatched] = useState(false);

  const appendVoiceTranscript = useCallback((transcript: string) => {
    setText((prev) => (prev ? `${prev} ${transcript}` : transcript));
    setPendingVoice({
      surface: 'web',
      source_device: 'browser',
      locale: typeof navigator === 'undefined' ? 'en-US' : navigator.language ?? 'en-US',
      transcript_origin: 'local_browser_stt',
      recorded_at: new Date().toISOString(),
    });
  }, []);

  const {
    isSupported: voiceSupported,
    isListening,
    error: voiceError,
    start: startVoice,
    stop: stopVoice,
    interimTranscript,
  } = useSpeechRecognition({
    onResult: appendVoiceTranscript,
    continuous: true,
  });

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

  /** Text committed in the textarea plus any in-flight STT (not yet appended to `text`). */
  const mergedMessage = useMemo(() => {
    const t = text.trim();
    const i = interimTranscript.trim();
    if (t && i) return `${t} ${i}`;
    return t || i;
  }, [text, interimTranscript]);

  const hasSendablePayload = mergedMessage.trim().length > 0;

  const send = useCallback(async () => {
    const trimmed = mergedMessage.trim();
    if (!trimmed || sending) return;
    setError(null);
    setSending(true);
    const clientMessageId = onOptimisticSend?.(trimmed);
    try {
      const res = await submitAssistantEntry(trimmed, conversationId, pendingVoice);
      if (res.ok && res.data) {
        onSent(clientMessageId, res.data);
        setText('');
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
  }, [mergedMessage, conversationId, sending, pendingVoice, onOptimisticSend, onSent, onSendFailed]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  const beginVoiceCapture = (): boolean => {
    if (sending || !voiceSupported || voicePressActiveRef.current) {
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
    if (sending || !voiceSupported) return;
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
    if (sending || !voiceSupported) return;
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
    if (sending || !voiceSupported) return;
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

  /**
   * Floating row: gray shell ends at the round control midpoint (not under full mic/send).
   * `gap-2` (0.5rem) + overlay `pr-1` (0.25rem) + send 2.75rem + gap 0.375rem + half mic ~1.5rem; mic-only / send-only variants.
   */
  const floatingTextShellClass = useMemo(() => {
    if (!floating) return '';
    if (voiceSupported && showSendButton) {
      return 'max-w-[calc(100%-(0.5rem+0.25rem+2.75rem+0.375rem+1.5rem))]';
    }
    if (voiceSupported) {
      return 'max-w-[calc(100%-(0.5rem+0.25rem+1.5rem))]';
    }
    if (showSendButton) {
      return 'max-w-[calc(100%-(0.5rem+0.25rem+1.375rem))]';
    }
    return '';
  }, [floating, voiceSupported, showSendButton]);

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

  const micButtonEl = voiceSupported ? (
    floating ? (
      <div className={`flex items-center ${isListening ? 'gap-1.5' : ''}`}>
        {isListening ? (
          <span
            className="min-w-[4.75rem] text-right font-mono text-[11px] leading-tight tabular-nums text-zinc-300"
            aria-live="polite"
            aria-label={`Recording ${formatVoiceClock(voiceElapsedMs)} of ${formatVoiceClock(FLOATING_VOICE_MAX_MS)}`}
          >
            <span className="text-zinc-100">{formatVoiceClock(voiceElapsedMs)}</span>
            <span className="text-zinc-500"> / </span>
            <span className="text-zinc-500">{formatVoiceClock(FLOATING_VOICE_MAX_MS)}</span>
          </span>
        ) : null}
        <div
          className={cn(
            'vel-composer-mic-shell vel-composer-gradient-border shrink-0 rounded-full p-[2px]',
            isListening && 'vel-composer-mic--recording',
          )}
        >
          <button
            type="button"
            onPointerDown={handleMicPointerDown}
            onPointerUp={handleMicPointerUp}
            onPointerCancel={handleMicPointerUp}
            onDoubleClick={handleMicDoubleClick}
            onKeyDown={handleVoiceKeyDown}
            onKeyUp={handleVoiceKeyUp}
            disabled={sending}
            aria-pressed={isListening}
            aria-label={micAriaLabel}
            title={micTitle}
            className={cn(
              'relative flex h-11 w-11 shrink-0 items-center justify-center overflow-hidden rounded-full border-0 transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-[#ff6b00]/45 disabled:pointer-events-none disabled:opacity-50',
              isListening
                ? 'bg-zinc-950/90 text-white shadow-[inset_0_0_0_1px_rgba(248,113,113,0.25)]'
                : 'vel-brand-shimmer-surface vel-composer-mic-fill text-white hover:brightness-110',
            )}
          >
            {isListening ? (
              <FloatingMicPieRing progress={voiceProgress} warn={voiceProgress >= 0.85} />
            ) : null}
            <span className="relative z-10 flex items-center justify-center">
              <MicIcon listening={isListening} size={20} className="text-white" />
            </span>
          </button>
        </div>
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
        disabled={sending}
        aria-pressed={isListening}
        aria-label={micAriaLabel}
        title={micTitle}
        className={`shrink-0 rounded-full border px-3 py-2 text-xs uppercase tracking-[0.18em] transition-colors focus:outline-none focus:ring-2 focus:ring-[#ff6b00]/35 disabled:pointer-events-none disabled:opacity-50 ${
          isListening
            ? 'bg-red-900/40 border-red-600/60 text-red-200'
            : 'bg-zinc-800/50 border-zinc-700 text-zinc-300 hover:bg-zinc-700/50 hover:text-zinc-100'
        }`}
      >
        <MicIcon listening={isListening} />
      </button>
    )
  ) : null;

  const sendButtonEl = showSendButton ? (
    <button
      type="button"
      onClick={send}
      disabled={sending || !mergedMessage.trim()}
      aria-label="Send"
      className={
        floating
          ? 'flex h-11 w-11 shrink-0 items-center justify-center rounded-full border-2 border-[#ff8f40]/90 p-0 text-white transition hover:brightness-110 focus:outline-none focus-visible:ring-2 focus-visible:ring-[#ff6b00]/45 disabled:pointer-events-none disabled:opacity-40'
          : `shrink-0 rounded-full bg-[#ff6b00] px-3 py-2 text-sm font-medium text-zinc-950 hover:bg-[#ff8f40] disabled:pointer-events-none disabled:opacity-50 ${
              mergedMessage.trim() ? 'opacity-100' : 'opacity-0'
            }`
      }
      style={floating ? { backgroundColor: uiTheme.brandHex } : undefined}
    >
      {sending ? (
        <span className="text-lg leading-none" aria-hidden>
          …
        </span>
      ) : (
        <SendArrowIcon size={floating ? 18 : 16} strokeWidth={2.2} className={floating ? 'text-white' : undefined} />
      )}
    </button>
  ) : null;

  return (
    <div className={`${floating ? 'fixed inset-x-0 bottom-10 z-30 px-4' : 'shrink-0 border-t border-zinc-800 p-3'}`}>
      {displayError && (
        <p className="mx-auto mb-2 max-w-2xl text-sm text-red-400" role="alert">
          {displayError}
        </p>
      )}
      <div className={cn('mx-auto flex max-w-2xl gap-2', floating ? 'group w-full items-center' : 'items-end')}>
        {floating ? (
          <div className="vel-composer-gradient-border min-w-0 w-full flex-1 rounded-full p-[2px]">
            <div className="rounded-full bg-zinc-950/95 p-3 shadow-[0_24px_60px_rgba(0,0,0,0.55)] backdrop-blur">
              <div className="flex min-w-0 flex-1 flex-col gap-1">
                <div className="flex min-w-0 w-full items-center gap-2 pl-4 pr-2">
                  <div
                    className={cn(
                      'flex min-h-[2.75rem] min-w-0 flex-1 items-center rounded-l-full rounded-r-none py-2 pr-2',
                      floatingTextShellClass,
                    )}
                  >
                    <textarea
                      value={text}
                      onChange={(e) => setText(e.target.value)}
                      onKeyDown={handleKeyDown}
                      placeholder="Ask, capture, or talk to Vel… (Enter to send, Shift+Enter for newline)"
                      rows={1}
                      className="vel-composer-floating-textarea min-h-[2.75rem] w-full max-h-40 flex-1 resize-none border-0 bg-transparent px-2 py-2 leading-snug focus:outline-none focus:ring-0 disabled:opacity-50"
                      disabled={sending}
                      style={{ scrollbarWidth: 'none' }}
                    />
                  </div>
                  <div className="flex shrink-0 items-center gap-1.5 pr-1">
                    {micButtonEl}
                    {sendButtonEl}
                  </div>
                </div>
                {interimTranscript && !hideHelperText ? (
                  <p className="text-zinc-500 text-xs opacity-40" aria-live="polite">
                    Listening locally… {interimTranscript}
                  </p>
                ) : null}
                {!interimTranscript && !hideHelperText ? (
                  <p className="text-zinc-500 text-xs opacity-40" aria-live="polite">
                    {voiceHint}
                  </p>
                ) : null}
              </div>
            </div>
          </div>
        ) : (
          <>
            <div className="flex min-w-0 flex-1 flex-col gap-1">
              <textarea
                value={text}
                onChange={(e) => setText(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Ask, capture, or talk to Vel… (Enter to send, Shift+Enter for newline)"
                rows={1}
                className="w-full resize-none rounded-full border-0 bg-zinc-800/30 px-4 py-2 text-zinc-100 placeholder-zinc-500 transition-colors duration-150 focus:bg-zinc-700/55 focus:outline-none"
                disabled={sending}
                style={{ scrollbarWidth: 'none' }}
              />
              {interimTranscript && !hideHelperText ? (
                <p className="text-zinc-500 text-xs" aria-live="polite">
                  Listening locally… {interimTranscript}
                </p>
              ) : null}
              {!interimTranscript && !hideHelperText ? (
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
