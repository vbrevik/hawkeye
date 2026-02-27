# Design Doc: Nexus API

**Author:** David Park
**Date:** 2025-11-06
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Axum as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [report-generator] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Grafana
3. Request is forwarded to report-generator
4. Response is cached in Axum with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Axum | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

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
