# Research Notes — Celery Evaluation

**Author:** Nadia Kovač
**Date:** 2025-10-02

## Purpose

Evaluating Celery for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for report-generator.

## Key Findings

### Performance

Initial benchmarks show Celery handles approximately 22,479 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is lower — roughly
388MB under load compared to our current 331MB.

### Operational Complexity

Celery requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Celery community is mature and stable.
Documentation quality is adequate.
Last major release: 2025-12-12.

### Integration

Integration with our existing user-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Evaluate further Celery.

**Rationale:** Agreed to sunset the legacy Python service by end of Q2.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Sofia Andersen
- [ ] Prototype integration with scheduler
- [ ] Get sign-off from Quinn Murphy
