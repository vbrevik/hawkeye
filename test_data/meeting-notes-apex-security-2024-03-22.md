# Meeting Notes — Apex Security — 2024-03-22

**Date:** 2024-03-22
**Attendees:** Quinn Murphy, Oscar Lindberg
**Project:** Apex Security

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Quinn Murphy raised concerns about token expiry edge case when clock skew > 30s.
Quinn Murphy explained that this was related to the recent changes in api-gateway.

We discussed migrating to ArgoCD for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and payment-processor was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- Will use Redis for session storage — simple and battle-tested.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Write runbooks for the on-call team — **Quinn Murphy** — Due 2026-01-18
- [ ] Document the deployment process — **Quinn Murphy** — Due 2026-02-11

## Notes

Stack: ArgoCD
Services involved: api-gateway, notification-service, payment-processor
