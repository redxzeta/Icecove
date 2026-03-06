# ProjectName — Technical Debt

> **Updated**: 2026-03-06

## Summary

| Severity | Count | Trend |
|----------|-------|-------|
| Critical | 0 | — |
| High | 1 | ↑ |
| Medium | 2 | → |
| Low | 1 | → |

## Active Debt

### DEBT-001: Missing input validation on bulk endpoints

- **Severity**: High
- **Introduced**: 2026-03-03
- **Location**: `src/api/projects.rs` — `create_bulk()`, `update_bulk()`
- **Impact**: Accepts arbitrarily large payloads; potential OOM under load
- **Workaround**: Nginx request size limit (10MB) partially mitigates
- **Fix**: Add payload size validation + item count limit at handler level
- **Effort**: Small (< 1 day)
- **Blocked by**: None

### DEBT-002: N+1 query in project listing

- **Severity**: Medium
- **Introduced**: 2026-03-05
- **Location**: `src/domain/services/project_service.rs` — `list_with_owner()`
- **Impact**: Each project triggers a separate user query; ~200ms for 50 items
- **Workaround**: None
- **Fix**: Join query or dataloader pattern
- **Effort**: Small (< 1 day)
- **Blocked by**: None

### DEBT-003: Hardcoded CORS origins

- **Severity**: Medium
- **Introduced**: 2026-03-02
- **Location**: `src/api/middleware/cors.rs`
- **Impact**: Adding new frontend domains requires code change + redeploy
- **Workaround**: None
- **Fix**: Move to environment variable or config file
- **Effort**: Trivial (< 2 hours)
- **Blocked by**: None

### DEBT-004: Test coverage gap in error paths

- **Severity**: Low
- **Introduced**: 2026-03-05
- **Location**: `src/errors/`
- **Impact**: Edge case errors untested; may produce unexpected responses
- **Workaround**: Manual QA covers critical paths
- **Fix**: Add error path unit tests for each AppError variant
- **Effort**: Small (< 1 day)
- **Blocked by**: None

## Resolved Debt

| ID | Description | Resolved | Resolution |
|----|-------------|----------|------------|
| — | — | — | — |

<!-- Move items here when fixed. Keep as historical record. -->

## Known Vulnerabilities

<!-- Security-relevant debt. Agents must prioritize these. -->

| ID | Description | CVE | Mitigation |
|----|-------------|-----|------------|
| — | None currently tracked | — | — |

## Debt Policy

- **Critical/High**: Must resolve before next release
- **Medium**: Schedule within current phase
- **Low**: Address when touching related code
- New debt requires a DEBT-NNN entry before merging
