# Design Doc: Stratos Deploy

**Author:** Clara Johansson
**Date:** 2024-02-11
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using FastAPI as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [report-generator] → [Database]
                ↓
          [FastAPI]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **FastAPI**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Nginx
3. Request is forwarded to report-generator
4. Response is cached in FastAPI with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| FastAPI | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

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
