# Design Doc: Meridian Data

**Author:** Jae-won Kim
**Date:** 2024-08-22
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using S3 as the primary technology.

### Architecture

```
[Client] → [user-service] → [webhook-handler] → [Database]
                ↓
          [S3]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **S3**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using ArgoCD
3. Request is forwarded to webhook-handler
4. Response is cached in S3 with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| S3 | Fast, well-supported | Higher operational complexity |
| ArgoCD | Simpler | Less performant |

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
