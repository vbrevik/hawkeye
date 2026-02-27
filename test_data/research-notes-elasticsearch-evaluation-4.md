# Research Notes — Elasticsearch Evaluation

**Author:** Nadia Kovač
**Date:** 2025-02-20

## Purpose

Evaluating Elasticsearch for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for cache-layer.

## Key Findings

### Performance

Initial benchmarks show Elasticsearch handles approximately 32,684 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is lower — roughly
293MB under load compared to our current 171MB.

### Operational Complexity

Elasticsearch requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Elasticsearch community is mature and stable.
Documentation quality is good.
Last major release: 2025-07-04.

### Integration

Integration with our existing webhook-handler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Elasticsearch.

**Rationale:** Decided to go with a pull-based deployment model using ArgoCD.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Jae-won Kim
- [ ] Prototype integration with webhook-handler
- [ ] Get sign-off from Elena Rossi
