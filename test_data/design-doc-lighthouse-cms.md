# Design Doc: Lighthouse CMS

**Author:** Quinn Murphy
**Date:** 2023-01-21
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Prometheus as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [payment-processor] → [Database]
                ↓
          [Prometheus]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **payment-processor**: Core business logic
- **Prometheus**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using Terraform
3. Request is forwarded to payment-processor
4. Response is cached in Prometheus with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Prometheus | Fast, well-supported | Higher operational complexity |
| Terraform | Simpler | Less performant |

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
