# Meeting Notes — Auth Overhaul — 2023-06-09

**Date:** 2023-06-09
**Attendees:** Sofia Andersen, Nadia Kovač, Mohamed Al-Rashid, Tomas Novak
**Project:** Auth Overhaul

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Nadia Kovač raised concerns about goroutine leak in the WebSocket handler.
Tomas Novak explained that this was related to the recent changes in event-bus.

We discussed migrating to Redis for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and report-generator was identified as a risk.
Tomas Novak will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Set up alerting for p99 latency — **Mohamed Al-Rashid** — Due 2026-02-17
- [ ] Migrate the legacy monolith to microservices — **Mohamed Al-Rashid** — Due 2026-01-15

## Notes

Stack: Redis, gRPC, Kafka
Services involved: event-bus, webhook-handler, report-generator
