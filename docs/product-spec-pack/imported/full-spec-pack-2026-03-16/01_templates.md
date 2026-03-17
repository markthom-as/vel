# Template System Spec

## Overview
Templates are reusable structured blueprints.

## Types
- Task templates (Todoist)
- Event templates (Calendar)
- Prompt templates
- Document templates
- Workflow templates

## Schema
- id
- name
- type
- payload
- parameters
- metadata
- version

## Behavior
- Preview before execution
- Parameter substitution
- Save-as-template flow

## API
POST /templates
GET /templates
PATCH /templates/:id
DELETE /templates/:id
