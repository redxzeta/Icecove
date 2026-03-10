# Architecture

## Overview

my-app uses JWT-based authentication with short-lived access tokens and
rotating refresh tokens stored in httpOnly cookies. OAuth2 social login
is delegated to Auth0.

## Components

- **auth-service**: issues and validates JWT tokens, handles authentication
  flows, manages refresh token rotation
- **user-service**: manages user profiles; requires authenticated requests
- **api-gateway**: validates JWT on every inbound request, rejects
  unauthenticated calls before they reach downstream services

## Security

All authentication state is stateless — no server-side sessions.
Token expiry: access 15 min, refresh 7 days.
