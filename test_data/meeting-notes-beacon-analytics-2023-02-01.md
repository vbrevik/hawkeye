# Meeting Notes — Beacon Analytics — 2023-02-01

**Date:** 2023-02-01
**Attendees:** Clara Johansson, Henrik Larsen, Ravi Sharma, Jae-won Kim
**Project:** Beacon Analytics

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Clara Johansson raised concerns about race condition during concurrent writes.
Ravi Sharma explained that this was related to the recent changes in user-service.

We discussed migrating to SQLite for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and media-uploader was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Document the deployment process — **Henrik Larsen** — Due 2026-01-29
- [ ] Write runbooks for the on-call team — **Jae-won Kim** — Due 2026-01-02

## Notes

Stack: SQLite, DynamoDB, Axum
Services involved: user-service, media-uploader
