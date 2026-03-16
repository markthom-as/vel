import type { VelSyncPacket } from "./types";

export function serializePacket(packet: VelSyncPacket): string {
  return JSON.stringify(packet);
}

export function parsePacket(raw: string): VelSyncPacket {
  return JSON.parse(raw) as VelSyncPacket;
}
