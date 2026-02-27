# Meeting Notes — Migration to K8s — 2025-03-06

**Date:** 2025-03-06
**Attendees:** Jae-won Kim, Elena Rossi, David Park, Tomas Novak, Priya Patel
**Project:** Migration to K8s

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Jae-won Kim raised concerns about goroutine leak in the WebSocket handler.
David Park explained that this was related to the recent changes in event-bus.

We discussed migrating to Terraform for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and data-warehouse was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Add rate limiting to the public api — **Tomas Novak** — Due 2026-02-10
- [ ] Set up alerting for p99 latency — **Tomas Novak** — Due 2026-01-06
- [ ] Implement circuit breakers for downstream calls — **Jae-won Kim** — Due 2026-02-02

## Notes

Stack: Terraform, Helm, S3, gRPC
Services involved: event-bus, data-warehouse
