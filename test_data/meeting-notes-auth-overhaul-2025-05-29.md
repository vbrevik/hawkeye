# Meeting Notes — Auth Overhaul — 2025-05-29

**Date:** 2025-05-29
**Attendees:** Priya Patel, Isabelle Dupont, Kofi Mensah
**Project:** Auth Overhaul

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Isabelle Dupont raised concerns about memory leak in the worker pool.
Priya Patel explained that this was related to the recent changes in auth-service.

We discussed migrating to DynamoDB for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and auth-service was identified as a risk.
Isabelle Dupont will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Isabelle Dupont** — Due 2026-01-01
- [ ] Add structured logging with trace ids — **Isabelle Dupont** — Due 2026-01-13
- [ ] Set up alerting for p99 latency — **Kofi Mensah** — Due 2026-01-09
- [ ] Benchmark the new storage backend — **Kofi Mensah** — Due 2026-01-20

## Notes

Stack: DynamoDB
Services involved: auth-service
