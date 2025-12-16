# AI Todolist Bot

## ВАЖНО: Workflow

**ВСЕГДА используй beads для трекинга задач!**
- `bd create --title="..." --type=task` — создать задачу
- `bd update <id> --status=in_progress` — взять в работу
- `bd close <id>` — закрыть
- НЕ используй TodoWrite — только beads

---

Telegram бот — AI-powered todolist который не просто напоминает, а помогает выполнять задачи.

## Features

- **Smart Input**: Natural language task parsing (text + voice)
- **AI Parsing**: Extracts title, due date, tags (gpt-5-nano)
- **Vague Task Clarification**: AI suggests specific actions for broad tasks
- **Voice Messages**: Whisper API transcription → task creation
- **Message Drafting**: "Draft message to..." generates drafts
- **Duplicate Detection**: Warns before creating similar tasks
- **Reminders**: 30 min before due time, with snooze (1h/tomorrow)
- **Morning Brief**: Daily summary at user's preferred time
- **Weekly Review**: Sunday stats with celebration messages
- **Stale Task Nudge**: Warning for tasks not updated in 7+ days
- **Tags & Grouping**: `/tasks` groups by category (work, personal, etc.)
- **Celebration Stats**: Progress tracking on task completion
- **Subscription**: 7-day trial, Telegram Stars payments
- **Admin Panel**: User management, stats, broadcast

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
│   ├── weekly_review.rs # Sunday weekly digest
│   ├── rate_limit.rs   # Rate limiting for API calls
│   └── context.rs      # Conversation context storage
├── models/
│   ├── user.rs         # User model with subscriptions, admin
│   └── task.rs         # Task model with reminders, tags
└── db/                 # Database initialization & migrations
```

## Development

```bash
make build              # Build debug
make run                # Run locally (needs .env)
make check              # cargo check + clippy
```

## Deploy

```bash
make deploy             # Build for Linux + deploy to VPS
make logs               # Tail logs
make status             # Check service status
make restart            # Restart service
make backup-db          # Backup database
```

**Manual deploy (if make unavailable):**
```bash
docker run --rm --platform linux/amd64 -v "$(pwd)":/app -w /app rust:latest cargo build --release
ssh root@164.92.143.168 "systemctl stop ai-todolist"
scp target/release/ai-todolist root@164.92.143.168:/opt/ai-todolist-bot
ssh root@164.92.143.168 "systemctl start ai-todolist"
```

## Environment Variables

```
TELOXIDE_TOKEN=          # Bot token from @BotFather
OPENAI_API_KEY=          # OpenAI API key
DATABASE_URL=sqlite:data/bot.db
ADMIN_IDS=123,456        # Comma-separated Telegram user IDs for /admin
```

## Bot Commands

- `/start` - Welcome + trial info
- `/help` - Available commands
- `/tasks` - List pending tasks (grouped by tags)
- `/today` - Today's tasks
- `/settings` - Subscription, timezone, referrals
- `/support` - Send feedback
- `/admin` - Admin panel (ADMIN_IDS only)

## Usage Examples

Text:
- "Call mom tomorrow at 5pm" → Task with reminder
- "Buy groceries" → Simple task
- "Draft message to boss about delay" → Message draft

Voice:
- 🎤 Any voice message → Transcribed → Parsed → Task/Draft

## Future Ideas

- Calendar integration (Google Calendar, Apple Calendar)
- Team/shared tasks
- Native apps
- Recurring tasks
- Task priorities
