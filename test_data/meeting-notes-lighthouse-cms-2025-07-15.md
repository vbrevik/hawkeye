# Meeting Notes — Lighthouse CMS — 2025-07-15

**Date:** 2025-07-15
**Attendees:** Jae-won Kim, Bob Martins
**Project:** Lighthouse CMS

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Bob Martins raised concerns about flaky tests in the integration suite.
Jae-won Kim explained that this was related to the recent changes in cache-layer.

We discussed migrating to Prometheus for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and cache-layer was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- We will use Rust for the new service due to memory safety and performance.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Add rate limiting to the public api — **Jae-won Kim** — Due 2026-02-10
- [ ] Migrate the legacy monolith to microservices — **Bob Martins** — Due 2026-01-26
- [ ] Implement circuit breakers for downstream calls — **Jae-won Kim** — Due 2026-02-17
- [ ] Write runbooks for the on-call team — **Jae-won Kim** — Due 2026-01-29

## Notes

Stack: Prometheus, ArgoCD
Services involved: cache-layer
