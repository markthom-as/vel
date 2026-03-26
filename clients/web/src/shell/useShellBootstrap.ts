import { useMemo } from 'react';
import { contextQueryKeys, loadNow } from '../data/context';
import { loadIntegrations, loadSettings, operatorQueryKeys } from '../data/operator';
import { useQuery } from '../data/query';

export function useShellBootstrap() {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const settingsKey = useMemo(() => operatorQueryKeys.settings(), []);
  const integrationsKey = useMemo(() => operatorQueryKeys.integrations(), []);

  const { loading: nowLoading } = useQuery(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { loading: settingsLoading } = useQuery(
    settingsKey,
    async () => {
      const response = await loadSettings();
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to load settings');
      }
      return response.data ?? null;
    },
  );
  const { loading: integrationsLoading } = useQuery(
    integrationsKey,
    async () => {
      const response = await loadIntegrations();
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to load integrations');
      }
      return response.data ?? null;
    },
  );

  return {
    shellBootLoading: nowLoading || settingsLoading || integrationsLoading,
  };
}
