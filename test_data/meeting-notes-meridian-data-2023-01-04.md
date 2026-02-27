# Meeting Notes — Meridian Data — 2023-01-04

**Date:** 2023-01-04
**Attendees:** Alice Chen, Kofi Mensah
**Project:** Meridian Data

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Meridian Data. Alice Chen raised concerns about cache invalidation not propagating across regions.
Kofi Mensah explained that this was related to the recent changes in media-uploader.

We discussed migrating to Go for better performance. Alice Chen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and media-uploader was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Document the deployment process — **Alice Chen** — Due 2026-02-02
- [ ] Set up alerting for p99 latency — **Alice Chen** — Due 2026-02-22
- [ ] Add rate limiting to the public api — **Kofi Mensah** — Due 2026-01-02
- [ ] Add structured logging with trace ids — **Alice Chen** — Due 2026-02-04

## Notes

Stack: Go, ArgoCD, TypeScript, Grafana
Services involved: media-uploader
