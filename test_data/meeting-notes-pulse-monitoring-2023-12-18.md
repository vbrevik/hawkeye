# Meeting Notes — Pulse Monitoring — 2023-12-18

**Date:** 2023-12-18
**Attendees:** Oscar Lindberg, Mohamed Al-Rashid, Priya Patel, Jae-won Kim, Laura Bianchi
**Project:** Pulse Monitoring

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Mohamed Al-Rashid raised concerns about memory leak in the worker pool.
Mohamed Al-Rashid explained that this was related to the recent changes in api-gateway.

We discussed migrating to Celery for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and api-gateway was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Will use Redis for session storage — simple and battle-tested.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Mohamed Al-Rashid** — Due 2026-01-28
- [ ] Refactor the authentication middleware — **Laura Bianchi** — Due 2026-01-13

## Notes

Stack: Celery
Services involved: api-gateway
