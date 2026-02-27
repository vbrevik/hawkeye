# Design Doc: Beacon Analytics

**Author:** Gina Torres
**Date:** 2025-08-26
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using Celery as the primary technology.

### Architecture

```
[Client] → [user-service] → [report-generator] → [Database]
                ↓
          [Celery]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Celery**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using SQLite
3. Request is forwarded to report-generator
4. Response is cached in Celery with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Celery | Fast, well-supported | Higher operational complexity |
| SQLite | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle disk I/O bottleneck during bulk import?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
