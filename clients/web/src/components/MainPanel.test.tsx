import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { MainPanel } from './MainPanel'
import {
  detailSurfaces,
  operatorSurfaces,
  primarySurfaces,
  secondarySurfaces,
} from '../data/operatorSurfaces'

vi.mock('./NowView', () => ({
  NowView: () => <div>Now view</div>,
}))

vi.mock('./InboxView', () => ({
  InboxView: () => <div>Inbox view</div>,
}))

vi.mock('./ProjectsView', () => ({
  ProjectsView: () => <div>Projects view</div>,
}))

vi.mock('./ThreadView', () => ({
  ThreadView: ({ conversationId }: { conversationId: string | null }) => (
    <div>{conversationId ? `Thread ${conversationId}` : 'Thread empty'}</div>
  ),
}))

vi.mock('./SuggestionsView', () => ({
  SuggestionsView: () => <div>Suggestions view</div>,
}))

vi.mock('./StatsView', () => ({
  StatsView: () => <div>Stats view</div>,
}))

vi.mock('./SettingsPage', () => ({
  SettingsPage: ({ initialTab }: { initialTab: string }) => <div>Settings page {initialTab}</div>,
}))

describe('MainPanel', () => {
  function renderMainPanel(mainView: 'now' | 'inbox' | 'threads' | 'suggestions' | 'projects' | 'stats' | 'settings') {
    return render(
      <MainPanel
        conversationId={mainView === 'threads' ? 'conv_1' : null}
        mainView={mainView}
        onNavigate={() => {}}
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

  it('shows the Suggestions view when mainView is suggestions', () => {
    renderMainPanel('suggestions')
    expect(screen.getByText('Suggestions view')).toBeInTheDocument()
  })

  it('shows the Projects view when mainView is projects', () => {
    renderMainPanel('projects')
    expect(screen.getByText('Projects view')).toBeInTheDocument()
  })

  it('shows the Stats surface when mainView is stats', () => {
    renderMainPanel('stats')
    expect(screen.getByText('Stats view')).toBeInTheDocument()
  })

  it('shows the Settings surface when mainView is settings', () => {
    renderMainPanel('settings')
    expect(screen.getByText('Settings page general')).toBeInTheDocument()
  })

  it('uses the approved taxonomy as the first-class route set', () => {
    expect(primarySurfaces.map((surface) => surface.view)).toEqual(['now', 'inbox'])
    expect(secondarySurfaces.map((surface) => surface.view)).toEqual(['threads', 'projects'])
    expect(
      operatorSurfaces.filter((surface) => surface.navVisible).map((surface) => surface.view),
    ).toEqual(['now', 'inbox', 'threads', 'projects', 'settings'])
  })

  it('keeps suggestions and stats as detail surfaces instead of primary peers', () => {
    expect(detailSurfaces.map((surface) => surface.view)).toEqual(['suggestions', 'stats'])
    expect(detailSurfaces.every((surface) => surface.navVisible === false)).toBe(true)
  })
})
