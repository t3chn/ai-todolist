# Decision: Remove repo-local task tracker workflow

Date: 2026-05-05
Project: ai-todolist
Status: Accepted

## Decision

Remove the repo-local task tracker data and workflow references from repository documentation.

## Reason

The repository should keep product and engineering documentation simple, portable, and not dependent on a local task-tracking workflow.

## What this prevents

- Agent confusion around unavailable local tooling.
- Extra repository noise.
- Workflow coupling to a local tracker.
- Accidental inclusion of task database/history in a public repository.

## Consequences

Task tracking should be handled outside the repository or via GitHub issues/PRs unless a new lightweight repo-native workflow is explicitly chosen.

## Review date

After product roadmap is converted into implementation backlog.
