# Meeting Notes — Pulse Monitoring — 2024-10-10

**Date:** 2024-10-10
**Attendees:** Isabelle Dupont, Mohamed Al-Rashid, Tomas Novak, Oscar Lindberg
**Project:** Pulse Monitoring

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Isabelle Dupont raised concerns about slow query on the user lookup table (missing index).
Mohamed Al-Rashid explained that this was related to the recent changes in notification-service.

We discussed migrating to RabbitMQ for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and notification-service was identified as a risk.
Isabelle Dupont will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Review and rotate all secrets in vault — **Isabelle Dupont** — Due 2026-01-05
- [ ] Migrate the legacy monolith to microservices — **Oscar Lindberg** — Due 2026-02-05
- [ ] Benchmark the new storage backend — **Isabelle Dupont** — Due 2026-01-02

## Notes

Stack: RabbitMQ, Terraform
Services involved: notification-service
