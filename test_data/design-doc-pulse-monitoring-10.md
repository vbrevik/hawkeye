# Design Doc: Pulse Monitoring

**Author:** Alice Chen
**Date:** 2023-03-19
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [report-generator] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using Prometheus
3. Request is forwarded to report-generator
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
| Prometheus | Simpler | Less performant |

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
