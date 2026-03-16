import { useCallback, useEffect, useSyncExternalStore } from 'react';

type QueryKeyPart = string | number | boolean | null | undefined;
export type QueryKey = readonly QueryKeyPart[];

type QuerySnapshot<T> = {
  data: T | undefined;
  error: string | null;
  loading: boolean;
  refreshing: boolean;
};

interface QueryEntry<T> {
  data: T | undefined;
  error: string | null;
  hasLoaded: boolean;
  isFetching: boolean;
  stale: boolean;
  fetcher: (() => Promise<T>) | null;
  listeners: Set<() => void>;
  inFlight: Promise<T> | null;
  snapshot: QuerySnapshot<T>;
}

interface UseQueryOptions {
  enabled?: boolean;
}

const cache = new Map<string, QueryEntry<unknown>>();

function serializeKey(key: QueryKey): string {
  return JSON.stringify(key);
}

function getEntryBySerialized<T>(serialized: string): QueryEntry<T> {
  let entry = cache.get(serialized) as QueryEntry<T> | undefined;
  if (!entry) {
    entry = {
      data: undefined,
      error: null,
      hasLoaded: false,
      isFetching: false,
      stale: false,
      fetcher: null,
      listeners: new Set(),
      inFlight: null,
      snapshot: {
        data: undefined,
        error: null,
        loading: false,
        refreshing: false,
      },
    };
    cache.set(serialized, entry as QueryEntry<unknown>);
  }
  return entry;
}

function getEntry<T>(key: QueryKey): QueryEntry<T> {
  return getEntryBySerialized<T>(serializeKey(key));
}

function notify(entry: QueryEntry<unknown>) {
  for (const listener of entry.listeners) {
    listener();
  }
}

function refreshSnapshot<T>(entry: QueryEntry<T>) {
  entry.snapshot = {
    data: entry.data,
    error: entry.error,
    loading: entry.isFetching && entry.data === undefined,
    refreshing: entry.isFetching && entry.data !== undefined,
  };
}

function subscribe(key: QueryKey, listener: () => void): () => void {
  const entry = getEntry(key);
  entry.listeners.add(listener);
  return () => {
    entry.listeners.delete(listener);
  };
}

function getSnapshot<T>(key: QueryKey): QuerySnapshot<T> {
  const entry = getEntry<T>(key);
  return entry.snapshot;
}

async function runFetch<T>(key: QueryKey, fetcher: () => Promise<T>, force: boolean): Promise<T> {
  const entry = getEntry<T>(key);
  entry.fetcher = fetcher;

  if (entry.inFlight && !force) {
    return entry.inFlight;
  }
  if (entry.hasLoaded && !entry.stale && !force) {
    return Promise.resolve(entry.data as T);
  }

  entry.isFetching = true;
  entry.error = null;
  refreshSnapshot(entry);
  notify(entry as QueryEntry<unknown>);

  const promise = fetcher()
    .then((data) => {
      entry.data = data;
      entry.error = null;
      entry.hasLoaded = true;
      entry.stale = false;
      refreshSnapshot(entry);
      return data;
    })
    .catch((error) => {
      entry.error = error instanceof Error ? error.message : String(error);
      entry.hasLoaded = true;
      refreshSnapshot(entry);
      throw error;
    })
    .finally(() => {
      entry.isFetching = false;
      entry.inFlight = null;
      refreshSnapshot(entry);
      notify(entry as QueryEntry<unknown>);
    });

  entry.inFlight = promise;
  return promise;
}

export function useQuery<T>(
  key: QueryKey,
  fetcher: () => Promise<T>,
  options: UseQueryOptions = {},
): QuerySnapshot<T> & { refetch: () => Promise<T> } {
  const { enabled = true } = options;
  const serializedKey = serializeKey(key);
  const snapshot = useSyncExternalStore(
    (listener) => subscribe(key, listener),
    () => getSnapshot<T>(key),
    () => getSnapshot<T>(key),
  );

  useEffect(() => {
    if (!enabled) {
      return;
    }
    const entry = getEntryBySerialized<T>(serializedKey);
    entry.fetcher = fetcher;
    if (!entry.hasLoaded || entry.stale) {
      void runFetch(key, fetcher, false);
    }
  }, [enabled, fetcher, key, serializedKey]);

  const refetch = useCallback(() => runFetch(key, fetcher, true), [fetcher, key]);

  return {
    ...snapshot,
    refetch,
  };
}

export function setQueryData<T>(
  key: QueryKey,
  updater: T | undefined | ((current: T | undefined) => T | undefined),
): void {
  const entry = getEntry<T>(key);
  entry.data = typeof updater === 'function'
    ? (updater as (current: T | undefined) => T | undefined)(entry.data)
    : updater;
  entry.error = null;
  entry.hasLoaded = true;
  entry.stale = false;
  refreshSnapshot(entry);
  notify(entry as QueryEntry<unknown>);
}

export function invalidateQuery(key: QueryKey, options: { refetch?: boolean } = {}): void {
  const entry = getEntry(key);
  entry.stale = true;
  refreshSnapshot(entry);
  notify(entry);

  if (options.refetch && entry.fetcher && entry.listeners.size > 0) {
    void runFetch(key, entry.fetcher, true);
  }
}

export function clearQueryCache(): void {
  cache.clear();
}
