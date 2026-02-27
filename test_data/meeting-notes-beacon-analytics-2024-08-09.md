# Meeting Notes — Beacon Analytics — 2024-08-09

**Date:** 2024-08-09
**Attendees:** Oscar Lindberg, David Park, Isabelle Dupont, Elena Rossi
**Project:** Beacon Analytics

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. David Park raised concerns about token expiry edge case when clock skew > 30s.
David Park explained that this was related to the recent changes in api-gateway.

We discussed migrating to ArgoCD for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and cache-layer was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Refactor the authentication middleware — **Isabelle Dupont** — Due 2026-02-09
- [ ] Benchmark the new storage backend — **Isabelle Dupont** — Due 2026-02-04

## Notes

Stack: ArgoCD, Prometheus, Terraform
Services involved: api-gateway, payment-processor, cache-layer
