import { cn } from '../cn';
import { itemPillCard } from '../itemPill';
import type { ObjectRowDensity, ObjectRowTone } from './ObjectRow';

const rowToneMap = {
  neutral: 'muted',
  accent: 'brand',
  warning: 'warm',
  emphasis: 'emphasis',
  ghost: 'ghost',
  selected: 'muted',
} as const;

const rowDensityMap = {
  compact: 'compact',
  standard: 'laneRow',
  comfortable: 'comfortable',
  button: 'rowButton',
  sectionHeader: 'sectionHeader',
} as const;

export function objectRowFrameClass(tone: ObjectRowTone = 'neutral', density: ObjectRowDensity = 'standard') {
  if (tone === 'selected') {
    return 'relative w-full overflow-hidden rounded-[20px] border border-zinc-100 bg-zinc-100 p-3 text-left text-zinc-950 shadow-none transition';
  }

  if (tone === 'activeBrand') {
    return cn(
      itemPillCard('emphasis', density === 'button' ? 'rowButton' : rowDensityMap[density]),
      'border-[#ff6b00] bg-[color:var(--vel-color-panel-2)]/48 shadow-[0_0_0_1px_rgba(255,107,0,0.74),0_0_36px_rgba(255,107,0,0.24),inset_0_0_0_1px_rgba(255,190,130,0.22)] before:pointer-events-none before:absolute before:inset-0 before:rounded-[20px] before:bg-[linear-gradient(120deg,rgba(255,145,66,0.12),transparent_45%,rgba(255,145,66,0.08))] before:content-[\'\']',
    );
  }

  return itemPillCard(rowToneMap[tone], rowDensityMap[density]);
}
