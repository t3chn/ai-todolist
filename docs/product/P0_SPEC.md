# P0 Product Spec

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Purpose

This document turns the P0 product backlog into exact product rules.

It is product-only. It defines limits, paywall moments, onboarding behavior, and metrics events. It does not define architecture, database schema, Telegram API implementation, deployment, or vendor-specific technical design.

Architecture should start after this spec is accepted and should be scoped to P0/P1, not the full roadmap.

## Product position

`ai-todolist` is an AI action assistant inside Telegram.

Core promise:

> Turn thoughts, voice notes, forwarded messages, and messy tasks into a clear execution flow.

P0 must preserve this positioning:

- fast capture in Telegram;
- useful Free mode;
- clear Community Bonus layer;
- Pro prompts only at planning/execution intent;
- no native app, team product, calendar sync, or architecture expansion yet.

## Tier rules

### Free Forever

Free must remain useful after any trial expires.

Free includes:

- unlimited manual text capture;
- task create/edit/delete/done;
- `/tasks` and `/today`;
- basic due dates;
- basic reminders;
- basic snooze;
- simple tags;
- basic duplicate warning;
- basic morning brief;
- limited AI parsing;
- limited voice input;
- limited forwarded-message conversion;
- manual/basic fallback when AI or voice limits are reached.

Free limits:

| Capability | Free limit | Limit behavior |
| --- | --- | --- |
| Active pending tasks | 50 | Show upgrade/cleanup prompt after limit. User can still complete/delete existing tasks. |
| AI parsing | 20 requests/month | Fall back to basic task creation without AI parsing. |
| Voice input | 5 voice messages/month | Ask user to type the task instead. |
| Forwarded message workflows | 5 forwarded messages/month | Allow manual task creation from copied text. |
| Morning brief | Basic daily brief | Include today's tasks only. |
| Weekly review | Basic stats only | Show completed/added/pending counts without cleanup plan. |
| Reminder rules | One reminder per task | No advanced rules or repeated schedules. |
| Task history search | Recent active tasks only | No full completed-history search. |

Free must not include:

- recurring tasks;
- AI Plan My Day;
- full Weekly Review Pro;
- backlog cleanup;
- advanced reminder rules;
- full projects/areas planning;
- full Mini App planner;
- calendar sync;
- native app access;
- team/shared tasks.

### Community Bonus

Community Bonus rewards Telegram channel subscription and growth loops. It must feel better than Free but must not replace Pro.

Community Bonus unlocks:

| Capability | Bonus limit | Notes |
| --- | --- | --- |
| Active pending tasks | 100 | More room, not unlimited. |
| AI parsing | 60 requests/month | Higher comfort layer. |
| Voice input | 20 voice messages/month | Still below Pro. |
| Forwarded message workflows | 20 forwarded messages/month | More Telegram-native usage. |
| Morning brief | Enhanced basic brief | May include one extra suggestion. |
| Weekly insight | 1 lightweight weekly insight | Not full Weekly Review Pro. |
| Templates | Basic templates | Capture templates only, not planning automation. |
| Experiments | Early access previews | Preview access does not imply Pro unlocks. |
| Trial extension | Optional one-time extension | Product/ops decision, not guaranteed. |

Community Bonus must not unlock:

- recurring tasks;
- AI Plan My Day;
- full Weekly Review Pro;
- backlog cleanup;
- advanced reminder rules;
- full projects/areas;
- full Mini App planner;
- calendar sync;
- native app;
- team/shared tasks.

Bonus verification product rule:

- If subscription cannot be verified, keep the user on Free and explain that Bonus requires successful channel subscription verification.
- Do not silently grant Pro behavior through Bonus.

### Pro

Pro sells planning and execution outcomes, not only higher quotas.

Pro unlocks:

- recurring tasks/routines;
- AI Plan My Day;
- priorities;
- duration/effort;
- full Weekly Review Pro;
- backlog cleanup;
- advanced forwarded-message follow-up;
- higher/fair-use AI parsing;
- higher/fair-use voice input;
- higher/fair-use forwarded-message workflows;
- advanced reminder rules;
- projects/areas when introduced;
- full Mini App planner when introduced.

Pro limits for P0 product design:

| Capability | Pro rule |
| --- | --- |
| Active pending tasks | Fair-use, no visible P0 hard limit. |
| AI parsing | Fair-use, protect abuse operationally later. |
| Voice input | Fair-use, protect abuse operationally later. |
| Forwarded message workflows | Fair-use, protect abuse operationally later. |
| Planning features | Full access to available Pro planning features. |

P0 does not need to implement all Pro features. P0 must define where Pro value is promised and where upgrade prompts appear.

## Pro intent paywalls

### Paywall principle

Never block basic capture behind a paywall.

Show upgrade only when the user expresses planning, automation, cleanup, or advanced execution intent.

### Trigger matrix

| User action / intent | Free behavior | Community Bonus behavior | Pro behavior |
| --- | --- | --- | --- |
| Create simple text task | Allow | Allow | Allow |
| Create task after AI limit | Basic fallback | Basic fallback after Bonus limit | Allow/fair-use |
| Send voice after limit | Ask to type | Ask to type after Bonus limit | Allow/fair-use |
| Create recurring task | Pro paywall | Pro paywall | Allow |
| Ask "plan my day" | Pro paywall or limited preview | Limited preview max | Full plan |
| Ask for priority planning | Pro paywall | Pro paywall | Allow |
| Add duration/effort for planning | Pro paywall when used for planning | Pro paywall when used for planning | Allow |
| Open full weekly review | Pro paywall | Limited insight only | Full review |
| Request backlog cleanup | Pro paywall | Pro paywall | Allow |
| Use forwarded messages after limit | Manual fallback | Manual fallback after Bonus limit | Allow/fair-use |
| Request advanced reminder rules | Pro paywall | Pro paywall | Allow |
| Open full Mini App planner later | Limited preview | Limited preview | Full access |
| Request calendar sync | Not available | Not available | Not available in P0 |
| Request team/shared task | Not available | Not available | Not available in P0 |

