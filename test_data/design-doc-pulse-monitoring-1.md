# Design Doc: Pulse Monitoring

**Author:** Bob Martins
**Date:** 2025-06-21
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using DynamoDB as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [report-generator] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using Kafka
3. Request is forwarded to report-generator
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| Kafka | Simpler | Less performant |

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
