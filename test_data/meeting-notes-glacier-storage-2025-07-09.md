# Meeting Notes — Glacier Storage — 2025-07-09

**Date:** 2025-07-09
**Attendees:** David Park, Bob Martins, Jae-won Kim
**Project:** Glacier Storage

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Bob Martins raised concerns about flaky tests in the integration suite.
Bob Martins explained that this was related to the recent changes in report-generator.

We discussed migrating to Nginx for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and payment-processor was identified as a risk.
David Park will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **David Park** — Due 2026-01-26
- [ ] Set up alerting for p99 latency — **Jae-won Kim** — Due 2026-01-30

## Notes

Stack: Nginx, Vault
Services involved: report-generator, analytics-pipeline, payment-processor
