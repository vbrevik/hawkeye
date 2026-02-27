# Design Doc: Meridian Data

**Author:** Isabelle Dupont
**Date:** 2026-01-24
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using GraphQL as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [webhook-handler] → [Database]
                ↓
          [GraphQL]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **GraphQL**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using React
3. Request is forwarded to webhook-handler
4. Response is cached in GraphQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| GraphQL | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle retry storm after upstream timeout?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
