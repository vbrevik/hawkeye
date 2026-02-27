# Design Doc: Apex Security

**Author:** Mohamed Al-Rashid
**Date:** 2023-07-10
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using DynamoDB as the primary technology.

### Architecture

```
[Client] → [report-generator] → [user-service] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using gRPC
3. Request is forwarded to user-service
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| gRPC | Simpler | Less performant |

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
