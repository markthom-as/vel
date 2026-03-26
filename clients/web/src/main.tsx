import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import '@fontsource/ibm-plex-serif/400.css'
import '@fontsource/ibm-plex-serif/600.css'
import '@fontsource-variable/space-grotesk/wght.css'
import '@fontsource-variable/outfit/wght.css'
import './index.css'
import App from './App.tsx'
import { bootstrapEmbeddedBridgePacketRuntime } from './data/embeddedBridgeWasmRuntime'

void bootstrapEmbeddedBridgePacketRuntime()

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
