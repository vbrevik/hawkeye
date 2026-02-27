# Meeting Notes — Core Refactor Q1 — 2023-11-12

**Date:** 2023-11-12
**Attendees:** Elena Rossi, Priya Patel
**Project:** Core Refactor Q1

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Priya Patel raised concerns about flaky tests in the integration suite.
Priya Patel explained that this was related to the recent changes in data-warehouse.

We discussed migrating to Terraform for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and report-generator was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Set up alerting for p99 latency — **Elena Rossi** — Due 2026-02-04
- [ ] Add structured logging with trace ids — **Priya Patel** — Due 2026-01-21
- [ ] Implement circuit breakers for downstream calls — **Priya Patel** — Due 2026-01-18
- [ ] Refactor the authentication middleware — **Elena Rossi** — Due 2026-01-05

## Notes

Stack: Terraform, PostgreSQL
Services involved: data-warehouse, analytics-pipeline, report-generator
