export function damp(current: number, target: number, factor = 0.12): number {
  return current + (target - current) * factor;
}
