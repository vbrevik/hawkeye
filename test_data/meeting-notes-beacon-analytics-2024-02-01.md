# Meeting Notes — Beacon Analytics — 2024-02-01

**Date:** 2024-02-01
**Attendees:** Frank Müller, Clara Johansson, Tomas Novak, Ravi Sharma, David Park
**Project:** Beacon Analytics

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Clara Johansson raised concerns about memory leak in the worker pool.
Clara Johansson explained that this was related to the recent changes in search-service.

We discussed migrating to Kubernetes for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and search-service was identified as a risk.
David Park will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Benchmark the new storage backend — **Frank Müller** — Due 2026-02-19
- [ ] Write runbooks for the on-call team — **David Park** — Due 2026-02-23
- [ ] Migrate the legacy monolith to microservices — **Tomas Novak** — Due 2026-02-01

## Notes

Stack: Kubernetes, Prometheus, GraphQL
Services involved: search-service
