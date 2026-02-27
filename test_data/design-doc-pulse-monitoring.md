# Design Doc: Pulse Monitoring

**Author:** Frank Müller
**Date:** 2026-01-24
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using SQLite as the primary technology.

### Architecture

```
[Client] → [event-bus] → [analytics-pipeline] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using GraphQL
3. Request is forwarded to analytics-pipeline
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| GraphQL | Simpler | Less performant |

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
