import { createWsUrl } from '../api/client';
import { decodeWsEvent, type WsEvent } from '../types';

type Listener = (event: WsEvent) => void;

const RECONNECT_DELAY_MS = 1000;

let socket: WebSocket | null = null;
let reconnectTimer: number | null = null;
const listeners = new Set<Listener>();

function notify(event: WsEvent) {
  for (const listener of listeners) {
    listener(event);
  }
}

function clearReconnectTimer() {
  if (reconnectTimer != null) {
    window.clearTimeout(reconnectTimer);
    reconnectTimer = null;
  }
}

function scheduleReconnect() {
  if (reconnectTimer != null || listeners.size === 0) {
    return;
  }
  reconnectTimer = window.setTimeout(() => {
    reconnectTimer = null;
    ensureSocket();
  }, RECONNECT_DELAY_MS);
}

function teardownSocket() {
  if (!socket) {
    return;
  }
  socket.onopen = null;
  socket.onmessage = null;
  socket.onerror = null;
  socket.onclose = null;
  socket.close();
  socket = null;
}

function ensureSocket() {
  if (socket || listeners.size === 0) {
    return;
  }

  socket = new WebSocket(createWsUrl('/ws'));
  socket.onmessage = (message) => {
    try {
      notify(decodeWsEvent(JSON.parse(message.data as string) as unknown));
    } catch {
      // Ignore malformed frames; the client should stay connected.
    }
  };
  socket.onerror = () => {
    teardownSocket();
    scheduleReconnect();
  };
  socket.onclose = () => {
    socket = null;
    scheduleReconnect();
  };
}

export function subscribeWs(listener: Listener): () => void {
  listeners.add(listener);
  clearReconnectTimer();
  ensureSocket();

  return () => {
    listeners.delete(listener);
    if (listeners.size === 0) {
      clearReconnectTimer();
      teardownSocket();
    }
  };
}
