# Meeting Notes — Apex Security — 2024-09-09

**Date:** 2024-09-09
**Attendees:** Jae-won Kim, Oscar Lindberg
**Project:** Apex Security

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Jae-won Kim raised concerns about token expiry edge case when clock skew > 30s.
Oscar Lindberg explained that this was related to the recent changes in report-generator.

We discussed migrating to SQLite for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Jae-won Kim** — Due 2026-01-03
- [ ] Benchmark the new storage backend — **Jae-won Kim** — Due 2026-02-06
- [ ] Implement circuit breakers for downstream calls — **Oscar Lindberg** — Due 2026-02-04

## Notes

Stack: SQLite, ArgoCD, Docker, DynamoDB
Services involved: report-generator
