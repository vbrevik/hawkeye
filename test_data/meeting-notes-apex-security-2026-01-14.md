# Meeting Notes — Apex Security — 2026-01-14

**Date:** 2026-01-14
**Attendees:** Sofia Andersen, Quinn Murphy
**Project:** Apex Security

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Quinn Murphy raised concerns about race condition during concurrent writes.
Quinn Murphy explained that this was related to the recent changes in scheduler.

We discussed migrating to Celery for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and payment-processor was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Refactor the authentication middleware — **Quinn Murphy** — Due 2026-02-25
- [ ] Add rate limiting to the public api — **Sofia Andersen** — Due 2026-01-22
- [ ] Migrate the legacy monolith to microservices — **Quinn Murphy** — Due 2026-01-12
- [ ] Document the deployment process — **Quinn Murphy** — Due 2026-01-02

## Notes

Stack: Celery, Helm
Services involved: scheduler, payment-processor
