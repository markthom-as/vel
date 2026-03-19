import { createWsUrl } from '../api/client';
import { decodeWsEvent, type WsEvent } from '../types';

type Listener = (event: WsEvent) => void;

const RECONNECT_DELAY_MS = 1000;
const IDLE_CLOSE_DELAY_MS = 250;

let socket: WebSocket | null = null;
let reconnectTimer: number | null = null;
let idleCloseTimer: number | null = null;
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

function clearIdleCloseTimer() {
  if (idleCloseTimer != null) {
    window.clearTimeout(idleCloseTimer);
    idleCloseTimer = null;
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
  clearIdleCloseTimer();
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
  clearIdleCloseTimer();
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
  clearIdleCloseTimer();
  ensureSocket();

  return () => {
    listeners.delete(listener);
    if (listeners.size === 0) {
      clearReconnectTimer();
      clearIdleCloseTimer();
      idleCloseTimer = window.setTimeout(() => {
        idleCloseTimer = null;
        if (listeners.size === 0) {
          teardownSocket();
        }
      }, IDLE_CLOSE_DELAY_MS);
    }
  };
}
