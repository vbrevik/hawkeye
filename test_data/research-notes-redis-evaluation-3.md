# Research Notes — Redis Evaluation

**Author:** Nadia Kovač
**Date:** 2023-01-08

## Purpose

Evaluating Redis for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for scheduler.

## Key Findings

### Performance

Initial benchmarks show Redis handles approximately 94,863 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is lower — roughly
75MB under load compared to our current 467MB.

### Operational Complexity

Redis requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Redis community is growing.
Documentation quality is good.
Last major release: 2025-11-17.

### Integration

Integration with our existing cache-layer is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Redis.

**Rationale:** Feature flags will be managed via LaunchDarkly.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Gina Torres
- [ ] Prototype integration with report-generator
- [ ] Get sign-off from Priya Patel
