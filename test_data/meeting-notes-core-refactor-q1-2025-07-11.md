# Meeting Notes — Core Refactor Q1 — 2025-07-11

**Date:** 2025-07-11
**Attendees:** Jae-won Kim, Bob Martins, Henrik Larsen, Tomas Novak, Quinn Murphy
**Project:** Core Refactor Q1

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Henrik Larsen raised concerns about cache invalidation not propagating across regions.
Jae-won Kim explained that this was related to the recent changes in cache-layer.

We discussed migrating to DynamoDB for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and cache-layer was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Document the deployment process — **Henrik Larsen** — Due 2026-02-08
- [ ] Implement circuit breakers for downstream calls — **Quinn Murphy** — Due 2026-01-19
- [ ] Review and rotate all secrets in vault — **Tomas Novak** — Due 2026-01-08

## Notes

Stack: DynamoDB, Rust
Services involved: cache-layer
