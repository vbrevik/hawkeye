# Design Doc: Nexus API

**Author:** Henrik Larsen
**Date:** 2024-12-21
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Docker as the primary technology.

### Architecture

```
[Client] → [auth-service] → [event-bus] → [Database]
                ↓
          [Docker]
```

The system will consist of:
- **auth-service**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Docker**: Caching and state management

### Data Flow

1. Client sends request to auth-service
2. auth-service validates the token using GraphQL
3. Request is forwarded to event-bus
4. Response is cached in Docker with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Docker | Fast, well-supported | Higher operational complexity |
| GraphQL | Simpler | Less performant |

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
