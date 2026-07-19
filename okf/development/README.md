---
okf_document_id: "development-readme"
title: "Development"
project: "ZinharCMS"
category: "development"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "package.json"
  - "backend/Cargo.toml"
  - "frontend/package.json"
  - "README.md"
related_documents:
  - "../backend/README.md"
  - "../frontend/README.md"
  - "../database/README.md"
  - "../api/README.md"
  - "../maintenance/README.md"
related_diagrams: []
---

# Development

## Purpose

This section describes the repository-supported local setup, command, build, quality, test, database-development, debugging, and contribution workflows. Executable manifests, scripts, CI definitions, and code outrank prose.

## Supported Development Environments

Windows PowerShell is directly evidenced by the operational scripts and the observed repository workflow. GitHub Actions uses Ubuntu, development containers use Debian or Alpine Linux, and the application toolchains are cross-platform. An owner-approved operating-system support matrix was not found (`ENVIRONMENT_REQUIREMENT_UNCLEAR`).

## Command Confidence

- `VERIFIED` means the command is defined by a current manifest, workflow, script, Dockerfile, or tracked documentation and its definition was inspected.
- `DOCUMENTED_NOT_VERIFIED` means the repository documents the command but Phase 10 did not execute it.
- `ENVIRONMENT_DEPENDENT` means success requires services, configuration, credentials, or data not supplied by the command itself.
- `COMMAND_STATUS_UNCLEAR` means no current executable definition or safe verification path was found.

## Recommended Reading Order

1. [Prerequisites](prerequisites.md)
2. [Local Environment](local-environment.md)
3. [Commands](commands.md)
4. [Development Workflow](development-workflow.md)
5. [Build and Quality](build-and-quality.md)
6. [Testing Workflow](testing-workflow.md)
7. [Database Development](database-development.md)
8. [Debugging](debugging.md)
9. [Contribution Workflow](contribution-workflow.md)
10. [Development Risks](development-risks.md)

Subsystem context: [Backend](../backend/README.md), [Frontend](../frontend/README.md), [Database](../database/README.md), [API](../api/README.md), [Backend Testing](../backend/testing-map.md), and [Frontend Testing](../frontend/testing-map.md).

## For AI Coding Agents

1. Read [prerequisites.md](prerequisites.md).
2. Read [commands.md](commands.md).
3. Identify the correct working directory.
4. Use only commands defined by the repository.
5. Run the smallest relevant validation command first.
6. Avoid generated and dependency directories.
7. Do not modify environment secrets.
8. Review relevant OKF subsystem documentation before changing code.
9. Update OKF documents when implementation changes make them stale.
10. Report failing commands without hiding errors.

