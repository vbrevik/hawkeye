# Design Doc: Nexus API

**Author:** Kofi Mensah
**Date:** 2024-09-17
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using SQLite as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [audit-logger] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using DynamoDB
3. Request is forwarded to audit-logger
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| DynamoDB | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle goroutine leak in the WebSocket handler?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
