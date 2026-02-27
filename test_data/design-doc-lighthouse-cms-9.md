# Design Doc: Lighthouse CMS

**Author:** Sofia Andersen
**Date:** 2023-05-19
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Kafka as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [search-service] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Kubernetes
3. Request is forwarded to search-service
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| Kubernetes | Simpler | Less performant |

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
