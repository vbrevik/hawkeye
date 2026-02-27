# Meeting Notes — Auth Overhaul — 2024-08-21

**Date:** 2024-08-21
**Attendees:** Isabelle Dupont, Henrik Larsen, Alice Chen, Gina Torres, Jae-won Kim
**Project:** Auth Overhaul

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Henrik Larsen raised concerns about disk I/O bottleneck during bulk import.
Jae-won Kim explained that this was related to the recent changes in cache-layer.

We discussed migrating to Nginx for better performance. Alice Chen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and audit-logger was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Write runbooks for the on-call team — **Alice Chen** — Due 2026-01-25
- [ ] Set up alerting for p99 latency — **Gina Torres** — Due 2026-02-11

## Notes

Stack: Nginx, S3, Go, Grafana
Services involved: cache-layer, report-generator, audit-logger
