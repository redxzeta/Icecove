# Architecture

## Overview

my-api is an internal REST API using API key authentication.
Machine-to-machine only — no user-facing authentication flows.
Clients are trusted internal services and approved third-party partners.

## Components

- **key-manager**: generates, stores, and rotates API keys; handles
  authentication validation on every request
- **rate-limiter**: per-key throttling at 1000 req/min
- **audit-log**: records every authenticated API call with key ID and
  endpoint for compliance

## Security

API keys are hashed (SHA-256) at rest. Plain-text keys are never logged.
Authentication failures trigger rate limiting after 5 consecutive errors.
