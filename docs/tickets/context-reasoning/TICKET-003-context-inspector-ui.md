---
title: Context Inspector UI
status: open
---

# Goal

Expose Vel belief state to user.

# Features

Confidence-sorted belief list.

Each belief supports:

- confirm
- correct
- mark irrelevant
- suppress
- explanation

# Implementation

React component:

ContextInspectorPanel

Endpoints:

GET /context/beliefs
POST /context/feedback
