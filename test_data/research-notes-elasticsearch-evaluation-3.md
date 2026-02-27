# Research Notes — Elasticsearch Evaluation

**Author:** Jae-won Kim
**Date:** 2024-09-20

## Purpose

Evaluating Elasticsearch for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for notification-service.

## Key Findings

### Performance

Initial benchmarks show Elasticsearch handles approximately 28,292 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is lower — roughly
409MB under load compared to our current 585MB.

### Operational Complexity

Elasticsearch requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Elasticsearch community is growing.
Documentation quality is adequate.
Last major release: 2025-01-17.

### Integration

Integration with our existing notification-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Evaluate further Elasticsearch.

**Rationale:** Adopted conventional commits across all repositories.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Henrik Larsen
- [ ] Prototype integration with media-uploader
- [ ] Get sign-off from Sofia Andersen
