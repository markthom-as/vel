import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

const VELD_URL = process.env.VELD_URL ?? 'http://127.0.0.1:4130'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': { target: VELD_URL, changeOrigin: true },
      '/v1': { target: VELD_URL, changeOrigin: true },
      '/ws': { target: VELD_URL, ws: true },
    },
  },
})
