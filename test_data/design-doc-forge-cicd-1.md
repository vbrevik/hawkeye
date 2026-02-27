# Design Doc: Forge CI/CD

**Author:** Tomas Novak
**Date:** 2023-01-13
**Status:** Draft

## Overview

This document describes the design for Forge CI/CD. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using Prometheus as the primary technology.

### Architecture

```
[Client] → [scheduler] → [report-generator] → [Database]
                ↓
          [Prometheus]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Prometheus**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Kubernetes
3. Request is forwarded to report-generator
4. Response is cached in Prometheus with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Prometheus | Fast, well-supported | Higher operational complexity |
| Kubernetes | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle SSL certificate not renewing automatically?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
