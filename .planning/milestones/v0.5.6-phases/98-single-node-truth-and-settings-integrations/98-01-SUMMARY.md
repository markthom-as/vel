# Phase 98 Summary

## Completed slices

1. Core settings and onboarding truth
   - added persisted `core_settings` at the backend settings seam
   - exposed operator identity/profile, developer mode, and setup bypass in `System`
   - gated the floating composer on minimum Core readiness
   - raised a stable onboarding/setup nudge instead of relying on frontend-local implication

2. Provider configuration and routing truth
   - surfaced discovered LLM profiles in `System -> Integrations -> Providers`
   - exposed default-chat and fallback routing through the persisted settings seam
   - kept profile routing on the backend-owned `/api/settings` contract instead of introducing UI-local state

3. Google and Todoist lifecycle truth
   - exposed Google client credential replacement and OAuth reconnect/start in `System`
   - exposed Todoist token replacement in `System`
   - kept Google/Todoist refresh/disconnect actions wired to real integration endpoints

4. `Now` and current-day contract truth
   - moved drag-to-`Next Up` commit-to-today behavior into the backend `Now` lane route
   - assigned `next_up` tasks to the operator’s current local session day instead of leaving the behavior UI-only
   - persisted `scheduled_for`, `has_due_time`, and `assigned_via_now_lane` metadata so the `Now` surface can render the committed-today truth durably

5. Assistant failure retry truth
   - added an explicit `assistant_error_retryable` contract on chat responses instead of collapsing all assistant failures into one generic string
   - surfaced retryable thread/runtime failures through a retry-capable inline feedback path in the shell
   - raised a truthful local nudge when a thread-owned assistant reply failed in a retryable way, while preserving the conversation/thread target for operator inspection

6. Thread call-mode truth
   - added persisted `call_mode_active` state to the canonical conversation contract instead of inventing a separate browser-only voice session model
   - exposed `Start call` / `End call` on the thread header through the existing `/api/conversations/:id` patch seam
   - kept browser speech-to-text on the normal assistant-entry path while enabling browser-local text-to-speech only for call-mode threads
   - documented the thread-owned call-mode rule in the nearby chat API and daily-use docs

7. Multimodal attachment truth
   - kept attachment transport on the existing assistant-entry seam instead of inventing upload-only side channels
   - preserved `mime_type` plus basic file facts (`size_bytes`, `last_modified_ms`) when the web composer submits file or image attachments
   - surfaced persisted attachment chips in rendered thread messages so attachment state no longer hides inside raw JSON

## Verification checkpoint

- `npm --prefix clients/web test -- --run src/views/system/SystemView.test.tsx src/shell/MainPanel/MainPanel.test.tsx src/data/operator.test.ts src/core/MessageComposer/MessageComposer.test.tsx`
- `cargo test -p veld chat_settings_patch_persists_core_settings`
- `cargo test -p veld chat_settings_patch_persists_web_settings`
- `npm --prefix clients/web test -- --run src/views/now/NowView.test.tsx`
- `cargo test -p veld late_night_current_day_bucket_keeps_commitments_in_play -- --exact`
- `cargo test -p veld now_endpoint_prioritizes_urgent_todoist_tasks -- --exact`
- `npm --prefix clients/web test -- --run src/views/now/components/AssistantEntryFeedback.test.tsx src/shell/MainPanel/MainPanel.test.tsx src/core/MessageComposer/MessageComposer.test.tsx`
- `cargo test -p veld now_task_lane_patch_persists_lane_membership_and_completion_state`
- `cargo test -p veld now_task_lane_patch_assigns_next_up_commitment_to_current_day`
- `cargo test -p veld assistant_error_retryable_flags_retryable_provider_failures`
- `cargo test -p veld assistant_error_retryable_rejects_configuration_and_auth_failures`
- `cargo test -p veld --test chat_conversation_call_mode`
- `cargo test -p veld --test chat_assistant_entry conversation_list_includes_thread_row_metadata -- --exact`
- `cargo test -p veld --test chat_assistant_entry existing_conversation_and_missing_model_still_persist_user_message_safely -- --exact`
- `npm --prefix clients/web test -- --run src/views/threads/ThreadView.test.tsx src/data/chat.test.ts src/shell/MainPanel/MainPanel.test.tsx`
- `npm --prefix clients/web test -- --run src/core/MessageComposer/MessageComposer.test.tsx src/core/MessageRenderer/MessageRenderer.test.tsx src/data/chat.test.ts`
- `cargo test -p veld --test chat_assistant_entry assistant_entry_honors_explicit_intent_and_attachments -- --exact`

## Deferred follow-up

1. Current-day boundary enrichment
   - the live runtime now owns current-day truth, but it still uses the existing local-session rollover substrate
   - explicit bedtime/end-of-day and sunrise fallback signals are not yet present in the runtime, so that richer boundary rule remains deferred instead of being simulated in the web shell

2. Richer multimodal objects
   - no remaining attachment contract gap is identified in the current thread/web seam
   - broader upload/object-binding work should stay deferred until a later milestone needs durable file objects instead of bounded attachment facts

## Result

Phase 98 is complete. `0.5.6` can now treat Core setup, provider routing, integration lifecycle controls, backend-owned `Now` truth, retryable assistant failure handling, persisted thread call mode, and richer attachment transport/rendering as the stable foundation for Phase 99 polish work.
