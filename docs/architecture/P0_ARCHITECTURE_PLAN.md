# P0 Architecture Plan

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Purpose

This document maps the accepted P0 product spec to a bounded technical plan.

It is intentionally scoped to P0:

- Free Forever;
- Community Bonus;
- Pro intent paywalls;
- first successful task onboarding;
- product metrics events;
- safe feature/plan gating.

It does not design the full roadmap.

Primary inputs:

- `docs/product/P0_SPEC.md`
- `docs/product/PRODUCT_BACKLOG.md`
- `docs/architecture/CURRENT_STATE.md`

## Architecture goal

Create a small entitlement, limit, paywall, onboarding, and metrics layer that can support P0 product rules without rewriting the bot.

P0 architecture should preserve the current single-binary Telegram bot shape and make the smallest durable changes needed to stop product logic from being scattered through handlers.

## Non-goals

Do not build in this architecture slice:

- recurring tasks;
- priority/duration planning implementation;
- AI Plan My Day implementation;
- Mini App;
- calendar sync;
- native apps;
- team/shared tasks;
- integrations;
- full analytics dashboard;
- full database redesign;
- multi-instance scheduler redesign beyond noting current risks.

## Proposed P0 components

### 1. Entitlements

Add a central entitlement model that answers:

- what tier the user is in;
- which P0 capabilities are allowed;
- which limit bucket applies;
- which paywall should be shown when a feature is not allowed.

Suggested module:

- `src/services/entitlements.rs`

Suggested core types:

```rust
enum ProductTier {
    Free,
    CommunityBonus,
    Pro,
}

enum Capability {
    BasicCapture,
    AiParsing,
    VoiceInput,
    ForwardedMessageWorkflow,
    BasicMorningBrief,
    BasicWeeklyStats,
    RecurringTasks,
    PlanMyDay,
    BacklogCleanup,
    AdvancedReminderRules,
}

struct Entitlement {
    tier: ProductTier,
    allowed: bool,
    limit: Option<LimitPolicy>,
    paywall_trigger: Option<PaywallTrigger>,
}
```

P0 rule:

- handlers must not call `user.has_active_subscription()` directly for product access decisions;
- handlers should ask the entitlement service for capability access;
- basic capture must remain allowed for Free users.

### 2. Tier state

Current user state is trial/subscription-date based. P0 needs explicit representation of Free, Community Bonus, and Pro.

Minimal schema options:

Option A - add columns to `users`:

- `product_tier TEXT DEFAULT 'free'`;
- `community_bonus_expires_at TEXT`;
- `community_bonus_source TEXT`;
- `pro_expires_at TEXT`;

Option B - add append-friendly entitlement table:

- `user_entitlements`;
- columns: `user_id`, `kind`, `source`, `starts_at`, `expires_at`, `created_at`.

Recommended P0 direction:

- use Option A only if implementation speed is the priority;
- use Option B if we want a cleaner path for multiple sources such as paid subscription, admin grant, trial, channel bonus, referral bonus.

Decision needed before implementation:

- whether to keep existing `trial_ends_at` and `subscription_expires_at` as compatibility fields or migrate product reads fully to the new entitlement source.

P0 constraint:

- do not introduce a large auth/billing redesign;
- preserve existing Telegram Stars payment flow while routing product access through the entitlement service.

### 3. Limit policy

Replace trial/paid-only `RateLimits` with product-tier policy.

Suggested module:

- keep `src/services/rate_limit.rs` for counters;
- add policy mapping in `src/services/limits.rs` or inside `entitlements.rs`.

P0 limit policies from product spec:

Free:

- active pending tasks: 50;
- AI parsing: 20/month;
- voice input: 5/month;
- forwarded-message workflows: 5/month.

Community Bonus:

- active pending tasks: 100;
- AI parsing: 60/month;
- voice input: 20/month;
- forwarded-message workflows: 20/month.

Pro:

- fair-use for P0; no visible hard limit.

Technical changes:

- support monthly windows in `rate_limits`;
- add action types:
  - `ai_parse`;
  - `voice`;
  - `forwarded_message`;
  - optionally `task_create` if active task count is insufficient;
- enforce active-task caps by counting pending/in-progress tasks before create;
- when AI/voice limits are reached, route to manual fallback instead of blocking the whole bot.

Important P0 behavior:

- a Free user over AI limit can still create a basic text task;
- a Free user over voice limit is asked to type;
- an active-task cap should still allow completion and deletion.

### 4. Paywall triggers

Add a structured paywall trigger model.

Suggested module:

- `src/services/paywall.rs`

Suggested enum:

```rust
enum PaywallTrigger {
    RecurringTaskRequested,
    PlanMyDayRequested,
    PriorityPlanningRequested,
    DurationPlanningRequested,
    FullWeeklyReviewRequested,
    BacklogCleanupRequested,
    AdvancedReminderRulesRequested,
    ForwardedMessageLimitReached,
    MiniAppPlannerRequested,
}
```

Current state:

- subscribe CTA is generic;
- expired users are blocked;
- paywall views are not measured.

P0 target:

- create a helper that returns user-facing paywall copy and product event properties;
- basic task capture never calls this helper;
- each paywall view emits `paywall_viewed` with `trigger`, `tier`, `feature`, and `source`.

### 5. Onboarding

Current `/start` creates the user and sends welcome/trial/settings copy.

