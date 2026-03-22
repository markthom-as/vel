# Product Goals and Principles

## Product goals

The skill system should make Vel:

1. **Modular**
   - Features can be added, removed, upgraded, disabled, or scoped cleanly.

2. **Governable**
   - Every skill should have explicit permissions, policies, and observability.

3. **Composable**
   - Skills should be able to call other skills and participate in workflows.

4. **CLI-native**
   - Skills must be first-class in the Vel CLI rather than bolted on afterward.

5. **Context-aware but bounded**
   - Skills should consume typed context slices rather than unbounded prompt soup.

6. **Portable**
   - The core runtime should be usable from web, desktop, server, and automation environments.

7. **Testable**
   - Skills should support validation, smoke tests, permission checks, and golden outputs.

8. **Incrementally adoptable**
   - Vel should be able to start with a small internal system and grow into a larger ecosystem.

## Non-goals for MVP

The MVP should **not** try to solve all of the following on day one:

- full public plugin marketplace
- arbitrary untrusted third-party code execution
- live hot-reload across every surface simultaneously
- universal external format compatibility
- complete graph planner / autonomous agent runtime
- full visual workflow editor
- remote skill distribution, signing, and trust chains
- multi-tenant enterprise policy hierarchy across orgs and sub-orgs

Those can come later. Trying to swallow them at MVP is how roadmaps get mugged by ambition.

## Design principles

### 1. Vel-native first
The internal representation must be the source of truth.

### 2. Strong interfaces over vibes
Every skill should declare inputs, outputs, requirements, and permissions.

### 3. Least privilege by default
A skill should not silently inherit ambient authority.

### 4. Deterministic where possible
Gathering, normalization, policy checks, and validation should be deterministic. Use the model where actual semantic judgment is needed.

### 5. Prompts are not enough
Prompt-only skills are useful, but the system must support code hooks and typed workflows.

### 6. Context should be mounted, not dumped
Skills should receive structured context slices under explicit runtime control.

### 7. Surfaces should share one runtime contract
CLI, chat, automations, background jobs, and UI actions should all run through the same skill runtime contract whenever possible.

### 8. Compatibility is an adapter problem
External compatibility should not dictate the internal architecture.
