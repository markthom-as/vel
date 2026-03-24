import { describe, expect, it } from 'vitest'
import { nudgeOpenSystemTarget } from './nowModel'

describe('nudgeOpenSystemTarget', () => {
  it('maps core setup checklist actions to specific field anchors', () => {
    expect(
      nudgeOpenSystemTarget(
        { id: 'core_setup_required' },
        { kind: 'open_settings:core_settings:user_display_name:missing' },
      ),
    ).toEqual({
      section: 'core',
      subsection: 'core_settings',
      anchor: 'core-settings-user-display-name',
    })

    expect(
      nudgeOpenSystemTarget(
        { id: 'core_setup_required' },
        { kind: 'open_settings:core_settings:llm_provider:missing' },
      ),
    ).toEqual({
      section: 'integrations',
      subsection: 'providers',
      anchor: 'providers-llm-routing',
    })
  })
})
