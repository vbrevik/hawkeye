# Meeting Notes — Apex Security — 2024-08-27

**Date:** 2024-08-27
**Attendees:** Tomas Novak, Frank Müller, Ravi Sharma
**Project:** Apex Security

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Ravi Sharma raised concerns about slow query on the user lookup table (missing index).
Tomas Novak explained that this was related to the recent changes in event-bus.

We discussed migrating to Kubernetes for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and audit-logger was identified as a risk.
Frank Müller will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Benchmark the new storage backend — **Ravi Sharma** — Due 2026-01-15
- [ ] Add structured logging with trace ids — **Ravi Sharma** — Due 2026-01-10
- [ ] Implement circuit breakers for downstream calls — **Ravi Sharma** — Due 2026-02-27

## Notes

Stack: Kubernetes, FastAPI, gRPC
Services involved: event-bus, webhook-handler, audit-logger
