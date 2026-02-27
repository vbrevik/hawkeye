# Meeting Notes — Search Rewrite — 2023-07-11

**Date:** 2023-07-11
**Attendees:** Frank Müller, Nadia Kovač, Priya Patel
**Project:** Search Rewrite

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Nadia Kovač raised concerns about SSL certificate not renewing automatically.
Priya Patel explained that this was related to the recent changes in data-warehouse.

We discussed migrating to SQLite for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and data-warehouse was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Frank Müller** — Due 2026-01-13
- [ ] Review and rotate all secrets in vault — **Nadia Kovač** — Due 2026-01-14

## Notes

Stack: SQLite, GraphQL, Docker
Services involved: data-warehouse
