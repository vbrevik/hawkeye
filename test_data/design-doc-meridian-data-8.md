# Design Doc: Meridian Data

**Author:** Mohamed Al-Rashid
**Date:** 2023-01-07
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using Nginx as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [scheduler] → [Database]
                ↓
          [Nginx]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **scheduler**: Core business logic
- **Nginx**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using gRPC
3. Request is forwarded to scheduler
4. Response is cached in Nginx with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Nginx | Fast, well-supported | Higher operational complexity |
| gRPC | Simpler | Less performant |

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
