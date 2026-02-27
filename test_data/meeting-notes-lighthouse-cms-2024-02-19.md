# Meeting Notes — Lighthouse CMS — 2024-02-19

**Date:** 2024-02-19
**Attendees:** Mohamed Al-Rashid, David Park
**Project:** Lighthouse CMS

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Mohamed Al-Rashid raised concerns about slow query on the user lookup table (missing index).
Mohamed Al-Rashid explained that this was related to the recent changes in event-bus.

We discussed migrating to React for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and event-bus was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Benchmark the new storage backend — **Mohamed Al-Rashid** — Due 2026-02-08
- [ ] Document the deployment process — **David Park** — Due 2026-02-05
- [ ] Add rate limiting to the public api — **David Park** — Due 2026-01-06

## Notes

Stack: React, Terraform
Services involved: event-bus
