# Research Notes — SQLite Evaluation

**Author:** Ravi Sharma
**Date:** 2025-09-27

## Purpose

Evaluating SQLite for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for event-bus.

## Key Findings

### Performance

Initial benchmarks show SQLite handles approximately 46,027 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is lower — roughly
214MB under load compared to our current 336MB.

### Operational Complexity

SQLite requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The SQLite community is growing.
Documentation quality is adequate.
Last major release: 2025-10-12.

### Integration

Integration with our existing webhook-handler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt SQLite.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Nadia Kovač
- [ ] Prototype integration with report-generator
- [ ] Get sign-off from Clara Johansson
