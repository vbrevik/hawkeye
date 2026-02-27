# Meeting Notes — Stratos Deploy — 2023-05-20

**Date:** 2023-05-20
**Attendees:** Jae-won Kim, Isabelle Dupont, Bob Martins, Laura Bianchi, Clara Johansson
**Project:** Stratos Deploy

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Clara Johansson raised concerns about SSL certificate not renewing automatically.
Laura Bianchi explained that this was related to the recent changes in api-gateway.

We discussed migrating to S3 for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and media-uploader was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Refactor the authentication middleware — **Clara Johansson** — Due 2026-02-20
- [ ] Set up alerting for p99 latency — **Jae-won Kim** — Due 2026-01-29

## Notes

Stack: S3
Services involved: api-gateway, media-uploader
