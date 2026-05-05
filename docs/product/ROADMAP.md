# Product Roadmap

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Roadmap overview

| Phase | Goal | Main outcome |
|---|---|---|
| Phase 0 | Product reset | Clear positioning and value ladder |
| Phase 1 | Useful Free | Users can use the bot daily without paying |
| Phase 2 | Pro reasons | Users have obvious reasons to buy Pro |
| Phase 3 | Mini App MVP | Visual planning surface inside Telegram |
| Phase 4 | Premium AI | The bot becomes an execution assistant, not just a todo list |

## Phase 0 — Product reset

Goal: make the product explainable, sellable, and bounded before adding more features.

Scope:

1. Fix positioning:
   - From: AI todo list.
   - To: AI action assistant inside Telegram.

2. Define UX split:
   - Bot = capture, voice, quick actions, nudges.
   - Mini App = planning, review, billing, settings.

3. Define value ladder:
   - Free Forever.
   - Community Bonus.
   - Pro.
   - 7-day Pro Trial as a demonstration of Pro, not a hard product death.

4. Repackage message drafting:
   - From: general drafting.
   - To: task-based follow-up/reply-later drafting.

5. Define activation moment:
   - User creates 3 tasks.
   - User sets 1 reminder.
   - User completes 1 task.
   - User receives first useful Today or Morning Brief.

6. Add product docs:
   - Product strategy.
   - Roadmap.
   - Pricing model.
   - UX direction.
   - Metrics and experiments.
   - Decision log.

Exit criteria:

- The product can be explained in one sentence.
- Free/Bonus/Pro are clearly different.
- No one is debating native app vs Mini App for the immediate roadmap.
- Product docs are committed to the repo.

## Phase 1 — Make Free genuinely useful

Goal: Free users should get daily value and build a habit.

Scope:

1. Better onboarding:
   - Ask user to send first task.
   - Confirm parsed task.
   - Ask if they want a reminder.
   - Show Today view.
   - Explain Free, Bonus, and Pro without blocking.

2. Core Free loop:
   - Capture.
   - Clarify.
   - Remind.
   - Snooze.
   - Done.
   - Celebrate.
   - Return tomorrow.

3. Better Today:
   - Show due today.
   - Show overdue.
   - Show no-date tasks separately.
   - Offer "choose top 3" prompt.
   - Offer "clean stale tasks" prompt.

4. Free AI allowance:
   - Limited AI parsing.
   - Limited voice input.
   - Fallback to manual/basic task mode after limits.
   - No full bot shutdown.

5. Community Bonus entry:
   - Add "Get bonus" in settings.
   - Explain bonus clearly.
   - Verify channel subscription later in technical phase.
   - Bonus must not unlock Pro planning layer.

6. Basic metrics:
   - Track activation.
   - Track tasks created.
   - Track reminders set.
   - Track tasks completed.
   - Track D1/D7 retention.
   - Track trial start.
   - Track paid conversion.

Exit criteria:

- Free user can keep using the bot after trial.
- Bot remains useful even with limited AI quota.
- Users understand what Pro gives.
- Onboarding leads to first successful task.

## Phase 2 — First Pro-worthy features

Goal: create strong purchase intent.

Pro features to prioritize:

1. Recurring tasks / routines.
   - Daily.
   - Weekly.
   - Monthly.
   - Every N days/weeks.
   - Natural language input.

2. Priorities.
   - P1/P2/P3 or Important/Normal/Later.
   - Used in Today and Plan My Day.

3. Duration / effort.
   - 5 min, 15 min, 30 min, 1h.
   - Natural-language extraction.
   - Required for better planning.

4. AI Plan My Day.
   - Select top tasks.
   - Order tasks.
   - Consider due dates, overdue tasks, priority, duration, and stale tasks.
   - Offer a realistic plan.
   - Let user accept/edit.

5. Weekly Review Pro.
   - Completed tasks.
   - Added tasks.
   - Stale tasks.
   - Suggested cleanup.
   - Top focus for next week.
   - Celebration and reflection.

