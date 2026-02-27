# Design Doc: Auth Overhaul

**Author:** Tomas Novak
**Date:** 2023-05-21
**Status:** Draft

## Overview

This document describes the design for Auth Overhaul. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using PostgreSQL as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [webhook-handler] → [Database]
                ↓
          [PostgreSQL]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **PostgreSQL**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using Vault
3. Request is forwarded to webhook-handler
4. Response is cached in PostgreSQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| PostgreSQL | Fast, well-supported | Higher operational complexity |
| Vault | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle slow query on the user lookup table (missing index)?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
