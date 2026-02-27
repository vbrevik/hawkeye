# Design Doc: Auth Overhaul

**Author:** Henrik Larsen
**Date:** 2023-08-14
**Status:** Draft

## Overview

This document describes the design for Auth Overhaul. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Rust as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [event-bus] → [Database]
                ↓
          [Rust]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Rust**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using GraphQL
3. Request is forwarded to event-bus
4. Response is cached in Rust with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Rust | Fast, well-supported | Higher operational complexity |
| GraphQL | Simpler | Less performant |

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
