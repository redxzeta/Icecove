# Decisions

## ADR-001: API Key Authentication over JWT

Chose API key authentication for this M2M service over JWT because:
clients are long-lived server processes, not short-lived user sessions.
Keys are simpler to issue, rotate, and revoke without token expiry logic.

**Rotation policy:** keys expire every 90 days, automated via CI pipeline.

## ADR-002: No User Authentication

my-api is exclusively called by internal services. Adding user
authentication would increase attack surface with no benefit.
All callers must be pre-approved and issued a key by the key-manager.
