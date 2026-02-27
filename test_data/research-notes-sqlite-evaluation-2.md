# Research Notes — SQLite Evaluation

**Author:** Henrik Larsen
**Date:** 2025-04-29

## Purpose

Evaluating SQLite for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for auth-service.

## Key Findings

### Performance

Initial benchmarks show SQLite handles approximately 59,336 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is lower — roughly
63MB under load compared to our current 178MB.

### Operational Complexity

SQLite requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The SQLite community is mature and stable.
Documentation quality is adequate.
Last major release: 2025-11-04.

### Integration

Integration with our existing analytics-pipeline is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Evaluate further SQLite.

**Rationale:** Decided to go with a pull-based deployment model using ArgoCD.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Sofia Andersen
- [ ] Prototype integration with data-warehouse
- [ ] Get sign-off from Henrik Larsen
