# Phase 85 Verification

## Automated Checks

### Targeted primitive and surface tests

Command:

```bash
npm --prefix clients/web test -- --run \
  src/core/MessageRenderer/MessageRenderer.test.tsx \
  src/shell/MainPanel/MainPanel.test.tsx \
  src/views/now/NowView.test.tsx \
  src/views/threads/ThreadView.test.tsx \
  src/views/system/SystemView.test.tsx
```

Result:

- 5 test files passed
- 19 tests passed

### Web build

Command:

```bash
npm --prefix clients/web run build
```

Result:

- `tsc -b` passed
- `vite build` passed

## Behavioral Claims Backed By Execution

- shared primitive indirection does not break existing `Now`, `Threads`, `System`, or rendered message surfaces
- the new canonical row/card primitives compile through the production client build
- compatibility wrappers are thin enough to preserve current behavior while centralizing style authority

## Known Limits

- phase 85 does not yet apply the primitives deeply enough to finish page-level redesign
- metric strips still exist; they are only toned down, not fully retired