P0 target:

- first-run flow should ask for one task before long explanations or paywall;
- trial/Pro explanation should happen after first useful task or clear Pro intent;
- activation should be tracked.

Suggested technical shape:

- add `onboarding_state` to user state, or infer from events/tasks for P0;
- add a small onboarding helper:
  - `is_first_run`;
  - `has_created_first_task`;
  - `next_onboarding_message`;
- emit:
  - `activation_first_task_created`;
  - `activation_first_today_view`;
  - `activation_first_reminder_set`;
  - `activation_first_task_completed`.

Minimal implementation path:

- use task count to detect first task;
- after first task creation, send Today/next-action prompt;
- avoid storing complex onboarding FSM until needed.

### 6. Metrics events

Add a durable event sink for P0 product events.

Suggested table:

```sql
CREATE TABLE IF NOT EXISTS product_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    event_name TEXT NOT NULL,
    properties_json TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_product_events_user_created
    ON product_events(user_id, created_at);

CREATE INDEX IF NOT EXISTS idx_product_events_name_created
    ON product_events(event_name, created_at);
```

Suggested module:

- `src/services/metrics.rs`

Suggested API:

```rust
async fn track(
    pool: &SqlitePool,
    user_id: Option<i64>,
    event_name: ProductEvent,
    properties: serde_json::Value,
) -> Result<(), sqlx::Error>;
```

P0 requirements:

- event tracking should never block the user flow;
- handler code may log event errors but should not fail user actions on metrics failure;
- event properties should use product names from `P0_SPEC.md`.

Event coverage must include:

- activation;
- engagement;
- monetization/paywall;
- Community Bonus verification;
- retention-supporting activity events.

### 7. Community Bonus

Current referral bonus extends trial days. There is no Telegram channel bonus verification model.

P0 architecture needs:

- explicit bonus entitlement state;
- verification flow state;
- Bonus vs Pro separation.

Suggested product-state fields/table:

- `community_bonus_status`;
- `community_bonus_verified_at`;
- `community_bonus_expires_at`;
- `community_bonus_source`;

Or if using `user_entitlements`:

- `kind = 'community_bonus'`;
- `source = 'telegram_channel'`;
- `expires_at` optional.

Needed callbacks/events:

- `bonus_prompt_viewed`;
- `bonus_channel_clicked`;
- `bonus_verification_started`;
- `bonus_verification_succeeded`;
- `bonus_verification_failed`;
- `bonus_to_pro_paywall_viewed`.

P0 constraint:

- Bonus must not unlock Pro capabilities such as recurring tasks, full Plan My Day, full Weekly Review Pro, or backlog cleanup.

### 8. Handler boundaries

Current `src/handlers/mod.rs` is the highest-risk file for P0 changes because most product logic is centralized there.

P0 refactor should be narrow:

- do not split the whole handler file yet;
- introduce small service helpers first;
- replace direct trial/subscription checks in the highest-traffic flows:
  - text task creation;
  - voice task creation;
  - AI parsing;
  - settings subscribe CTA;
  - weekly/morning brief gating;
  - future Pro-intent commands/callbacks.

Recommended helper calls:

```rust
let tier = entitlements.resolve_user_tier(&pool, &user).await?;
let decision = entitlements.check(&pool, &user, Capability::AiParsing).await?;
```

Then route:

- allowed -> execute feature;
- limited with fallback -> execute fallback;
- paywall -> show paywall and track event.

## Migration plan

P0 should use additive migrations only.

Suggested migrations:

1. Product events table.
2. Entitlement/tier state.
3. Optional limit window support if current `window_start` representation is insufficient for monthly limits.

Avoid in P0:

- rewriting existing `users` table;
- rewriting existing `tasks` table;
- changing task primary keys;
- migrating tags format unless required for P0 enforcement.

## P0 implementation order

1. Add metrics event table and tracking service.
2. Add entitlement/tier resolution service with compatibility adapter over current trial/subscription fields.
3. Add product-tier limit policy and monthly counter support.
4. Change basic text capture so expired users fall back to Free instead of being blocked.
5. Add active pending task cap behavior for Free and Bonus.
6. Add AI and voice fallback behavior when limits are reached.
7. Add structured paywall helper and event tracking.
8. Update onboarding flow for first successful task.
9. Align morning brief and weekly review with Free/Bonus/Pro behavior.
10. Add Community Bonus verification state and events.

## P0 acceptance checklist

Architecture is P0-ready when:

- Free users can create/edit/delete/done basic tasks after trial expiry;
- AI and voice limits degrade to manual/basic fallback;
- Community Bonus has explicit state and cannot unlock Pro capabilities;
- Pro intent paywalls are represented as structured triggers;
- first successful task onboarding is represented without a large FSM;
- P0 metrics events can be written durably;
- handlers no longer make raw subscription-date decisions for P0 features;
- current payment flow still activates Pro access;
- migrations are additive and reversible by normal rollback/restore.

## Known deferred work

The following are intentionally deferred until after P0 architecture is accepted:

- recurring task scheduler and data model;
- priority/duration data model;
- AI Plan My Day planner;
- full Weekly Review Pro implementation;
- backlog cleanup workflows;
- Mini App frontend/backend;
- calendar sync;
- native apps;
- team/shared tasks;
- multi-instance-safe scheduler locks.
