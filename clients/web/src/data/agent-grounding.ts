import { apiGet } from '../api/client';
import { decodeAgentInspectData, decodeApiResponse, type AgentInspectData, type ApiResponse } from '../types';

export function loadAgentInspect(): Promise<ApiResponse<AgentInspectData>> {
  return apiGet<ApiResponse<AgentInspectData>>('/v1/agent/inspect', (value) =>
    decodeApiResponse(value, decodeAgentInspectData),
  );
}
