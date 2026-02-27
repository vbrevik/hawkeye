# Research Notes — RabbitMQ Evaluation

**Author:** David Park
**Date:** 2025-09-02

## Purpose

Evaluating RabbitMQ for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for media-uploader.

## Key Findings

### Performance

Initial benchmarks show RabbitMQ handles approximately 22,793 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is similar — roughly
421MB under load compared to our current 351MB.

### Operational Complexity

RabbitMQ requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The RabbitMQ community is mature and stable.
Documentation quality is good.
Last major release: 2025-04-25.

### Integration

Integration with our existing audit-logger is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt RabbitMQ.

**Rationale:** Chose gRPC over REST for the internal service mesh.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Mohamed Al-Rashid
- [ ] Prototype integration with audit-logger
- [ ] Get sign-off from Mohamed Al-Rashid
