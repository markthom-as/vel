import type { Decoder } from '../types';

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

async function readApiError(res: Response, path: string): Promise<Error> {
  const fallback = new Error(`API ${res.status}: ${path}`);
  const contentType = res.headers.get('content-type') ?? '';
  if (!contentType.includes('application/json')) {
    return fallback;
  }

  try {
    const body = await res.json() as {
      error?: {
        message?: unknown;
      };
    };
    const message = body?.error?.message;
    if (typeof message === 'string' && message.trim()) {
      return new Error(`API ${res.status}: ${message}`);
    }
  } catch {
    return fallback;
  }

  return fallback;
}

async function decodeResponseBody<T>(res: Response, decode?: Decoder<T>): Promise<T> {
  const body = (await res.json()) as unknown;
  return decode ? decode(body) : (body as T);
}

export async function apiGet<T>(path: string, decode?: Decoder<T>): Promise<T> {
  try {
    const res = await fetch(`${API_BASE}${path}`);
    if (!res.ok) throw await readApiError(res, path);
    return await decodeResponseBody(res, decode);
  } catch (e) {
    throw wrapNetworkError(e);
  }
}

export async function apiPost<T>(path: string, body: unknown, decode?: Decoder<T>): Promise<T> {
  try {
    const res = await fetch(`${API_BASE}${path}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw await readApiError(res, path);
    return await decodeResponseBody(res, decode);
  } catch (e) {
    throw wrapNetworkError(e);
  }
}

export async function apiPatch<T>(path: string, body: unknown, decode?: Decoder<T>): Promise<T> {
  try {
    const res = await fetch(`${API_BASE}${path}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw await readApiError(res, path);
    return await decodeResponseBody(res, decode);
  } catch (e) {
    throw wrapNetworkError(e);
  }
}

export { API_BASE };
