# Meeting Notes — Migration to K8s — 2023-09-23

**Date:** 2023-09-23
**Attendees:** Henrik Larsen, Clara Johansson, David Park
**Project:** Migration to K8s

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. David Park raised concerns about disk I/O bottleneck during bulk import.
David Park explained that this was related to the recent changes in report-generator.

We discussed migrating to Prometheus for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
David Park will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Will use Redis for session storage — simple and battle-tested.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Clara Johansson** — Due 2026-02-18
- [ ] Write runbooks for the on-call team — **Henrik Larsen** — Due 2026-01-21

## Notes

Stack: Prometheus, Celery, GraphQL
Services involved: report-generator
