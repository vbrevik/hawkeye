# Meeting Notes — Atlas Platform — 2025-09-27

**Date:** 2025-09-27
**Attendees:** Ravi Sharma, Kofi Mensah, Frank Müller
**Project:** Atlas Platform

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Kofi Mensah raised concerns about slow query on the user lookup table (missing index).
Ravi Sharma explained that this was related to the recent changes in data-warehouse.

We discussed migrating to Redis for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and data-warehouse was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- Team agreed on a 2-week sprint cadence going forward.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Write runbooks for the on-call team — **Kofi Mensah** — Due 2026-01-31
- [ ] Document the deployment process — **Ravi Sharma** — Due 2026-01-08
- [ ] Benchmark the new storage backend — **Ravi Sharma** — Due 2026-01-03
- [ ] Refactor the authentication middleware — **Kofi Mensah** — Due 2026-01-12

## Notes

Stack: Redis, ArgoCD
Services involved: data-warehouse
