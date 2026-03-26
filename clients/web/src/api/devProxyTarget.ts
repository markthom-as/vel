export function resolveDevProxyTarget(env: Record<string, string | undefined>): string {
  return env.VELD_URL ?? env.VITE_API_URL ?? 'http://127.0.0.1:4130'
}
