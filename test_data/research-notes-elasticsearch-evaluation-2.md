# Research Notes — Elasticsearch Evaluation

**Author:** Oscar Lindberg
**Date:** 2024-09-01

## Purpose

Evaluating Elasticsearch for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for api-gateway.

## Key Findings

### Performance

Initial benchmarks show Elasticsearch handles approximately 95,257 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is lower — roughly
275MB under load compared to our current 312MB.

### Operational Complexity

Elasticsearch requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Elasticsearch community is growing.
Documentation quality is excellent.
Last major release: 2025-06-24.

### Integration

Integration with our existing search-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Elasticsearch.

**Rationale:** Chose gRPC over REST for the internal service mesh.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Kofi Mensah
- [ ] Prototype integration with report-generator
- [ ] Get sign-off from Laura Bianchi
