# Meeting Notes — Atlas Platform — 2023-06-07

**Date:** 2023-06-07
**Attendees:** Ravi Sharma, Gina Torres, Priya Patel, Laura Bianchi
**Project:** Atlas Platform

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Ravi Sharma raised concerns about memory leak in the worker pool.
Priya Patel explained that this was related to the recent changes in payment-processor.

We discussed migrating to Celery for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and user-service was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Write runbooks for the on-call team — **Priya Patel** — Due 2026-01-06
- [ ] Set up alerting for p99 latency — **Gina Torres** — Due 2026-01-09
- [ ] Document the deployment process — **Gina Torres** — Due 2026-01-20
- [ ] Review and rotate all secrets in vault — **Gina Torres** — Due 2026-01-08

## Notes

Stack: Celery, ArgoCD, Vault
Services involved: payment-processor, cache-layer, user-service
