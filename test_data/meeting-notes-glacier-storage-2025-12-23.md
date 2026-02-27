# Meeting Notes — Glacier Storage — 2025-12-23

**Date:** 2025-12-23
**Attendees:** Bob Martins, David Park, Nadia Kovač
**Project:** Glacier Storage

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. David Park raised concerns about token expiry edge case when clock skew > 30s.
Nadia Kovač explained that this was related to the recent changes in audit-logger.

We discussed migrating to Axum for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and webhook-handler was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Nadia Kovač** — Due 2026-02-06
- [ ] Refactor the authentication middleware — **Bob Martins** — Due 2026-02-04
- [ ] Document the deployment process — **Nadia Kovač** — Due 2026-02-16
- [ ] Implement circuit breakers for downstream calls — **Bob Martins** — Due 2026-01-12

## Notes

Stack: Axum, Docker
Services involved: audit-logger, event-bus, webhook-handler
