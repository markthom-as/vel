import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor, within } from '@testing-library/react'
import { SettingsPage } from './SettingsPage'
import * as client from '../api/client'
import { clearQueryCache } from '../data/query'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPatch: vi.fn(),
}))

function getSettingsRoot(container: HTMLElement) {
  return container.firstElementChild as HTMLElement
}

describe('SettingsPage', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(client.apiGet).mockResolvedValue({
      ok: true,
      data: { disable_proactive: false, toggle_risks: true, toggle_reminders: true },
      meta: { request_id: 'req_1' },
    })
    vi.mocked(client.apiPatch).mockResolvedValue({} as never)
  })

  it('shows Back button and Settings heading when loaded', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    expect(screen.getByText(/loading settings/i)).toBeInTheDocument()
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('button', { name: /back/i })).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    expect(within(root).getByRole('heading', { name: /settings/i })).toBeInTheDocument()
  })

  it('renders checkboxes for disable_proactive, toggle_risks, toggle_reminders', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/disable proactive/i)).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    expect(within(root).getByText(/show risks/i)).toBeInTheDocument()
    expect(within(root).getByText(/show reminders/i)).toBeInTheDocument()
  })

  it('calls onBack when Back is clicked', async () => {
    const onBack = vi.fn()
    const { container } = render(<SettingsPage onBack={onBack} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('button', { name: /back/i })).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    fireEvent.click(within(root).getByRole('button', { name: /back/i }))
    expect(onBack).toHaveBeenCalledTimes(1)
  })

  it('calls apiPatch when a checkbox is toggled', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/show risks/i)).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    const risksCheckbox = within(root).getByRole('checkbox', { name: /show risks/i })
    fireEvent.click(risksCheckbox)
    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith('/api/settings', { toggle_risks: false })
    })
  })
})
