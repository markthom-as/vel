import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { ProjectsView } from './ProjectsView'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

describe('ProjectsView', () => {
  beforeEach(() => {
    cleanup()
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
  })

  it('renders grouped project families and shows the selected project roots', async () => {
    vi.mocked(api.apiGet).mockResolvedValueOnce({
      ok: true,
      data: {
        projects: [
          buildProject({ id: 'proj_personal', name: 'Home Ops', slug: 'home-ops', family: 'personal' }),
          buildProject({ id: 'proj_creative', name: 'Album Draft', slug: 'album-draft', family: 'creative' }),
          buildProject({ id: 'proj_work', name: 'Launch Train', slug: 'launch-train', family: 'work' }),
        ],
      },
      meta: { request_id: 'req_projects' },
    } as never)

    render(<ProjectsView />)

    await waitFor(() => {
      expect(screen.getByText('Project registry')).toBeInTheDocument()
    })

    expect(screen.getByText('Project context and durable roots')).toBeInTheDocument()
    expect(screen.getByText('Secondary surface')).toBeInTheDocument()
    expect(screen.getByText('Project-owned context')).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: 'Personal' })).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: 'Creative' })).toBeInTheDocument()
    expect(screen.getByRole('heading', { name: 'Work' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Home Ops/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Album Draft/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Launch Train/i })).toBeInTheDocument()
    expect(screen.getByText('/workspace/home-ops')).toBeInTheDocument()
    expect(screen.getByText('/notes/home-ops')).toBeInTheDocument()
    expect(screen.getByText('Local roots ready')).toBeInTheDocument()
    expect(screen.getByText('0 extra repos and 0 extra notes roots.')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Use project as draft' })).toBeInTheDocument()
  })

  it('can prefill the create draft from the selected project without inventing inline edits', async () => {
    vi.mocked(api.apiGet).mockResolvedValueOnce({
      ok: true,
      data: {
        projects: [
          buildProject({
            id: 'proj_work',
            name: 'Launch Train',
            slug: 'launch-train',
            family: 'work',
          }),
        ],
      },
      meta: { request_id: 'req_projects_prefill' },
    } as never)

    render(<ProjectsView />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Use project as draft' })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: 'Use project as draft' }))

    expect(screen.getByLabelText('Name')).toHaveValue('Launch Train')
    expect(screen.getByLabelText('Slug')).toHaveValue('launch-train')
    expect(screen.getByLabelText('Family')).toHaveValue('work')
    expect(screen.getByLabelText('Primary repo path')).toHaveValue('/workspace/launch-train')
    expect(screen.getByLabelText('Primary notes root')).toHaveValue('/notes/launch-train')
    expect(screen.getByText(/the create form is also the supported edit handoff today/i)).toBeInTheDocument()
  })

  it('creates a project with explicit local-first confirmations', async () => {
    vi.mocked(api.apiGet).mockResolvedValueOnce({
      ok: true,
      data: { projects: [] },
      meta: { request_id: 'req_projects_empty' },
    } as never)
    vi.mocked(api.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        project: buildProject({
          id: 'proj_new',
          name: 'Weekly Review Overhaul',
          slug: 'weekly-review-overhaul',
          family: 'work',
        }),
      },
      meta: { request_id: 'req_project_create' },
    } as never)

    render(<ProjectsView />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Create project' })).toBeInTheDocument()
    })

    fireEvent.change(screen.getByLabelText('Name'), {
      target: { value: 'Weekly Review Overhaul' },
    })
    fireEvent.change(screen.getByLabelText('Slug'), {
      target: { value: 'weekly-review-overhaul' },
    })
    fireEvent.change(screen.getByLabelText('Family'), {
      target: { value: 'work' },
    })
    fireEvent.change(screen.getByLabelText('Primary repo path'), {
      target: { value: '/workspace/weekly-review-overhaul' },
    })
    fireEvent.change(screen.getByLabelText('Primary notes root'), {
      target: { value: '/notes/weekly-review-overhaul' },
    })
    fireEvent.click(screen.getByLabelText('Create upstream repo later'))
    fireEvent.click(screen.getByLabelText('Create notes root later'))
    fireEvent.click(screen.getByRole('button', { name: 'Create project' }))

    await waitFor(() => {
      expect(screen.getByText('Created Weekly Review Overhaul.')).toBeInTheDocument()
    })

    expect(vi.mocked(api.apiPost)).toHaveBeenCalledWith(
      '/v1/projects',
      {
        slug: 'weekly-review-overhaul',
        name: 'Weekly Review Overhaul',
        family: 'work',
        status: 'active',
        primary_repo: {
          path: '/workspace/weekly-review-overhaul',
          label: 'Primary repo',
          kind: 'repo',
        },
        primary_notes_root: {
          path: '/notes/weekly-review-overhaul',
          label: 'Primary notes root',
          kind: 'notes_root',
        },
        secondary_repos: [],
        secondary_notes_roots: [],
        upstream_ids: {},
        pending_provision: {
          create_repo: true,
          create_notes_root: true,
        },
      },
      expect.any(Function),
    )
    expect(screen.getAllByText('Weekly Review Overhaul').length).toBeGreaterThan(0)
  })
})

function buildProject({
  id,
  name,
  slug,
  family,
}: {
  id: string
  name: string
  slug: string
  family: 'personal' | 'creative' | 'work'
}) {
  return {
    id,
    slug,
    name,
    family,
    status: 'active',
    primary_repo: {
      path: `/workspace/${slug}`,
      label: 'Primary repo',
      kind: 'repo',
    },
    primary_notes_root: {
      path: `/notes/${slug}`,
      label: 'Primary notes root',
      kind: 'notes_root',
    },
    secondary_repos: [],
    secondary_notes_roots: [],
    upstream_ids: {},
    pending_provision: {
      create_repo: false,
      create_notes_root: false,
    },
    created_at: '2026-03-16T18:00:00Z',
    updated_at: '2026-03-16T18:00:00Z',
    archived_at: null,
  }
}
