import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import '@fontsource/ibm-plex-serif/400.css'
import '@fontsource/ibm-plex-serif/600.css'
import '@fontsource-variable/space-grotesk/wght.css'
import '@fontsource-variable/outfit/wght.css'
import './index.css'
import App from './App.tsx'
import { bootstrapEmbeddedBridgePacketRuntime } from './data/embeddedBridgeWasmRuntime'

async function start() {
  const runtime = await bootstrapEmbeddedBridgePacketRuntime()

  if (runtime == null) {
    createRoot(document.getElementById('root')!).render(
      <StrictMode>
        <div
          style={{
            minHeight: '100vh',
            display: 'grid',
            placeItems: 'center',
            padding: '24px',
            background: '#111111',
            color: '#f5f5f5',
            fontFamily: '"Space Grotesk Variable", sans-serif',
          }}
        >
          <div style={{ maxWidth: '720px', lineHeight: 1.6 }}>
            <h1 style={{ fontSize: '24px', marginBottom: '12px' }}>Embedded Rust runtime is required</h1>
            <p style={{ margin: 0 }}>
              Set <code>VITE_VEL_EMBEDDED_BRIDGE_WASM_URL</code> to the generated browser module,
              for example <code>/embedded-bridge/vel-embedded-bridge.js</code>.
            </p>
          </div>
        </div>
      </StrictMode>,
    )
    return
  }

  createRoot(document.getElementById('root')!).render(
    <StrictMode>
      <App />
    </StrictMode>,
  )
}

void start()
