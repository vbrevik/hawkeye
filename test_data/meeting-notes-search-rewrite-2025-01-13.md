# Meeting Notes — Search Rewrite — 2025-01-13

**Date:** 2025-01-13
**Attendees:** Henrik Larsen, Bob Martins, Tomas Novak, Clara Johansson
**Project:** Search Rewrite

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Clara Johansson raised concerns about slow query on the user lookup table (missing index).
Bob Martins explained that this was related to the recent changes in media-uploader.

We discussed migrating to ArgoCD for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and media-uploader was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Benchmark the new storage backend — **Tomas Novak** — Due 2026-01-29
- [ ] Migrate the legacy monolith to microservices — **Clara Johansson** — Due 2026-01-08

## Notes

Stack: ArgoCD, Go, Terraform, Vault
Services involved: media-uploader
