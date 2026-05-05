# Decision: Telegram-first AI action assistant

Date: 2026-05-05
Project: ai-todolist
Status: Accepted

## Decision

Keep `ai-todolist` Telegram-first and position it as an AI action assistant, not as a generic todo list.

Build the product around:

- Telegram bot for capture, voice, quick actions, reminders, and nudges.
- Telegram Mini App later for planning, review, billing, and settings.
- Free Forever + Community Bonus + Pro value ladder.
- Pro focused on planning, routines, prioritization, and execution.
- Community Bonus as a growth perk, not a Pro replacement.

Do not build a standalone native app yet.

## Reason

The current repo already contains many Telegram-native primitives:

- Natural-language parsing.
- Voice input.
- Vague task clarification.
- Duplicate detection.
- Reminders and snooze.
- Morning brief.
- Weekly review.
- Stale task nudges.
- Tags/grouping.
- Subscriptions.
- Referrals.
- Admin tooling.
- Telegram Stars payments.

The bottleneck is not raw feature count. The bottleneck is product packaging, value ladder, and planning UX.

Telegram is the best surface for capture. Chat is not the best surface for planning. A Telegram Mini App is the natural next interface before considering native apps.

## What this prevents

- Premature native app development.
- Turning the bot into a generic assistant.
- Making Community Bonus equivalent to Pro.
- Killing Free users after trial.
- Adding team/project-management complexity too early.
- Building calendar sync before priorities/duration/planning are proven.
- Mixing product docs with technical architecture prematurely.

## Consequences

Product roadmap should prioritize:

1. Product documentation and positioning.
2. Free useful mode.
3. Community Bonus.
4. Pro intent paywalls.
5. Recurring tasks.
6. AI Plan My Day.
7. Forwarded message to task/follow-up.
8. Mini App planning MVP later.

Architecture discussions should happen after product roadmap is accepted.

## Review date

Review after Phase 2 or after the first stable paid cohort.
