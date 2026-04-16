import { useMemo, useState } from 'react';
import {
  loadIntegrations,
  operatorQueryKeys,
  updateGoogleCalendarIntegration,
  updateSettings,
} from '../../data/operator';
import {
  contextQueryKeys,
  loadNow,
  rescheduleNowCalendarEvent,
} from '../../data/context';
import {
  invalidateQuery,
  setQueryData,
  useQuery,
} from '../../data/query';
import {
  invalidateInboxQueries,
} from '../../data/chat';
import type { IntegrationsData, NowData, NowEventData } from '../../types';

function calendarEventDuration(event: NowEventData): number {
  if (!event.end_ts || event.end_ts <= event.start_ts) {
    return 30 * 60;
  }
  return event.end_ts - event.start_ts;
}

export function useNudgeZoneData(integrationsQueryKey: readonly ['integrations'] = ['integrations']) {
  const [pendingActionKey, setPendingActionKey] = useState<string | null>(null);
  const [pendingCalendarToggleId, setPendingCalendarToggleId] = useState<string | null>(null);
  const [pendingCalendarEventId, setPendingCalendarEventId] = useState<string | null>(null);
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const settingsKey = useMemo(() => operatorQueryKeys.settings(), []);
  const integrationsKey = useMemo(() => integrationsQueryKey, [integrationsQueryKey]);
  const { data, loading, error } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: integrations } = useQuery<IntegrationsData | null>(
    integrationsKey,
    async () => {
      const response = await loadIntegrations();
      return response.ok ? response.data ?? null : null;
    },
  );

  async function runNudgeMutation(
    actionKey: string,
    callback: () => Promise<unknown>,
  ) {
    setPendingActionKey(actionKey);
    try {
      await callback();
      invalidateInboxQueries();
      invalidateQuery(nowKey, { refetch: true });
    } finally {
      setPendingActionKey(null);
    }
  }

  async function toggleCalendar(calendarId: string | null) {
    if (!integrations) {
      return;
    }
    const googleCalendar = integrations.google_calendar;
    const pendingId = calendarId ?? '__all__';
    setPendingCalendarToggleId(pendingId);
    try {
      const patch = calendarId == null
        ? {
          calendar_settings: googleCalendar.calendars.map((calendar) => ({
            id: calendar.id,
            display_enabled: calendar.sync_enabled,
          })),
        }
        : {
          calendar_settings: googleCalendar.calendars
            .filter((calendar) => calendar.id === calendarId)
            .map((calendar) => ({
              id: calendar.id,
              display_enabled: !calendar.display_enabled,
            })),
        };
      const response = await updateGoogleCalendarIntegration(patch);
      if (!response.ok) {
        return;
      }
      setQueryData(integrationsKey, response.data ?? null);
      invalidateQuery(nowKey, { refetch: true });
    } finally {
      setPendingCalendarToggleId(null);
    }
  }

  async function moveCalendarEvent(event: NowEventData, startTs: number) {
    if (!event.event_id) {
      return;
    }
    setPendingCalendarEventId(event.event_id);
    try {
      const duration = calendarEventDuration(event);
      const response = await rescheduleNowCalendarEvent({
        event_id: event.event_id,
        calendar_id: event.calendar_id,
        start_ts: startTs,
        end_ts: event.end_ts ? startTs + duration : null,
      });
      if (!response.ok) {
        return;
      }
      setQueryData(nowKey, response.data ?? null);
    } finally {
      setPendingCalendarEventId(null);
    }
  }

  async function acceptCoreSetupSuggestion(itemId: string, value: string) {
    const trimmed = value.trim();
    if (!trimmed) {
      return;
    }
    setPendingActionKey(`core-setup:${itemId}`);
    try {
      const patch =
        itemId === 'user_display_name'
          ? { core_settings: { user_display_name: trimmed } }
          : itemId === 'node_display_name'
            ? { node_display_name: trimmed }
            : itemId === 'agent_profile'
              ? { core_settings: { agent_profile: { role: trimmed } } }
              : null;
      if (!patch) {
        return;
      }
      const response = await updateSettings(patch);
      if (response.ok) {
        setQueryData(settingsKey, response.data ?? null);
        invalidateQuery(nowKey, { refetch: true });
      }
    } finally {
      setPendingActionKey(null);
    }
  }

  return {
    acceptCoreSetupSuggestion,
    data,
    error,
    integrations: integrations ?? null,
    integrationsKey,
    loading,
    moveCalendarEvent,
    pendingActionKey,
    pendingCalendarEventId,
    pendingCalendarToggleId,
    runNudgeMutation,
    toggleCalendar,
  };
}
