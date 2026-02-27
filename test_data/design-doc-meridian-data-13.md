# Design Doc: Meridian Data

**Author:** Jae-won Kim
**Date:** 2023-05-03
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using GraphQL as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [audit-logger] → [Database]
                ↓
          [GraphQL]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **GraphQL**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Nginx
3. Request is forwarded to audit-logger
4. Response is cached in GraphQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| GraphQL | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle flaky tests in the integration suite?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
