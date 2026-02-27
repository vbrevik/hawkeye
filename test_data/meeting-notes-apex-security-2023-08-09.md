# Meeting Notes — Apex Security — 2023-08-09

**Date:** 2023-08-09
**Attendees:** Sofia Andersen, David Park
**Project:** Apex Security

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. David Park raised concerns about goroutine leak in the WebSocket handler.
Sofia Andersen explained that this was related to the recent changes in report-generator.

We discussed migrating to GraphQL for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and analytics-pipeline was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Document the deployment process — **Sofia Andersen** — Due 2026-02-10
- [ ] Review and rotate all secrets in vault — **David Park** — Due 2026-01-07
- [ ] Benchmark the new storage backend — **David Park** — Due 2026-01-27

## Notes

Stack: GraphQL, Helm, Vault, PostgreSQL
Services involved: report-generator, auth-service, analytics-pipeline
