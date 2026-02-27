# Meeting Notes — Glacier Storage — 2025-03-15

**Date:** 2025-03-15
**Attendees:** Gina Torres, Kofi Mensah, David Park, Nadia Kovač, Bob Martins
**Project:** Glacier Storage

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Nadia Kovač raised concerns about disk I/O bottleneck during bulk import.
Nadia Kovač explained that this was related to the recent changes in auth-service.

We discussed migrating to ArgoCD for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and cache-layer was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Document the deployment process — **Bob Martins** — Due 2026-01-12
- [ ] Benchmark the new storage backend — **Kofi Mensah** — Due 2026-01-07
- [ ] Migrate the legacy monolith to microservices — **Nadia Kovač** — Due 2026-02-12

## Notes

Stack: ArgoCD
Services involved: auth-service, cache-layer
