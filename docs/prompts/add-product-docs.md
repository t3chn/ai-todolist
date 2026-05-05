# Prompt: Add product documentation and roadmap to ai-todolist

Use this prompt with a coding/docs agent that has repository write access.

---

You are working in the `t3chn/ai-todolist` repository.

Before making changes, read the current repository context:

- `CLAUDE.md`
- existing README if present
- existing docs folder if present
- any product/architecture docs if present

Goal:

Add product documentation for the current strategic direction of `ai-todolist`.

The product should be documented as:

> AI action assistant inside Telegram: turns thoughts, voice notes, forwarded messages, and messy tasks into a clear execution flow.

Do not position it as merely "an AI todo list".
Do not start technical architecture work in this change.
This change is product documentation only.

Create the following files:

```text
docs/
  product/
    README.md
    PRODUCT_STRATEGY.md
    ROADMAP.md
    PRICING_MODEL.md
    UX_DIRECTION.md
    METRICS_AND_EXPERIMENTS.md
  decisions/
    2026-05-05-product-direction.md
  prompts/
    add-product-docs.md
```

If `docs/` already exists, preserve existing files and add these files without overwriting unrelated content.
If similar files already exist, update them carefully instead of duplicating.

## Required content

### `docs/product/README.md`

Include:

- Short explanation of the product docs folder.
- Current product direction.
- Links to:
  - Product strategy.
  - Roadmap.
  - Pricing model.
  - UX direction.
  - Metrics and experiments.
  - Decision log.
- Product principle:
  - Free helps users capture and remember.
  - Pro helps users decide and execute.
  - Community Bonus improves comfort and growth but must not unlock the core Pro planning layer.

### `docs/product/PRODUCT_STRATEGY.md`

Include:

- Date: 2026-05-05.
- Strategic verdict:
  - Telegram-first AI action assistant.
  - Not a generic task manager.
  - Not a standalone native app yet.
- Product thesis:
  - "AI action assistant inside Telegram: turns thoughts, voice notes, forwarded messages, and messy tasks into a clear execution flow."
- Jobs-to-be-done:
  - Capture messy intent quickly.
  - Clarify vague tasks into next actions.
  - Bring tasks back at the right time.
  - Help the user decide what to do next.
  - Help the user close loops.
- Product boundaries:
  - In scope:
    - solo productivity;
    - Telegram-native capture;
    - voice-to-task;
    - reminders/snooze;
    - daily/weekly review;
    - AI clarification;
    - AI planning;
    - recurring tasks;
    - projects/areas;
    - forwarded-message-to-task/follow-up;
    - Telegram Mini App later.
  - Out of scope for now:
    - standalone native app;
    - team/shared tasks;
    - full project management suite;
    - Kanban;
    - comments/attachments/workspaces;
    - deep calendar product before duration/priorities;
    - general-purpose message drafting;
    - B2B/team product.
- Repackaging of message drafting:
  - Keep only as task-based follow-up/reply-later drafting.
- Differentiation:
  - Telegram-native capture + AI clarification + forwarded-message ingestion + nudges + later Mini App planning.
- Positioning options:
  - "Your AI action assistant inside Telegram."
  - "Turn Telegram messages, voice notes, and thoughts into a clear action plan."
- Product principles and main risk.

### `docs/product/ROADMAP.md`

Include roadmap phases:

Phase 0 — Product reset

- Fix positioning.
- Define UX split.
- Define value ladder.
- Repackage message drafting.
- Define activation moment.
- Add product docs.

Phase 1 — Make Free genuinely useful

- Better onboarding.
- Core Free loop:
  - capture;
  - clarify;
  - remind;
  - snooze;
  - done;
  - celebrate;
  - return tomorrow.
- Better Today.
- Free AI allowance with fallback to manual/basic mode.
- Community Bonus entry.
- Basic metrics.

Phase 2 — First Pro-worthy features

- Recurring tasks/routines.
- Priorities.
- Duration/effort.
- AI Plan My Day.
- Weekly Review Pro.
- Backlog cleanup.
- Forwarded message → task/follow-up.
- Intent-based paywalls.

Phase 3 — Telegram Mini App MVP

- Today screen.
- Upcoming screen.
- Week screen.
- Projects/Areas screen.
- Task detail screen.
- Subscription and bonus screen.
- Explicitly exclude Kanban/team/comments/attachments/integrations marketplace/heavy analytics.

Phase 4 — Premium AI execution layer

- Smart rescheduling.
- Stale-task compression.
- Project decomposition.
- Focus mode.
- Calendar sync later.
- Personal productivity memory.
- Smart templates.

Also include an immediate product sprint:

- Commit product docs.
- Update positioning in README/CLAUDE if needed.
- Define Free/Bonus/Pro.
- Move language from trial-only to Free Forever + Pro Trial.
- Define Pro intent paywalls.
- Prepare backlog tasks for:
  - Free useful mode;
  - Community Bonus;
  - Recurring tasks;
  - AI Plan My Day;
  - Forwarded message to task/follow-up.
- Defer Mini App technical design until Phase 2 signal.

### `docs/product/PRICING_MODEL.md`

Include:

- Free Forever.
- Community Bonus.
- Pro.
- Optional 7-day Pro Trial.

Define philosophy:

- Free helps users capture and remember.
- Community Bonus makes Free more comfortable and supports channel growth.
- Pro helps users decide, plan, and execute.

