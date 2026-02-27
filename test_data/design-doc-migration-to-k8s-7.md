# Design Doc: Migration to K8s

**Author:** Frank Müller
**Date:** 2025-08-05
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Kafka as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [data-warehouse] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using FastAPI
3. Request is forwarded to data-warehouse
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| FastAPI | Simpler | Less performant |

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
