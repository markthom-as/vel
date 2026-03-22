import { useMemo } from 'react';
import { chatQueryKeys, loadConversationList } from '../../data/chat';
import { useQuery } from '../../data/query';

/**
 * Selected thread id, or the most recently updated conversation when none is selected.
 * Matches {@link ThreadView} routing so shell chrome (e.g. floating composer) stays aligned.
 *
 * @param resolveFallback — When false, only `conversationId` is returned (no list fetch). Use in
 *   {@link MainPanel} for non-threads views so the shell does not load conversations unnecessarily.
 */
export function useResolvedThreadConversationId(
  conversationId: string | null,
  resolveFallback = true,
): string | null {
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data: conversations = [] } = useQuery(
    conversationsKey,
    async () => {
      const response = await loadConversationList();
      return response.ok && response.data ? response.data : [];
    },
    { enabled: resolveFallback },
  );
  return useMemo(() => {
    if (conversationId) {
      return conversationId;
    }
    if (!resolveFallback) {
      return null;
    }
    if (conversations.length === 0) {
      return null;
    }
    return [...conversations].sort((left, right) => right.updated_at - left.updated_at)[0]?.id ?? null;
  }, [conversationId, conversations, resolveFallback]);
}
