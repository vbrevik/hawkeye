# Meeting Notes — Lighthouse CMS — 2024-12-21

**Date:** 2024-12-21
**Attendees:** Henrik Larsen, Quinn Murphy, Isabelle Dupont, Bob Martins
**Project:** Lighthouse CMS

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Bob Martins raised concerns about SSL certificate not renewing automatically.
Quinn Murphy explained that this was related to the recent changes in media-uploader.

We discussed migrating to FastAPI for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and notification-service was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- Team agreed on a 2-week sprint cadence going forward.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Write runbooks for the on-call team — **Quinn Murphy** — Due 2026-01-28
- [ ] Implement circuit breakers for downstream calls — **Isabelle Dupont** — Due 2026-02-08
- [ ] Migrate the legacy monolith to microservices — **Isabelle Dupont** — Due 2026-01-29
- [ ] Add structured logging with trace ids — **Quinn Murphy** — Due 2026-01-14

## Notes

Stack: FastAPI
Services involved: media-uploader, notification-service
