# Phase 106 Verification

## Goal-Backwards Questions

1. Can a reviewer point to the exact code paths where fake polymorphism was removed?
2. Can a reviewer confirm that `policy_config.rs` only retains policies with an active runtime consumer?
3. Is the warning posture more truthful after the change, even if some targeted allowances remain?
4. Are any leftovers explicit enough that Phase 107+ can pick them up without rediscovery?

## Evidence To Collect

- before/after search results for the removed trait and dead policy symbols
- command output from the targeted test/check commands
- a short note describing any remaining item-level `dead_code` allowances and why they remain

## Review Traps

- removing types that are only “unused today” but actually required by checked-in config or documented contracts
- replacing a blanket suppression with scattered unexplained item-level suppressions
- bundling unrelated warning cleanup into the same patch and losing reviewability
