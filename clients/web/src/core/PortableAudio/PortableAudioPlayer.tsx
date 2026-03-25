import { useEffect, useRef, useState } from 'react';
import { cn } from '../cn';

interface PortableAudioPlayerProps {
  src: string;
  title?: string | null;
  className?: string;
}

type SpeechRecognitionWindow = Window & {
  SpeechRecognition?: { new (): SpeechRecognition };
  webkitSpeechRecognition?: { new (): SpeechRecognition };
};

function formatSeconds(totalSeconds: number): string {
  const total = Math.max(0, Math.floor(totalSeconds));
  const minutes = Math.floor(total / 60);
  const seconds = total % 60;
  return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}

function hasSpeechRecognitionSupport(): boolean {
  if (typeof window === 'undefined') {
    return false;
  }

  const root = window as SpeechRecognitionWindow;
  return !!(root.SpeechRecognition || root.webkitSpeechRecognition);
}

export function PortableAudioPlayer({ src, title, className }: PortableAudioPlayerProps) {
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const recognitionRef = useRef<SpeechRecognition | null>(null);
  const [duration, setDuration] = useState(0);
  const [position, setPosition] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [isScrubbing, setIsScrubbing] = useState(false);
  const [isTranscribing, setIsTranscribing] = useState(false);
  const [speechStatus, setSpeechStatus] = useState('STT');
  const [speechTranscript, setSpeechTranscript] = useState('');
  const speechSupported = hasSpeechRecognitionSupport();

  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) {
      return;
    }

    const onMetadata = () => setDuration(audio.duration || 0);
    const onTimeUpdate = () => {
      if (!isScrubbing) {
        setPosition(audio.currentTime);
      }
    };
    const onPlay = () => setIsPlaying(true);
    const onPause = () => setIsPlaying(false);
    const onEnded = () => setIsPlaying(false);

    audio.addEventListener('loadedmetadata', onMetadata);
    audio.addEventListener('timeupdate', onTimeUpdate);
    audio.addEventListener('play', onPlay);
    audio.addEventListener('pause', onPause);
    audio.addEventListener('ended', onEnded);

    return () => {
      audio.removeEventListener('loadedmetadata', onMetadata);
      audio.removeEventListener('timeupdate', onTimeUpdate);
      audio.removeEventListener('play', onPlay);
      audio.removeEventListener('pause', onPause);
      audio.removeEventListener('ended', onEnded);
    };
  }, [isScrubbing]);

  useEffect(() => () => {
    recognitionRef.current?.stop();
  }, []);

  async function handlePlayToggle() {
    const audio = audioRef.current;
    if (!audio) {
      return;
    }

    if (audio.paused) {
      try {
        await audio.play();
      } catch {
        // Autoplay restrictions and gesture requirements are expected in browser contexts.
      }
    } else {
      audio.pause();
    }
  }

  function handleScrubStart() {
    setIsScrubbing(true);
  }

  function handleScrubMove(value: string) {
    const parsed = Number(value);
    if (Number.isNaN(parsed)) {
      return;
    }
    setPosition(parsed);
    const audio = audioRef.current;
    if (audio) {
      audio.currentTime = parsed;
    }
  }

  function handleScrubEnd() {
    setIsScrubbing(false);
  }

  function startTranscription() {
    if (!speechSupported) {
      setSpeechStatus('STT unsupported');
      return;
    }

    const root = window as SpeechRecognitionWindow;
    const Recognition = root.SpeechRecognition ?? root.webkitSpeechRecognition;
    if (!Recognition) {
      setSpeechStatus('STT unsupported');
      return;
    }

    const recognition = new Recognition();
    recognitionRef.current = recognition;
    recognition.continuous = true;
    recognition.interimResults = true;

    recognition.onstart = () => {
      setIsTranscribing(true);
      setSpeechStatus('Listening…');
      setSpeechTranscript('');
    };

    recognition.onresult = (event: SpeechRecognitionEvent) => {
      let committed = '';
      for (let i = event.resultIndex; i < event.results.length; i += 1) {
        const result = event.results[i];
        const text = result?.[0]?.transcript ?? '';
        if (result.isFinal) {
          committed += text;
        }
      }
      if (committed) {
        setSpeechTranscript((current) => `${current}${committed}`.trim());
        setSpeechStatus('STT: captured');
      }
    };

    recognition.onerror = (event: SpeechRecognitionErrorEvent) => {
      setIsTranscribing(false);
      setSpeechStatus(event.message || event.error || 'Speech recognition error');
      recognitionRef.current = null;
    };

    recognition.onend = () => {
      setIsTranscribing(false);
      if (!speechTranscript) {
        setSpeechStatus('STT');
      }
      recognitionRef.current = null;
    };

    try {
      recognition.start();
    } catch {
      setSpeechStatus('Unable to start STT');
    }
  }

  function stopTranscription() {
    recognitionRef.current?.stop();
    setIsTranscribing(false);
    setSpeechStatus('STT');
  }

  if (!src.trim()) {
    return null;
  }

  const label = title?.trim() ?? 'Audio attachment';

  return (
    <div className={cn('w-full rounded-lg border border-zinc-800 bg-zinc-950 p-3', className)}>
      <p className="mb-2 text-xs uppercase tracking-[0.16em] text-zinc-400">{label}</p>
      <audio
        ref={audioRef}
        src={src}
        preload="metadata"
        controls={false}
        className="sr-only"
        aria-label={label}
        data-testid="portable-audio-player"
      />
      <div className="space-y-3">
        <div className="grid gap-2">
          <div className="flex items-center justify-between text-[11px] text-zinc-300">
            <span>{formatSeconds(position)}</span>
            <span>{formatSeconds(duration)}</span>
          </div>
          <input
            aria-label={`Audio scrubber for ${label}`}
            type="range"
            min={0}
            max={duration || 0}
            step="0.1"
            value={position}
            onMouseDown={handleScrubStart}
            onChange={(event) => handleScrubMove(event.currentTarget.value)}
            onMouseUp={handleScrubEnd}
            onTouchStart={handleScrubStart}
            onTouchEnd={handleScrubEnd}
            className="h-2 w-full cursor-pointer appearance-none bg-zinc-800"
          />
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={() => void handlePlayToggle()}
              className="rounded-full border border-zinc-700 bg-zinc-900 px-3 py-1 text-xs font-semibold text-zinc-100 transition hover:bg-zinc-800"
            >
              {isPlaying ? 'Pause' : 'Play'}
            </button>
            <button
              type="button"
              onClick={isTranscribing ? stopTranscription : startTranscription}
              disabled={speechSupported === false && !isTranscribing}
              className="rounded-full border border-zinc-700 bg-zinc-900 px-3 py-1 text-xs font-semibold text-zinc-100 transition hover:bg-zinc-800 disabled:cursor-not-allowed disabled:opacity-60"
            >
              {isTranscribing ? 'Stop STT' : speechStatus}
            </button>
          </div>
          {speechTranscript ? (
            <div className="rounded border border-zinc-700 bg-black/40 p-2 text-xs text-zinc-300">
              <span className="font-semibold text-zinc-200">Transcript:</span> {speechTranscript}
            </div>
          ) : null}
          {speechStatus !== 'STT' && speechStatus !== 'STT: captured' && !speechTranscript ? (
            <p className="text-[11px] text-zinc-400">{speechStatus}</p>
          ) : null}
        </div>
      </div>
    </div>
  );
}
