# Meeting Notes — Lighthouse CMS — 2023-03-26

**Date:** 2023-03-26
**Attendees:** Oscar Lindberg, Isabelle Dupont, Laura Bianchi
**Project:** Lighthouse CMS

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Oscar Lindberg raised concerns about memory leak in the worker pool.
Isabelle Dupont explained that this was related to the recent changes in event-bus.

We discussed migrating to ArgoCD for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and cache-layer was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Refactor the authentication middleware — **Isabelle Dupont** — Due 2026-01-21
- [ ] Set up alerting for p99 latency — **Oscar Lindberg** — Due 2026-01-01
- [ ] Benchmark the new storage backend — **Laura Bianchi** — Due 2026-02-04
- [ ] Add rate limiting to the public api — **Laura Bianchi** — Due 2026-01-22

## Notes

Stack: ArgoCD, Docker
Services involved: event-bus, cache-layer
