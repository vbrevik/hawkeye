# Design Doc: Core Refactor Q1

**Author:** Elena Rossi
**Date:** 2025-08-20
**Status:** Draft

## Overview

This document describes the design for Core Refactor Q1. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using Kafka as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [analytics-pipeline] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using Terraform
3. Request is forwarded to analytics-pipeline
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| Terraform | Simpler | Less performant |

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
