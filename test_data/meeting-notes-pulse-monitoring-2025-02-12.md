# Meeting Notes — Pulse Monitoring — 2025-02-12

**Date:** 2025-02-12
**Attendees:** Isabelle Dupont, Ravi Sharma
**Project:** Pulse Monitoring

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Ravi Sharma raised concerns about disk I/O bottleneck during bulk import.
Isabelle Dupont explained that this was related to the recent changes in data-warehouse.

We discussed migrating to PostgreSQL for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and webhook-handler was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Set up alerting for p99 latency — **Ravi Sharma** — Due 2026-01-19
- [ ] Document the deployment process — **Isabelle Dupont** — Due 2026-01-01
- [ ] Review and rotate all secrets in vault — **Ravi Sharma** — Due 2026-02-13

## Notes

Stack: PostgreSQL, Helm
Services involved: data-warehouse, user-service, webhook-handler
