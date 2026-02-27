# Meeting Notes — Core Refactor Q1 — 2023-02-07

**Date:** 2023-02-07
**Attendees:** Henrik Larsen, Clara Johansson, Jae-won Kim, Oscar Lindberg
**Project:** Core Refactor Q1

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Clara Johansson raised concerns about SSL certificate not renewing automatically.
Clara Johansson explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Rust for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and audit-logger was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Chose gRPC over REST for the internal service mesh.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Benchmark the new storage backend — **Oscar Lindberg** — Due 2026-01-03
- [ ] Refactor the authentication middleware — **Henrik Larsen** — Due 2026-02-10
- [ ] Set up alerting for p99 latency — **Oscar Lindberg** — Due 2026-02-05

## Notes

Stack: Rust, GraphQL, FastAPI, Nginx
Services involved: analytics-pipeline, audit-logger
