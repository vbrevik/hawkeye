# Meeting Notes — Beacon Analytics — 2026-02-26

**Date:** 2026-02-26
**Attendees:** Henrik Larsen, David Park
**Project:** Beacon Analytics

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Henrik Larsen raised concerns about slow query on the user lookup table (missing index).
Henrik Larsen explained that this was related to the recent changes in search-service.

We discussed migrating to S3 for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and audit-logger was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Agreed to sunset the legacy Python service by end of Q2.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Add structured logging with trace ids — **Henrik Larsen** — Due 2026-01-16
- [ ] Write runbooks for the on-call team — **Henrik Larsen** — Due 2026-02-05
- [ ] Review and rotate all secrets in vault — **Henrik Larsen** — Due 2026-01-11

## Notes

Stack: S3
Services involved: search-service, audit-logger
