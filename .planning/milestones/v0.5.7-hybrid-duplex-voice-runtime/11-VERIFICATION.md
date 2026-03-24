# Verification

## Automated Verification

- unit tests for ring-buffer correctness, segment state machines, cancellation semantics, and turn transitions
- integration tests for end-to-end duplex loop behavior through a proving adapter or fixture harness
- stress tests for repeated barge-in and rapid cancel/restart behavior

## Execution-Backed Verification

- harness or desktop run that captures metrics for normal duplex turns
- real iOS-device run for interruption, route-change, and speaker/mic path behavior
- focused manual exercise of thread call mode from start -> speak -> interrupt -> recover

## Metrics To Capture

- interruption reaction latency
- first partial transcript latency
- utterance-finalization latency
- first-audio-out latency
- underrun / overflow counts
- canceled-audio leakage after interruption

## Evidence Artifacts

- trace logs with stable run ids or equivalent correlation
- latency summaries
- route/interruption notes
- manual proof notes for real-device testing

## Closing Rule

No phase or milestone may be described as complete unless the relevant verification command or manual run actually happened and its limits are recorded.
