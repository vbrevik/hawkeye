# Meeting Notes — Migration to K8s — 2024-12-25

**Date:** 2024-12-25
**Attendees:** Quinn Murphy, Jae-won Kim, Isabelle Dupont, Nadia Kovač, Henrik Larsen
**Project:** Migration to K8s

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Henrik Larsen raised concerns about memory leak in the worker pool.
Quinn Murphy explained that this was related to the recent changes in report-generator.

We discussed migrating to Redis for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Set up alerting for p99 latency — **Nadia Kovač** — Due 2026-02-21
- [ ] Add rate limiting to the public api — **Quinn Murphy** — Due 2026-01-28
- [ ] Benchmark the new storage backend — **Henrik Larsen** — Due 2026-02-16

## Notes

Stack: Redis, Go
Services involved: report-generator
