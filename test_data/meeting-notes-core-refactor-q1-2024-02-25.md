# Meeting Notes — Core Refactor Q1 — 2024-02-25

**Date:** 2024-02-25
**Attendees:** Clara Johansson, Quinn Murphy, Oscar Lindberg, Bob Martins, Ravi Sharma
**Project:** Core Refactor Q1

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Oscar Lindberg raised concerns about memory leak in the worker pool.
Quinn Murphy explained that this was related to the recent changes in scheduler.

We discussed migrating to Elasticsearch for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and report-generator was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Add structured logging with trace ids — **Clara Johansson** — Due 2026-01-28
- [ ] Refactor the authentication middleware — **Oscar Lindberg** — Due 2026-02-14
- [ ] Document the deployment process — **Clara Johansson** — Due 2026-02-04

## Notes

Stack: Elasticsearch
Services involved: scheduler, user-service, report-generator
