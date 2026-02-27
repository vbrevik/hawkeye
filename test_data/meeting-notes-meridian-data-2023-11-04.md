# Meeting Notes — Meridian Data — 2023-11-04

**Date:** 2023-11-04
**Attendees:** Tomas Novak, Ravi Sharma, Frank Müller, Jae-won Kim
**Project:** Meridian Data

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Meridian Data. Jae-won Kim raised concerns about token expiry edge case when clock skew > 30s.
Tomas Novak explained that this was related to the recent changes in user-service.

We discussed migrating to PostgreSQL for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and user-service was identified as a risk.
Frank Müller will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Refactor the authentication middleware — **Tomas Novak** — Due 2026-01-04
- [ ] Add structured logging with trace ids — **Frank Müller** — Due 2026-01-09
- [ ] Document the deployment process — **Frank Müller** — Due 2026-02-26

## Notes

Stack: PostgreSQL
Services involved: user-service
