# Metrics and Experiments

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Product metrics

### Activation

Track:

- New users.
- Users who create first task.
- Users who create 3 tasks.
- Users who set first reminder.
- Users who complete first task.
- Users who receive first morning brief.
- Users who return next day.

Activation definition v0.1:

A user is activated when they:

- create at least 3 tasks;
- set at least 1 reminder;
- complete at least 1 task.

### Engagement

Track:

- Tasks created per active user.
- Tasks completed per active user.
- Reminders set.
- Snoozes used.
- Voice messages used.
- AI parsing usage.
- Today views.
- Morning brief opens/actions.
- Weekly review actions.
- Stale task cleanup actions.

### Retention

Track:

- D1 retention.
- D7 retention.
- D14 retention.
- D30 retention.
- Weekly active users.
- Activated user retention vs non-activated user retention.
- Free vs Bonus vs Pro retention.

### Monetization

Track:

- Trial starts.
- Trial completion.
- Trial-to-paid conversion.
- Paid conversion by trigger:
  - recurring tasks;
  - Plan My Day;
  - Weekly Review Pro;
  - backlog cleanup;
  - Mini App planner;
  - forwarded-message workflows.
- Monthly recurring revenue.
- Annual subscription share.
- Churn.
- Refunds if available.
- Stars payment failures.

### Community Bonus

Track:

- Bonus prompt views.
- Channel click-through.
- Bonus check attempts.
- Successful bonus grants.
- Bonus retention lift.
- Bonus-to-Pro conversion.
- Bonus abuse signals.

## Experiment 1 — Free Forever vs hard trial

Hypothesis:

Free Forever + optional Pro Trial will improve retention and organic growth compared with hard trial expiration.

Variants:

A. Trial-first wall.
B. Free Forever + 7-day Pro Trial + intent paywalls.

Success metrics:

- D7 retention.
- Tasks created.
- Reminders set.
- Trial starts.
- Paid conversion.
- Bot block/unsubscribe rate.

Expected outcome:

Free Forever should improve long-term base and trust.

## Experiment 2 — Community Bonus

Hypothesis:

A bonus for Telegram channel subscriptions will grow owned channels without materially reducing Pro conversion.

Bonus v1:

- Extra AI quota.
- Extra voice quota.
- Extra active task limit.
- Community templates.
- Bonus badge.

Exclusions:

- No recurring tasks.
- No full Plan My Day.
- No full Weekly Review Pro.
- No Mini App planner.
- No calendar sync.

Success metrics:

- Bonus claim rate.
- Channel subscription rate.
- Bonus user retention.
- Bonus-to-Pro conversion.
- Pro conversion impact.

## Experiment 3 — Intent paywalls

Hypothesis:

Paywalls shown at high-intent moments convert better than generic subscription prompts.

Intent moments:

- User requests recurring task.
- User asks "plan my day".
- User opens full weekly review.
- User tries backlog cleanup.
- User tries advanced reminders.
- User uses forwarded-message workflow beyond quota.
- User opens full Mini App planner.

Success metrics:

- Paywall view to trial start.
- Paywall view to paid conversion.
- Dismiss rate.
- Repeat intent after dismiss.
- Conversion by feature.

## Experiment 4 — Mini App validation before build

Hypothesis:

Users will value a visual planning surface if they already use Today, Plan My Day, and Weekly Review in chat.

Pre-Mini-App validation:

- Simulate Week overview in chat.
- Simulate Plan My Day with buttons.
- Simulate Projects/Areas with inline menus.
- Track repeated usage.

Build Mini App only when:

- Users repeatedly use planning flows.
- Chat UI becomes a clear bottleneck.
- Pro conversion is tied to planning features.

## Experiment 5 — First Pro feature

Hypothesis:

Recurring tasks are the most understandable first Pro feature.

Compare purchase intent for:

- Recurring tasks.
- AI Plan My Day.
- Weekly Review Pro.
- Backlog cleanup.
- Forwarded-message follow-up.

Success metrics:

- Feature paywall conversion.
- Repeated feature use.
- Paid retention.
- Support complaints/confusion.

## Reporting cadence

Weekly product review should include:

- Activation funnel.
- Retention by cohort.
- Top commands/actions.
- AI/voice usage and cost.
- Paywall conversion by feature.
- Community Bonus performance.
- Top support issues.
- Product decisions needed.

## North Star metric candidate

Weekly completed tasks per activated user.

Why:

The product should not optimize only for task capture. It should help users complete actions.

Supporting metrics:

- Tasks created.
- Reminders set.
- Today usage.
- Plan My Day usage.
- Completion rate.
- Stale task cleanup rate.
