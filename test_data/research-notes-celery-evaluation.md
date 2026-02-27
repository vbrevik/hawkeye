# Research Notes — Celery Evaluation

**Author:** Clara Johansson
**Date:** 2025-11-25

## Purpose

Evaluating Celery for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for api-gateway.

## Key Findings

### Performance

Initial benchmarks show Celery handles approximately 49,342 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is higher — roughly
445MB under load compared to our current 184MB.

### Operational Complexity

Celery requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Celery community is active.
Documentation quality is good.
Last major release: 2026-01-24.

### Integration

Integration with our existing auth-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Celery.

**Rationale:** Adopted conventional commits across all repositories.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Bob Martins
- [ ] Prototype integration with media-uploader
- [ ] Get sign-off from Quinn Murphy