Free should include:

- text task capture;
- basic natural-language task creation;
- Inbox/Tasks/Today;
- done/delete/edit;
- basic one-time reminders;
- basic snooze;
- basic duplicate warning;
- simple tags;
- basic morning brief;
- limited AI parsing;
- limited voice input;
- limited forwarded-message-to-task usage if implemented;
- Community Bonus entry;
- Pro Trial entry.

Community Bonus should include:

- higher AI quota;
- higher voice quota;
- higher active-task limit;
- more forwarded-message conversions;
- extra templates;
- community badge;
- early access;
- one extra weekly insight;
- optional one-time Pro trial extension.

Community Bonus must not unlock:

- recurring tasks;
- full AI Plan My Day;
- full Mini App planner;
- full Weekly Review Pro;
- calendar sync;
- advanced custom reminder rules;
- unlimited AI/voice;
- smart backlog cleanup;
- full projects/areas.

Pro should include:

- recurring tasks/routines;
- priorities;
- duration/effort;
- projects/areas;
- AI Plan My Day;
- full Weekly Review Pro;
- smart backlog cleanup;
- advanced reminders;
- custom snooze;
- smart duplicate merge/cleanup;
- forwarded message to task/follow-up;
- reply-later workflow;
- task-based draft generation;
- Mini App planner;
- history/search;
- richer stats;
- calendar sync later;
- higher AI/voice quota with fair-use.

Include tier matrix and paywall strategy.
Paywalls should happen at Pro intent moments, not at basic capture.

### `docs/product/UX_DIRECTION.md`

Include:

- Strategic UX decision:
  - Telegram bot first.
  - Mini App later.
  - No standalone native app yet.
- Why bot-first.
- Why not text-only forever.
- Why Mini App before native app.
- Surface split:
  - Bot = capture, voice, forwarded messages, AI clarification, done/delete/snooze, reminders, morning brief, weekly review summary, support.
  - Mini App = Today, Upcoming, Week, Projects/Areas, Task Detail, recurrence, billing, bonus.
- First-run UX.
- Everyday UX loop.
- UX copy principles.
- Example bot flows:
  - Capture.
  - Vague task.
  - Pro Plan My Day.
  - Community Bonus.
- Mini App MVP UX.
- UX success criteria.

### `docs/product/METRICS_AND_EXPERIMENTS.md`

Include:

- Activation metrics.
- Engagement metrics.
- Retention metrics.
- Monetization metrics.
- Community Bonus metrics.
- North Star candidate:
  - weekly completed tasks per activated user.
- Experiments:
  - Free Forever vs hard trial.
  - Community Bonus.
  - Intent paywalls.
  - Mini App validation before build.
  - First Pro feature comparison.

### `docs/decisions/2026-05-05-product-direction.md`

Include decision log:

- Date: 2026-05-05.
- Project: ai-todolist.
- Decision:
  - Keep Telegram-first.
  - Position as AI action assistant.
  - Bot for capture.
  - Mini App later for planning.
  - Free + Community Bonus + Pro.
  - Do not build native app yet.
- Reason:
  - Current repo already has many Telegram-native primitives.
  - Bottleneck is product packaging and planning UX, not raw feature count.
- What this prevents:
  - premature native app;
  - generic assistant scope creep;
  - Community Bonus becoming Pro;
  - hard trial death;
  - early team/project-management complexity;
  - premature calendar sync.
- Consequences.
- Review date:
  - after Phase 2 or first stable paid cohort.

### `docs/prompts/add-product-docs.md`

Save this prompt itself, or a cleaned-up version of it, so future agents understand the product documentation baseline.

## Guardrails

- Do not modify runtime code.
- Do not add technical architecture proposals in this PR.
- Do not add secrets, server IPs, tokens, credentials, or deployment details.
- Do not remove existing useful repo instructions.
- Do not create duplicate docs if equivalent docs already exist.
- Keep docs in English unless the repo has an established documentation language requiring otherwise.
- Mention that architecture should be discussed after the product roadmap is accepted.
- Do not make Community Bonus equal to Pro.
- Do not make Free useless after trial.
- Do not recommend native app as immediate next step.

## Suggested commit

```bash
git checkout -b docs/product-roadmap
mkdir -p docs/product docs/decisions docs/prompts
# add files
git add docs
git commit -m "docs: add product roadmap and pricing strategy"
```

## Suggested PR title

```text
docs: add product roadmap, pricing model, and UX direction
```

## Suggested PR description

```markdown
## Summary

Adds product documentation for the Telegram-first AI action assistant direction.

Includes:

- Product strategy
- Product roadmap
- Free / Community Bonus / Pro pricing model
- UX direction
- Metrics and experiments
- Product decision log
- Prompt for future docs agents

## Key decisions

- Keep bot-first UX.
- Defer standalone native app.
- Add Telegram Mini App later for planning.
- Move toward Free Forever + Community Bonus + Pro.
- Keep Community Bonus below Pro.
- Focus Pro on planning, routines, and execution.

## Out of scope

- Runtime code changes.
- Technical architecture.
- Native app planning.
- Team/shared task features.
```

## Definition of done

- Product docs are added under `docs/`.
- Existing repo docs are not broken.
- No secrets or deployment details are added.
- Roadmap clearly separates product phases from technical architecture.
- Free, Community Bonus, and Pro are clearly distinct.
- Decision log is present.
