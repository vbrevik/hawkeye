# Research Notes — Kafka Evaluation

**Author:** Isabelle Dupont
**Date:** 2025-05-25

## Purpose

Evaluating Kafka for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for notification-service.

## Key Findings

### Performance

Initial benchmarks show Kafka handles approximately 16,855 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is lower — roughly
143MB under load compared to our current 492MB.

### Operational Complexity

Kafka requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Kafka community is growing.
Documentation quality is good.
Last major release: 2025-09-11.

### Integration

Integration with our existing data-warehouse is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Kafka.

**Rationale:** Team agreed on a 2-week sprint cadence going forward.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Tomas Novak
- [ ] Prototype integration with scheduler
- [ ] Get sign-off from Kofi Mensah
