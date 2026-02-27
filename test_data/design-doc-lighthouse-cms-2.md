# Design Doc: Lighthouse CMS

**Author:** Quinn Murphy
**Date:** 2026-01-26
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Helm as the primary technology.

### Architecture

```
[Client] → [search-service] → [cache-layer] → [Database]
                ↓
          [Helm]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **Helm**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Kafka
3. Request is forwarded to cache-layer
4. Response is cached in Helm with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Helm | Fast, well-supported | Higher operational complexity |
| Kafka | Simpler | Less performant |

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
