# Meeting Notes — Nexus API — 2023-08-06

**Date:** 2023-08-06
**Attendees:** Bob Martins, Sofia Andersen
**Project:** Nexus API

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Bob Martins raised concerns about memory leak in the worker pool.
Bob Martins explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Axum for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and report-generator was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Adopted conventional commits across all repositories.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Add rate limiting to the public api — **Sofia Andersen** — Due 2026-02-27
- [ ] Benchmark the new storage backend — **Sofia Andersen** — Due 2026-02-27

## Notes

Stack: Axum, TypeScript, ArgoCD, FastAPI
Services involved: analytics-pipeline, user-service, report-generator
