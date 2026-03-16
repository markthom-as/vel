import { useEffect, useState } from 'react';
import { apiGet } from '../api/client';

interface ContextCurrent {
  ok: boolean;
  data?: Record<string, unknown>;
}

export function ContextPanel() {
  const [context, setContext] = useState<Record<string, unknown> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    apiGet<ContextCurrent>('/v1/context/current')
      .then((res) => {
        if (!cancelled && res.ok && res.data) setContext(res.data);
        else if (!cancelled) setContext(null);
      })
      .catch((err) => {
        if (!cancelled) setError(err instanceof Error ? err.message : 'Failed to load context');
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => { cancelled = true; };
  }, []);

  if (loading) return <div className="p-4 text-zinc-500 text-sm">Loading context…</div>;
  if (error) return <div className="p-4 text-amber-500 text-sm">{error}</div>;
  if (!context || Object.keys(context).length === 0) {
    return (
      <div className="p-4 text-zinc-500 text-sm">
        <h3 className="font-medium text-zinc-400 mb-2">Context</h3>
        <p>No context data. Run evaluate or start the engine.</p>
      </div>
    );
  }

  return (
    <div className="p-4 text-sm overflow-y-auto">
      <h3 className="font-medium text-zinc-400 mb-2">Context</h3>
      <dl className="space-y-2">
        {Object.entries(context).map(([k, v]) => (
          <div key={k}>
            <dt className="text-zinc-500 text-xs">{k}</dt>
            <dd className="text-zinc-200 break-words">
              {typeof v === 'object' && v !== null ? JSON.stringify(v) : String(v)}
            </dd>
          </div>
        ))}
      </dl>
    </div>
  );
}
