import type { ReactNode } from 'react';
import { ObjectRowTitleMetaBand } from '../ObjectRow';

/**
 * Temporary compatibility wrapper over the canonical `ObjectRowTitleMetaBand`.
 */
export function ItemRowTitleMetaBand({
  title,
  titleClassName,
  meta,
}: {
  title: ReactNode;
  titleClassName?: string;
  meta: ReactNode;
}) {
  return <ObjectRowTitleMetaBand title={title} titleClassName={titleClassName} meta={meta} />;
}
