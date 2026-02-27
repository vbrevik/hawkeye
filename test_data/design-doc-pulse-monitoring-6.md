# Design Doc: Pulse Monitoring

**Author:** Elena Rossi
**Date:** 2023-11-26
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Kubernetes as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [auth-service] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using React
3. Request is forwarded to auth-service
4. Response is cached in Kubernetes with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kubernetes | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle SSL certificate not renewing automatically?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
