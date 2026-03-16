import { useEffect, useState } from 'react';
import { apiGet, apiPatch } from '../api/client';
import type { ApiResponse, SettingsData } from '../types';

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const [settings, setSettings] = useState<SettingsData | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  const load = () => {
    apiGet<ApiResponse<SettingsData>>('/api/settings')
      .then((res) => {
        if (res.ok && res.data) setSettings(res.data as SettingsData);
        else setSettings({});
      })
      .catch(() => setSettings({}))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    load();
  }, []);

  const update = async (key: keyof SettingsData, value: boolean | unknown) => {
    setSaving(true);
    try {
      await apiPatch('/api/settings', { [key]: value });
      setSettings((prev) => (prev ? { ...prev, [key]: value } : { [key]: value }));
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
            checked={settings?.disable_proactive ?? false}
            onChange={(e) => update('disable_proactive', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Show risks</span>
          <input
            type="checkbox"
            checked={settings?.toggle_risks !== false}
            onChange={(e) => update('toggle_risks', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Show reminders</span>
          <input
            type="checkbox"
            checked={settings?.toggle_reminders !== false}
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
