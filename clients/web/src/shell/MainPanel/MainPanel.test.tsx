import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { MainPanel } from './MainPanel'
import {
  detailSurfaces,
  operatorSurfaces,
  primarySurfaces,
  supportSurfaces,
} from '../../data/operatorSurfaces'

vi.mock('../../views/now', () => ({
  NowView: () => <div>Now view</div>,
}))

vi.mock('../../views/inbox', () => ({
  InboxView: () => <div>Inbox view</div>,
}))

vi.mock('../../views/projects', () => ({
  ProjectsView: () => <div>Projects view</div>,
}))

vi.mock('../../views/threads', () => ({
  ThreadView: ({ conversationId }: { conversationId: string | null }) => (
    <div>{conversationId ? `Thread ${conversationId}` : 'Thread empty'}</div>
  ),
}))

vi.mock('../../views/settings', () => ({
  SettingsPage: ({ initialTab }: { initialTab: string }) => <div>Settings page {initialTab}</div>,
}))

describe('MainPanel', () => {
  function renderMainPanel(mainView: 'now' | 'inbox' | 'threads' | 'suggestions' | 'projects' | 'stats' | 'settings') {
    return render(
      <MainPanel
        conversationId={mainView === 'threads' ? 'conv_1' : null}
        mainView={mainView}
        onNavigate={() => {}}
        onOpenInbox={() => {}}
        onOpenThread={() => {}}
        onOpenSettings={() => {}}
        settingsTarget={{ tab: 'general' }}
      />,
    )
  }

  it('shows the Now view when mainView is now', () => {
    renderMainPanel('now')
    expect(screen.getByText('Now view')).toBeInTheDocument()
    expect(screen.queryByText('Inbox view')).toBeNull()
  })

  it('shows the Inbox view when mainView is inbox', () => {
    renderMainPanel('inbox')
    expect(screen.getByText('Inbox view')).toBeInTheDocument()
    expect(screen.queryByText('Thread empty')).toBeNull()
  })

  it('shows the thread view when mainView is threads', () => {
    renderMainPanel('threads')
    expect(screen.getByText('Thread conv_1')).toBeInTheDocument()
  })

  it('shows the Projects view when mainView is projects', () => {
    renderMainPanel('projects')
    expect(screen.getByText('Projects view')).toBeInTheDocument()
  })

  it('demotes hidden detail surfaces to placeholders instead of first-class routes', () => {
    renderMainPanel('suggestions')
    expect(screen.getByText('Suggestions is not part of the primary MVP shell.')).toBeInTheDocument()
    renderMainPanel('stats')
    expect(screen.getByText('Stats is not part of the primary MVP shell.')).toBeInTheDocument()
  })

  it('shows the Settings surface when mainView is settings', () => {
    renderMainPanel('settings')
    expect(screen.getByText('Settings page general')).toBeInTheDocument()
  })

  it('uses the approved shell parity taxonomy as the first-class route set', () => {
    expect(primarySurfaces.map((surface) => surface.view)).toEqual(['now', 'inbox', 'threads'])
    expect(supportSurfaces.map((surface) => surface.view)).toEqual(['settings'])
    expect(
      operatorSurfaces.filter((surface) => surface.navVisible).map((surface) => surface.view),
    ).toEqual(['now', 'inbox', 'threads', 'settings'])
  })

  it('keeps projects, suggestions, and stats as hidden detail surfaces', () => {
    expect(detailSurfaces.map((surface) => surface.view)).toEqual(['projects', 'suggestions', 'stats'])
    expect(detailSurfaces.every((surface) => surface.navVisible === false)).toBe(true)
  })
})
