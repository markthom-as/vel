import { useMemo, useState } from 'react';
import { apiPatch } from '../api/client';
import type { SettingsData } from '../types';
import { setQueryData, useQuery } from '../data/query';
import { loadSettings, queryKeys } from '../data/resources';

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const [saving, setSaving] = useState(false);
  const settingsKey = useMemo(() => queryKeys.settings(), []);
  const {
    data: settings = {},
    loading,
  } = useQuery<SettingsData>(
    settingsKey,
    async () => {
      const response = await loadSettings();
      return response.ok && response.data ? response.data : {};
    },
  );

  const update = async (key: keyof SettingsData, value: boolean | unknown) => {
    setSaving(true);
    try {
      await apiPatch('/api/settings', { [key]: value });
      setQueryData<SettingsData>(settingsKey, (prev = {}) => ({
        ...prev,
        [key]: value,
      }));
    } finally {
      setSaving(false);
    }
  };

  if (loading) return <div className="p-8 text-zinc-500">Loading settings…</div>;

  return (
    <div className="flex-1 overflow-y-auto p-8 max-w-lg">
      <button
        type="button"
        onClick={onBack}
        className="text-zinc-500 hover:text-zinc-300 text-sm mb-6"
      >
        ← Back
      </button>
      <h2 className="text-xl font-medium text-zinc-200 mb-6">Settings</h2>
      <div className="space-y-4">
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Disable proactive interventions</span>
          <input
            type="checkbox"
            checked={settings.disable_proactive ?? false}
            onChange={(e) => update('disable_proactive', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Show risks</span>
          <input
            type="checkbox"
            checked={settings.toggle_risks !== false}
            onChange={(e) => update('toggle_risks', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Show reminders</span>
          <input
            type="checkbox"
            checked={settings.toggle_reminders !== false}
            onChange={(e) => update('toggle_reminders', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
      </div>
      {saving && <p className="text-zinc-500 text-sm mt-4">Saving…</p>}
    </div>
  );
}
