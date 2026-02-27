# Design Doc: Auth Overhaul

**Author:** Nadia Kovač
**Date:** 2023-12-28
**Status:** Draft

## Overview

This document describes the design for Auth Overhaul. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using TypeScript as the primary technology.

### Architecture

```
[Client] → [notification-service] → [webhook-handler] → [Database]
                ↓
          [TypeScript]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **TypeScript**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using Rust
3. Request is forwarded to webhook-handler
4. Response is cached in TypeScript with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| TypeScript | Fast, well-supported | Higher operational complexity |
| Rust | Simpler | Less performant |

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