6. Backlog cleanup.
   - Detect stale/no-date tasks.
   - Suggest delete/keep/reschedule/split.
   - Batch interaction.

7. Forwarded message → task/follow-up.
   - Create task from forwarded message.
   - "Reply later."
   - "Remind me tomorrow."
   - "Draft reply."
   - "Follow up with this person."

Paywall principle:

- Do not block the whole bot.
- Show paywall at Pro intent moments:
  - recurring task request;
  - Plan My Day;
  - full weekly review;
  - backlog cleanup;
  - advanced reminders;
  - project/area planning;
  - Mini App planner;
  - forwarded-message follow-up beyond Free/Bonus quota.

Exit criteria:

- Users encounter Pro paywalls because they asked for valuable functionality.
- At least 3 Pro features are used during trial.
- Trial-to-paid conversion can be measured by feature.
- Pro is more than quota expansion.

## Phase 3 — Telegram Mini App MVP

Goal: solve planning and review UX without building a standalone native app.

Scope:

1. Today screen.
   - Due today.
   - Overdue.
   - Top 3.
   - Done/snooze.
   - AI Plan button.

2. Upcoming screen.
   - Tomorrow.
   - This week.
   - No date.
   - Overdue.

3. Week screen.
   - Simple weekly layout.
   - No complex calendar at first.
   - Good enough to review workload.

4. Projects/Areas screen.
   - Work.
   - Personal.
   - Finance.
   - Health.
   - Custom later.

5. Task detail screen.
   - Title.
   - Due date.
   - Reminder.
   - Priority.
   - Duration.
   - Project/area.
   - Recurrence.

6. Subscription and bonus screen.
   - Current plan.
   - Pro benefits.
   - Community Bonus status.
   - Check bonus.
   - Upgrade.

Out of scope for Mini App MVP:

- Drag-and-drop calendar.
- Kanban.
- Team spaces.
- Comments.
- Attachments.
- Integrations marketplace.
- Heavy analytics dashboard.

Exit criteria:

- Users open Mini App to plan, not just to browse.
- Plan My Day is easier in Mini App than in chat.
- Pro value is visible in Mini App.
- Free/Bonus users see a useful but limited planner.

## Phase 4 — Premium AI execution layer

Goal: make Pro feel like an assistant that helps the user execute.

Potential features:

1. Smart rescheduling.
   - User says "I did not manage this."
   - Bot suggests a realistic reschedule.

2. Stale-task compression.
   - Group old tasks.
   - Suggest deletion, merging, or project conversion.

3. Project decomposition.
   - Turn broad projects into next actions.
   - Ask minimal clarifying questions.

4. Focus mode.
   - Pick one task for the next 25–45 minutes.
   - Check in after.
   - Celebrate progress.

5. Calendar sync.
   - Add only after priority + duration + planning are useful.
   - Calendar should improve planning, not create complexity.

6. Personal productivity memory.
   - Learn realistic daily capacity.
   - Suggest fewer tasks if user overplans.
   - Detect patterns.

7. Smart templates.
   - Founder mode.
   - Content workflow.
   - Sales follow-up.
   - Study routine.
   - Health routine.
   - Weekly planning.

Exit criteria:

- Pro users rely on the assistant for planning and execution.
- The product becomes meaningfully different from a normal todo list.
- AI features improve completion rate, not just engagement.

## Immediate product sprint

Recommended first sprint:

1. Commit product docs.
2. Update positioning in README/CLAUDE if needed.
3. Define Free/Bonus/Pro in product docs.
4. Change product language from trial-only to Free Forever + Pro Trial.
5. Define Pro intent paywalls.
6. Prepare backlog tasks for:
   - Free useful mode.
   - Community Bonus.
   - Recurring tasks.
   - AI Plan My Day.
   - Forwarded message to task/follow-up.
7. Defer Mini App technical design until Phase 2 product signal.
