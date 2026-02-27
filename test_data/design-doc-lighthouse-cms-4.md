# Design Doc: Lighthouse CMS

**Author:** Kofi Mensah
**Date:** 2025-09-24
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Prometheus as the primary technology.

### Architecture

```
[Client] → [report-generator] → [cache-layer] → [Database]
                ↓
          [Prometheus]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **Prometheus**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Rust
3. Request is forwarded to cache-layer
4. Response is cached in Prometheus with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Prometheus | Fast, well-supported | Higher operational complexity |
| Rust | Simpler | Less performant |

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
