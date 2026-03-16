export function blendWeights(weights: number[]): number[] {
  const total = weights.reduce((a, b) => a + b, 0) || 1;
  return weights.map((w) => w / total);
}
