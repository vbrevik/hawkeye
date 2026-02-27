# Meeting Notes — Project Phoenix — 2023-11-06

**Date:** 2023-11-06
**Attendees:** Quinn Murphy, Oscar Lindberg, Laura Bianchi
**Project:** Project Phoenix

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Laura Bianchi raised concerns about token expiry edge case when clock skew > 30s.
Quinn Murphy explained that this was related to the recent changes in cache-layer.

We discussed migrating to Kubernetes for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and cache-layer was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Document the deployment process — **Laura Bianchi** — Due 2026-01-28
- [ ] Add structured logging with trace ids — **Quinn Murphy** — Due 2026-01-07
- [ ] Benchmark the new storage backend — **Quinn Murphy** — Due 2026-02-03
- [ ] Refactor the authentication middleware — **Quinn Murphy** — Due 2026-01-16

## Notes

Stack: Kubernetes, Vault, FastAPI, Terraform
Services involved: cache-layer
