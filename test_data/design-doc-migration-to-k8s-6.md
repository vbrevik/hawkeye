# Design Doc: Migration to K8s

**Author:** Isabelle Dupont
**Date:** 2025-11-13
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Grafana as the primary technology.

### Architecture

```
[Client] → [scheduler] → [event-bus] → [Database]
                ↓
          [Grafana]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Grafana**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Nginx
3. Request is forwarded to event-bus
4. Response is cached in Grafana with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Grafana | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

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
