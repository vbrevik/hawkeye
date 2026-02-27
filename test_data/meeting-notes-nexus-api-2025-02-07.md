# Meeting Notes — Nexus API — 2025-02-07

**Date:** 2025-02-07
**Attendees:** Ravi Sharma, Clara Johansson
**Project:** Nexus API

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Ravi Sharma raised concerns about slow query on the user lookup table (missing index).
Clara Johansson explained that this was related to the recent changes in notification-service.

We discussed migrating to Prometheus for better performance. Clara Johansson had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and auth-service was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Refactor the authentication middleware — **Clara Johansson** — Due 2026-01-16
- [ ] Review and rotate all secrets in vault — **Ravi Sharma** — Due 2026-01-04
- [ ] Migrate the legacy monolith to microservices — **Clara Johansson** — Due 2026-02-06
- [ ] Write runbooks for the on-call team — **Clara Johansson** — Due 2026-01-10

## Notes

Stack: Prometheus
Services involved: notification-service, auth-service
