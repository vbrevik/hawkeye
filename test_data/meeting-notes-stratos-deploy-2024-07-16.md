# Meeting Notes — Stratos Deploy — 2024-07-16

**Date:** 2024-07-16
**Attendees:** Isabelle Dupont, Sofia Andersen, Bob Martins, Priya Patel, Laura Bianchi
**Project:** Stratos Deploy

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Laura Bianchi raised concerns about memory leak in the worker pool.
Isabelle Dupont explained that this was related to the recent changes in event-bus.

We discussed migrating to Docker for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and report-generator was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Chose gRPC over REST for the internal service mesh.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Laura Bianchi** — Due 2026-01-25
- [ ] Write runbooks for the on-call team — **Bob Martins** — Due 2026-02-10
- [ ] Migrate the legacy monolith to microservices — **Bob Martins** — Due 2026-02-27

## Notes

Stack: Docker, Kubernetes, DynamoDB
Services involved: event-bus, report-generator
