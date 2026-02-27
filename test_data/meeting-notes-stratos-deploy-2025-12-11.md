# Meeting Notes — Stratos Deploy — 2025-12-11

**Date:** 2025-12-11
**Attendees:** Priya Patel, Mohamed Al-Rashid, Alice Chen
**Project:** Stratos Deploy

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Priya Patel raised concerns about retry storm after upstream timeout.
Mohamed Al-Rashid explained that this was related to the recent changes in search-service.

We discussed migrating to DynamoDB for better performance. Mohamed Al-Rashid had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and scheduler was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Chose gRPC over REST for the internal service mesh.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Alice Chen** — Due 2026-01-01
- [ ] Document the deployment process — **Alice Chen** — Due 2026-02-24
- [ ] Migrate the legacy monolith to microservices — **Alice Chen** — Due 2026-02-27

## Notes

Stack: DynamoDB
Services involved: search-service, scheduler
