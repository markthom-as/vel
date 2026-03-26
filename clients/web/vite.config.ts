import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolveDevProxyTarget } from './src/api/devProxyTarget'

// https://vite.dev/config/
export default defineConfig(() => {
  const veldUrl = resolveDevProxyTarget(process.env)

  return {
    plugins: [react()],
    server: {
      proxy: {
        '/api': { target: veldUrl, changeOrigin: true },
        '/v1': { target: veldUrl, changeOrigin: true },
        '/ws': { target: veldUrl, ws: true },
      },
    },
  }
})
