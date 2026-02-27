# Design Doc: Migration to K8s

**Author:** Oscar Lindberg
**Date:** 2025-10-08
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using Go as the primary technology.

### Architecture

```
[Client] → [scheduler] → [search-service] → [Database]
                ↓
          [Go]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **Go**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Axum
3. Request is forwarded to search-service
4. Response is cached in Go with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Go | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

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
