# Research Notes — Elasticsearch Evaluation

**Author:** Sofia Andersen
**Date:** 2025-04-10

## Purpose

Evaluating Elasticsearch for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for scheduler.

## Key Findings

### Performance

Initial benchmarks show Elasticsearch handles approximately 90,892 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is higher — roughly
384MB under load compared to our current 362MB.

### Operational Complexity

Elasticsearch requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Elasticsearch community is growing.
Documentation quality is excellent.
Last major release: 2025-04-27.

### Integration

Integration with our existing report-generator is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Elasticsearch.

**Rationale:** Feature flags will be managed via LaunchDarkly.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Henrik Larsen
- [ ] Prototype integration with cache-layer
- [ ] Get sign-off from Sofia Andersen
