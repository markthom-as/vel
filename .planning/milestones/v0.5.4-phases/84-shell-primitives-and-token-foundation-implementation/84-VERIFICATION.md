# Phase 84 Verification

## Automated Checks

### Targeted shell and surface tests

Command:

```bash
npm --prefix clients/web test -- --run \
  src/shell/AppShell/AppShell.test.tsx \
  src/shell/Navbar/Navbar.test.tsx \
  src/shell/MainPanel/MainPanel.test.tsx \
  src/views/now/NowView.test.tsx \
  src/views/threads/ThreadView.test.tsx \
  src/views/system/SystemView.test.tsx
```

Result:

- 6 test files passed
- 16 tests passed

### Web build

Command:

```bash
npm --prefix clients/web run build
```

Result:

- `tsc -b` passed
- `vite build` passed

## Behavioral Claims Backed By Execution

- the shared shell still renders all three primary surfaces without route regressions
- shell chrome now includes the top band, side nudge region, and docked action bar in the live app tree
- the token and typography changes compile through the real production build

## Known Limits

- this phase does not finish the page-level rebuild of `Now`, `Threads`, or `System`
- the shell-side nudge migration is partial by design until the bounded `Now` page pass in Phase 86
