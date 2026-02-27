# Meeting Notes — Search Rewrite — 2023-08-19

**Date:** 2023-08-19
**Attendees:** Nadia Kovač, Oscar Lindberg, Kofi Mensah, Henrik Larsen
**Project:** Search Rewrite

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Oscar Lindberg raised concerns about memory leak in the worker pool.
Kofi Mensah explained that this was related to the recent changes in data-warehouse.

We discussed migrating to SQLite for better performance. Kofi Mensah had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and notification-service was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Set up alerting for p99 latency — **Henrik Larsen** — Due 2026-01-04
- [ ] Add rate limiting to the public api — **Kofi Mensah** — Due 2026-01-17
- [ ] Document the deployment process — **Kofi Mensah** — Due 2026-01-27

## Notes

Stack: SQLite, Nginx, S3, gRPC
Services involved: data-warehouse, report-generator, notification-service
