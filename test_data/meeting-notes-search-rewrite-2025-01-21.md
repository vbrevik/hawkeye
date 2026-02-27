# Meeting Notes — Search Rewrite — 2025-01-21

**Date:** 2025-01-21
**Attendees:** Henrik Larsen, Isabelle Dupont, Jae-won Kim
**Project:** Search Rewrite

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Jae-won Kim raised concerns about disk I/O bottleneck during bulk import.
Henrik Larsen explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Nginx for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and event-bus was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Document the deployment process — **Isabelle Dupont** — Due 2026-01-03
- [ ] Implement circuit breakers for downstream calls — **Jae-won Kim** — Due 2026-02-21
- [ ] Add rate limiting to the public api — **Jae-won Kim** — Due 2026-01-25

## Notes

Stack: Nginx, Celery
Services involved: analytics-pipeline, scheduler, event-bus
