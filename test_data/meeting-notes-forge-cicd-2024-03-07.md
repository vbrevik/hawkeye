# Meeting Notes — Forge CI/CD — 2024-03-07

**Date:** 2024-03-07
**Attendees:** Sofia Andersen, Kofi Mensah, David Park, Laura Bianchi
**Project:** Forge CI/CD

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Forge CI/CD. Sofia Andersen raised concerns about slow query on the user lookup table (missing index).
David Park explained that this was related to the recent changes in audit-logger.

We discussed migrating to Docker for better performance. Kofi Mensah had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and audit-logger was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Write runbooks for the on-call team — **Kofi Mensah** — Due 2026-02-05
- [ ] Add structured logging with trace ids — **Kofi Mensah** — Due 2026-01-02
- [ ] Benchmark the new storage backend — **Sofia Andersen** — Due 2026-02-27

## Notes

Stack: Docker, Terraform
Services involved: audit-logger
