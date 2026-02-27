# Design Doc: Lighthouse CMS

**Author:** Frank Müller
**Date:** 2025-01-23
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using Nginx as the primary technology.

### Architecture

```
[Client] → [scheduler] → [api-gateway] → [Database]
                ↓
          [Nginx]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **Nginx**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Helm
3. Request is forwarded to api-gateway
4. Response is cached in Nginx with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Nginx | Fast, well-supported | Higher operational complexity |
| Helm | Simpler | Less performant |

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
