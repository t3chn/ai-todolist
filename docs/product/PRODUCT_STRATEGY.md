# Product Strategy

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Strategic verdict

Build a Telegram-first AI action assistant, not a generic task manager and not a standalone native app yet.

The current project already has a strong Telegram-native base: natural-language task parsing, voice input, vague task clarification, duplicate detection, reminders, morning brief, weekly review, stale-task nudges, tags/grouping, subscription mechanics, referrals, support/admin flows, and Telegram Stars payments.

The next product step is not adding more random features. The next product step is packaging the value ladder, clarifying the UX split, and making the bot useful enough for daily retention while reserving the real planning/execution layer for Pro.

## Product thesis

`ai-todolist` should be positioned as:

> AI action assistant inside Telegram: turns thoughts, voice notes, forwarded messages, and messy tasks into a clear execution flow.

This is stronger than "AI todo list" because the product should not merely store tasks. It should help users move from vague intent to completed action.

## Core jobs-to-be-done

1. Capture messy intent quickly.
   - Text message.
   - Voice note.
   - Forwarded message.
   - Natural-language reminder.
   - Draft/follow-up from a task.

2. Clarify vague tasks into next actions.
   - Detect broad/vague tasks.
   - Suggest a concrete next action.
   - Ask only when clarification is necessary.

3. Bring tasks back at the right time.
   - Due dates.
   - Reminder nudges.
   - Snooze.
   - Morning brief.
   - Stale task nudges.
   - Weekly review.

4. Help the user decide what to do next.
   - Today view.
   - Top-3 focus.
   - Priority.
   - Duration/effort.
   - AI Plan My Day.
   - Backlog cleanup.
   - Weekly planning.

5. Help the user close loops.
   - Done/delete/edit.
   - Follow-up reminders.
   - Reply-later from forwarded messages.
   - Completion celebration.
   - Stale-task pruning.

## Product boundaries

### In scope

- Solo personal productivity.
- Telegram-native task capture.
- Voice-to-task.
- Reminder and snooze workflows.
- Daily and weekly review.
- AI clarification.
- AI planning.
- Recurring tasks.
- Projects/areas.
- Forwarded-message-to-task/follow-up.
- Telegram Mini App for visual planning later.

### Out of scope for now

- Standalone iOS/Android app.
- Team/shared task management.
- Full project management suite.
- Kanban boards.
- Comments/attachments/workspaces.
- Deep calendar product before duration/priorities exist.
- General-purpose message drafting unrelated to tasks.
- B2B/team product.

## Repackaging of existing features

Existing message drafting should not be marketed as a separate general AI-writing assistant.

Repackage it as:

- Follow-up drafting from a task.
- Reply-later workflow.
- Draft response for a reminder.
- Create task from message and optionally draft a reply.

This keeps the product focused on execution instead of becoming a generic assistant.

## Differentiation

Most todo products are strongest in visual planning surfaces.
Telegram bots are strongest in capture and conversational nudges.

The product should win by combining:

- Ultra-fast Telegram capture.
- Voice-first task creation.
- AI clarification of vague intent.
- Forwarded-message ingestion.
- Timely nudges.
- Lightweight planning.
- Later: Mini App planning surface.

## Positioning line options

Option A:
> Turn Telegram messages, voice notes, and thoughts into a clear action plan.

Option B:
> Your AI action assistant inside Telegram.

Option C:
> Capture anything in Telegram. Let AI turn it into tasks, reminders, and next actions.

Preferred:
> Your AI action assistant inside Telegram.

Supporting line:
> Send a thought, voice note, or forwarded message — the bot turns it into a task, reminder, or follow-up.

## Product principles

1. Capture should be instant.
2. Planning should be visual when chat becomes too cramped.
3. AI should reduce thinking, not add conversation overhead.
4. Free should be useful enough to build habit.
5. Pro should unlock planning and execution, not just higher limits.
6. Community Bonus should support growth but never replace Pro.
7. Avoid native app complexity until Mini App value is proven.
8. Avoid team features until solo paid retention is proven.
9. Every new feature must either improve capture, decision, reminder, or completion.
10. Do not add features that turn the bot into a generic assistant.

## Main risk

The product can easily become "a bot for everything": tasks, drafting, planning, reminders, stats, productivity coaching, calendar, teams, and integrations.

The antidote is a strict product lens:

- Does this help capture?
- Does this help clarify?
- Does this help remind?
- Does this help decide?
- Does this help complete?

If not, defer.
