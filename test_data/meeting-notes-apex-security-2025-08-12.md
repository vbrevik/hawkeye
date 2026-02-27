# Meeting Notes — Apex Security — 2025-08-12

**Date:** 2025-08-12
**Attendees:** Sofia Andersen, Oscar Lindberg
**Project:** Apex Security

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Sofia Andersen raised concerns about memory leak in the worker pool.
Sofia Andersen explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Axum for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and analytics-pipeline was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Sofia Andersen** — Due 2026-02-26
- [ ] Add rate limiting to the public api — **Sofia Andersen** — Due 2026-01-20

## Notes

Stack: Axum
Services involved: analytics-pipeline
