# Meeting Notes — Stratos Deploy — 2023-06-16

**Date:** 2023-06-16
**Attendees:** Oscar Lindberg, Isabelle Dupont, Alice Chen, Kofi Mensah, Bob Martins
**Project:** Stratos Deploy

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Oscar Lindberg raised concerns about SSL certificate not renewing automatically.
Bob Martins explained that this was related to the recent changes in scheduler.

We discussed migrating to Prometheus for better performance. Kofi Mensah had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Feature flags will be managed via LaunchDarkly.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Document the deployment process — **Kofi Mensah** — Due 2026-01-17
- [ ] Migrate the legacy monolith to microservices — **Isabelle Dupont** — Due 2026-01-16
- [ ] Implement circuit breakers for downstream calls — **Kofi Mensah** — Due 2026-01-06
- [ ] Review and rotate all secrets in vault — **Oscar Lindberg** — Due 2026-02-24

## Notes

Stack: Prometheus, SQLite, PostgreSQL
Services involved: scheduler
