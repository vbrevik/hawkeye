# Meeting Notes — Atlas Platform — 2025-12-19

**Date:** 2025-12-19
**Attendees:** Tomas Novak, Jae-won Kim, Nadia Kovač, Kofi Mensah, Isabelle Dupont
**Project:** Atlas Platform

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Jae-won Kim raised concerns about memory leak in the worker pool.
Tomas Novak explained that this was related to the recent changes in audit-logger.

We discussed migrating to Celery for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and audit-logger was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Chose gRPC over REST for the internal service mesh.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Document the deployment process — **Tomas Novak** — Due 2026-02-17
- [ ] Add structured logging with trace ids — **Jae-won Kim** — Due 2026-02-21
- [ ] Benchmark the new storage backend — **Isabelle Dupont** — Due 2026-01-27
- [ ] Implement circuit breakers for downstream calls — **Kofi Mensah** — Due 2026-01-03

## Notes

Stack: Celery, Docker
Services involved: audit-logger
