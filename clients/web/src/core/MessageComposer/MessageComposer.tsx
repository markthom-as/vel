import { useRef, useState, useCallback } from 'react';
import { submitAssistantEntry } from '../../data/chat';
import { useSpeechRecognition } from '../../hooks/useSpeechRecognition';
import {
  type AssistantEntryResponse,
  type AssistantEntryVoiceProvenanceData,
} from '../../types';
import { MicIcon, SendArrowIcon } from '../Icons';
import { uiTheme } from '../Theme';

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
  compact = false,
  hideHelperText = false,
  floating = false,
}: MessageComposerProps) {
  const [text, setText] = useState('');
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pendingVoice, setPendingVoice] = useState<AssistantEntryVoiceProvenanceData | null>(null);
  const voicePressActiveRef = useRef(false);

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

  const send = useCallback(async () => {
    const trimmed = text.trim();
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
  }, [text, conversationId, sending, pendingVoice, onOptimisticSend, onSent, onSendFailed]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  const beginVoiceCapture = () => {
    if (sending || !voiceSupported || voicePressActiveRef.current) {
      return;
    }
    if (voiceError) setError(null);
    voicePressActiveRef.current = true;
    startVoice();
  };

  const endVoiceCapture = () => {
    if (!voicePressActiveRef.current) {
      return;
    }
    voicePressActiveRef.current = false;
    stopVoice();
  };

  const handleVoiceKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.repeat) {
      return;
    }
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      beginVoiceCapture();
    }
  };

  const handleVoiceKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      endVoiceCapture();
    }
  };

  const displayError = error ?? voiceError ?? null;
  const voiceHint = voiceSupported
    ? isListening
      ? 'Release to stop. Transcript stays local until you send it.'
      : 'Hold the mic to talk locally. Vel routes the transcript into Now, Inbox, or Threads.'
    : 'Local speech-to-text is not available in this browser yet. Type your message instead.';

  return (
    <div className={`${floating ? 'fixed inset-x-0 bottom-10 z-30 px-4' : 'shrink-0 border-t border-zinc-800 p-3'}`}>
      {displayError && (
        <p className="mx-auto mb-2 max-w-2xl text-sm text-red-400" role="alert">
          {displayError}
        </p>
      )}
      <div
        className={`mx-auto flex max-w-2xl items-end gap-2 ${
          floating ? `rounded-full border-2 ${uiTheme.brandBorder} bg-zinc-950/95 p-3 shadow-[0_24px_60px_rgba(0,0,0,0.55)] backdrop-blur` : ''
        }`}
      >
        <div className="flex-1 flex flex-col gap-1">
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask, capture, or talk to Vel… (Enter to send, Shift+Enter for newline)"
            rows={1}
            className="w-full resize-none rounded-full border border-transparent bg-zinc-800/30 px-4 py-2 text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-[#ff6b00]/35"
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
        {voiceSupported && (
          <button
            type="button"
            onMouseDown={beginVoiceCapture}
            onMouseUp={endVoiceCapture}
            onMouseLeave={endVoiceCapture}
            onTouchStart={beginVoiceCapture}
            onTouchEnd={endVoiceCapture}
            onTouchCancel={endVoiceCapture}
            onKeyDown={handleVoiceKeyDown}
            onKeyUp={handleVoiceKeyUp}
            disabled={sending}
            aria-pressed={isListening}
            aria-label={isListening ? 'Release to stop local voice input' : 'Hold to talk locally'}
            title={isListening ? 'Release to stop local voice input' : 'Hold to talk locally with browser speech-to-text'}
            className={`shrink-0 rounded-full border px-3 py-2 text-xs uppercase tracking-[0.18em] transition-colors focus:outline-none focus:ring-2 focus:ring-[#ff6b00]/35 disabled:pointer-events-none disabled:opacity-50 ${
              isListening
                ? 'bg-red-900/40 border-red-600/60 text-red-200'
                : 'bg-zinc-800/50 border-zinc-700 text-zinc-300 hover:bg-zinc-700/50 hover:text-zinc-100'
            }`}
          >
            <MicIcon listening={isListening} />
          </button>
        )}
        <button
          type="button"
          onClick={send}
          disabled={sending || !text.trim()}
          aria-label="Send"
          className={`shrink-0 rounded-full bg-[#ff6b00] px-3 py-2 text-sm font-medium text-zinc-950 hover:bg-[#ff8f40] disabled:pointer-events-none disabled:opacity-50 ${
            text.trim() ? 'opacity-100' : 'opacity-0'
          }`}
        >
          {sending ? '…' : <SendArrowIcon size={16} strokeWidth={2.2} />}
        </button>
      </div>
    </div>
  );
}
