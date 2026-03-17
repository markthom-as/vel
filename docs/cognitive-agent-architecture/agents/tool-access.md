# Tool Access Policy

Tool access should be partitioned by agent role.
Tool access should also be explicit, deny-by-default, and incapable of self-expansion.

## Example

### Context Synthesizer
Can read:
- calendar
- location
- commitments
- device state

Cannot:
- send notifications directly
- mutate commitments without authorization

### Notification Broker
Can:
- deliver notifications
- pick channel
- log delivery result

Cannot:
- rewrite policy
- fabricate context

## Principle

Least privilege. Always.
Because "just give the model everything" is how you end up with a spooky little tyrant.

## Hard Rules

- tools are granted by explicit allowlist, not by convention
- subagents cannot widen their own permissions
- unknown tool requests fail closed
- external access should be scoped by tool, action, host, path, or resource where possible
- raw third-party secrets should stay behind capability brokers or boundary-time injection layers

## Capability Boundary Pattern

Preferred pattern:

```text
agent intent
-> capability check
-> narrow resource match
-> point-of-use credential injection
-> execution
-> traced result
```
