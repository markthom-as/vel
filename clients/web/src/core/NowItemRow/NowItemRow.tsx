import type { ReactNode } from 'react';
import { ObjectRowFrame, ObjectRowLayout, type ObjectRowDensity, type ObjectRowTone } from '../ObjectRow';

export type NowItemRowSurface = 'brand' | 'warm' | 'muted' | 'emphasis' | 'risk' | 'ghost' | 'queue';
export type ItemPillShellKind = 'compact' | 'sectionHeader' | 'laneRow' | 'comfortable' | 'rowButton';

function mapSurface(surface: NowItemRowSurface): ObjectRowTone {
  switch (surface) {
    case 'brand':
      return 'accent';
    case 'warm':
    case 'risk':
      return 'warning';
    case 'emphasis':
      return 'emphasis';
    case 'ghost':
      return 'ghost';
    case 'muted':
    case 'queue':
    default:
      return 'neutral';
  }
}

function mapDensity(shell: ItemPillShellKind): ObjectRowDensity {
  switch (shell) {
    case 'compact':
      return 'compact';
    case 'comfortable':
      return 'comfortable';
    case 'rowButton':
      return 'button';
    case 'sectionHeader':
      return 'sectionHeader';
    case 'laneRow':
    default:
      return 'standard';
  }
}

/**
 * Temporary compatibility wrapper over the canonical `ObjectRowFrame`.
 * Remove once `Now`/`Threads`/`System` call the shared primitive directly.
 */
export function NowItemRowShell({
  surface = 'muted',
  shell = 'laneRow',
  as: Comp = 'div',
  className,
  children,
}: {
  surface?: NowItemRowSurface;
  shell?: ItemPillShellKind;
  as?: 'div' | 'article';
  className?: string;
  children: ReactNode;
}) {
  return (
    <ObjectRowFrame
      as={Comp}
      tone={mapSurface(surface)}
      density={mapDensity(shell)}
      className={className}
    >
      {children}
    </ObjectRowFrame>
  );
}

/**
 * Temporary compatibility wrapper over the canonical `ObjectRowLayout`.
 */
export function NowItemRowLayout({
  leading,
  children,
  actions,
  /** `stack`: vertical action column (default). `inline`: single row of pills (e.g. dense headers). */
  actionsLayout = 'stack',
}: {
  leading?: ReactNode;
  children: ReactNode;
  actions?: ReactNode;
  actionsLayout?: 'stack' | 'inline';
}) {
  return (
    <ObjectRowLayout leading={leading} actions={actions} actionsLayout={actionsLayout}>
      {children}
    </ObjectRowLayout>
  );
}
