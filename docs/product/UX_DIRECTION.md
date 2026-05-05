# UX Direction

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Strategic UX decision

Keep the product Telegram-first.

Do not build a standalone native app yet.

Use:

- Telegram bot for capture, quick actions, voice, reminders, nudges, and conversational flows.
- Telegram Mini App later for planning, review, task editing, billing, and settings.

## Why bot-first

Chat is the best interface for quick capture:

- "Call mom tomorrow at 5pm."
- "Buy groceries."
- Voice note while walking.
- Forwarded message that needs follow-up.
- Quick done/snooze action.

The bot should be extremely fast at:

- Capture.
- Clarification.
- Reminder.
- Snooze.
- Completion.
- Daily nudge.

## Why not text-only forever

Chat is weak for:

- Reviewing many tasks.
- Editing multiple fields.
- Planning a week.
- Comparing priorities.
- Managing projects/areas.
- Understanding backlog.
- Billing/settings.

The product will eventually need a visual planning surface.

## Why Mini App before native app

Telegram Mini App keeps the product inside the same user habit and distribution loop.

A native app would add:

- Separate install friction.
- Store review.
- More maintenance.
- Separate auth/session complexity.
- More design surface.
- Premature product complexity.

Mini App is the reversible MVP path.

## Surface split

### Bot surface

Use for:

- Task capture.
- Voice input.
- Forwarded-message handling.
- AI clarification.
- Done/delete/snooze.
- Reminder notifications.
- Morning brief.
- Weekly review summary.
- Quick paywall moments.
- Support.
- Lightweight settings.

### Mini App surface

Use for:

- Today planner.
- Upcoming view.
- Week view.
- Projects/areas.
- Task detail editing.
- Recurring task management.
- Priority/duration editing.
- Backlog cleanup.
- Subscription management.
- Community Bonus verification.
- Usage/limits view.

## First-run UX

Preferred onboarding:

1. Welcome:
   - "Send a task by text or voice."
   - Avoid long feature list.

2. First task:
   - User sends task.
   - Bot parses and shows result.
   - Bot asks for reminder if missing.

3. First Today:
   - Bot shows today's list.
   - Shows Done/Snooze buttons.

4. Value explanation:
   - "Free captures and reminds."
   - "Bonus gives extra limits."
   - "Pro plans your day and routines."

5. Trial offer:
   - Show after first successful task, not before.

## Everyday UX loop

Morning:

- Send brief.
- Show overdue/today/no-date summary.
- Offer "choose top 3" or "Plan My Day" if Pro.

During day:

- Accept new tasks.
- Voice capture.
- Reminder.
- Snooze/done.

Evening/week:

- Stale task nudge.
- Weekly review.
- Cleanup suggestions.

## UX copy principles

- Do not sound like a project management app.
- Use action language.
- Be brief in bot messages.
- Use buttons for common decisions.
- Ask clarifying questions only when useful.
- Prefer "I can make this concrete" over generic AI language.
- Make upgrade prompts contextual.

## Example bot flows

### Capture

User:
> Call Alex tomorrow after lunch

Bot:
> Added: Call Alex
> Due: Tomorrow, 13:00
> Reminder: 30 min before
> Buttons: Done / Edit / Snooze

### Vague task

User:
> Work on marketing

Bot:
> This is broad. Want to turn it into a next action?
> Suggestions:
> 1. Draft 3 post ideas
> 2. Review current campaign metrics
> 3. Write next Telegram post
> Buttons: Use 1 / Use 2 / Use 3 / Keep as is

### Pro Plan My Day

User:
> Plan my day

Bot:
> Plan My Day is Pro. Free helps you capture tasks; Pro helps you decide what to do first.
> Start 7-day Pro Trial?
> Buttons: Start Trial / Maybe later

### Community Bonus

Bot:
> Want extra Free limits?
> Subscribe to our Telegram channels and tap Check Bonus.
> Bonus gives extra AI/voice quota, but Pro unlocks planning and routines.
> Buttons: Open channels / Check Bonus

## Mini App MVP UX

Required screens:

1. Today.
2. Upcoming.
3. Week.
4. Projects/Areas.
5. Task Detail.
6. Subscription & Bonus.

Do not build:

- Kanban.
- Team workspaces.
- Complex analytics.
- Drag-and-drop calendar.
- Native app navigation patterns.
- Integrations marketplace.

## UX success criteria

- A new user can create a useful task within 30 seconds.
- A returning user can understand today's priorities quickly.
- A Free user keeps using the bot after trial.
- A Pro user uses Plan My Day or recurring tasks repeatedly.
- Community Bonus users understand that Bonus is not Pro.
