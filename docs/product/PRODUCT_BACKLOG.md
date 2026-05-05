# Product Backlog

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Purpose

This backlog translates the product strategy and roadmap into prioritized product work.

It is intentionally product-only.

Architecture, implementation details, database changes, Telegram API details, and deployment decisions should be handled later in `docs/architecture/` after this backlog is accepted.

## Product direction

`ai-todolist` is a Telegram-first AI action assistant.

Core promise:

> Turn thoughts, voice notes, forwarded messages, and messy tasks into a clear execution flow.

The product should not become:

- a generic todo list;
- a general AI assistant;
- a full project management suite;
- a native app before Telegram Mini App value is proven;
- a team/workspace product before solo retention and Pro conversion are proven.

## Backlog principles

1. Free must remain useful.
2. Community Bonus must be better than Free but clearly below Pro.
3. Pro must sell planning and execution, not only quota.
4. Bot UX is for capture and quick actions.
5. Mini App UX is for planning and review later.
6. Do not add team, calendar, native app, or integrations before Pro value is validated.
7. Every feature must improve at least one of:
   - capture;
   - clarification;
   - reminder;
   - decision;
   - completion.

---

# P0 - Product packaging and retention foundation

## P0.1 - Update public product positioning

### Problem

The product can be misunderstood as just another AI todo list.

### Goal

Make the product positioning clear and action-oriented.

### Scope

- Update README / public copy when needed.
- Use the phrase:
  - "AI action assistant inside Telegram"
- Avoid positioning as:
  - "AI todo list" only;
  - "productivity chatbot";
  - "general AI assistant".

### Acceptance criteria

- The product can be explained in one sentence.
- README and product docs use consistent language.
- Message drafting is positioned only as task/follow-up drafting.

### Priority

P0

---

## P0.2 - Free Forever product mode

### Problem

A hard trial-only model can kill habit formation.

### Goal

Make the bot useful even after the Pro trial ends.

### Scope

Free should include:

- text task capture;
- basic task creation;
- Today / Tasks / Inbox;
- done/delete/edit;
- basic reminders;
- basic snooze;
- basic duplicate warning;
- simple tags;
- basic morning brief;
- limited AI parsing;
- limited voice input;
- fallback to manual/basic mode after AI limits.

### Out of scope

- recurring tasks;
- full AI Plan My Day;
- full Weekly Review Pro;
- advanced reminder rules;
- full Mini App planner;
- calendar sync.

### Acceptance criteria

- User can continue using the bot without Pro.
- AI/voice limits do not make the entire bot unusable.
- Upgrade prompts appear at Pro intent moments, not basic capture.

### Priority

P0

---

## P0.3 - First successful task onboarding

### Problem

Users need to reach value quickly.

### Goal

A new user should create a useful task within 30 seconds.

### Scope

Onboarding flow:

1. Welcome.
2. Ask user to send a task by text or voice.
3. Parse task.
4. Confirm created task.
5. Offer reminder.
6. Show Today.
7. Explain Free / Bonus / Pro briefly after first value.

### Acceptance criteria

- First-run flow does not start with a paywall.
- User creates first task before seeing long explanations.
- Trial is offered after first successful task, not before.
- User understands what to do next.

### Priority

P0

---

## P0.4 - Community Bonus definition

### Problem

Community Bonus can cannibalize Pro if it unlocks too much.

### Goal

Use Telegram channel subscription as a growth loop without making it equal to Pro.

### Scope

Community Bonus may include:

- higher AI parsing quota;
- higher voice quota;
- higher active-task limit;
- extra templates;
- community badge;
- early access to experiments;
- one extra weekly insight;
- optional one-time Pro trial extension.

Community Bonus must not include:

- recurring tasks;
- full AI Plan My Day;
- full Weekly Review Pro;
- full Mini App planner;
- calendar sync;
- advanced reminder rules;
- smart backlog cleanup;
- full projects/areas.

### Acceptance criteria

- Bonus value is clear.
- Bonus is visibly better than Free.
- Bonus does not remove the reason to buy Pro.
- Product docs and bot copy explain the difference.

### Priority

P0

---

## P0.5 - Pro intent paywall model

### Problem

Generic subscription prompts are weaker than contextual upgrade moments.

### Goal

Show Pro upgrade prompts when the user expresses a Pro-level intent.

### Pro intent moments

- User asks for recurring task.
- User asks to plan the day.
- User opens full weekly review.
- User wants backlog cleanup.
- User tries advanced reminder rules.
- User opens full Mini App planner.
- User exceeds forwarded-message workflow quota.
- User wants projects/areas or priority planning.

### Acceptance criteria

