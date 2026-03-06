# Project Document Template

Standard document set for the **documents bridge** — a private knowledge base accessed by AI agents via MCP only.

## When to Use

Copy this template when starting a new project:

```bash
cp -r _template/ <project-name>/
rm <project-name>/README.md
```

Then fill in each document, removing unused optional files.

## Document Tiers

### Tier 1 — Required (every project)

| Document | Purpose | Agent Question It Answers |
|----------|---------|--------------------------|
| `PRD.md` | Requirements, goals, scope, constraints | "What are we building and why?" |
| `ARCHITECTURE.md` | Tech stack, structure, data model, API design | "How is this built?" |
| `PROGRESS.md` | Current phase, milestones, blockers, next actions | "Where are we now?" |
| `DECISIONS.md` | Architecture Decision Records (ADR) | "Why was this approach chosen?" |
| `CONVENTIONS.md` | Naming, patterns, forbidden practices, import order | "What style rules must I follow?" |
| `SECRETS_MAP.md` | Env variable names, rotation policy (never values) | "What config does this service need?" |
| `DEBT.md` | Known issues, vulnerabilities, workarounds | "What's broken or fragile?" |

### Tier 2 — Optional (add when needed)

| Document | When to Add | Agent Question It Answers |
|----------|-------------|--------------------------|
| `DEPLOYMENT.md` | Service has infra/CI/CD | "How do I deploy this?" |
| `INTEGRATION.md` | 2+ external service connections | "How do external services connect?" |
| `PERSONAS.md` | Complex user segmentation | "Who exactly uses this?" |
| `COST.md` | Budget-sensitive infra decisions | "What are the cost constraints?" |
| `MIGRATION.md` | Frequent schema changes | "How do I evolve the data model?" |
| `POSTMORTEM.md` | Production incidents occurred | "What went wrong before?" |
| `RESEARCH.md` | PoC or benchmark results to preserve | "What alternatives were evaluated?" |

### Tier 3 — Stays in the project repo (not here)

These belong in the **project repository itself**, not in the documents bridge:

| Document | Reason |
|----------|--------|
| `README.md` | Public entry point for the project |
| `CHANGELOG.md` | Tied to releases and git tags |
| `CONTRIBUTING.md` | Public contributor guide |
| `ROADMAP.md` | Public feature timeline |
| `LICENSE` | Legal requirement, must be in repo root |
| `CODE_OF_CONDUCT.md` | Community governance |

## Structure

```
<project-name>/
├── PRD.md              # Tier 1 — What & Why
├── ARCHITECTURE.md     # Tier 1 — How
├── PROGRESS.md         # Tier 1 — Status
├── DECISIONS.md        # Tier 1 — Decision log
├── CONVENTIONS.md      # Tier 1 — Code rules
├── SECRETS_MAP.md      # Tier 1 — Env config map
├── DEBT.md             # Tier 1 — Known issues
│
├── DEPLOYMENT.md       # Tier 2 — Optional
├── INTEGRATION.md      # Tier 2 — Optional
├── ...                 # Tier 2 — Optional
│
└── reports/            # Audits, benchmarks, analyses
    ├── AUDIT_YYYY-MM-DD.md
    └── BENCHMARKS.md
```

## Guidelines

1. **English only** — All documents in English for agent compatibility
2. **No secrets** — Never store actual values, keys, or credentials. SECRETS_MAP.md maps names only
3. **Keep current** — Update PROGRESS.md every session. Stale docs mislead agents
4. **One source of truth** — If info exists in PRD, don't duplicate in ARCHITECTURE. Cross-reference instead
5. **Agent-first writing** — Use tables, bullet points, code blocks. Avoid long prose paragraphs
6. **Date your updates** — Every document header has an `Updated` field
