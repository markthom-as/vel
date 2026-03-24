# Risks

## Technical Risks

- echo cancellation quality varies sharply by platform path
- callback starvation or accidental allocation causes audio glitches
- model latency spikes make “duplex” feel fake even if technically correct
- cancellation races allow stale TTS output to leak after interruption
- route and interruption events produce hard-to-reproduce state bugs

## Product Risks

- the system sounds responsive in quiet demos but degrades badly in real rooms
- platform differences encourage hidden divergence in user-visible behavior
- call mode becomes a special-case interaction model instead of a truthful extension of threads

## Planning Risks

- `0.5.6` scope may shift the exact call-mode entry point before this milestone starts
- desktop proving can create false confidence if it is mistaken for iOS quality proof

## Mitigations

- keep adapter boundaries explicit and narrow
- prove the Rust engine on a harness path, then separately prove Apple-quality behavior on real hardware
- log and trace turn transitions, cancellations, underruns, and route/interruption events
- treat latency and glitch budgets as milestone gates rather than “future optimization”
