# Design Doc: Nexus API

**Author:** Priya Patel
**Date:** 2025-09-16
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Axum as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [data-warehouse] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Grafana
3. Request is forwarded to data-warehouse
4. Response is cached in Axum with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Axum | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

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
