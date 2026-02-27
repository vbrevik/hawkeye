# Design Doc: Forge CI/CD

**Author:** Sofia Andersen
**Date:** 2023-05-09
**Status:** Draft

## Overview

This document describes the design for Forge CI/CD. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using Celery as the primary technology.

### Architecture

```
[Client] → [scheduler] → [webhook-handler] → [Database]
                ↓
          [Celery]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **Celery**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using TypeScript
3. Request is forwarded to webhook-handler
4. Response is cached in Celery with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Celery | Fast, well-supported | Higher operational complexity |
| TypeScript | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle slow query on the user lookup table (missing index)?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
