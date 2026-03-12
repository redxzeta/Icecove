---
name: alcove
description: >
  Grounds the agent in authoritative internal project documentation stored in a private Alcove docs repository.
  Covers project design, architecture, requirements, progress tracking, coding conventions,
  technical debt, secrets mapping, and environment configuration.
  Also initializes, organizes, audits, and validates project documentation.
  Activates whenever the agent needs authoritative project information — regardless of input language.
---

# Alcove

## Invocation

```
/alcove                          Summarize current project docs and status
/alcove status                   Show current progress and next steps
/alcove architecture             Explain the tech stack and system design
/alcove conventions              List coding rules and naming conventions
/alcove decisions                Review architecture decision records
/alcove debt                     List known issues and technical debt
/alcove search auth flow         Search docs for a specific topic
/alcove what conventions apply?  Ask a doc question directly
```

## When to Use

- User asks about **how this project is designed, architected, or specified**
- User asks about **project status, progress, or next steps**
- User asks about **coding conventions, naming rules, or forbidden patterns**
- User asks about **environment variables, secrets, or deployment config**
- User asks about **technical debt, known issues, or workarounds**
- User asks about **past decisions and their rationale**
- User wants to **initialize documentation for a new project**
- User asks to **organize, clean up, or audit project documentation**
- User wants to **configure project-specific doc settings** (diagram format, custom file lists)
- User asks to **validate docs against policy** (required sections, placeholders, completeness)
- User asks to **rebuild or refresh the search index** after bulk doc changes
- User asks to **check what changed** in docs since the last index build
- **The answer may exist in project docs** — check alcove before answering, not after

## How It Works

MCP server `alcove` via stdio. Auto-detects active project by matching CWD path components against `DOCS_ROOT` folders.

## Document Structure

### Required (always present)

| File | Contains |
|------|----------|
| `PRD.md` | Requirements, goals, scope, constraints |
| `ARCHITECTURE.md` | Tech stack, directory structure, data model, API design, security |
| `PROGRESS.md` | Current phase, milestones, blockers, next actions |
| `DECISIONS.md` | Architecture Decision Records (ADR) with rationale |
| `CONVENTIONS.md` | Naming, patterns, import order, forbidden practices |
| `SECRETS_MAP.md` | Env var names and rotation policy (never values) |
| `DEBT.md` | Technical debt, known vulnerabilities, workarounds |

### Supplementary (project-specific)

| File | When Present |
|------|-------------|
| `DEPLOYMENT.md` | Service has infra/CI/CD pipeline |
| `INTEGRATION.md` | 2+ external service connections |
| `reports/*.md` | Audits, benchmarks, competitive analyses |

## Tools

| Tool | Purpose |
|------|---------|
| `get_project_docs_overview` | List all docs with tier classification. **Call first.** |
| `search_project_docs` | BM25 ranked search (falls back to grep). Params: `query`, `scope`, `limit` |
| `get_doc_file` | Read a specific doc file. Params: `relative_path`, `offset`, `limit` |
| `list_projects` | List all projects with doc completeness |
| `audit_project` | Audit alcove + project repo. Returns file status + `suggested_actions` |
| `check_doc_changes` | Detect changes since last index build |
| `rebuild_index` | Rebuild BM25 search index after bulk changes |
| `validate_docs` | Validate against `policy.toml`. Returns pass/warn/fail per file |
| `configure_project` | Create/update `alcove.toml` in CWD. Preserves unmentioned fields |
| `init_project` | Initialize docs from template. Auto-rebuilds index |

### `audit_project` returns
- File status per doc: `populated`, `missing`, `template-unfilled`, `minimal`
- Cross-repo analysis: exposed internal docs, misplaced reports, missing public docs
- Structured `suggested_actions` with mandatory rules in `agent_instruction`

### `check_doc_changes` params & returns
- Param: `auto_rebuild` (bool, default: false) — auto-rebuild index if changes detected
- Returns: `index_exists`, `is_stale`, `added`, `modified`, `deleted`, `unchanged_count`, `total_indexed`

### `configure_project` args
- `project_name` (required) — alcove folder name
- `diagram_format` (optional) — `"mermaid"` | `"plantuml"`
- `core_files` / `team_files` / `public_files` (optional) — override file lists

### `init_project` args
- `project_name` (required)
- `project_path` (optional) — abs path to project repo for public docs
- `overwrite` (optional, default: false)
- `files` (optional) — specific files to create; omit for all required docs

