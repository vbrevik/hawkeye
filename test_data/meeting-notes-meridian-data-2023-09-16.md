# Meeting Notes — Meridian Data — 2023-09-16

**Date:** 2023-09-16
**Attendees:** Sofia Andersen, Mohamed Al-Rashid
**Project:** Meridian Data

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Meridian Data. Sofia Andersen raised concerns about memory leak in the worker pool.
Sofia Andersen explained that this was related to the recent changes in data-warehouse.

We discussed migrating to DynamoDB for better performance. Mohamed Al-Rashid had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and data-warehouse was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- We will require code review from 2 engineers before merging.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Sofia Andersen** — Due 2026-01-30
- [ ] Add rate limiting to the public api — **Mohamed Al-Rashid** — Due 2026-01-12
- [ ] Benchmark the new storage backend — **Sofia Andersen** — Due 2026-02-11

## Notes

Stack: DynamoDB
Services involved: data-warehouse
