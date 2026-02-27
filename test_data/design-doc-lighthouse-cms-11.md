# Design Doc: Lighthouse CMS

**Author:** Oscar Lindberg
**Date:** 2023-07-13
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Nginx as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [audit-logger] → [Database]
                ↓
          [Nginx]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **Nginx**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using Go
3. Request is forwarded to audit-logger
4. Response is cached in Nginx with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Nginx | Fast, well-supported | Higher operational complexity |
| Go | Simpler | Less performant |

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
