import { apiGet, apiPatch, apiPost } from '../api/client';
import type { ApiResponse, Decoder } from '../types';

interface CanonicalTransportOptions {
  allowDegraded?: boolean;
}

function normalizeCanonicalResponse<T>(
  response: ApiResponse<T>,
  path: string,
  kind: 'query' | 'mutation',
  options: CanonicalTransportOptions = {},
): ApiResponse<T> {
  if (!options.allowDegraded && response.meta.degraded && (import.meta.env.DEV || import.meta.env.MODE === 'test')) {
    throw new Error(`Degraded canonical ${kind} response for ${path}`);
  }
  return response;
}

export async function canonicalQuery<T>(
  path: string,
  decode: Decoder<ApiResponse<T>>,
  options?: CanonicalTransportOptions,
): Promise<ApiResponse<T>> {
  const response = await apiGet<ApiResponse<T>>(path, decode);
  return normalizeCanonicalResponse(response, path, 'query', options);
}

export async function canonicalPostMutation<T>(
  path: string,
  body: unknown,
  decode: Decoder<ApiResponse<T>>,
  options?: CanonicalTransportOptions,
): Promise<ApiResponse<T>> {
  const response = await apiPost<ApiResponse<T>>(path, body, decode);
  return normalizeCanonicalResponse(response, path, 'mutation', options);
}

export async function canonicalPatchMutation<T>(
  path: string,
  body: unknown,
  decode: Decoder<ApiResponse<T>>,
  options?: CanonicalTransportOptions,
): Promise<ApiResponse<T>> {
  const response = await apiPatch<ApiResponse<T>>(path, body, decode);
  return normalizeCanonicalResponse(response, path, 'mutation', options);
}
