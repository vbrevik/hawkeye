# Design Doc: Pulse Monitoring

**Author:** Nadia Kovač
**Date:** 2025-05-03
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [auth-service] → [user-service] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **auth-service**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to auth-service
2. auth-service validates the token using Celery
3. Request is forwarded to user-service
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
| Celery | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle memory leak in the worker pool?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
