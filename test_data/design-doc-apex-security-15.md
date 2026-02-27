# Design Doc: Apex Security

**Author:** Elena Rossi
**Date:** 2025-02-03
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Vault as the primary technology.

### Architecture

```
[Client] → [search-service] → [event-bus] → [Database]
                ↓
          [Vault]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Vault**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Docker
3. Request is forwarded to event-bus
4. Response is cached in Vault with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Vault | Fast, well-supported | Higher operational complexity |
| Docker | Simpler | Less performant |

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
