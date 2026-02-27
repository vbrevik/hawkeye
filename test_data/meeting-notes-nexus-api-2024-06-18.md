# Meeting Notes — Nexus API — 2024-06-18

**Date:** 2024-06-18
**Attendees:** Bob Martins, David Park, Nadia Kovač, Laura Bianchi
**Project:** Nexus API

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. David Park raised concerns about retry storm after upstream timeout.
Laura Bianchi explained that this was related to the recent changes in report-generator.

We discussed migrating to Kubernetes for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- Team agreed on a 2-week sprint cadence going forward.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Document the deployment process — **Nadia Kovač** — Due 2026-02-12
- [ ] Implement circuit breakers for downstream calls — **Bob Martins** — Due 2026-02-16
- [ ] Write runbooks for the on-call team — **Nadia Kovač** — Due 2026-02-02

## Notes

Stack: Kubernetes, Vault, SQLite, ArgoCD
Services involved: report-generator
