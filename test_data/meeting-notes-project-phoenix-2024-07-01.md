# Meeting Notes — Project Phoenix — 2024-07-01

**Date:** 2024-07-01
**Attendees:** Isabelle Dupont, Jae-won Kim, Kofi Mensah
**Project:** Project Phoenix

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Kofi Mensah raised concerns about SSL certificate not renewing automatically.
Isabelle Dupont explained that this was related to the recent changes in notification-service.

We discussed migrating to Redis for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and notification-service was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Document the deployment process — **Jae-won Kim** — Due 2026-01-22
- [ ] Review and rotate all secrets in vault — **Isabelle Dupont** — Due 2026-01-24

## Notes

Stack: Redis
Services involved: notification-service
