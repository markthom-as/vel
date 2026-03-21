import type { ReactNode } from 'react';
import { SurfaceMetricChip } from '../../../core/SurfaceChips';

export function StatPill({
  label,
  value,
  detail,
  icon,
}: {
  label: string;
  value: string;
  detail?: string;
  icon: ReactNode;
}) {
  return (
    <SurfaceMetricChip>
      {icon}
      <span>{label}</span>
      <span className="text-zinc-100">{value}</span>
      {detail ? <span className="text-zinc-500">{detail}</span> : null}
    </SurfaceMetricChip>
  );
}
