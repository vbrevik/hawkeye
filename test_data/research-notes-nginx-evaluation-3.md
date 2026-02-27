# Research Notes — Nginx Evaluation

**Author:** Quinn Murphy
**Date:** 2023-10-14

## Purpose

Evaluating Nginx for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for report-generator.

## Key Findings

### Performance

Initial benchmarks show Nginx handles approximately 44,257 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is higher — roughly
419MB under load compared to our current 272MB.

### Operational Complexity

Nginx requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Nginx community is mature and stable.
Documentation quality is adequate.
Last major release: 2025-01-07.

### Integration

Integration with our existing user-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt Nginx.

**Rationale:** Decided to go with a pull-based deployment model using ArgoCD.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Bob Martins
- [ ] Prototype integration with audit-logger
- [ ] Get sign-off from Bob Martins
