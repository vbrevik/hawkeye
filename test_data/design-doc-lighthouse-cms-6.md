# Design Doc: Lighthouse CMS

**Author:** Nadia Kovač
**Date:** 2024-03-29
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using gRPC as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [analytics-pipeline] → [Database]
                ↓
          [gRPC]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **gRPC**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Go
3. Request is forwarded to analytics-pipeline
4. Response is cached in gRPC with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| gRPC | Fast, well-supported | Higher operational complexity |
| Go | Simpler | Less performant |

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
