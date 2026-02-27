# Research Notes — Redis Evaluation

**Author:** Henrik Larsen
**Date:** 2023-03-31

## Purpose

Evaluating Redis for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for audit-logger.

## Key Findings

### Performance

Initial benchmarks show Redis handles approximately 73,125 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is similar — roughly
240MB under load compared to our current 334MB.

### Operational Complexity

Redis requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Redis community is active.
Documentation quality is adequate.
Last major release: 2025-02-18.

### Integration

Integration with our existing cache-layer is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Redis.

**Rationale:** Team agreed on a 2-week sprint cadence going forward.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Clara Johansson
- [ ] Prototype integration with search-service
- [ ] Get sign-off from Sofia Andersen
