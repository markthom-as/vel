# Trust Model

Vel's highest product constraint is trust.

## Trust Requirements

- explain why a suggestion appeared
- show uncertainty when confidence is low
- never fake integration state
- never imply action was taken when it was not
- preserve auditability for high-impact automation
- never expose raw secrets to an agent when a mediated capability can do the job
- reject unknown or unsupported actions safely by default
- require evidence from execution, traces, or tests before claiming a system behavior is fixed
- treat code modification as a high-impact action: require explicit writable scope, diff visibility, and verification before applying it

## User Control

The user must be able to:

- override suggestions
- inspect why a reminder fired
- tune or disable proactive behaviors
- review recent system actions
- understand what capabilities an agent had when it acted
- understand what repository or config surfaces an agent could read or write
- revoke or narrow future capability access

## Motto

Useful, inspectable, reversible.
Anything else is just manipulative middleware in a nice coat.
