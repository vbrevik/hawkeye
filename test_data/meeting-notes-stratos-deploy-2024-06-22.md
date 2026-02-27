# Meeting Notes — Stratos Deploy — 2024-06-22

**Date:** 2024-06-22
**Attendees:** Laura Bianchi, Elena Rossi, Jae-won Kim, Kofi Mensah
**Project:** Stratos Deploy

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Elena Rossi raised concerns about flaky tests in the integration suite.
Jae-won Kim explained that this was related to the recent changes in scheduler.

We discussed migrating to PostgreSQL for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- Adopted conventional commits across all repositories.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Review and rotate all secrets in vault — **Elena Rossi** — Due 2026-02-12
- [ ] Document the deployment process — **Jae-won Kim** — Due 2026-01-19
- [ ] Benchmark the new storage backend — **Jae-won Kim** — Due 2026-01-02

## Notes

Stack: PostgreSQL, gRPC, Grafana, DynamoDB
Services involved: scheduler
