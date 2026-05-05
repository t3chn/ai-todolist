.PHONY: build build-release build-linux run deploy logs status stop start restart require-server-env require-deploy-env require-backup-env

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
SERVER ?=
REMOTE_BIN ?=
SERVICE ?= ai-todolist
REMOTE_DB ?=
REMOTE_BACKUP_DIR ?=

require-server-env:
	@test -n "$(SERVER)" || (echo "SERVER is required, e.g. make status SERVER=user@host"; exit 1)

require-deploy-env: require-server-env
	@test -n "$(REMOTE_BIN)" || (echo "REMOTE_BIN is required, e.g. make deploy SERVER=user@host REMOTE_BIN=/path/to/bin"; exit 1)

require-backup-env: require-server-env
	@test -n "$(REMOTE_DB)" || (echo "REMOTE_DB is required, e.g. make backup-db SERVER=user@host REMOTE_DB=/path/to/db REMOTE_BACKUP_DIR=/path/to/backups"; exit 1)
	@test -n "$(REMOTE_BACKUP_DIR)" || (echo "REMOTE_BACKUP_DIR is required, e.g. make backup-db SERVER=user@host REMOTE_DB=/path/to/db REMOTE_BACKUP_DIR=/path/to/backups"; exit 1)

deploy: require-deploy-env build-linux
	ssh $(SERVER) "systemctl stop $(SERVICE)"
	scp target/release/ai-todolist $(SERVER):$(REMOTE_BIN)
	ssh $(SERVER) "systemctl start $(SERVICE)"
	@echo "Deployed."

logs: require-server-env
	ssh $(SERVER) "journalctl -u $(SERVICE) -f"

status: require-server-env
	ssh $(SERVER) "systemctl status $(SERVICE)"

stop: require-server-env
	ssh $(SERVER) "systemctl stop $(SERVICE)"

start: require-server-env
	ssh $(SERVER) "systemctl start $(SERVICE)"

restart: require-server-env
	ssh $(SERVER) "systemctl restart $(SERVICE)"

# Database
backup-db: require-backup-env
	ssh $(SERVER) "mkdir -p $(REMOTE_BACKUP_DIR) && cp $(REMOTE_DB) $(REMOTE_BACKUP_DIR)/bot-$$(date +%Y%m%d-%H%M%S).db"
	@echo "Database backed up."

# Quick check
check:
	cargo check
	cargo clippy
