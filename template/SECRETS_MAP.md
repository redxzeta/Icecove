# ProjectName — Secrets Map

> **Updated**: 2026-03-06
>
> ⚠️ This file maps environment variable NAMES and their purpose ONLY.
> NEVER store actual values, keys, tokens, or credentials here.

## Environment Variables

### Application

| Variable | Required | Description | Example Format |
|----------|----------|-------------|----------------|
| `APP_ENV` | Yes | Runtime environment | `development`, `staging`, `production` |
| `APP_PORT` | Yes | HTTP server port | `8080` |
| `APP_LOG_LEVEL` | No | Log verbosity | `info`, `debug`, `trace` |
| `APP_SECRET_KEY` | Yes | Application signing key | 256-bit hex string |

### Database

| Variable | Required | Description | Example Format |
|----------|----------|-------------|----------------|
| `DATABASE_URL` | Yes | PostgreSQL connection string | `postgres://user:pass@host:5432/dbname` |
| `DATABASE_MAX_CONNECTIONS` | No | Connection pool size | `20` (default: 10) |
| `DATABASE_TIMEOUT_SEC` | No | Query timeout | `30` (default: 30) |

### Cache

| Variable | Required | Description | Example Format |
|----------|----------|-------------|----------------|
| `REDIS_URL` | Yes | Redis connection string | `redis://host:6379/0` |
| `REDIS_PASSWORD` | Prod only | Redis auth | string |

### Authentication

| Variable | Required | Description | Example Format |
|----------|----------|-------------|----------------|
| `JWT_PRIVATE_KEY` | Yes | RS256 private key (PEM) | Base64-encoded PEM |
| `JWT_PUBLIC_KEY` | Yes | RS256 public key (PEM) | Base64-encoded PEM |
| `JWT_ACCESS_TTL_SEC` | No | Access token lifetime | `900` (default: 900) |
| `JWT_REFRESH_TTL_SEC` | No | Refresh token lifetime | `604800` (default: 604800) |

### External Services

| Variable | Required | Description | Provider |
|----------|----------|-------------|----------|
| `AWS_ACCESS_KEY_ID` | Yes | AWS IAM access key | AWS |
| `AWS_SECRET_ACCESS_KEY` | Yes | AWS IAM secret key | AWS |
| `AWS_S3_BUCKET` | Yes | File storage bucket name | AWS S3 |
| `AWS_REGION` | Yes | AWS region | AWS |
| `SMTP_HOST` | Prod only | Mail server host | Email provider |
| `SMTP_PORT` | Prod only | Mail server port | Email provider |
| `SMTP_USERNAME` | Prod only | Mail auth user | Email provider |
| `SMTP_PASSWORD` | Prod only | Mail auth password | Email provider |

## Environment Profiles

| Profile | .env file | Description |
|---------|-----------|-------------|
| Development | `.env.development` | Local dev with defaults |
| Testing | `.env.test` | CI/CD test runner |
| Staging | `.env.staging` | Pre-production validation |
| Production | `.env.production` | Live environment (managed via secrets manager) |

## Secret Rotation

| Secret | Rotation Frequency | Method |
|--------|--------------------|--------|
| `APP_SECRET_KEY` | 90 days | Manual rotation, redeploy |
| `JWT_PRIVATE_KEY` | 180 days | Key pair regeneration |
| `DATABASE_URL` (password) | 90 days | Via cloud secrets manager |
| AWS credentials | 90 days | IAM key rotation |

## Notes

- Production secrets are managed via AWS Secrets Manager / Vault — never in .env files
- All secrets must be injected at runtime, never baked into Docker images
- CI/CD pipeline uses GitHub Actions secrets for test/staging environments
