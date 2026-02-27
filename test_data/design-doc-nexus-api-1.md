# Design Doc: Nexus API

**Author:** Frank Müller
**Date:** 2024-06-10
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Terraform as the primary technology.

### Architecture

```
[Client] → [report-generator] → [analytics-pipeline] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using TypeScript
3. Request is forwarded to analytics-pipeline
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| TypeScript | Simpler | Less performant |

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
