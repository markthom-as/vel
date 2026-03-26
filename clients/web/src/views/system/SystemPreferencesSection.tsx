import type { SemanticAliasOverridesData } from '../../types';
import {
  SystemDocumentMetaRow,
  SystemDocumentSectionLabel,
  SystemDocumentStatsGrid,
  SystemDocumentToggleRow,
} from '../../core/SystemDocument';
import { SemanticAliasesEditor } from './SemanticAliasesEditor';
import { systemChildAnchor } from './systemNavigation';

type SystemAppearancePreferences = {
  denseRows: boolean;
  tabularNumbers: boolean;
  reducedMotion: boolean;
  strongFocus: boolean;
  dockedActionBar: boolean;
  semanticAliases: SemanticAliasOverridesData;
};

export function PreferencesAppearanceDetail({
  preferences,
  onToggle,
  onUpdateSemanticAliases,
}: {
  preferences: SystemAppearancePreferences;
  onToggle: (key: 'denseRows' | 'tabularNumbers' | 'reducedMotion' | 'strongFocus' | 'dockedActionBar') => void;
  onUpdateSemanticAliases: (aliases: SemanticAliasOverridesData) => void | Promise<void>;
}) {
  return (
    <div className="space-y-4">
      <div className="space-y-3">
        <SystemDocumentSectionLabel>Visual and interaction preferences</SystemDocumentSectionLabel>
        <SystemDocumentToggleRow
          id={systemChildAnchor('appearance', 'dense-rows')}
          title="Dense rows"
          detail="Keep rows slightly denser while preserving readability."
          value={preferences.denseRows}
          onToggle={() => onToggle('denseRows')}
        />
        <SystemDocumentToggleRow
          id={systemChildAnchor('appearance', 'tabular-numerals')}
          title="Tabular numerals"
          detail="Use stable numeric alignment for timestamps, counts, durations, and metrics."
          value={preferences.tabularNumbers}
          onToggle={() => onToggle('tabularNumbers')}
        />
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Preview posture</SystemDocumentSectionLabel>
        <SystemDocumentStatsGrid id={systemChildAnchor('appearance', 'preview-posture')} className="gap-x-6">
          <SystemDocumentMetaRow label="Theme temperament" value="warmer industrial" />
          <SystemDocumentMetaRow label="Action bar" value={preferences.dockedActionBar ? 'Docked' : 'Undocked'} />
          <SystemDocumentMetaRow label="Typography" value="Geist / Inter / JetBrains Mono" />
        </SystemDocumentStatsGrid>
      </div>

      <div className="space-y-3">
        <SystemDocumentSectionLabel>Semantic aliases</SystemDocumentSectionLabel>
        <SemanticAliasesEditor value={preferences.semanticAliases} onSave={onUpdateSemanticAliases} />
      </div>
    </div>
  );
}
