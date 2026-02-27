# Meeting Notes — Stratos Deploy — 2024-10-03

**Date:** 2024-10-03
**Attendees:** Mohamed Al-Rashid, Jae-won Kim
**Project:** Stratos Deploy

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Mohamed Al-Rashid raised concerns about retry storm after upstream timeout.
Jae-won Kim explained that this was related to the recent changes in auth-service.

We discussed migrating to Rust for better performance. Mohamed Al-Rashid had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and auth-service was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Adopted conventional commits across all repositories.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Document the deployment process — **Mohamed Al-Rashid** — Due 2026-01-13
- [ ] Add rate limiting to the public api — **Jae-won Kim** — Due 2026-02-22
- [ ] Refactor the authentication middleware — **Mohamed Al-Rashid** — Due 2026-02-14

## Notes

Stack: Rust
Services involved: auth-service
