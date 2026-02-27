# Design Doc: Nexus API

**Author:** Isabelle Dupont
**Date:** 2025-02-21
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Axum as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [event-bus] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using Celery
3. Request is forwarded to event-bus
4. Response is cached in Axum with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Axum | Fast, well-supported | Higher operational complexity |
| Celery | Simpler | Less performant |

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
