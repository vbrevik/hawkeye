# Research Notes — ArgoCD Evaluation

**Author:** Clara Johansson
**Date:** 2025-01-03

## Purpose

Evaluating ArgoCD for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for scheduler.

## Key Findings

### Performance

Initial benchmarks show ArgoCD handles approximately 97,757 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is similar — roughly
162MB under load compared to our current 589MB.

### Operational Complexity

ArgoCD requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The ArgoCD community is mature and stable.
Documentation quality is excellent.
Last major release: 2026-02-07.

### Integration

Integration with our existing cache-layer is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats ArgoCD.

**Rationale:** Team agreed on a 2-week sprint cadence going forward.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Ravi Sharma
- [ ] Prototype integration with cache-layer
- [ ] Get sign-off from Mohamed Al-Rashid
