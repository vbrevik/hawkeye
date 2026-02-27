# Meeting Notes — Auth Overhaul — 2024-03-05

**Date:** 2024-03-05
**Attendees:** Laura Bianchi, Nadia Kovač, Frank Müller, Sofia Andersen
**Project:** Auth Overhaul

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Frank Müller raised concerns about token expiry edge case when clock skew > 30s.
Frank Müller explained that this was related to the recent changes in auth-service.

We discussed migrating to TypeScript for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and auth-service was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Frank Müller** — Due 2026-01-02
- [ ] Document the deployment process — **Sofia Andersen** — Due 2026-01-03
- [ ] Set up alerting for p99 latency — **Nadia Kovač** — Due 2026-01-27
- [ ] Refactor the authentication middleware — **Laura Bianchi** — Due 2026-02-04

## Notes

Stack: TypeScript, Docker, Elasticsearch, DynamoDB
Services involved: auth-service
