.PHONY: build build-release build-linux run deploy logs status stop start restart

# Local development
build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run

# Cross-compile for Linux (from macOS)
build-linux:
	docker run --rm --platform linux/amd64 \
		-v "$(shell pwd)":/app -w /app \
		rust:latest cargo build --release

# Server management
SERVER = root@164.92.143.168
REMOTE_BIN = /opt/ai-todolist-bot
SERVICE = ai-todolist

deploy: build-linux
	ssh $(SERVER) "systemctl stop $(SERVICE)"
	scp target/release/ai-todolist $(SERVER):$(REMOTE_BIN)
	ssh $(SERVER) "systemctl start $(SERVICE)"
	@echo "✅ Deployed!"

logs:
	ssh $(SERVER) "journalctl -u $(SERVICE) -f"

status:
	ssh $(SERVER) "systemctl status $(SERVICE)"

stop:
	ssh $(SERVER) "systemctl stop $(SERVICE)"

start:
	ssh $(SERVER) "systemctl start $(SERVICE)"

restart:
	ssh $(SERVER) "systemctl restart $(SERVICE)"

# Database
backup-db:
	ssh $(SERVER) "cp /opt/data/bot.db /opt/backups/bot-$$(date +%Y%m%d-%H%M%S).db"
	@echo "✅ Database backed up"

# Quick check
check:
	cargo check
	cargo clippy
