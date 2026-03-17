# Workflow Engine Spec

## Overview
Workflows coordinate multi-step actions.

## Components
- Steps
- Dependencies
- State machine

## States
- planned
- running
- paused
- failed
- complete

## Features
- Retry
- Partial execution
- Audit logs

## Example
Call setup workflow:
1. Create calendar event
2. Create Zoom meeting
3. Send invite
