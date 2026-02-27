# Meeting Notes — Atlas Platform — 2025-06-08

**Date:** 2025-06-08
**Attendees:** Tomas Novak, Bob Martins, Jae-won Kim, Frank Müller, David Park
**Project:** Atlas Platform

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Jae-won Kim raised concerns about race condition during concurrent writes.
Tomas Novak explained that this was related to the recent changes in cache-layer.

We discussed migrating to Go for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and report-generator was identified as a risk.
Frank Müller will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Refactor the authentication middleware — **Jae-won Kim** — Due 2026-02-03
- [ ] Migrate the legacy monolith to microservices — **Tomas Novak** — Due 2026-01-17
- [ ] Add structured logging with trace ids — **David Park** — Due 2026-02-10
- [ ] Benchmark the new storage backend — **Tomas Novak** — Due 2026-02-01

## Notes

Stack: Go, Grafana, Rust
Services involved: cache-layer, scheduler, report-generator
