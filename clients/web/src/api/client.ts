// In dev, use relative URLs so Vite proxy forwards to veld (no CORS). In prod, use explicit API URL.
const API_BASE =
  import.meta.env.VITE_API_URL ??
  (import.meta.env.DEV ? '' : 'http://localhost:4130');

export function createWsUrl(path: string): string {
  if (!path.startsWith('/')) {
    throw new Error(`WebSocket path must start with '/': ${path}`);
  }

  if (!API_BASE) {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    return `${protocol}//${window.location.host}${path}`;
  }

  const url = new URL(API_BASE);
  url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
  url.pathname = path;
  url.search = '';
  url.hash = '';
  return url.toString();
}

function wrapNetworkError(err: unknown): Error {
  if (err instanceof TypeError && err.message === 'Failed to fetch') {
    return new Error("Can't reach the API. Is veld running? From repo root run: make dev");
  }
  if (err instanceof Error) return err;
  return new Error(String(err));
}

export async function apiGet<T>(path: string): Promise<T> {
  try {
    const res = await fetch(`${API_BASE}${path}`);
    if (!res.ok) throw new Error(`API ${res.status}: ${path}`);
    return res.json() as Promise<T>;
  } catch (e) {
    throw wrapNetworkError(e);
  }
}

export async function apiPost<T>(path: string, body: unknown): Promise<T> {
  try {
    const res = await fetch(`${API_BASE}${path}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`API ${res.status}: ${path}`);
    return res.json() as Promise<T>;
  } catch (e) {
    throw wrapNetworkError(e);
  }
}

export async function apiPatch<T>(path: string, body: unknown): Promise<T> {
  try {
    const res = await fetch(`${API_BASE}${path}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`API ${res.status}: ${path}`);
    return res.json() as Promise<T>;
  } catch (e) {
    throw wrapNetworkError(e);
  }
}

export { API_BASE };
