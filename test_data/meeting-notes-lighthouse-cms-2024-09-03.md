# Meeting Notes — Lighthouse CMS — 2024-09-03

**Date:** 2024-09-03
**Attendees:** Bob Martins, David Park
**Project:** Lighthouse CMS

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. David Park raised concerns about retry storm after upstream timeout.
David Park explained that this was related to the recent changes in search-service.

We discussed migrating to Axum for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and auth-service was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Write runbooks for the on-call team — **David Park** — Due 2026-02-23
- [ ] Benchmark the new storage backend — **Bob Martins** — Due 2026-01-19
- [ ] Migrate the legacy monolith to microservices — **David Park** — Due 2026-01-09
- [ ] Implement circuit breakers for downstream calls — **Bob Martins** — Due 2026-02-14

## Notes

Stack: Axum
Services involved: search-service, analytics-pipeline, auth-service
