# Design Doc: Project Phoenix

**Author:** Frank Müller
**Date:** 2023-01-19
**Status:** Draft

## Overview

This document describes the design for Project Phoenix. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using PostgreSQL as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [webhook-handler] → [Database]
                ↓
          [PostgreSQL]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **PostgreSQL**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using Grafana
3. Request is forwarded to webhook-handler
4. Response is cached in PostgreSQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| PostgreSQL | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

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
