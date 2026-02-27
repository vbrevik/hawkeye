# Design Doc: Beacon Analytics

**Author:** Kofi Mensah
**Date:** 2025-06-24
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Docker as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [report-generator] → [Database]
                ↓
          [Docker]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Docker**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using Axum
3. Request is forwarded to report-generator
4. Response is cached in Docker with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Docker | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle token expiry edge case when clock skew > 30s?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
