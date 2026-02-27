# Meeting Notes — Lighthouse CMS — 2023-11-07

**Date:** 2023-11-07
**Attendees:** Oscar Lindberg, Elena Rossi, Isabelle Dupont, Jae-won Kim
**Project:** Lighthouse CMS

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Jae-won Kim raised concerns about race condition during concurrent writes.
Oscar Lindberg explained that this was related to the recent changes in report-generator.

We discussed migrating to Helm for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Add structured logging with trace ids — **Elena Rossi** — Due 2026-01-07
- [ ] Migrate the legacy monolith to microservices — **Oscar Lindberg** — Due 2026-02-01
- [ ] Write runbooks for the on-call team — **Jae-won Kim** — Due 2026-01-21

## Notes

Stack: Helm, Grafana, S3
Services involved: report-generator
