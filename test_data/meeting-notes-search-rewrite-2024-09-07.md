# Meeting Notes — Search Rewrite — 2024-09-07

**Date:** 2024-09-07
**Attendees:** Sofia Andersen, Elena Rossi, Tomas Novak
**Project:** Search Rewrite

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Sofia Andersen raised concerns about slow query on the user lookup table (missing index).
Sofia Andersen explained that this was related to the recent changes in notification-service.

We discussed migrating to RabbitMQ for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and event-bus was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- We will use Rust for the new service due to memory safety and performance.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Sofia Andersen** — Due 2026-01-12
- [ ] Set up alerting for p99 latency — **Sofia Andersen** — Due 2026-02-24
- [ ] Add structured logging with trace ids — **Sofia Andersen** — Due 2026-01-28

## Notes

Stack: RabbitMQ
Services involved: notification-service, event-bus
