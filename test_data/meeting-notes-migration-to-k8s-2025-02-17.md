# Meeting Notes — Migration to K8s — 2025-02-17

**Date:** 2025-02-17
**Attendees:** Mohamed Al-Rashid, David Park, Laura Bianchi, Ravi Sharma, Jae-won Kim
**Project:** Migration to K8s

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Laura Bianchi raised concerns about memory leak in the worker pool.
David Park explained that this was related to the recent changes in scheduler.

We discussed migrating to Terraform for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and event-bus was identified as a risk.
David Park will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Write runbooks for the on-call team — **Jae-won Kim** — Due 2026-01-01
- [ ] Implement circuit breakers for downstream calls — **David Park** — Due 2026-02-03
- [ ] Add rate limiting to the public api — **David Park** — Due 2026-01-09
- [ ] Set up alerting for p99 latency — **Ravi Sharma** — Due 2026-02-18

## Notes

Stack: Terraform
Services involved: scheduler, data-warehouse, event-bus
