# Design Doc: Search Rewrite

**Author:** Isabelle Dupont
**Date:** 2023-12-19
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Go as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [search-service] → [Database]
                ↓
          [Go]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **Go**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using GraphQL
3. Request is forwarded to search-service
4. Response is cached in Go with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Go | Fast, well-supported | Higher operational complexity |
| GraphQL | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle goroutine leak in the WebSocket handler?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
