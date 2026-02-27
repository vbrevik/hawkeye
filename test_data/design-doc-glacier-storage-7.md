# Design Doc: Glacier Storage

**Author:** Laura Bianchi
**Date:** 2023-08-18
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using GraphQL as the primary technology.

### Architecture

```
[Client] → [scheduler] → [payment-processor] → [Database]
                ↓
          [GraphQL]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **payment-processor**: Core business logic
- **GraphQL**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Docker
3. Request is forwarded to payment-processor
4. Response is cached in GraphQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| GraphQL | Fast, well-supported | Higher operational complexity |
| Docker | Simpler | Less performant |

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