- Basic task capture is never blocked by Pro paywall.
- Paywall copy explains the outcome, not just the feature.
- Each paywall can be measured by trigger.
- Trial start is tied to Pro value.

### Priority

P0

---

## P0.6 - Product metrics baseline

### Problem

Without product metrics, roadmap decisions will be opinion-based.

### Goal

Define metrics before expanding the product.

### Required metrics

Activation:

- first task created;
- 3 tasks created;
- first reminder set;
- first task completed;
- first Today view;
- first morning brief received.

Engagement:

- tasks created per active user;
- tasks completed per active user;
- reminders set;
- snoozes used;
- voice usage;
- AI parsing usage;
- Today usage;
- weekly review actions.

Retention:

- D1;
- D7;
- D14;
- D30;
- weekly active users.

Monetization:

- trial starts;
- trial-to-paid conversion;
- conversion by Pro trigger;
- paid retention;
- churn.

Community Bonus:

- bonus prompt views;
- channel click-through;
- successful bonus checks;
- bonus-to-Pro conversion.

### North Star candidate

Weekly completed tasks per activated user.

### Priority

P0

---

# P1 - First Pro-worthy product layer

## P1.1 - Recurring tasks / routines

### Problem

Users need repeated tasks and routines. This is one of the clearest paid productivity features.

### Goal

Make recurring tasks the first obvious Pro feature.

### Scope

Support natural language like:

- every day at 9;
- every Monday;
- every weekday;
- every 2 weeks;
- every month;
- every 1st day of the month.

### Product behavior

- Free user requesting recurrence sees Pro paywall.
- Pro user can create and manage recurring tasks.
- Recurring tasks appear naturally in Today.

### Acceptance criteria

- Recurrence is understandable to non-technical users.
- Bot confirms recurrence clearly.
- User can pause/delete recurrence.
- Today correctly shows generated routine tasks.

### Priority

P1

---

## P1.2 - Priorities

### Problem

A task list without priority does not help users decide what to do first.

### Goal

Add simple priority that supports Today and Plan My Day.

### Scope

Priority model:

- P1 / High;
- P2 / Normal;
- P3 / Low/Later.

### Product behavior

- AI can infer priority when obvious.
- User can set or edit priority.
- Today uses priority in ordering.
- Plan My Day uses priority as an input.

### Acceptance criteria

- Priority is visible in task cards.
- Priority does not make basic task creation slower.
- Priority is useful but not overcomplicated.

### Priority

P1

---

## P1.3 - Duration / effort

### Problem

Planning is unrealistic without task size.

### Goal

Let the bot understand and use task duration.

### Scope

Support:

- 5 min;
- 15 min;
- 30 min;
- 1 hour;
- quick/medium/deep.

### Product behavior

- AI can infer duration from text.
- User can edit duration.
- Plan My Day uses duration to avoid overplanning.

### Acceptance criteria

- Duration is optional.
- Duration improves planning.
- The bot can suggest smaller next actions for oversized tasks.

### Priority

P1

---

## P1.4 - AI Plan My Day

### Problem

Users do not only need to store tasks. They need help deciding what to do.

### Goal

Make AI Plan My Day the flagship Pro feature.

### Scope

Plan should consider:

- overdue tasks;
- tasks due today;
- priority;
- duration;
- stale tasks;
- recurring tasks;
- no-date tasks when relevant.

### Product behavior

Bot produces:

- top 3 focus tasks;
- suggested order;
- optional time blocks or effort groups;
- explanation in simple language;
- buttons to accept/edit.

### Free/Bonus behavior

- Free: paywall or limited demo.
- Community Bonus: at most limited preview/demo.
- Pro: full feature.

### Acceptance criteria

- Plan feels realistic.
- User can accept or adjust.
- Plan leads to task completion.
- Conversion from this paywall is tracked.

### Priority

P1

---

## P1.5 - Forwarded message to task/follow-up

### Problem

Many Telegram tasks originate from messages, not from manually written todos.

### Goal

Make forwarded-message ingestion a Telegram-native differentiator.

### Scope

When user forwards a message, bot can offer:

- create task;
- remind later;
- reply later;
- draft reply;
- follow up with sender;
- extract due date if present.

### Product behavior

- Free has limited usage.
- Community Bonus has higher usage.
- Pro has high/fair-use usage and richer follow-up actions.

### Acceptance criteria

- Forwarded messages can become tasks in one or two taps.
- Reply-later is clearly task-related.
- Drafting is not marketed as a generic writing assistant.
- Feature has its own conversion metrics.

### Priority

P1

---

## P1.6 - Weekly Review Pro

### Problem

A simple weekly stats message is useful, but Pro should help users improve next week.

### Goal

Turn weekly review into a planning and cleanup feature.

