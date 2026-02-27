# Meeting Notes — Auth Overhaul — 2023-10-30

**Date:** 2023-10-30
**Attendees:** Alice Chen, Kofi Mensah, David Park, Jae-won Kim
**Project:** Auth Overhaul

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Kofi Mensah raised concerns about memory leak in the worker pool.
Jae-won Kim explained that this was related to the recent changes in cache-layer.

We discussed migrating to ArgoCD for better performance. Alice Chen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and cache-layer was identified as a risk.
David Park will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Kofi Mensah** — Due 2026-01-04
- [ ] Benchmark the new storage backend — **David Park** — Due 2026-01-13
- [ ] Document the deployment process — **David Park** — Due 2026-01-04

## Notes

Stack: ArgoCD, Nginx, S3
Services involved: cache-layer
