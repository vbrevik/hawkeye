# Research Notes — Prometheus Evaluation

**Author:** Jae-won Kim
**Date:** 2023-12-19

## Purpose

Evaluating Prometheus for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for auth-service.

## Key Findings

### Performance

Initial benchmarks show Prometheus handles approximately 20,544 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is lower — roughly
488MB under load compared to our current 455MB.

### Operational Complexity

Prometheus requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Prometheus community is mature and stable.
Documentation quality is excellent.
Last major release: 2025-06-07.

### Integration

Integration with our existing notification-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Prometheus.

**Rationale:** Decided to go with a pull-based deployment model using ArgoCD.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Clara Johansson
- [ ] Prototype integration with data-warehouse
- [ ] Get sign-off from David Park
