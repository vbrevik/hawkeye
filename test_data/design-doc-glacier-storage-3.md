# Design Doc: Glacier Storage

**Author:** Jae-won Kim
**Date:** 2024-09-01
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Terraform as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [report-generator] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using ArgoCD
3. Request is forwarded to report-generator
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| ArgoCD | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle token expiry edge case when clock skew > 30s?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
