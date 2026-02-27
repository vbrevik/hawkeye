# Meeting Notes — Project Phoenix — 2024-10-21

**Date:** 2024-10-21
**Attendees:** Bob Martins, Kofi Mensah, Oscar Lindberg, Henrik Larsen
**Project:** Project Phoenix

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Bob Martins raised concerns about cache invalidation not propagating across regions.
Henrik Larsen explained that this was related to the recent changes in data-warehouse.

We discussed migrating to Celery for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and search-service was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Team agreed on a 2-week sprint cadence going forward.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Review and rotate all secrets in vault — **Henrik Larsen** — Due 2026-01-11
- [ ] Add structured logging with trace ids — **Henrik Larsen** — Due 2026-01-08
- [ ] Add rate limiting to the public api — **Henrik Larsen** — Due 2026-02-12

## Notes

Stack: Celery, Rust
Services involved: data-warehouse, user-service, search-service
