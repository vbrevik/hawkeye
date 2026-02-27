# Meeting Notes — Stratos Deploy — 2023-08-06

**Date:** 2023-08-06
**Attendees:** Tomas Novak, Gina Torres, Isabelle Dupont
**Project:** Stratos Deploy

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Tomas Novak raised concerns about race condition during concurrent writes.
Isabelle Dupont explained that this was related to the recent changes in report-generator.

We discussed migrating to Prometheus for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Gina Torres will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Chose gRPC over REST for the internal service mesh.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Tomas Novak** — Due 2026-01-06
- [ ] Refactor the authentication middleware — **Isabelle Dupont** — Due 2026-01-05
- [ ] Add structured logging with trace ids — **Isabelle Dupont** — Due 2026-02-16

## Notes

Stack: Prometheus, React, Kubernetes, RabbitMQ
Services involved: report-generator
