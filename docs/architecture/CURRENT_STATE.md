# Current Architecture State

Date: 2026-05-05
Project: ai-todolist
Status: Draft v0.1

## Purpose

This document captures the current technical state before implementing the accepted P0 product spec.

It is an inventory and risk map only. It does not propose the full target architecture and does not design P1/P2 features such as recurring tasks, Mini App, calendar sync, native apps, or team features.

Primary product input:

- `docs/product/P0_SPEC.md`

## Runtime shape

The application is a single Rust Telegram bot binary.

Main runtime entrypoint:

- `src/main.rs`

Core runtime setup:

- loads `.env` through `dotenvy`;
- initializes tracing;
- reads `DATABASE_URL`, defaulting to `sqlite:data/bot.db?mode=rwc`;
- initializes SQLite through `db::init_pool`;
- creates optional `AiService` when `OPENAI_API_KEY` is present;
- creates shared `ConversationContext` in memory;
- creates `I18n` from embedded Fluent files;
- registers Telegram bot commands;
- builds a teloxide dispatcher for:
  - messages;
  - callback queries;
  - pre-checkout queries;
- starts background loops for:
  - reminders;
  - morning brief;
  - weekly review.

## Module map

### `src/handlers/mod.rs`

Main Telegram interaction surface.

Responsibilities currently mixed in one file:

- command handling: `/start`, `/help`, `/tasks`, `/today`, `/settings`, `/support`, `/invite`, `/admin`;
- natural-language text task creation;
- voice task flow;
- AI parse/draft/clarify flows;
- duplicate confirmation;
- edit flows;
- reminder edit flows;
- timezone and morning brief settings;
- referral/invite flow;
- subscription invoice and payment handling;
- stale task review callbacks;
- admin panel and broadcasts;
- support forwarding and admin replies.

P0-relevant current behavior:

- `check_subscription` blocks task/voice AI flows when trial/subscription is inactive.
- `get_rate_limits` chooses `RateLimits::trial()` when `subscription_type == "trial"`, otherwise paid limits.
- simple task capture is not Free Forever today because expired users are blocked.
- subscription prompts are generic settings/expired-state prompts, not product-intent paywalls.
- onboarding starts with welcome, trial copy, and settings button; it does not force first successful task before explaining tiers.

### `src/services/ai.rs`

OpenAI integration.

Capabilities:

- parse task fields;
- classify input intent into task/draft/command/clarify/rejected;
- parse reminder time;
- parse timezone;
- transcribe voice through Whisper;
- parse task edit instructions.

Current AI service traits:

- uses `gpt-5-nano` for chat completions;
- uses `whisper-1` for voice transcription;
- expects JSON from model responses and strips basic markdown fences;
- does not expose product-tier gating itself;
- does not emit product metrics events.

### `src/services/rate_limit.rs`

Rate limiting helper.

Current model:

- `RateLimits::trial()`:
  - `tasks_per_day = 10`;
  - `voice_per_day = 5`;
  - `ai_calls_per_hour = 20`.
- `RateLimits::paid()`:
  - `tasks_per_day = 100`;
  - `voice_per_day = 30`;
  - `ai_calls_per_hour = 100`.
- `RateLimiter::check_and_increment` stores counters in `rate_limits`.
- windows are daily for `window_minutes >= 1440`, otherwise hourly.

P0 mismatch:

- P0 spec needs Free/Bonus/Pro product tiers;
- P0 limits are mostly monthly or active-task based;
- current logic only knows trial vs non-trial;
- current limit exceeded behavior blocks rather than consistently falling back to basic/manual modes.

### `src/services/context.rs`

In-memory conversation and pending state.

Current state:

- stores last 5 messages per user in a `Mutex<HashMap<i64, Vec<Message>>>`;
- stores pending edits/actions in `Mutex<HashMap<i64, PendingEdit>>`;
- covers edit confirmation, reminders, timezone, support, duplicate confirmation, and admin pending actions.

Risk:

- state disappears on process restart;
- multi-instance runtime would not share state;
- pending duplicate/edit/support flows can expire silently after restart.

### `src/services/reminder.rs`

Background reminder loop.

Current behavior:

- runs every 60 seconds;
- loads due reminders with `Task::find_due_reminders`;
- sends reminder with done/snooze buttons;
- clears `reminder_at` only after successful send;
- logs send errors.

Risk:

- no delivery attempt table;
- no retry/backoff metadata;
- single reminder timestamp model only;
- no explicit idempotency record beyond clearing `reminder_at`;
- all due reminders are processed by every running instance if the app is scaled horizontally.

### `src/services/morning_brief.rs`

Background morning brief loop.

Current behavior:

- runs every 60 seconds;
- checks users whose `morning_brief_time` equals current UTC hour/minute;
- currently sends only to `ADMIN_IDS`;
- generates a simple today/pending summary.

P0 mismatch:

- P0 Free includes basic morning brief;
- current implementation is effectively disabled for regular users;
- timezone is stored but not applied to scheduling;
- no event is emitted for `morning_brief_sent`.

### `src/services/weekly_review.rs`

Background weekly review loop.

Current behavior:

- checks hourly;
- runs Sunday 18:00 UTC;
- sends only to users with `has_active_subscription()`;
- includes completed/created/pending/stale counts and stale review button.

P0/P1 mismatch:

- P0 Free/Bonus require clearer basic vs limited insight vs Pro review behavior;
- current weekly review is subscription-gated by trial/paid active status;
- no metrics events are emitted;
- no persistent sent marker, so repeated sends can happen if loop runs more than once in the matching hour.

### `src/models/task.rs`

Task data access.

Current task fields:

- `id`;
- `user_id`;
- `title`;
- `description`;
- `status`;
- `due_at`;
- `reminder_at`;
- `tags`;
- `created_at`;
- `updated_at`.

