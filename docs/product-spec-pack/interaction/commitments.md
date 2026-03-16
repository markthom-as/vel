# Commitment Model

Commitments are the **core unit of Vel**.

Everything revolves around commitments.

## Lifecycle

```
create
→ pending
→ active
→ completed
→ archived
```

## Fields

```
id
title
description
due_time
priority
status
created_at
updated_at
```

## Actions

- create
- resolve
- snooze
- escalate