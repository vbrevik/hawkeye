# Meeting Notes — Auth Overhaul — 2024-11-03

**Date:** 2024-11-03
**Attendees:** Kofi Mensah, Priya Patel, Ravi Sharma, Bob Martins, Clara Johansson
**Project:** Auth Overhaul

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Priya Patel raised concerns about token expiry edge case when clock skew > 30s.
Kofi Mensah explained that this was related to the recent changes in scheduler.

We discussed migrating to GraphQL for better performance. Kofi Mensah had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Add rate limiting to the public api — **Kofi Mensah** — Due 2026-01-19
- [ ] Review and rotate all secrets in vault — **Ravi Sharma** — Due 2026-01-24

## Notes

Stack: GraphQL
Services involved: scheduler
