# Meeting Notes — Glacier Storage — 2023-01-21

**Date:** 2023-01-21
**Attendees:** Ravi Sharma, Sofia Andersen, Bob Martins, Alice Chen, Henrik Larsen
**Project:** Glacier Storage

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Bob Martins raised concerns about disk I/O bottleneck during bulk import.
Sofia Andersen explained that this was related to the recent changes in user-service.

We discussed migrating to Prometheus for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and user-service was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Henrik Larsen** — Due 2026-01-26
- [ ] Add structured logging with trace ids — **Alice Chen** — Due 2026-02-12

## Notes

Stack: Prometheus
Services involved: user-service
