# Design Doc: Auth Overhaul

**Author:** Kofi Mensah
**Date:** 2023-02-15
**Status:** Draft

## Overview

This document describes the design for Auth Overhaul. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Helm as the primary technology.

### Architecture

```
[Client] → [search-service] → [data-warehouse] → [Database]
                ↓
          [Helm]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **Helm**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Vault
3. Request is forwarded to data-warehouse
4. Response is cached in Helm with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Helm | Fast, well-supported | Higher operational complexity |
| Vault | Simpler | Less performant |

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
