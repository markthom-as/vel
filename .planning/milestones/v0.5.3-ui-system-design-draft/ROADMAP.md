# Roadmap: Vel `v0.5.3` UI System Design Draft

## Status

Draft milestone packet. Not yet active roadmap authority.

## Milestone Framing

This milestone is the full UI/UX definition pass for the next web line.

Its purpose is to lock:

- product paradigms
- interaction rules
- theming and visual language
- component-system contracts
- page-level layout and UX application

before implementation work begins.

This milestone exists because the current web client has a reusable technical structure, but not yet a sufficiently explicit design-system and interaction-system contract to keep future surface work consistent.

The design target should preserve the accepted product law:

- `Now` stays execution-first, compact, and decision-supporting
- `Threads` stays continuity- and object-grounded rather than generic chat-first
- `System` stays structural and authoritative rather than admin-sprawl

## Scope Guardrails

`v0.5.3` is only about design-definition and contract lock:

- define the product paradigms for all first-class surfaces
- define global UX states, actions, and interaction patterns
- define the shared theme, typography, iconography, spacing, and color system
- define component primitives and composition rules
- define page-by-page layout/UX contracts for `Now`, `Threads`, and `System`
- identify implementation sequencing for a later execution line

Do not widen this milestone into:

- implementation-driven redesign-by-drift
- backend schema renegotiation unless a design contract proves a boundary bug
- new providers or new top-level surfaces
- speculative workflow-builder or planner-studio work
- Apple implementation

## Deliverables

The milestone should end with these durable design artifacts:

- [00-SOURCE-INTEGRATION-NOTES.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/00-SOURCE-INTEGRATION-NOTES.md)
- [78-PARADIGM-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/78-PARADIGM-SPEC.md)
- [79-INTERACTION-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md)
- [80-FOUNDATION-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/80-FOUNDATION-SPEC.md)
- [81-COMPONENT-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.3-ui-system-design-draft/81-COMPONENT-SPEC.md)
- page-level UI specs for `Now`, `Threads`, and `System`
- implementation sequencing and acceptance criteria for the follow-on execution milestone

## Requirement Buckets

| ID | Description |
|----|-------------|
| PARADIGM-53-01 | Each top-level surface has a clearly defined operating model, boundary, and escalation rule. |
| INTERACTION-53-01 | Global UX states, actions, feedback loops, disclosure patterns, and selection rules are consistent across the client. |
| FOUNDATION-53-01 | Theme, typography, iconography, spacing, color semantics, and object/project color mapping are globally defined. |
| COMPONENT-53-01 | Reusable component primitives have stable roles, variants, and composition rules. |
| SURFACE-53-01 | `Now`, `Threads`, and `System` each receive page-level layout and UX contracts derived from the shared specs. |
| IMPLEMENT-53-01 | The next execution milestone can implement the UI pass without re-litigating core design decisions. |

## Proposed Phases

- [ ] **Phase 78: Product paradigms and surface doctrine** - define what each surface is for, what belongs inline, and what escalates.
- [ ] **Phase 79: Interaction system and UX-state law** - define actions, feedback, transitions, selection/disclosure rules, and state handling.
- [ ] **Phase 80: Foundation design system and theming** - lock typography, icons, spacing, token model, and semantic/object/project color rules.
- [ ] **Phase 81: Component-system contract** - define primitives, variants, and composition constraints.
- [ ] **Phase 82: Page-level UI specs** - apply the first four specs to `Now`, `Threads`, and `System`.
- [ ] **Phase 83: Implementation planning and acceptance lock** - turn the approved design packet into an execution-ready plan.

## Execution Order

Planned sequence:

`78 -> 79 -> 80 -> 81 -> 82 -> 83`

This order is intentional:

1. product behavior first
2. interaction logic second
3. visual system third
4. components fourth
5. page application fifth
6. implementation planning last

## Success Criteria

1. The repo has one explicit answer for how the web client should look, behave, and communicate meaning.
2. Color semantics are stable across states, object types, and projects.
3. Components become composable because their roles and constraints are defined before surface-specific implementation.
4. Page discussions become concrete and bounded because every page spec inherits from the same paradigm, interaction, and foundation docs.
5. The follow-on implementation milestone can focus on execution quality instead of re-deciding design law.
6. The imported 2026-03-23 UI spec packs are captured inside repo planning docs instead of remaining external-only references.
7. Clickable mockups and browser-proof artifacts exist so the milestone is interaction-validated rather than prose-only.

---
*Drafted: 2026-03-23 as a proposed successor to `v0.5.2`*
