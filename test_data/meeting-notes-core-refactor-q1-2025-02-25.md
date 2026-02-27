# Meeting Notes — Core Refactor Q1 — 2025-02-25

**Date:** 2025-02-25
**Attendees:** Mohamed Al-Rashid, Ravi Sharma, Tomas Novak
**Project:** Core Refactor Q1

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Ravi Sharma raised concerns about token expiry edge case when clock skew > 30s.
Tomas Novak explained that this was related to the recent changes in notification-service.

We discussed migrating to gRPC for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and notification-service was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Document the deployment process — **Tomas Novak** — Due 2026-01-12
- [ ] Migrate the legacy monolith to microservices — **Mohamed Al-Rashid** — Due 2026-02-27
- [ ] Benchmark the new storage backend — **Mohamed Al-Rashid** — Due 2026-01-13

## Notes

Stack: gRPC
Services involved: notification-service
