# Design Doc: Apex Security

**Author:** Oscar Lindberg
**Date:** 2025-01-03
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using ArgoCD as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [data-warehouse] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using FastAPI
3. Request is forwarded to data-warehouse
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| FastAPI | Simpler | Less performant |

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
