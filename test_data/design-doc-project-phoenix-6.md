# Design Doc: Project Phoenix

**Author:** Clara Johansson
**Date:** 2023-03-07
**Status:** Draft

## Overview

This document describes the design for Project Phoenix. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Terraform as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [api-gateway] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using Grafana
3. Request is forwarded to api-gateway
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

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
