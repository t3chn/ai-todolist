# AI Todolist Bot

Telegram бот — AI-powered todolist который не просто напоминает, а помогает выполнять задачи.

## Tech Stack

- **Language:** Rust
- **Bot Framework:** teloxide
- **Database:** SQLite (sqlx)
- **AI:** OpenAI API (gpt-4o-mini)
- **Hosting:** Fly.io (free tier)

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
cargo run                 # Run locally
cargo build --release     # Build for production
fly deploy                # Deploy to Fly.io
```

## Environment Variables

```
TELOXIDE_TOKEN=          # Bot token from @BotFather
OPENAI_API_KEY=          # OpenAI API key
DATABASE_URL=sqlite:data/bot.db
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
