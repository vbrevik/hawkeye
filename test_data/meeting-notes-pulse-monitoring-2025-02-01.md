# Meeting Notes — Pulse Monitoring — 2025-02-01

**Date:** 2025-02-01
**Attendees:** Quinn Murphy, Nadia Kovač, Clara Johansson
**Project:** Pulse Monitoring

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Quinn Murphy raised concerns about memory leak in the worker pool.
Clara Johansson explained that this was related to the recent changes in data-warehouse.

We discussed migrating to S3 for better performance. Clara Johansson had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and analytics-pipeline was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Clara Johansson** — Due 2026-01-19
- [ ] Document the deployment process — **Quinn Murphy** — Due 2026-01-21

## Notes

Stack: S3, gRPC, DynamoDB
Services involved: data-warehouse, report-generator, analytics-pipeline
