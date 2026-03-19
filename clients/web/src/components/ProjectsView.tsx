import { useEffect, useMemo, useState, type FormEvent } from 'react';
import type {
  ProjectCreateRequestData,
  ProjectFamilyData,
  ProjectRecordData,
} from '../types';
import { createProject, loadProjects, operatorQueryKeys } from '../data/operator';
import { setQueryData, useQuery } from '../data/query';
import { SurfaceState } from './SurfaceState';

interface ProjectDraft {
  name: string;
  slug: string;
  family: ProjectFamilyData;
  primaryRepoPath: string;
  primaryNotesPath: string;
  createRepoLater: boolean;
  createNotesRootLater: boolean;
}

const DEFAULT_DRAFT: ProjectDraft = {
  name: '',
  slug: '',
  family: 'personal',
  primaryRepoPath: '',
  primaryNotesPath: '',
  createRepoLater: false,
  createNotesRootLater: false,
};

const FAMILY_ORDER: ProjectFamilyData[] = ['personal', 'creative', 'work'];

export function ProjectsView() {
  const projectsKey = useMemo(() => operatorQueryKeys.projects(), []);
  const { data: projects = [], loading, error } = useQuery<ProjectRecordData[]>(
    projectsKey,
    async () => {
      const response = await loadProjects();
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to load projects');
      }
      return response.data.projects;
    },
  );
  const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null);
  const [draft, setDraft] = useState<ProjectDraft>(DEFAULT_DRAFT);
  const [submitting, setSubmitting] = useState(false);
  const [submitMessage, setSubmitMessage] = useState<{ status: 'success' | 'error'; text: string } | null>(null);

  useEffect(() => {
    if (projects.length === 0) {
      setSelectedProjectId(null);
      return;
    }
    if (!selectedProjectId || !projects.some((project) => project.id === selectedProjectId)) {
      setSelectedProjectId(projects[0].id);
    }
  }, [projects, selectedProjectId]);

  const selectedProject = projects.find((project) => project.id === selectedProjectId) ?? null;
  const groupedProjects = FAMILY_ORDER.map((family) => ({
    family,
    label: familyLabel(family),
    projects: projects.filter((project) => project.family === family),
  }));

  function useProjectAsDraft(project: ProjectRecordData) {
    setDraft({
      name: project.name,
      slug: project.slug,
      family: project.family,
      primaryRepoPath: project.primary_repo.path,
      primaryNotesPath: project.primary_notes_root.path,
      createRepoLater: project.pending_provision.create_repo,
      createNotesRootLater: project.pending_provision.create_notes_root,
    });
    setSubmitMessage(null);
  }

  async function handleCreateProject(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    setSubmitMessage(null);

    const payload: ProjectCreateRequestData = {
      slug: draft.slug.trim(),
      name: draft.name.trim(),
      family: draft.family,
      status: 'active',
      primary_repo: {
        path: draft.primaryRepoPath.trim(),
        label: 'Primary repo',
        kind: 'repo',
      },
      primary_notes_root: {
        path: draft.primaryNotesPath.trim(),
        label: 'Primary notes root',
        kind: 'notes_root',
      },
      secondary_repos: [],
      secondary_notes_roots: [],
      upstream_ids: {},
      pending_provision: {
        create_repo: draft.createRepoLater,
        create_notes_root: draft.createNotesRootLater,
      },
    };

    try {
      const response = await createProject(payload);
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to create project');
      }
      setQueryData<ProjectRecordData[]>(projectsKey, (current = []) => [
        response.data!.project,
        ...current,
      ]);
      setSelectedProjectId(response.data.project.id);
      setDraft(DEFAULT_DRAFT);
      setSubmitMessage({ status: 'success', text: `Created ${response.data.project.name}.` });
    } catch (submitError) {
      setSubmitMessage({
        status: 'error',
        text: submitError instanceof Error ? submitError.message : String(submitError),
      });
    } finally {
      setSubmitting(false);
    }
  }

  if (loading) {
    return <SurfaceState message="Loading projects…" layout="centered" />;
  }
  if (error) {
    return <SurfaceState message={error} layout="centered" tone="warning" />;
  }

  return (
    <div className="flex-1 overflow-y-auto bg-zinc-950">
      <div className="mx-auto max-w-6xl px-6 py-8">
        <header className="mb-8">
          <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">Projects</p>
          <h1 className="mt-2 text-3xl font-semibold text-zinc-100">Anchor work to durable project records</h1>
          <p className="mt-2 max-w-3xl text-sm leading-6 text-zinc-400">
            Keep project roots local-first, then explicitly confirm any upstream repo or notes-root
            provisioning when the project is ready for wider continuity.
          </p>
        </header>

        <div className="grid gap-6 xl:grid-cols-[1.1fr_0.9fr]">
          <section className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-5">
            <div className="mb-4">
              <h2 className="text-lg font-medium text-zinc-100">Project registry</h2>
              <p className="mt-1 text-sm text-zinc-500">
                Grouped by the canonical project families used across the runtime.
              </p>
            </div>
            <div className="space-y-5">
              {groupedProjects.map((group) => (
                <section key={group.family}>
                  <h3 className="text-sm font-medium uppercase tracking-[0.18em] text-zinc-500">
                    {group.label}
                  </h3>
                  <div className="mt-3 space-y-2">
                    {group.projects.length === 0 ? (
                      <p className="rounded-xl border border-dashed border-zinc-800 bg-zinc-950/60 px-4 py-3 text-sm text-zinc-500">
                        No {group.label.toLowerCase()} projects yet.
                      </p>
                    ) : (
                      group.projects.map((project) => (
                        <button
                          key={project.id}
                          type="button"
                          onClick={() => setSelectedProjectId(project.id)}
                          className={`block w-full rounded-xl border px-4 py-3 text-left transition ${
                            selectedProjectId === project.id
                              ? 'border-emerald-500/60 bg-emerald-500/10 text-zinc-100'
                              : 'border-zinc-800 bg-zinc-950/60 text-zinc-300 hover:border-zinc-700'
                          }`}
                        >
                          <div className="flex items-center justify-between gap-3">
                            <div>
                              <p className="font-medium">{project.name}</p>
                              <p className="mt-1 text-sm text-zinc-500">{project.slug}</p>
                            </div>
                            <span className="rounded-full border border-zinc-800 px-2.5 py-1 text-xs uppercase tracking-wide text-zinc-500">
                              {project.status}
                            </span>
                          </div>
                        </button>
                      ))
                    )}
                  </div>
                </section>
              ))}
            </div>
          </section>

          <div className="space-y-6">
            <section className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-5">
              <div className="mb-4">
                <h2 className="text-lg font-medium text-zinc-100">Project details</h2>
                <p className="mt-1 text-sm text-zinc-500">
                  Primary repo and notes roots stay visible so every surfaced action can trace back
                  to a durable workspace.
                </p>
              </div>
              {selectedProject ? (
                <div className="space-y-4 text-sm">
                  <div>
                    <p className="text-xl font-medium text-zinc-100">{selectedProject.name}</p>
                    <p className="mt-1 text-zinc-500">{selectedProject.slug}</p>
                  </div>
                  <div className="grid gap-3 md:grid-cols-2">
                    <DetailCard
                      label="Family"
                      value={familyLabel(selectedProject.family)}
                      detail="Matches the canonical project grouping shared across the runtime."
                    />
                    <DetailCard
                      label="Status"
                      value={selectedProject.status}
                      detail={`Updated ${formatProjectTimestamp(selectedProject.updated_at)}.`}
                    />
                    <DetailCard
                      label="Extra roots"
                      value={`${selectedProject.secondary_repos.length + selectedProject.secondary_notes_roots.length}`}
                      detail={`${selectedProject.secondary_repos.length} extra repos and ${selectedProject.secondary_notes_roots.length} extra notes roots.`}
                    />
                    <DetailCard
                      label="Upstream follow-up"
                      value={projectProvisionSummary(selectedProject)}
                      detail="Provisioning intent stays explicit instead of assuming remote creation."
                    />
                  </div>
                  <dl className="space-y-3">
                    <div className="rounded-xl border border-zinc-800 bg-zinc-950/70 p-4">
                      <dt className="text-zinc-500">Primary repo</dt>
                      <dd className="mt-2 text-zinc-100">{selectedProject.primary_repo.path}</dd>
                    </div>
                    <div className="rounded-xl border border-zinc-800 bg-zinc-950/70 p-4">
                      <dt className="text-zinc-500">Notes root</dt>
                      <dd className="mt-2 text-zinc-100">{selectedProject.primary_notes_root.path}</dd>
                    </div>
                  </dl>
                  <div className="rounded-xl border border-zinc-800 bg-zinc-950/50 p-4">
                    <div className="flex flex-wrap items-start justify-between gap-3">
                      <div>
                        <p className="text-sm font-medium text-zinc-100">Bounded edit path</p>
                        <p className="mt-1 text-sm text-zinc-500">
                          Reuse this record as a draft when you want a nearby project with the
                          same local roots or provisioning intent. Durable edits still stay on the
                          typed project contract.
                        </p>
                      </div>
                      <button
                        type="button"
                        onClick={() => useProjectAsDraft(selectedProject)}
                        className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white"
                      >
                        Use project as draft
                      </button>
                    </div>
                  </div>
                </div>
              ) : (
                <SurfaceState message="Select a project to inspect its local roots." />
              )}
            </section>

            <section className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-5">
              <div className="mb-4">
                <h2 className="text-lg font-medium text-zinc-100">Create project</h2>
                <p className="mt-1 text-sm text-zinc-500">
                  Draft the local record first. Upstream repo and notes-root work stays opt-in.
                </p>
              </div>
              <div className="mb-4 rounded-xl border border-zinc-800 bg-zinc-950/50 p-4 text-sm text-zinc-400">
                The create form is also the supported edit handoff today. Start from a blank draft
                or prefill from an existing project, then submit a new bounded project record.
              </div>
              <form className="space-y-4" onSubmit={(event) => void handleCreateProject(event)}>
                <div className="grid gap-4 md:grid-cols-2">
                  <label className="space-y-1">
                    <span className="text-sm text-zinc-300">Name</span>
                    <input
                      type="text"
                      value={draft.name}
                      onChange={(event) => setDraft((current) => ({ ...current, name: event.target.value }))}
                      placeholder="Weekly review overhaul"
                      required
                      disabled={submitting}
                      className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                    />
                  </label>
                  <label className="space-y-1">
                    <span className="text-sm text-zinc-300">Slug</span>
                    <input
                      type="text"
                      value={draft.slug}
                      onChange={(event) => setDraft((current) => ({ ...current, slug: event.target.value }))}
                      placeholder="weekly-review-overhaul"
                      required
                      disabled={submitting}
                      className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                    />
                  </label>
                </div>
                <label className="space-y-1">
                  <span className="text-sm text-zinc-300">Family</span>
                  <select
                    value={draft.family}
                    onChange={(event) =>
                      setDraft((current) => ({
                        ...current,
                        family: event.target.value as ProjectFamilyData,
                      }))}
                    disabled={submitting}
                    className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100"
                  >
                    <option value="personal">Personal</option>
                    <option value="creative">Creative</option>
                    <option value="work">Work</option>
                  </select>
                </label>
                <label className="space-y-1">
                  <span className="text-sm text-zinc-300">Primary repo path</span>
                  <input
                    type="text"
                    value={draft.primaryRepoPath}
                    onChange={(event) =>
                      setDraft((current) => ({ ...current, primaryRepoPath: event.target.value }))}
                    placeholder="/home/jove/code/vel"
                    required
                    disabled={submitting}
                    className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                  />
                </label>
                <label className="space-y-1">
                  <span className="text-sm text-zinc-300">Primary notes root</span>
                  <input
                    type="text"
                    value={draft.primaryNotesPath}
                    onChange={(event) =>
                      setDraft((current) => ({ ...current, primaryNotesPath: event.target.value }))}
                    placeholder="/home/jove/notes/projects/vel"
                    required
                    disabled={submitting}
                    className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                  />
                </label>
                <label className="flex items-start gap-3 rounded-xl border border-zinc-800 bg-zinc-950/60 p-3">
                  <input
                    type="checkbox"
                    checked={draft.createRepoLater}
                    onChange={(event) =>
                      setDraft((current) => ({ ...current, createRepoLater: event.target.checked }))}
                    disabled={submitting}
                    className="mt-1 rounded border-zinc-600 bg-zinc-900 text-emerald-600 focus:ring-emerald-500"
                  />
                  <span className="text-sm text-zinc-300">Create upstream repo later</span>
                </label>
                <label className="flex items-start gap-3 rounded-xl border border-zinc-800 bg-zinc-950/60 p-3">
                  <input
                    type="checkbox"
                    checked={draft.createNotesRootLater}
                    onChange={(event) =>
                      setDraft((current) => ({
                        ...current,
                        createNotesRootLater: event.target.checked,
                      }))}
                    disabled={submitting}
                    className="mt-1 rounded border-zinc-600 bg-zinc-900 text-emerald-600 focus:ring-emerald-500"
                  />
                  <span className="text-sm text-zinc-300">Create notes root later</span>
                </label>
                <button
                  type="submit"
                  disabled={submitting}
                  className="min-h-[44px] rounded-xl bg-emerald-600 px-4 py-2 text-sm font-medium text-zinc-950 hover:bg-emerald-500 disabled:cursor-not-allowed disabled:bg-zinc-700 disabled:text-zinc-300"
                >
                  Create project
                </button>
                {submitMessage ? (
                  <p className={`text-sm ${submitMessage.status === 'error' ? 'text-rose-400' : 'text-emerald-400'}`}>
                    {submitMessage.text}
                  </p>
                ) : null}
              </form>
            </section>
          </div>
        </div>
      </div>
    </div>
  );
}

function familyLabel(family: ProjectFamilyData): string {
  switch (family) {
    case 'personal':
      return 'Personal';
    case 'creative':
      return 'Creative';
    case 'work':
      return 'Work';
  }
}

function projectProvisionSummary(project: ProjectRecordData): string {
  if (project.pending_provision.create_repo && project.pending_provision.create_notes_root) {
    return 'Create repo and notes root later';
  }
  if (project.pending_provision.create_repo) {
    return 'Create repo later';
  }
  if (project.pending_provision.create_notes_root) {
    return 'Create notes root later';
  }
  return 'Local roots ready';
}

function formatProjectTimestamp(value: string): string {
  return new Intl.DateTimeFormat('en-US', {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  }).format(new Date(value));
}

function DetailCard({
  label,
  value,
  detail,
}: {
  label: string;
  value: string;
  detail: string;
}) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-950/70 p-4">
      <dt className="text-zinc-500">{label}</dt>
      <dd className="mt-2 text-base text-zinc-100">{value}</dd>
      <dd className="mt-1 text-xs text-zinc-500">{detail}</dd>
    </div>
  );
}
