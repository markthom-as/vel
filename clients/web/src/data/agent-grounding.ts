import { canonicalQuery } from './canonicalTransport';
import { decodeAgentInspectData, decodeApiResponse, type AgentInspectData, type ApiResponse } from '../types';

export function loadAgentInspect(): Promise<ApiResponse<AgentInspectData>> {
  return canonicalQuery<AgentInspectData>('/v1/agent/inspect', (value) =>
    decodeApiResponse(value, decodeAgentInspectData),
  );
}
