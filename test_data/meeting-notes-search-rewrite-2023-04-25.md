# Meeting Notes — Search Rewrite — 2023-04-25

**Date:** 2023-04-25
**Attendees:** Laura Bianchi, Nadia Kovač, Sofia Andersen, David Park
**Project:** Search Rewrite

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Sofia Andersen raised concerns about SSL certificate not renewing automatically.
Nadia Kovač explained that this was related to the recent changes in cache-layer.

We discussed migrating to ArgoCD for better performance. Nadia Kovač had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and auth-service was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Review and rotate all secrets in vault — **David Park** — Due 2026-01-29
- [ ] Refactor the authentication middleware — **Laura Bianchi** — Due 2026-01-15

## Notes

Stack: ArgoCD, Helm, Rust
Services involved: cache-layer, auth-service
