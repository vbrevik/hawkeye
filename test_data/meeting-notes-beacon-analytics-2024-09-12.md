# Meeting Notes — Beacon Analytics — 2024-09-12

**Date:** 2024-09-12
**Attendees:** Priya Patel, David Park, Henrik Larsen, Elena Rossi
**Project:** Beacon Analytics

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Henrik Larsen raised concerns about memory leak in the worker pool.
David Park explained that this was related to the recent changes in payment-processor.

We discussed migrating to FastAPI for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and webhook-handler was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Review and rotate all secrets in vault — **Elena Rossi** — Due 2026-01-09
- [ ] Add rate limiting to the public api — **Priya Patel** — Due 2026-01-12
- [ ] Implement circuit breakers for downstream calls — **David Park** — Due 2026-02-26

## Notes

Stack: FastAPI, Nginx, Axum, Vault
Services involved: payment-processor, webhook-handler
