# Meeting Notes — Migration to K8s — 2025-09-18

**Date:** 2025-09-18
**Attendees:** Bob Martins, Gina Torres, David Park, Tomas Novak, Quinn Murphy
**Project:** Migration to K8s

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Gina Torres raised concerns about disk I/O bottleneck during bulk import.
Gina Torres explained that this was related to the recent changes in auth-service.

We discussed migrating to S3 for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and api-gateway was identified as a risk.
David Park will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Add rate limiting to the public api — **Bob Martins** — Due 2026-01-06
- [ ] Write runbooks for the on-call team — **Gina Torres** — Due 2026-02-24

## Notes

Stack: S3
Services involved: auth-service, scheduler, api-gateway
