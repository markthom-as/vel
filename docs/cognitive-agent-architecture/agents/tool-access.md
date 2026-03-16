# Tool Access Policy

Tool access should be partitioned by agent role.

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
