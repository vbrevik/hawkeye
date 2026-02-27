# Design Doc: Core Refactor Q1

**Author:** Tomas Novak
**Date:** 2024-12-21
**Status:** Draft

## Overview

This document describes the design for Core Refactor Q1. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Axum as the primary technology.

### Architecture

```
[Client] → [report-generator] → [scheduler] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **scheduler**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Grafana
3. Request is forwarded to scheduler
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

- How do we handle retry storm after upstream timeout?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
