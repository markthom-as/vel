import { useState, useCallback, useRef, useEffect } from 'react';

export interface UseSpeechRecognitionOptions {
  /** Called with final transcript segments (committed text). */
  onResult?: (transcript: string) => void;
  /** Called with interim transcript for live feedback (optional). */
  onInterim?: (transcript: string) => void;
  /** Language code, e.g. 'en-US'. */
  lang?: string;
  /** Keep listening across multiple utterances. */
  continuous?: boolean;
}

export interface UseSpeechRecognitionReturn {
  isSupported: boolean;
  isListening: boolean;
  error: string | null;
  start: () => void;
  stop: () => void;
  /** Latest interim transcript while listening. */
  interimTranscript: string;
}

function getSpeechRecognition(): typeof SpeechRecognition | null {
  if (typeof window === 'undefined') return null;
  return window.SpeechRecognition ?? window.webkitSpeechRecognition ?? null;
}

export function useSpeechRecognition(
  options: UseSpeechRecognitionOptions = {}
): UseSpeechRecognitionReturn {
  const { onResult, onInterim, lang = 'en-US', continuous = true } = options;
  const [isListening, setIsListening] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [interimTranscript, setInterimTranscript] = useState('');
  const recognitionRef = useRef<SpeechRecognition | null>(null);

  const isSupported = !!getSpeechRecognition();

  const stop = useCallback(() => {
    const rec = recognitionRef.current;
    if (rec) {
      try {
        rec.abort();
      } catch {
        try {
          rec.stop();
        } catch {
          // ignore
        }
      }
      recognitionRef.current = null;
    }
    setIsListening(false);
    setInterimTranscript('');
  }, []);

  const start = useCallback(() => {
    if (!isSupported) {
      setError('Voice input is not supported in this browser.');
      return;
    }
    setError(null);
    const Klass = getSpeechRecognition();
    if (!Klass) return;
    const rec = new Klass();
    recognitionRef.current = rec;
    rec.continuous = continuous;
    rec.interimResults = true;
    rec.lang = lang;

    rec.onresult = (event: SpeechRecognitionEvent) => {
      let interim = '';
      let finalText = '';
      for (let i = event.resultIndex; i < event.results.length; i++) {
        const result = event.results[i];
        const transcript = result[0]?.transcript ?? '';
        if (result.isFinal) {
          finalText += transcript;
        } else {
          interim += transcript;
        }
      }
      if (finalText) onResult?.(finalText);
      if (interim) {
        setInterimTranscript(interim);
        onInterim?.(interim);
      }
    };

    rec.onend = () => {
      setInterimTranscript('');
      if (recognitionRef.current === rec) {
        setIsListening(false);
        recognitionRef.current = null;
      }
    };

    rec.onerror = (e: SpeechRecognitionErrorEvent) => {
      if (e.error === 'aborted') return;
      setError(e.message ?? e.error ?? 'Speech recognition error');
      setIsListening(false);
      recognitionRef.current = null;
    };

    rec.onstart = () => setIsListening(true);

    try {
      rec.start();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not start microphone');
      recognitionRef.current = null;
    }
  }, [isSupported, continuous, lang, onResult, onInterim]);

  useEffect(() => {
    return () => {
      const rec = recognitionRef.current;
      if (rec) {
        try {
          rec.abort();
        } catch {
          // ignore
        }
        recognitionRef.current = null;
      }
    };
  }, []);

  return { isSupported, isListening, error, start, stop, interimTranscript };
}
