# Meeting Notes — Migration to K8s — 2025-12-10

**Date:** 2025-12-10
**Attendees:** Kofi Mensah, Quinn Murphy, Sofia Andersen
**Project:** Migration to K8s

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Sofia Andersen raised concerns about cache invalidation not propagating across regions.
Sofia Andersen explained that this was related to the recent changes in media-uploader.

We discussed migrating to Kubernetes for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and notification-service was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Kofi Mensah** — Due 2026-01-05
- [ ] Add structured logging with trace ids — **Quinn Murphy** — Due 2026-01-21
- [ ] Set up alerting for p99 latency — **Kofi Mensah** — Due 2026-01-22
- [ ] Benchmark the new storage backend — **Quinn Murphy** — Due 2026-01-04

## Notes

Stack: Kubernetes
Services involved: media-uploader, data-warehouse, notification-service
