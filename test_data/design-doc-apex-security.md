# Design Doc: Apex Security

**Author:** Nadia Kovač
**Date:** 2024-03-16
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, webhook-handler suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Kafka as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [payment-processor] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **payment-processor**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Celery
3. Request is forwarded to payment-processor
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| Celery | Simpler | Less performant |

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
