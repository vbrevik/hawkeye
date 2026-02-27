# Meeting Notes — Nexus API — 2025-07-16

**Date:** 2025-07-16
**Attendees:** Sofia Andersen, Mohamed Al-Rashid, Henrik Larsen, Alice Chen
**Project:** Nexus API

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Sofia Andersen raised concerns about race condition during concurrent writes.
Alice Chen explained that this was related to the recent changes in audit-logger.

We discussed migrating to Prometheus for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and audit-logger was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Write runbooks for the on-call team — **Henrik Larsen** — Due 2026-01-18
- [ ] Migrate the legacy monolith to microservices — **Henrik Larsen** — Due 2026-02-18
- [ ] Review and rotate all secrets in vault — **Henrik Larsen** — Due 2026-01-26

## Notes

Stack: Prometheus
Services involved: audit-logger