Current capabilities:

- create task;
- list pending tasks;
- list today's tasks;
- update title and due date;
- update status;
- delete;
- set/clear/snooze reminders;
- set default reminder from due date;
- find similar pending tasks by simple string comparison;
- find stale tasks by `updated_at`;
- count created/completed tasks.

Risks:

- `tags` is documented as JSON in migration but handlers store comma-separated strings;
- there is no priority, duration, project/area, source/input type, or created-via metadata;
- deletes are hard deletes;
- completed history search is not modeled separately.

### `src/models/user.rs`

User data access and subscription/admin/referral helpers.

Current user fields include:

- Telegram identity: `telegram_id`, `username`, `first_name`;
- preferences: `timezone`, `morning_brief_time`, `language`;
- subscription/trial: `trial_ends_at`, `subscription_expires_at`, `subscription_type`;
- referral: `referral_code`, `referred_by`, `referral_count`, `bonus_days`;
- admin/moderation: `is_banned`, `banned_at`, `ban_reason`, `last_active_at`.

Current capabilities:

- get/create user;
- create with referral;
- calculate trial/subscription status;
- activate paid subscription;
- grant subscription from admin;
- update timezone/language/brief time;
- generate referral code;
- list/search/admin stats.

P0 mismatch:

- no explicit `tier` model for Free, Community Bonus, Pro;
- referral `bonus_days` extends trial-like access, not Community Bonus capability limits;
- `last_active_at` helper exists but is not clearly wired into every user interaction;
- subscription state is date-based and does not express product entitlement sources.

## Database schema

Migrations are SQL files under `migrations/` and are executed manually from `src/db/mod.rs` by `include_str!`.

Important implementation detail:

- `001_init.sql` uses `CREATE TABLE IF NOT EXISTS`.
- Later migrations use `ALTER TABLE ADD COLUMN` statements and execution errors are ignored statement-by-statement.
- There is no migration version table.

Current tables:

### `users`

Created in `001_init.sql`, expanded by migrations `002` through `006`.

Columns:

- `id`;
- `telegram_id`;
- `username`;
- `first_name`;
- `timezone`;
- `morning_brief_time`;
- `created_at`;
- `updated_at`;
- `trial_ends_at`;
- `subscription_expires_at`;
- `subscription_type`;
- `referral_code`;
- `referred_by`;
- `referral_count`;
- `bonus_days`;
- `is_banned`;
- `banned_at`;
- `ban_reason`;
- `last_active_at`;
- `language`.

Indexes:

- `idx_users_telegram_id`;
- partial unique `idx_users_referral_code`.

### `tasks`

Created in `001_init.sql`.

Columns:

- `id`;
- `user_id`;
- `title`;
- `description`;
- `status`;
- `due_at`;
- `reminder_at`;
- `tags`;
- `created_at`;
- `updated_at`.

Indexes:

- `idx_tasks_user_id`;
- `idx_tasks_status`;
- `idx_tasks_due_at`.

### `rate_limits`

Created in `003_rate_limits.sql`.

Columns:

- `id`;
- `user_id`;
- `action_type`;
- `count`;
- `window_start`.

Unique key:

- `(user_id, action_type, window_start)`.

### `admin_logs`

Created in `005_admin.sql`.

Columns:

- `id`;
- `admin_id`;
- `action`;
- `target_user_id`;
- `details`;
- `created_at`.

## Payment/subscription state

Telegram Stars payments are handled in `src/handlers/mod.rs`.

Current flow:

- `subscribe` callback shows fixed plan buttons;
- `buy:<months>` sends invoice with Stars currency;
- successful payment parses payload `sub:{user_id}:{months}`;
- `User::activate_subscription` sets `subscription_expires_at` and `subscription_type = 'monthly'`;
- admin grant can set `subscription_type = 'paid'`.

Risks:

- subscription values are inconsistent (`trial`, `monthly`, `paid`);
- `get_rate_limits` treats only exact `trial` as trial and everything else as paid;
- Free Forever is not represented;
- Community Bonus is not represented;
- paywall triggers are not structured or measurable.

## Metrics state

There is no dedicated product analytics/event module or table.

Current available signals:

- `tracing` logs;
- admin aggregate queries over users and tasks;
- task counts from `Task` helpers;
- rate limit usage counters.

Gaps against `docs/product/P0_SPEC.md`:

- no event sink for activation events;
- no event sink for task engagement events;
- no paywall trigger event;
- no trial/subscription conversion event coverage beyond payment-side effects;
- no Community Bonus events;
- no retention event model;
- no event properties such as `input_type`, `tier`, `used_ai`, `limit_state`, `trigger`, or `source`.

## Main P0 risks

1. Trial gating conflicts with Free Forever.
   - Expired users are blocked from basic task capture.

2. Entitlements are implicit.
   - Current code infers access from date fields and `subscription_type`.

3. Rate limits do not match P0 tiers.
   - Current limits are trial vs paid and daily/hourly, not Free/Bonus/Pro and monthly/active-task.

4. Community Bonus has no technical representation.
   - Referral bonus days exist, but channel-subscription bonus state does not.

5. Paywalls are not intent-based.
   - There is no structured trigger model for paywall display.

6. Metrics are missing.
   - Product spec requires events before roadmap decisions can be evidence-based.

7. In-memory pending state is fragile.
   - User flows can break after restart and cannot scale horizontally.

8. Reminder and review loops are not multi-instance safe.
   - No claim/lock/sent-marker model exists.

9. Tags format is inconsistent.
   - Schema comment says JSON array, handlers use comma-separated text.

10. Background service product behavior is not P0-aligned.
    - Morning brief is admin-only; weekly review is subscription/trial-active gated.
