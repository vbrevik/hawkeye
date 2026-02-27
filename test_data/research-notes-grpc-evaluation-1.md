# Research Notes — gRPC Evaluation

**Author:** Ravi Sharma
**Date:** 2026-01-07

## Purpose

Evaluating gRPC for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for scheduler.

## Key Findings

### Performance

Initial benchmarks show gRPC handles approximately 84,069 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is higher — roughly
162MB under load compared to our current 239MB.

### Operational Complexity

gRPC requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The gRPC community is growing.
Documentation quality is excellent.
Last major release: 2025-07-19.

### Integration

Integration with our existing analytics-pipeline is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt gRPC.

**Rationale:** Decided to go with a pull-based deployment model using ArgoCD.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Oscar Lindberg
- [ ] Prototype integration with audit-logger
- [ ] Get sign-off from Elena Rossi
