export type MainView =
  | 'now'
  | 'threads'
  | 'system';

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
    view: 'threads',
    label: 'Threads',
    icon: '◌',
    disclosure: 'primary',
    navVisible: true,
    blurb: 'Bounded continuation, history, and search',
  },
  {
    view: 'system',
    label: 'System',
    icon: '◇',
    disclosure: 'primary',
    navVisible: true,
    blurb: 'Canonical object, capability, and configuration truth',
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
