# Meeting Notes — Migration to K8s — 2023-02-11

**Date:** 2023-02-11
**Attendees:** Quinn Murphy, Mohamed Al-Rashid
**Project:** Migration to K8s

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Quinn Murphy raised concerns about goroutine leak in the WebSocket handler.
Mohamed Al-Rashid explained that this was related to the recent changes in scheduler.

We discussed migrating to Terraform for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and payment-processor was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Write runbooks for the on-call team — **Quinn Murphy** — Due 2026-01-05
- [ ] Refactor the authentication middleware — **Quinn Murphy** — Due 2026-02-02
- [ ] Implement circuit breakers for downstream calls — **Mohamed Al-Rashid** — Due 2026-01-02

## Notes

Stack: Terraform
Services involved: scheduler, report-generator, payment-processor
