export type MainView =
  | 'now'
  | 'inbox'
  | 'threads'
  | 'projects'
  | 'settings';

export type SurfaceDisclosure = 'primary' | 'support' | 'detail';

export interface OperatorSurfaceDefinition {
  view: MainView;
  label: string;
  icon: string;
  disclosure: SurfaceDisclosure;
  navVisible: boolean;
  blurb: string;
}

export const operatorSurfaces: OperatorSurfaceDefinition[] = [
  {
    view: 'now',
    label: 'Now',
    icon: '◉',
    disclosure: 'primary',
    navVisible: true,
    blurb: 'Urgent context and immediate action',
  },
  {
    view: 'inbox',
    label: 'Inbox',
    icon: '◎',
    disclosure: 'primary',
    navVisible: true,
    blurb: 'Explicit triage and queued work',
  },
  {
    view: 'threads',
    label: 'Threads',
    icon: '◌',
    disclosure: 'primary',
    navVisible: true,
    blurb: 'Bounded continuation, history, and search',
  },
  {
    view: 'projects',
    label: 'Projects',
    icon: '▣',
    disclosure: 'detail',
    navVisible: false,
    blurb: 'Project context and drill-down when a daily-use surface points there',
  },
  {
    view: 'settings',
    label: 'Settings',
    icon: '◇',
    disclosure: 'support',
    navVisible: true,
    blurb: 'Setup, trust, and runtime controls',
  },
];

export const primarySurfaces = operatorSurfaces.filter((surface) => surface.disclosure === 'primary');
export const supportSurfaces = operatorSurfaces.filter(
  (surface) => surface.disclosure === 'support' && surface.navVisible,
);
export const detailSurfaces = operatorSurfaces.filter((surface) => surface.disclosure === 'detail');

export function getSurfaceDefinition(view: MainView): OperatorSurfaceDefinition {
  const surface = operatorSurfaces.find((entry) => entry.view === view);
  if (!surface) {
    throw new Error(`Unknown operator surface: ${view}`);
  }
  return surface;
}
