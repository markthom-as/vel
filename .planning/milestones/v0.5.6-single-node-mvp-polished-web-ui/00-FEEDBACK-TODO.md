# v0.6.0 Source Feedback From TODO.md

Copied directly from [TODO.md](/home/jove/code/vel/TODO.md) on 2026-03-23. Per operator instruction, the feedback is preserved verbatim for milestone planning.

```md
~Note: bullet points that start with "!" are to be ignored for a later milestone.

navbar:
 - sync icon should not be spinning unless a sync is actively happening
 - the current event and active task text should be in the same size tags at the other tags in the navbar
 - the link to docs should link to a page where the markdown is actully rendered
 - the docs link shuold better match the navlink icons next to it in terms of size and color

floating action / composer bar:
 - !cli commands should be available as /* commands with strong autocomplete / suggestions
 - sending message give API 422: /api/assistant/entry error, provider error: transport: error sending request for url (http://127.0.0.1:8012/v1/chat/completions)
 - make sure that there is a shadow when it goes over content

nudges:
 - nudge pills should take on the same styling (bg, border) as their assoxciated icons and icon rings.
 - these icon rings should be moved to the left such that they do not touch the nudge pills
 - all nudge action buttons should be the same size, open in threads should be the threads view icon 
 - nudges should be sticky and srcroll with the navbar

now:
 - the active task is repreated twice in the section header. put the icon to the right of the first occurence on the same line as the current event and remove the other one
 - active task heading should have the now icon before it
 - the active task pill should not have the now icon inside of it, but to the left like the icons for nudges
 - the inbox tag should follow the todoist inbox paradigm (tasks with no date, usually that need to be processed for additional metadata)
 - next up tasks are specifically committed tasks that are set to today
 - backlog tasks are everything else left on the day that hasn't been committed to
 - specifically make a task that the front end shows all uncompleted tasks on the now page that todist has for today
 - current and future calendar events on the current day (before next sleep / bedtime event / sunrise next day)

threads:
 - when the page first loads the default (latest) thread, please make sure that the threads state/style is set to active
 - in the current thread section header change LAST to LATEST
 - in the current thread section header, the archive button should be the same component as the standardized action chip in nudges
 - when a message from the user is sending. the message duplicates in its thread view
 - the background of the vel chat bubble needs to be darker, more muted such that the text can be read. the border is great
 - the chat interface should follow standard modern chat design  (remove the tails for now, messages should be very standard chat interface where neither users messages are full width, and are aligned oppositely with the user on the right side and other participants on the left )

system:
 - left bar should be sticky with navbar and stay locked as scrolling
 - integrations and services should be displayed with their corresponding logo icons
 - services that are not available on this system (services that do not have an additional non-vel internal source) should be hidden in a collapsed container
 - for activity, remove the grounding upcoming events section
 - activity items should be in a tight table with as much info as possible on a single line. most sections can be this table format
 - make sure that clients have a location field in config and in backend, it should be able to be autoset with some open api
 - current mode should not have text overlapping
 - writeback and writegrant info should not be shown here, but can be showin in developer mode
 - deep technical settings should be hidden behind developer mode toggle that is on the same line as SYSTEM at the top of the sidebar

general functionality:
 - google and todoist integrations working, fulling configurable and editble in settings, resulting data shows up properly in now
 - multimodal chat working both with local agent via llama.ccp. claude, and openai (api keys and openai-oauth)
 - chat backing / priority / etc fully configurable in system
 - full drag and drop for tasks on now page working
 - llm-driven task creation / tagging / editing (voice included)
```
