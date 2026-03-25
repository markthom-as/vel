import { useEffect, useRef, useState } from 'react';

import 'video.js/dist/video-js.css';
import { cn } from '../cn';

interface PortableVideoPlayerProps {
  src: string;
  title?: string | null;
  mimeType?: string | null;
  poster?: string | null;
  className?: string;
}

type VideoJsFactory = (
  element: HTMLVideoElement,
  options: unknown,
  ready?: () => void,
) => VideoJsPlayer;

type VideoJsPlayer = {
  dispose?: () => void;
  destroy?: () => void;
};

function resolveVideoJsFactory(module: unknown): VideoJsFactory | null {
  if (typeof module === 'function') {
    return module as VideoJsFactory;
  }

  const record = module as {
    default?: unknown;
    videojs?: unknown;
    core?: {
      Video?: unknown;
    };
  };

  if (typeof record.default === 'function') {
    return record.default as VideoJsFactory;
  }
  if (typeof record.videojs === 'function') {
    return record.videojs as VideoJsFactory;
  }
  if (typeof record.core?.Video === 'function') {
    return record.core.Video as VideoJsFactory;
  }
  return null;
}

function disposeVideoPlayer(player: VideoJsPlayer | null) {
  if (!player) {
    return;
  }
  player.dispose?.();
  player.destroy?.();
}

export function PortableVideoPlayer({
  src,
  title,
  mimeType,
  poster,
  className,
}: PortableVideoPlayerProps) {
  const source = src.trim();
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const [isFallback, setIsFallback] = useState(false);

  useEffect(() => {
    if (!source || !videoRef.current) {
      return;
    }

    let disposed = false;
    let player: VideoJsPlayer | null = null;
    setIsFallback(false);

    void (async () => {
      try {
        const videoJsModule = await import('video.js');
        const factory = resolveVideoJsFactory(videoJsModule);
        if (!factory) {
          throw new Error('Unsupported video.js API shape');
        }

        if (disposed || !videoRef.current) {
          return;
        }

        player = factory(
          videoRef.current,
          {
            controls: true,
            autoplay: false,
            preload: 'auto',
            fluid: true,
            responsive: true,
            playsinline: true,
            poster: poster?.trim() || undefined,
            sources: [
              {
                src: source,
                type: mimeType?.trim() || undefined,
              },
            ],
          },
        );
        setIsFallback(false);
      } catch {
        if (!disposed) {
          setIsFallback(true);
        }
      }
    })();

    return () => {
      disposed = true;
      disposeVideoPlayer(player);
    };
  }, [source, mimeType, poster]);

  if (!source) {
    return null;
  }

  return (
    <div className={cn('w-full rounded-lg border border-zinc-800 bg-zinc-950', className)}>
      {isFallback ? (
        <video
          ref={videoRef}
          controls
          playsInline
          preload="auto"
          src={source}
          poster={poster?.trim() || undefined}
          className="w-full rounded-lg object-cover"
          aria-label={title ?? 'Video attachment'}
          data-testid="portable-video-player"
        />
      ) : (
        <video
          ref={videoRef}
          className="video-js vjs-big-play-centered"
          controls
          playsInline
          preload="auto"
          aria-label={title ?? 'Video attachment'}
          data-testid="portable-video-player"
        />
      )}
    </div>
  );
}
