# Design Doc: Search Rewrite

**Author:** Isabelle Dupont
**Date:** 2024-05-23
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using Vault as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [user-service] → [Database]
                ↓
          [Vault]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Vault**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using Elasticsearch
3. Request is forwarded to user-service
4. Response is cached in Vault with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Vault | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

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
