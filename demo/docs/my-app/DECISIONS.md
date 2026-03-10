# Decisions

## ADR-001: JWT Authentication over Sessions

Chose stateless JWT authentication to support horizontal scaling without
shared session storage. Refresh tokens rotate on each use.

**Alternatives considered:** Redis-backed sessions — rejected due to
operational overhead and tight coupling to a single region.

## ADR-002: Auth0 for OAuth2 Social Login

Delegated social authentication (Google, GitHub) to Auth0 to avoid
maintaining OAuth2 flows in-house. Auth0 issues a JWT that our
auth-service validates and exchanges for an internal token.