### `search_project_docs` scope rules
- **Default: current project only.** Do NOT scan all projects unless user explicitly requests it.
- Ambiguous phrases that do NOT imply global scope (treat as current project, or ask):
  - "docs repo", "documentation", "check the docs", "review docs"
  - "remaining items", "what's missing", "status check", "doc health"
  - "look through everything", "go over everything", "summarize docs"
  - "clean up docs", "organize docs", "doc audit"
- Global scope only when user says: "all projects", "everywhere", "across projects", or references cross-project knowledge.

## Agent Instructions

### Scope principle
**Always scope to the current project unless the user explicitly says otherwise.**
- Ambiguous intent between current project and all projects → **ask the user** before proceeding.

### Answering project questions
**Never answer architecture, conventions, or environment questions from memory.** Check alcove first.

1. `get_project_docs_overview` → see available docs and tiers
2. Read the most relevant file:
   - "What does this do?" → `PRD.md`
   - "How is this built?" → `ARCHITECTURE.md`
   - "What's the status?" → `PROGRESS.md`
   - "Why was X chosen?" → `DECISIONS.md`
   - "What style to use?" → `CONVENTIONS.md`
   - "What env vars needed?" → `SECRETS_MAP.md`
   - "Any known issues?" → `DEBT.md`
3. Unsure which file → `search_project_docs` with keywords
4. Summarize key decisions/constraints. Do not dump full files unless explicitly requested.
5. **Never contradict existing decisions** — if DECISIONS.md says "use JWT", don't suggest sessions.

### Configuring project-specific settings

Triggers: configure/set up project doc settings, change diagram format, add files to a recognized tier, create/update per-project `alcove.toml`.

**Do NOT ask clarifying questions. Act immediately.**

1. `get_project_docs_overview` silently → detect unrecognized files
2. `configure_project` → create/update `alcove.toml`; add unrecognized files to `team_files` by default
3. Show user what was written to `alcove.toml`

```
configure_project(project_name: "my-api", diagram_format: "plantuml")
configure_project(project_name: "my-api", team_files: ["RUNBOOK.md", "DEPLOYMENT.md"])
```

### Initializing a new project
1. `init_project` with project name (+ optional project repo path)
2. Tell user which files were created
3. Suggest filling `PRD.md` and `ARCHITECTURE.md` first

### Organizing project documentation
1. `audit_project` → scans alcove + project repo
2. Present findings. **Do NOT auto-execute any actions.**
3. Follow `suggested_actions` with these mandatory rules:

**Document separation:**

| Direction | Rule |
|-----------|------|
| alcove → project repo | OK: generate **public-facing** docs derived from internal content |
| project repo → alcove | OK: restructure/incorporate reference materials into internal docs |
| Internal docs → project repo | **NEVER** copy PRD/ARCHITECTURE etc. into the project repo |

**Action handling:**

- **`resolve_exposed_internal_docs`**: If internal docs (PRD, ARCHITECTURE, etc.) exist in the project repo:
  1. Diff against the alcove version
  2. Merge any **additional content** from the project repo version into alcove **first**
  3. Remove from the project repo **only after** user confirmation

- **`move_reports_to_doc_repo`**: Move analysis/benchmark/audit reports to alcove `reports/`
- **`incorporate_to_doc_repo`**: Restructure project repo reference materials into alcove internal docs
- **`generate_public_docs`**: Generate missing public docs from internal docs. Never expose internal information.
- **`create_missing_internal`**: Create missing required internal docs via `init_project`

4. **Always confirm with the user** before moving or deleting any file
5. Re-run `audit_project` after cleanup to verify results

### Disambiguating "doc status" requests

When the user asks about doc status, health, or state — pick the right tool:

| User intent | Tool | Signal words |
|-------------|------|--------------|
| Pass/fail against policy rules | `validate_docs` | validate, pass, fail, policy, compliance, required sections |
| Overall file inventory + cross-repo analysis | `audit_project` | audit, organize, cleanup, what's missing, inventory |
| What changed since last index | `check_doc_changes` | changed, modified, stale, out of date, new files, diff |

If still ambiguous, prefer `audit_project` as the broadest starting point.

### Before writing code
Always check `CONVENTIONS.md` first to ensure generated code follows project-specific rules (naming, error handling, import order, forbidden patterns).
