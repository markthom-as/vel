export type MainView =
  | 'now'
  | 'inbox'
  | 'threads'
  | 'projects'
  | 'settings'
  | 'suggestions'
  | 'stats';

export type SurfaceDisclosure = 'primary' | 'secondary' | 'advanced' | 'detail';

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
    disclosure: 'secondary',
    navVisible: true,
    blurb: 'Parallel work, history, and search',
  },
  {
    view: 'projects',
    label: 'Projects',
    icon: '▣',
    disclosure: 'secondary',
    navVisible: true,
    blurb: 'Project context and drill-down',
  },
  {
    view: 'settings',
    label: 'Settings',
    icon: '◇',
    disclosure: 'advanced',
    navVisible: true,
    blurb: 'Setup, trust, and runtime controls',
  },
  {
    view: 'suggestions',
    label: 'Suggestions',
    icon: '△',
    disclosure: 'detail',
    navVisible: false,
    blurb: 'Detail surface for suggestion review',
  },
  {
    view: 'stats',
    label: 'Stats',
    icon: '□',
    disclosure: 'detail',
    navVisible: false,
    blurb: 'Deeper context and operational stats',
  },
];

export const primarySurfaces = operatorSurfaces.filter((surface) => surface.disclosure === 'primary');
export const secondarySurfaces = operatorSurfaces.filter(
  (surface) => surface.disclosure === 'secondary' && surface.navVisible,
);
export const advancedSurfaces = operatorSurfaces.filter(
  (surface) => surface.disclosure === 'advanced' && surface.navVisible,
);
export const detailSurfaces = operatorSurfaces.filter((surface) => surface.disclosure === 'detail');

export function getSurfaceDefinition(view: MainView): OperatorSurfaceDefinition {
  const surface = operatorSurfaces.find((entry) => entry.view === view);
  if (!surface) {
    throw new Error(`Unknown operator surface: ${view}`);
  }
  return surface;
}
