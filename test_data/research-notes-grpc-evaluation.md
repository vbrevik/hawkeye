# Research Notes — gRPC Evaluation

**Author:** Laura Bianchi
**Date:** 2025-03-28

## Purpose

Evaluating gRPC for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for payment-processor.

## Key Findings

### Performance

Initial benchmarks show gRPC handles approximately 23,131 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is higher — roughly
96MB under load compared to our current 486MB.

### Operational Complexity

gRPC requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The gRPC community is active.
Documentation quality is excellent.
Last major release: 2025-11-18.

### Integration

Integration with our existing notification-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt gRPC.

**Rationale:** Adopted conventional commits across all repositories.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Clara Johansson
- [ ] Prototype integration with data-warehouse
- [ ] Get sign-off from Mohamed Al-Rashid
