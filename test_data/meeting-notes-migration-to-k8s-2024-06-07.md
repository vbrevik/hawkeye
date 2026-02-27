# Meeting Notes — Migration to K8s — 2024-06-07

**Date:** 2024-06-07
**Attendees:** Elena Rossi, Jae-won Kim
**Project:** Migration to K8s

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Jae-won Kim raised concerns about token expiry edge case when clock skew > 30s.
Jae-won Kim explained that this was related to the recent changes in user-service.

We discussed migrating to Kafka for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and user-service was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Set up alerting for p99 latency — **Elena Rossi** — Due 2026-01-16
- [ ] Migrate the legacy monolith to microservices — **Elena Rossi** — Due 2026-02-01
- [ ] Refactor the authentication middleware — **Jae-won Kim** — Due 2026-01-08
- [ ] Document the deployment process — **Jae-won Kim** — Due 2026-01-04

## Notes

Stack: Kafka, Prometheus, Rust, gRPC
Services involved: user-service
