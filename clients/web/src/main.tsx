import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import '@fontsource/ibm-plex-serif/400.css'
import '@fontsource/ibm-plex-serif/600.css'
import '@fontsource-variable/space-grotesk/wght.css'
import '@fontsource-variable/outfit/wght.css'
import './index.css'
import App from './App.tsx'
import { bootstrapEmbeddedBridgePacketRuntime } from './data/embeddedBridgeWasmRuntime'

const root = createRoot(document.getElementById('root')!)

type BootstrapState =
  | { kind: 'booting' }
  | { kind: 'ready' }
  | { kind: 'missing-runtime' }
  | { kind: 'error'; message: string }

function render(state: BootstrapState) {
  root.render(
    <StrictMode>
      {state.kind === 'ready' ? (
        <App />
      ) : (
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
            <h1 style={{ fontSize: '24px', marginBottom: '12px' }}>
              {state.kind === 'booting'
                ? 'Starting embedded Rust runtime…'
                : state.kind === 'missing-runtime'
                  ? 'Embedded Rust runtime is required'
                  : 'Embedded Rust runtime failed to start'}
            </h1>
            <p style={{ margin: 0 }}>
              {state.kind === 'booting'
                ? 'Loading the browser packet runtime before the application shell mounts.'
                : state.kind === 'missing-runtime'
                  ? (
                    <>
                      Set <code>VITE_VEL_EMBEDDED_BRIDGE_WASM_URL</code> to the generated browser module,
                      for example <code>/embedded-bridge/vel-embedded-bridge.js</code>.
                    </>
                  )
                  : state.message}
            </p>
          </div>
        </div>
      )}
    </StrictMode>,
  )
}

async function start() {
  render({ kind: 'booting' })

  try {
    const runtime = await bootstrapEmbeddedBridgePacketRuntime()
    if (runtime == null) {
      render({ kind: 'missing-runtime' })
      return
    }
    render({ kind: 'ready' })
  } catch (error) {
    render({
      kind: 'error',
      message: error instanceof Error ? error.message : 'Unknown embedded bridge bootstrap failure.',
    })
  }
}

void start()
