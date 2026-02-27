# Meeting Notes — Auth Overhaul — 2025-10-17

**Date:** 2025-10-17
**Attendees:** Quinn Murphy, Isabelle Dupont, Nadia Kovač
**Project:** Auth Overhaul

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Nadia Kovač raised concerns about SSL certificate not renewing automatically.
Isabelle Dupont explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Celery for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and search-service was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Chose gRPC over REST for the internal service mesh.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Document the deployment process — **Quinn Murphy** — Due 2026-02-03
- [ ] Add structured logging with trace ids — **Nadia Kovač** — Due 2026-01-06

## Notes

Stack: Celery, Redis, Rust, FastAPI
Services involved: analytics-pipeline, api-gateway, search-service
