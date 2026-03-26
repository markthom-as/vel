import { useEffect, useMemo, useRef, useState } from 'react';
import { Button } from '../../core/Button';
import { cn } from '../../core/cn';
import {
  normalizeSemanticAliasOverrides,
  semanticAliasFamilyLabels,
  semanticAliasFamilyOrder,
  type SemanticAliasFamily,
  type SemanticAliasOverrides,
} from '../../core/Theme/semanticAliases';
import type { SemanticAliasOverridesData } from '../../types';

type AliasRow = {
  id: string;
  key: string;
  value: string;
};

type AliasDraft = Record<SemanticAliasFamily, AliasRow[]>;

function buildEmptyDraft(): AliasDraft {
  return {
    provider: [],
    project: [],
    calendar: [],
    mode: [],
    nudge: [],
    alert: [],
  };
}

function buildDraftFromValue(
  value: SemanticAliasOverridesData,
  nextId: () => string,
): AliasDraft {
  const draft = buildEmptyDraft();
  for (const family of semanticAliasFamilyOrder) {
    const entries = value[family] ?? {};
    draft[family] = Object.entries(entries).map(([key, label]) => ({
      id: nextId(),
      key,
      value: label,
    }));
  }
  return draft;
}

function buildOverridesFromDraft(draft: AliasDraft): SemanticAliasOverrides {
  const raw: SemanticAliasOverrides = {};
  for (const family of semanticAliasFamilyOrder) {
    const entries = draft[family].reduce<Record<string, string>>((acc, row) => {
      acc[row.key] = row.value;
      return acc;
    }, {});
    if (Object.keys(entries).length > 0) {
      raw[family] = entries;
    }
  }
  return normalizeSemanticAliasOverrides(raw);
}

export function SemanticAliasesEditor({
  value,
  onSave,
}: {
  value: SemanticAliasOverridesData;
  onSave: (aliases: SemanticAliasOverridesData) => void | Promise<void>;
}) {
  const idRef = useRef(0);
  const nextId = () => {
    idRef.current += 1;
    return `semantic-alias-row-${idRef.current}`;
  };
  const [draft, setDraft] = useState<AliasDraft>(() => buildDraftFromValue(value, nextId));
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setDraft(buildDraftFromValue(value, nextId));
    setError(null);
  }, [value]);

  const normalizedPersisted = useMemo(
    () => JSON.stringify(normalizeSemanticAliasOverrides(value)),
    [value],
  );
  const normalizedDraft = useMemo(() => buildOverridesFromDraft(draft), [draft]);
  const serializedDraft = useMemo(() => JSON.stringify(normalizedDraft), [normalizedDraft]);
  const hasChanges = serializedDraft !== normalizedPersisted;

  const duplicateKeys = useMemo(() => {
    const collisions: string[] = [];
    for (const family of semanticAliasFamilyOrder) {
      const seen = new Set<string>();
      for (const row of draft[family]) {
        const normalizedKey = row.key.trim().toLowerCase().replace(/\s+/g, '_');
        if (normalizedKey.length === 0) {
          continue;
        }
        if (seen.has(normalizedKey)) {
          collisions.push(`${semanticAliasFamilyLabels[family]}: ${normalizedKey}`);
        }
        seen.add(normalizedKey);
      }
    }
    return collisions;
  }, [draft]);

  const blocked = duplicateKeys.length > 0;

  async function saveDraft() {
    if (blocked) {
      setError(`Duplicate canonical keys: ${duplicateKeys.join(', ')}`);
      return;
    }
    setSaving(true);
    setError(null);
    try {
      await onSave(normalizedDraft);
    } catch (nextError) {
      setError(nextError instanceof Error ? nextError.message : 'Failed to save semantic aliases.');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="space-y-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className="text-xs leading-5 text-[var(--vel-color-muted)]">
          Override shared labels for providers, projects, calendars, nudges, alerts, and modes using canonical snake_case keys.
        </p>
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="sm"
            disabled={!hasChanges || saving}
            onClick={() => {
              setDraft(buildDraftFromValue(value, nextId));
              setError(null);
            }}
          >
            Reset
          </Button>
          <Button variant="secondary" size="sm" loading={saving} disabled={!hasChanges || blocked} onClick={() => void saveDraft()}>
            Save aliases
          </Button>
        </div>
      </div>
      {error ? (
        <p className="text-xs leading-5 text-[#f7b5a7]">{error}</p>
      ) : duplicateKeys.length > 0 ? (
        <p className="text-xs leading-5 text-[#f7b5a7]">
          Duplicate canonical keys: {duplicateKeys.join(', ')}
        </p>
      ) : null}
      {semanticAliasFamilyOrder.map((family) => (
        <div key={family} className="space-y-2 rounded-[18px] border border-[var(--vel-color-border)] bg-[var(--vel-surface-overlay)]/40 p-3">
          <div className="flex flex-wrap items-center justify-between gap-2">
            <h3 className="text-sm font-medium text-[var(--vel-color-text)]">{semanticAliasFamilyLabels[family]}</h3>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => {
                setDraft((current) => ({
                  ...current,
                  [family]: [...current[family], { id: nextId(), key: '', value: '' }],
                }));
              }}
            >
              Add alias
            </Button>
          </div>
          {draft[family].length === 0 ? (
            <p className="text-xs leading-5 text-[var(--vel-color-muted)]">No overrides configured.</p>
          ) : (
            <div className="space-y-2">
              {draft[family].map((row) => (
                <div key={row.id} className="grid gap-2 sm:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto]">
                  <label className="space-y-1">
                    <span className="text-[10px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">Canonical key</span>
                    <input
                      value={row.key}
                      placeholder="google_calendar"
                      onChange={(event) => {
                        const nextValue = event.target.value;
                        setDraft((current) => ({
                          ...current,
                          [family]: current[family].map((candidate) => (
                            candidate.id === row.id ? { ...candidate, key: nextValue } : candidate
                          )),
                        }));
                      }}
                      className={cn(
                        'w-full rounded-[12px] border border-[var(--vel-color-border)] bg-transparent px-3 py-2 text-sm text-[var(--vel-color-text)] outline-none',
                        'focus:border-[var(--vel-color-text-muted)]',
                      )}
                    />
                  </label>
                  <label className="space-y-1">
                    <span className="text-[10px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">Display label</span>
                    <input
                      value={row.value}
                      placeholder="Google Calendar"
                      onChange={(event) => {
                        const nextValue = event.target.value;
                        setDraft((current) => ({
                          ...current,
                          [family]: current[family].map((candidate) => (
                            candidate.id === row.id ? { ...candidate, value: nextValue } : candidate
                          )),
                        }));
                      }}
                      className={cn(
                        'w-full rounded-[12px] border border-[var(--vel-color-border)] bg-transparent px-3 py-2 text-sm text-[var(--vel-color-text)] outline-none',
                        'focus:border-[var(--vel-color-text-muted)]',
                      )}
                    />
                  </label>
                  <div className="flex items-end">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => {
                        setDraft((current) => ({
                          ...current,
                          [family]: current[family].filter((candidate) => candidate.id !== row.id),
                        }));
                      }}
                    >
                      Remove
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
