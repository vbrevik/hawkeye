# Meeting Notes — Core Refactor Q1 — 2024-12-14

**Date:** 2024-12-14
**Attendees:** Oscar Lindberg, Isabelle Dupont, Bob Martins
**Project:** Core Refactor Q1

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Isabelle Dupont raised concerns about cache invalidation not propagating across regions.
Oscar Lindberg explained that this was related to the recent changes in auth-service.

We discussed migrating to ArgoCD for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and auth-service was identified as a risk.
Isabelle Dupont will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Document the deployment process — **Oscar Lindberg** — Due 2026-02-16
- [ ] Review and rotate all secrets in vault — **Isabelle Dupont** — Due 2026-02-06
- [ ] Add rate limiting to the public api — **Isabelle Dupont** — Due 2026-02-27

## Notes

Stack: ArgoCD, DynamoDB, PostgreSQL, Grafana
Services involved: auth-service
