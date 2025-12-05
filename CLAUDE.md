# AI Todolist Bot

Telegram бот — AI-powered todolist который не просто напоминает, а помогает выполнять задачи.

## Features (MVP Complete)

- **Smart Input**: Natural language task parsing (text + voice)
- **AI Parsing**: Extracts title, due date, tags (gpt-5-nano)
- **Voice Messages**: Whisper API transcription → task creation
- **Message Drafting**: "Draft message to..." generates drafts
- **Reminders**: 30 min before due time, with snooze (1h/tomorrow)
- **Morning Brief**: Daily summary at user's preferred time (default 09:00 UTC)
- **Conversation Context**: Remembers last 5 messages for follow-ups
- **Inline Buttons**: Done/Delete for quick task management
- **7-day Trial**: Subscription model ready

## Tech Stack

- **Language:** Rust
- **Bot Framework:** teloxide
- **Database:** SQLite (sqlx, bundled)
- **AI:** OpenAI API (gpt-5-nano for parsing, Whisper for voice)
- **Hosting:** DigitalOcean VPS (164.92.143.168)

## Project Structure

```
src/
├── main.rs              # Entry point, dispatcher setup
├── handlers/            # Telegram message & callback handlers
├── services/
│   ├── ai.rs           # OpenAI integration (parse, transcribe)
│   ├── reminder.rs     # Background reminder service (60s loop)
│   ├── morning_brief.rs # Daily brief service
│   └── context.rs      # Conversation context storage
├── models/
│   ├── user.rs         # User model with subscriptions
│   └── task.rs         # Task model with reminders
└── db/                 # Database initialization & migrations
```

## Commands

```bash
# Local development
cargo run                 # Needs TELOXIDE_TOKEN, OPENAI_API_KEY

# Cross-compile for Linux (from macOS)
docker run --rm --platform linux/amd64 -v "$(pwd)":/app -w /app rust:latest cargo build --release

# Deploy to VPS
ssh root@164.92.143.168 "systemctl stop ai-todolist"
scp target/release/ai-todolist root@164.92.143.168:/opt/ai-todolist-bot
ssh root@164.92.143.168 "systemctl start ai-todolist"

# Check logs
ssh root@164.92.143.168 "journalctl -u ai-todolist -f"
```

## Environment Variables

```
TELOXIDE_TOKEN=          # Bot token from @BotFather
OPENAI_API_KEY=          # OpenAI API key
DATABASE_URL=sqlite:data/bot.db
```

## Bot Commands

- `/start` - Welcome + trial info
- `/help` - Available commands
- `/tasks` - List pending tasks with buttons

## Usage Examples

Text:
- "Call mom tomorrow at 5pm" → Task with reminder
- "Buy groceries" → Simple task
- "Draft message to boss about delay" → Message draft

Voice:
- 🎤 Any voice message → Transcribed → Parsed → Task/Draft

## Out of Scope (v1.1)

- Calendar integration
- Team/shared tasks
- Native apps
- Telegram payments (needs provider token)