### Paywall copy rules

Paywall copy must explain the outcome:

- "Plan today realistically with priorities and effort."
- "Turn repeated work into routines."
- "Clean stale tasks without reviewing everything manually."
- "Turn forwarded messages into follow-ups faster."

Paywall copy must not say only:

- "Upgrade for more limits."
- "Subscribe to continue."
- "This feature is paid."

### Trial start rule

Trial should be offered after the user has created at least one useful task or triggered a clear Pro intent.

Do not lead first-run onboarding with a paywall.

## First successful task onboarding

### Goal

A new user creates a useful task within 30 seconds.

### First-run flow

1. Welcome with one-line positioning.
2. Ask user to send one task by text or voice.
3. Parse input.
4. Show task confirmation.
5. Offer reminder if due date is missing or ambiguous.
6. Show Today after creation.
7. Explain Free / Bonus / Pro briefly after first value.

### Required UX rules

- No long feature list before first task.
- No paywall before first task.
- No architecture, account setup, or settings prompt before first task unless required for basic function.
- Voice is allowed in onboarding but text must work as the primary fallback.
- If AI parsing fails, create a basic task and explain the user can edit it.
- If date/time is ambiguous, ask one short clarification or create without due date.

### Successful activation definition

User is activated when they complete at least one of:

- creates first task and opens Today;
- creates first task and sets reminder;
- creates first task and marks it done.

Preferred activation event:

- `activation_first_task_created`

## Product metrics events

Events below are product requirements. Technical naming can be adjusted later, but architecture should preserve this event coverage.

### Activation events

| Event | Meaning | Required properties |
| --- | --- | --- |
| `activation_first_task_created` | User created first task. | `input_type`, `has_due_date`, `has_reminder`, `used_ai` |
| `activation_three_tasks_created` | User created third task. | `days_since_signup` |
| `activation_first_reminder_set` | User set first reminder. | `source`, `task_has_due_date` |
| `activation_first_task_completed` | User completed first task. | `days_since_signup`, `source` |
| `activation_first_today_view` | User opened Today first time. | `source` |
| `activation_first_morning_brief_received` | User received first morning brief. | `task_count` |

### Engagement events

| Event | Meaning | Required properties |
| --- | --- | --- |
| `task_created` | Any task created. | `input_type`, `tier`, `used_ai`, `has_due_date`, `has_tags` |
| `task_completed` | Task marked done. | `tier`, `source`, `task_age_days` |
| `task_deleted` | Task deleted. | `tier`, `source`, `task_age_days` |
| `task_edited` | Task edited. | `tier`, `edited_fields` |
| `reminder_set` | Reminder added. | `tier`, `source` |
| `snooze_used` | Snooze used. | `tier`, `duration` |
| `voice_used` | Voice message processed. | `tier`, `limit_state` |
| `ai_parse_used` | AI parsing used. | `tier`, `limit_state`, `success` |
| `today_viewed` | Today viewed. | `tier`, `task_count` |
| `morning_brief_sent` | Morning brief sent. | `tier`, `task_count` |
| `weekly_review_opened` | Weekly review opened. | `tier`, `review_type` |

### Monetization events

| Event | Meaning | Required properties |
| --- | --- | --- |
| `paywall_viewed` | User saw paywall. | `trigger`, `tier`, `feature`, `source` |
| `trial_started` | Trial started. | `trigger`, `source` |
| `subscription_started` | Paid subscription started. | `source`, `trigger` |
| `subscription_cancelled` | Subscription cancelled. | `source` |
| `pro_feature_used` | Pro feature used. | `feature`, `source` |

### Community Bonus events

| Event | Meaning | Required properties |
| --- | --- | --- |
| `bonus_prompt_viewed` | User saw Bonus prompt. | `source`, `tier` |
| `bonus_channel_clicked` | User clicked channel link. | `source` |
| `bonus_verification_started` | Verification started. | `source` |
| `bonus_verification_succeeded` | Bonus granted. | `source` |
| `bonus_verification_failed` | Verification failed. | `source`, `reason` |
| `bonus_to_pro_paywall_viewed` | Bonus user saw Pro paywall. | `trigger`, `feature` |

### Retention metrics

Product reporting must support:

- D1 retention;
- D7 retention;
- D14 retention;
- D30 retention;
- weekly active users;
- weekly completed tasks per activated user.

North Star candidate:

> Weekly completed tasks per activated user.

## P0 out of scope

Do not include in P0:

- database redesign;
- architecture documents;
- Telegram Mini App architecture;
- calendar sync;
- native apps;
- team/shared tasks;
- integrations;
- full analytics dashboard;
- full project management;
- comments/attachments/workspaces;
- generic AI assistant behavior;
- general-purpose message drafting unrelated to tasks.

## P0 acceptance criteria

P0 product spec is accepted when:

- Free limits are explicit.
- Community Bonus limits are explicit.
- Pro unlocks are explicit.
- Paywall triggers are explicit.
- Onboarding flow is explicit.
- Metrics events are explicit.
- Out-of-scope items are explicit.
- Architecture can be designed against these product rules without guessing.
