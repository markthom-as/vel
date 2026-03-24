# Threading Model

## Execution Lanes

1. audio callback thread
2. audio worker thread
3. STT worker
4. TTS worker
5. conversation / agent worker
6. telemetry and diagnostics lane

## Rules

### Audio Callback Thread

- no allocation
- no blocking locks
- no async waits
- no model inference
- no platform-policy decision making
- only frame movement, watermark checks, and bounded signaling

### Audio Worker Thread

- may allocate
- may resample/normalize
- may assemble segments
- may forward into STT or output pipelines

### STT / TTS / Agent Threads

- may block within bounded, observable limits
- must support cancellation
- must never back-propagate latency into the callback path

## Communication Primitives

- SPSC rings for hot-path audio movement
- bounded MPSC channels for control messages
- explicit cancellation tokens for turn-scoped work
- monotonic clocks for measurement

## Failure Cases To Design Around

- callback blocked by a lock or allocator
- model cancellation delayed until after a stale turn already spoke
- TTS continuing after barge-in
- route changes causing engine confusion about who owns output state

## Core Principle

If the callback thread ever needs the agent thread to “be fast enough,” the architecture is already wrong.
