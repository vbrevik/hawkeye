# Design Doc: Search Rewrite

**Author:** Laura Bianchi
**Date:** 2025-03-17
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [report-generator] → [api-gateway] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Kubernetes
3. Request is forwarded to api-gateway
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
| Kubernetes | Simpler | Less performant |

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
