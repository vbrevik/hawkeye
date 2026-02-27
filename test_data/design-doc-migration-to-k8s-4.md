# Design Doc: Migration to K8s

**Author:** Laura Bianchi
**Date:** 2025-02-22
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using DynamoDB as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [scheduler] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **scheduler**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using SQLite
3. Request is forwarded to scheduler
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| SQLite | Simpler | Less performant |

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
