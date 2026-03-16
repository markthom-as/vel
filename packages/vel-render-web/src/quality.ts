export type RenderQualityTier = "desktop-high" | "mobile-medium" | "mobile-low";

export function pickQualityTier(isMobile: boolean, fps?: number): RenderQualityTier {
  if (!isMobile) return "desktop-high";
  if (fps !== undefined && fps < 40) return "mobile-low";
  return "mobile-medium";
}
