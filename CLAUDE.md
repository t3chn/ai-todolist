# AI Todolist Bot

Telegram бот — AI-powered todolist который не просто напоминает, а помогает выполнять задачи.

## Tech Stack

- **Language:** Rust
- **Bot Framework:** teloxide
- **Database:** PostgreSQL (shuttle shared-db)
- **AI:** OpenAI API (gpt-4o-mini)
- **Hosting:** Shuttle.dev (free tier)

## Project Structure

```
src/
├── main.rs              # Entry point
├── handlers/            # Telegram handlers
├── services/            # Business logic
├── models/              # Data models
└── db/                  # Database layer
```

## Commands

```bash
cargo shuttle run         # Run locally
cargo shuttle deploy      # Deploy to Shuttle.dev
```

## Secrets (Secrets.toml)

```toml
TELOXIDE_TOKEN = "xxx"   # Bot token from @BotFather
OPENAI_API_KEY = "xxx"   # OpenAI API key
```

## beads

```bash
bd ready                  # Available tasks
bd create "Task" -t task  # Create task
bd close <id>            # Complete task
bd sync                  # Sync with git
```

## MVP Scope

**Features:**
- Smart input (text + voice → tasks)
- Task management (CRUD)
- Reminders
- Morning brief
- Message drafting
- Telegram payments

**Out of scope:**
- Calendar integration
- Team features
- Native apps

## Sprint

See `~/planner/mvps/ai-todolist/sprint.md` for full sprint plan.
Current: Day 1 of 14 (Solid scope)
