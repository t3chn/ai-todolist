# Pricing Model

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Strategic pricing direction

Replace a hard trial-only mental model with:

- Free Forever.
- Community Bonus.
- Pro.
- Optional 7-day Pro Trial.

Free must remain useful. Pro must sell planning and execution outcomes. Community Bonus must reward Telegram channel subscriptions but must not become a free Pro replacement.

## Tier philosophy

Free:
> Helps users capture and remember.

Community Bonus:
> Makes Free more comfortable and supports channel growth.

Pro:
> Helps users decide, plan, and execute.

## Free

Goal: make the bot useful enough for daily solo usage.

Include:

- Text task capture.
- Basic natural-language task creation.
- Inbox / Tasks / Today.
- Done / delete / edit.
- Basic one-time reminders.
- Basic snooze.
- Basic duplicate warning.
- Simple tags.
- Basic morning brief.
- Limited AI parsing.
- Limited voice input.
- Limited forwarded-message-to-task usage if implemented.
- Community Bonus entry point.
- Pro Trial entry point.

Free should not include:

- Full recurring tasks.
- AI Plan My Day.
- Full Weekly Review Pro.
- Advanced backlog cleanup.
- Full Mini App planner.
- Calendar sync.
- Advanced custom reminder rules.
- High/unlimited AI or voice limits.
- Advanced project/area planning.

## Community Bonus

Goal: reward users who subscribe to selected Telegram channels without cannibalizing Pro.

Community Bonus should be stackable above Free and below Pro.

Possible unlocks:

- Higher AI parsing quota.
- Higher voice quota.
- Higher active-task limit.
- More forwarded-message conversions.
- Additional templates.
- Community badge.
- Early access to experiments.
- One extra weekly insight.
- One-time Pro trial extension.

Community Bonus must not unlock:

- Recurring tasks.
- Full AI Plan My Day.
- Full Mini App planner.
- Full Weekly Review Pro.
- Calendar sync.
- Advanced custom reminder rules.
- Unlimited AI/voice.
- Smart backlog cleanup.
- Full projects/areas.

Why:

Community Bonus should create goodwill and distribution, but it should not remove the reason to buy Pro.

## Pro

Goal: make the product feel like an execution assistant.

Include:

- Recurring tasks and routines.
- Priorities.
- Duration/effort estimates.
- Projects/areas.
- AI Plan My Day.
- Full Weekly Review Pro.
- Smart backlog cleanup.
- Advanced reminders.
- Custom snooze.
- Advanced duplicate detection / merge suggestions.
- Forwarded message to task/follow-up with higher limits.
- Reply-later workflow.
- Task-based draft generation.
- Mini App planner.
- History/search.
- Richer stats.
- Calendar sync later.
- Higher AI/voice quotas with fair-use policy.

## Suggested tier matrix

| Feature | Free | Community Bonus | Pro |
|---|---:|---:|---:|
| Text task capture | Yes | Yes | Yes |
| Basic Today / Tasks | Yes | Yes | Yes |
| Done / delete / edit | Yes | Yes | Yes |
| Basic reminders | Yes | Yes | Yes |
| AI parsing | Limited | Higher limit | High/fair-use |
| Voice input | Small limit | Medium limit | High/fair-use |
| Tags | Basic | Basic+ | Advanced |
| Morning brief | Basic | Basic+ | Smart brief |
| Weekly review | Mini | Mini+ | Full AI review |
| Duplicate detection | Basic | Basic+ | Smart merge/cleanup |
| Recurring tasks | No | No | Yes |
| Priorities | Very limited or no | Preview | Yes |
| Duration/effort | No | No | Yes |
| Projects/areas | No | No | Yes |
| AI Plan My Day | No | 1 demo/week max | Yes |
| Mini App planner | Teaser/read-only | Limited | Full |
| Forwarded message to task | Limited | Higher limit | High/fair-use |
| Calendar sync | No | No | Later Pro |
| Custom reminder rules | No | No | Yes |
| Backlog cleanup | No | No | Yes |

## Paywall strategy

Prefer intent-based paywalls over time-based blocking.

Good paywall moments:

- User requests recurring task.
- User asks to plan the day.
- User opens full weekly review.
- User wants to clean backlog.
- User tries advanced reminder rules.
- User uses forwarded-message workflows beyond quota.
- User opens full Mini App planner.
- User wants projects/areas or priority planning.

Bad paywall moments:

- First message.
- First task.
- Basic completion.
- Basic reminders.
- After trial expiration with no free fallback.
- When user is trying to recover from an error.

## Pricing hypothesis

Keep Pro in an easy consumer impulse range until the Mini App and advanced planning are proven.

The current Telegram Stars-based subscription direction can remain, but the product should clearly prefer annual pricing once retention improves.

Suggested positioning:

- Monthly: low friction.
- Annual: best value.
- Trial: 7 days to experience Pro planning.
- Community Bonus: free growth perk, not a plan.

## Upgrade copy examples

### Recurring tasks

"Recurring tasks are a Pro feature because they turn the bot from a reminder list into a routine system."

### AI Plan My Day

"Plan My Day is Pro. Free helps you capture tasks; Pro helps you decide what to do first."

### Weekly Review

"Full Weekly Review is Pro. It finds stale tasks, celebrates progress, and suggests next week’s focus."

### Mini App planner

"The planner is part of Pro because it gives you a visual execution system, not just a chat list."

## Community Bonus copy

"Subscribe to our Telegram channels and get extra Free limits. This bonus gives you more room to use the bot, while Pro unlocks planning and automation."

## Guardrails

- Do not make Community Bonus equal to Pro.
- Do not remove all Free utility after trial.
- Do not sell only quotas.
- Do not block core task capture.
- Do not overcomplicate pricing before conversion data exists.
