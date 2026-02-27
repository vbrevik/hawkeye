# Meeting Notes — Auth Overhaul — 2024-07-27

**Date:** 2024-07-27
**Attendees:** Quinn Murphy, Sofia Andersen, Priya Patel
**Project:** Auth Overhaul

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Quinn Murphy raised concerns about retry storm after upstream timeout.
Priya Patel explained that this was related to the recent changes in data-warehouse.

We discussed migrating to Nginx for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and media-uploader was identified as a risk.
Priya Patel will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Document the deployment process — **Priya Patel** — Due 2026-01-31
- [ ] Add rate limiting to the public api — **Priya Patel** — Due 2026-02-01
- [ ] Migrate the legacy monolith to microservices — **Sofia Andersen** — Due 2026-01-18

## Notes

Stack: Nginx, Vault, GraphQL
Services involved: data-warehouse, scheduler, media-uploader
