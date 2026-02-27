# Design Doc: Pulse Monitoring

**Author:** Frank Müller
**Date:** 2023-08-28
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Prometheus as the primary technology.

### Architecture

```
[Client] → [report-generator] → [notification-service] → [Database]
                ↓
          [Prometheus]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **Prometheus**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Kafka
3. Request is forwarded to notification-service
4. Response is cached in Prometheus with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Prometheus | Fast, well-supported | Higher operational complexity |
| Kafka | Simpler | Less performant |

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
