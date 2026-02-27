# Design Doc: Apex Security

**Author:** Kofi Mensah
**Date:** 2025-08-01
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Kafka as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [user-service] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Elasticsearch
3. Request is forwarded to user-service
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle disk I/O bottleneck during bulk import?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
