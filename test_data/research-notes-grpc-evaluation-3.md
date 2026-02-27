# Research Notes — gRPC Evaluation

**Author:** Clara Johansson
**Date:** 2024-01-21

## Purpose

Evaluating gRPC for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for notification-service.

## Key Findings

### Performance

Initial benchmarks show gRPC handles approximately 22,907 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is lower — roughly
376MB under load compared to our current 333MB.

### Operational Complexity

gRPC requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The gRPC community is growing.
Documentation quality is good.
Last major release: 2025-12-25.

### Integration

Integration with our existing search-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt gRPC.

**Rationale:** Agreed to sunset the legacy Python service by end of Q2.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Kofi Mensah
- [ ] Prototype integration with auth-service
- [ ] Get sign-off from Elena Rossi
