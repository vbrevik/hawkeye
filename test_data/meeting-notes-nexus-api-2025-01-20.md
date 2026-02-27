# Meeting Notes — Nexus API — 2025-01-20

**Date:** 2025-01-20
**Attendees:** Ravi Sharma, Gina Torres
**Project:** Nexus API

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Ravi Sharma raised concerns about retry storm after upstream timeout.
Gina Torres explained that this was related to the recent changes in event-bus.

We discussed migrating to Axum for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and scheduler was identified as a risk.
Gina Torres will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Benchmark the new storage backend — **Gina Torres** — Due 2026-01-04
- [ ] Add rate limiting to the public api — **Ravi Sharma** — Due 2026-01-24

## Notes

Stack: Axum, FastAPI
Services involved: event-bus, report-generator, scheduler
