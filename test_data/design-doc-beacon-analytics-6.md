# Design Doc: Beacon Analytics

**Author:** Clara Johansson
**Date:** 2025-12-28
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using TypeScript as the primary technology.

### Architecture

```
[Client] → [search-service] → [notification-service] → [Database]
                ↓
          [TypeScript]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **TypeScript**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Celery
3. Request is forwarded to notification-service
4. Response is cached in TypeScript with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| TypeScript | Fast, well-supported | Higher operational complexity |
| Celery | Simpler | Less performant |

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
