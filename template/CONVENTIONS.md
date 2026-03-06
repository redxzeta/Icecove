# ProjectName — Conventions

> **Updated**: 2026-03-06

## Naming

| Target | Convention | Example |
|--------|-----------|---------|
| Files | snake_case | `user_service.rs` |
| Functions | snake_case | `get_user_by_id()` |
| Types/Structs | PascalCase | `UserProfile` |
| Constants | SCREAMING_SNAKE | `MAX_RETRY_COUNT` |
| DB tables | snake_case, plural | `user_profiles` |
| DB columns | snake_case | `created_at` |
| API endpoints | kebab-case, plural | `/user-profiles/:id` |
| Env variables | SCREAMING_SNAKE | `DATABASE_URL` |

## Project Patterns

### Error Handling

<!-- Define how errors propagate in this project. Agents must follow this pattern. -->

```
- Use `thiserror` for domain errors, `anyhow` for infrastructure errors
- Never panic in library code; return Result<T, AppError>
- Map external errors at the boundary (infra layer), not in domain logic
- HTTP handlers convert AppError → HTTP status via IntoResponse impl
```

### Module Structure

```
- One public struct per file (exceptions: small related types)
- mod.rs only re-exports, no logic
- Tests in same file (#[cfg(test)] mod tests)
- Integration tests in tests/ directory
```

### API Response Format

```json
{
  "data": {},
  "error": null,
  "meta": {
    "request_id": "uuid",
    "timestamp": "ISO8601"
  }
}
```

### Logging

```
- Use tracing crate (not log or println)
- Levels: error (user-facing failures), warn (recoverable), info (state changes), debug (dev only)
- Always include request_id in span context
- Never log secrets, tokens, or PII
```

### Git Workflow

```
- Branch: feature/<name>, fix/<name>, chore/<name>
- Commit: Conventional Commits format
- PR: squash merge to main
- No direct pushes to main
```

## Import Order

```
1. std library
2. External crates
3. Internal crates / workspace members
4. Current crate modules (crate::)
5. Super / self references

Blank line between each group.
```

## Forbidden Patterns

<!-- Things agents must NEVER do in this project. -->

| Pattern | Reason | Use Instead |
|---------|--------|-------------|
| `unwrap()` in non-test code | Runtime panic | `?` operator or `expect("reason")` |
| Raw SQL strings | SQL injection risk | sqlx query macros with bindings |
| `println!` | Not structured | `tracing::info!` |
| Nested callbacks > 2 levels | Readability | Extract into named functions |
| `clone()` without justification | Performance | Borrow or reference |
