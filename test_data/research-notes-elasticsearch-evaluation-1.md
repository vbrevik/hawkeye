# Research Notes — Elasticsearch Evaluation

**Author:** Frank Müller
**Date:** 2024-11-25

## Purpose

Evaluating Elasticsearch for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for event-bus.

## Key Findings

### Performance

Initial benchmarks show Elasticsearch handles approximately 95,710 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is higher — roughly
458MB under load compared to our current 365MB.

### Operational Complexity

Elasticsearch requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Elasticsearch community is active.
Documentation quality is adequate.
Last major release: 2025-08-06.

### Integration

Integration with our existing api-gateway is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Elasticsearch.

**Rationale:** Adopted conventional commits across all repositories.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Frank Müller
- [ ] Prototype integration with cache-layer
- [ ] Get sign-off from Jae-won Kim
