# ProjectName — Architecture Decision Records

> Records key technical decisions. Prevents agents from proposing contradictory approaches.

---

## ADR-001: Programming Language

- **Date**: 2026-03-01
- **Status**: Accepted
- **Context**: Need a language suitable for high-performance backend services
- **Options**:
  - Go — fast development, simple concurrency model
  - Rust — best performance, memory safety, steep learning curve
  - TypeScript (Node) — full-stack unification, rich ecosystem
- **Decision**: Rust
- **Rationale**: High CPU-bound workload, memory safety required, lower long-term maintenance cost
- **Consequences**: Slower initial development, fewer runtime bugs, performance targets achievable

---

## ADR-002: Database

- **Date**: 2026-03-01
- **Status**: Accepted
- **Context**: Need structured data storage with flexible metadata support
- **Options**:
  - PostgreSQL — JSONB, extensibility, mature ecosystem
  - SQLite — embedded, simple, limited concurrency
  - MongoDB — schemaless, horizontal scaling
- **Decision**: PostgreSQL
- **Rationale**: JSONB provides flexibility while maintaining transactions, type-safe queries with sqlx
- **Consequences**: Slightly higher operational complexity, strong data integrity guarantees

---

## ADR-003: Authentication

- **Date**: 2026-03-05
- **Status**: Proposed
- **Context**: Stateless API auth needed, future microservice decomposition planned
- **Options**:
  - Session + Cookie — requires server-side state
  - JWT (RS256) — stateless, key rotation support
  - API Key — simple but unsuitable for user authentication
- **Decision**: JWT (RS256)
- **Rationale**: Stateless for horizontal scaling, RS256 enables cross-service verification
- **Consequences**: Token revocation requires blacklist (Redis)

---

<!-- Append new decisions as ADR-NNN in sequential order. -->
