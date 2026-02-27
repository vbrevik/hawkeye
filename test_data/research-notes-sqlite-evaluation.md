# Research Notes — SQLite Evaluation

**Author:** Tomas Novak
**Date:** 2026-02-01

## Purpose

Evaluating SQLite for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for report-generator.

## Key Findings

### Performance

Initial benchmarks show SQLite handles approximately 94,727 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is lower — roughly
132MB under load compared to our current 299MB.

### Operational Complexity

SQLite requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The SQLite community is active.
Documentation quality is excellent.
Last major release: 2026-02-26.

### Integration

Integration with our existing notification-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt SQLite.

**Rationale:** Adopted conventional commits across all repositories.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Nadia Kovač
- [ ] Prototype integration with api-gateway
- [ ] Get sign-off from Clara Johansson
