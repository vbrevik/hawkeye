# Design Doc: Glacier Storage

**Author:** Sofia Andersen
**Date:** 2024-07-24
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using Grafana as the primary technology.

### Architecture

```
[Client] → [scheduler] → [data-warehouse] → [Database]
                ↓
          [Grafana]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **Grafana**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using ArgoCD
3. Request is forwarded to data-warehouse
4. Response is cached in Grafana with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Grafana | Fast, well-supported | Higher operational complexity |
| ArgoCD | Simpler | Less performant |

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