### Scope

Weekly Review Pro includes:

- completed tasks;
- added tasks;
- stale tasks;
- overdue tasks;
- suggested deletions;
- suggested reschedules;
- top focus for next week;
- celebration.

### Acceptance criteria

- Review produces actions, not only stats.
- User can clean stale tasks from review.
- User can create next week focus.
- Pro value is obvious.

### Priority

P1

---

## P1.7 - Backlog cleanup

### Problem

Todo lists decay when old tasks accumulate.

### Goal

Help users prune and reorganize stale/no-date tasks.

### Scope

Bot identifies:

- stale tasks;
- no-date tasks;
- duplicates;
- vague tasks;
- tasks that should become projects.

Suggested actions:

- keep;
- delete;
- reschedule;
- split;
- convert to project/area;
- merge duplicate.

### Acceptance criteria

- Cleanup can be done in batches.
- User does not need to manually inspect every old task.
- Cleanup improves trust in the task list.

### Priority

P1

---

# P2 - Planning surface

## P2.1 - Telegram Mini App MVP

### Problem

Chat is not enough for visual planning.

### Goal

Create a minimal Mini App for planning without building a native app.

### Screens

1. Today.
2. Upcoming.
3. Week.
4. Projects/Areas.
5. Task detail.
6. Subscription & Bonus.

### Out of scope

- Kanban;
- teams;
- comments;
- attachments;
- integrations marketplace;
- heavy analytics;
- native app.

### Acceptance criteria

- Mini App improves planning UX.
- Bot remains the main capture surface.
- Pro value is visible in Mini App.
- Free/Bonus users see limited but useful preview.

### Priority

P2

---

## P2.2 - Projects / Areas

### Problem

Tags are not enough for long-term organization.

### Goal

Allow users to group tasks into life/work areas.

### Scope

Default areas:

- Work;
- Personal;
- Finance;
- Health;
- Learning;
- Other.

### Acceptance criteria

- Projects/areas do not complicate quick capture.
- User can assign/edit area later.
- Today and Plan My Day can use areas.
- Mini App can show area views.

### Priority

P2

---

## P2.3 - Advanced search and history

### Problem

Users need to retrieve completed and old tasks.

### Goal

Add searchable task history as a Pro retention feature.

### Scope

- Search by text.
- Search by tag/project.
- Completed history.
- Old reminders.
- Follow-up history.

### Acceptance criteria

- Search is useful for Pro users.
- Free can have limited recent history.
- History does not clutter default UX.

### Priority

P2

---

# P3 - Later expansion

## P3.1 - Calendar sync

### Reason to defer

Calendar sync is premature before priority, duration, recurrence, and planning are useful.

### Entry criteria

Start calendar sync only when:

- users actively use Plan My Day;
- duration exists;
- recurring tasks exist;
- Mini App planning is validated;
- users request calendar sync repeatedly.

### Priority

P3

---

## P3.2 - Native mobile apps

### Reason to defer

Native apps add distribution and maintenance complexity before product value is proven.

### Entry criteria

Consider native apps only when:

- Telegram Mini App is used regularly;
- Pro retention is proven;
- users request phone-native notifications/widgets;
- revenue justifies extra surface area.

### Priority

P3

---

## P3.3 - Team/shared tasks

### Reason to defer

Team tasks are a different product.

### Entry criteria

Consider only after:

- solo Pro retention is strong;
- there is repeated demand from small teams;
- product has clear collaboration use case.

### Priority

P3

---

# Immediate next sprint

## Sprint goal

Turn product docs into actionable product changes without starting architecture.

## Recommended sprint items

1. Finalize `PRODUCT_BACKLOG.md`.
2. Update `docs/product/README.md` to link to `PRODUCT_BACKLOG.md`.
3. Review README/CLAUDE positioning for consistency.
4. Define exact Free limits.
5. Define exact Community Bonus limits.
6. Define exact Pro intent paywalls.
7. Create implementation tickets/issues for:
   - Free useful mode;
   - Community Bonus;
   - Pro intent paywalls;
   - Recurring tasks;
   - AI Plan My Day;
   - Forwarded message to task/follow-up.
8. Defer technical architecture until these tickets are accepted.

## Suggested backlog-to-implementation order

1. Free useful mode.
2. Pro intent paywalls.
3. Community Bonus.
4. Recurring tasks.
5. Priorities + duration.
6. AI Plan My Day.
7. Forwarded message to task/follow-up.
8. Weekly Review Pro.
9. Backlog cleanup.
10. Mini App MVP.

## Do not start yet

- Database redesign.
- Mini App architecture.
- Calendar sync.
- Native app.
- Team tasks.
- Integrations.
- Full analytics dashboard.
