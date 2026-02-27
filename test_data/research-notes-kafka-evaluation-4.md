# Research Notes — Kafka Evaluation

**Author:** Nadia Kovač
**Date:** 2025-06-29

## Purpose

Evaluating Kafka for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for event-bus.

## Key Findings

### Performance

Initial benchmarks show Kafka handles approximately 39,479 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is lower — roughly
192MB under load compared to our current 474MB.

### Operational Complexity

Kafka requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Kafka community is active.
Documentation quality is good.
Last major release: 2026-02-16.

### Integration

Integration with our existing api-gateway is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Kafka.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Kofi Mensah
- [ ] Prototype integration with notification-service
- [ ] Get sign-off from Quinn Murphy
