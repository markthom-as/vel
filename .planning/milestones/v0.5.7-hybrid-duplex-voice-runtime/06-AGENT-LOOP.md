# Agent Loop

## Call-Mode Turn Loop

1. receive a finalized user segment
2. normalize it into a turn input
3. append it to thread/conversation state
4. invoke the conversation model
5. stream assistant output
6. feed output into TTS
7. publish final artifacts and metrics

## State Owned By Rust

- thread / conversation history
- active turn id
- cancel/interruption flags
- tool-call state
- memory/context injection
- output stream state

## Interrupt Handling

If interrupted:

- mark the active assistant turn interrupted
- cancel model streaming
- cancel TTS
- discard any queued output not yet played
- open the next user turn without resetting the thread

## Tool Calls

Tool use is allowed only if:

- it obeys the existing Vel policy boundaries
- it remains truthful in logs/traces
- interruption semantics are defined for long-running tool work

## Invariant

Only one active assistant turn may speak at once.

If two turns can both believe they own playback, the milestone is not done.
