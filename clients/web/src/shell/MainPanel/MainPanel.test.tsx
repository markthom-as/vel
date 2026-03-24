import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { MainPanel } from './MainPanel'
import {
  operatorSurfaces,
  primarySurfaces,
  supportSurfaces,
} from '../../data/operatorSurfaces'

vi.mock('../../views/now', () => ({
  NowView: () => <div>Now view</div>,
}))

vi.mock('../../views/threads', () => ({
  ThreadView: ({ conversationId }: { conversationId: string | null }) => (
    <div>{conversationId ? `Thread ${conversationId}` : 'Thread empty'}</div>
  ),
}))

vi.mock('../../views/threads/useResolvedThreadConversationId', () => ({
  useResolvedThreadConversationId: (conversationId: string | null) => conversationId,
}))

vi.mock('../../views/system', () => ({
  SystemView: () => <div>System view</div>,
}))

describe('MainPanel', () => {
  function renderMainPanel(mainView: 'now' | 'threads' | 'system') {
    return render(
      <MainPanel
        conversationId={mainView === 'threads' ? 'conv_1' : null}
        mainView={mainView}
        onNavigate={() => {}}
        onOpenThread={() => {}}
        onOpenSystem={() => {}}
        shellOwnsNowNudges
        systemTarget={{ section: 'integrations' }}
      />,
    )
  }

  it('shows the Now view when mainView is now', () => {
    renderMainPanel('now')
    expect(screen.getByText('Now view')).toBeInTheDocument()
    expect(screen.queryByText('Inbox view')).toBeNull()
  })

  it('shows the thread view when mainView is threads', () => {
    renderMainPanel('threads')
    expect(screen.getByText('Thread conv_1')).toBeInTheDocument()
  })

  it('shows the System surface when mainView is system', () => {
    renderMainPanel('system')
    expect(screen.getByText('System view')).toBeInTheDocument()
  })

  it('uses the approved shell parity taxonomy as the first-class route set', () => {
    expect(primarySurfaces.map((surface) => surface.view)).toEqual(['now', 'threads', 'system'])
    expect(supportSurfaces.map((surface) => surface.view)).toEqual([])
    expect(
      operatorSurfaces.filter((surface) => surface.navVisible).map((surface) => surface.view),
    ).toEqual(['now', 'threads', 'system'])
  })
})
