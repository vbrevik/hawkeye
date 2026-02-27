# Design Doc: Apex Security

**Author:** Jae-won Kim
**Date:** 2023-07-26
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using Kafka as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [analytics-pipeline] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Helm
3. Request is forwarded to analytics-pipeline
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| Helm | Simpler | Less performant |

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
