# Meeting Notes — Project Phoenix — 2025-08-28

**Date:** 2025-08-28
**Attendees:** Mohamed Al-Rashid, Bob Martins, David Park, Jae-won Kim, Tomas Novak
**Project:** Project Phoenix

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Bob Martins raised concerns about slow query on the user lookup table (missing index).
Bob Martins explained that this was related to the recent changes in media-uploader.

We discussed migrating to Go for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and media-uploader was identified as a risk.
Tomas Novak will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Write runbooks for the on-call team — **Bob Martins** — Due 2026-01-31
- [ ] Add structured logging with trace ids — **Jae-won Kim** — Due 2026-01-19
- [ ] Add rate limiting to the public api — **David Park** — Due 2026-02-12
- [ ] Implement circuit breakers for downstream calls — **Mohamed Al-Rashid** — Due 2026-02-07

## Notes

Stack: Go
Services involved: media-uploader
