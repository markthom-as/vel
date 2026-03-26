/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_VEL_EMBEDDED_BRIDGE_WASM_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
