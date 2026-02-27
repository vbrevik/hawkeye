# Design Doc: Migration to K8s

**Author:** Henrik Larsen
**Date:** 2023-03-22
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using Celery as the primary technology.

### Architecture

```
[Client] → [event-bus] → [audit-logger] → [Database]
                ↓
          [Celery]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **Celery**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using Redis
3. Request is forwarded to audit-logger
4. Response is cached in Celery with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Celery | Fast, well-supported | Higher operational complexity |
| Redis | Simpler | Less performant |

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
