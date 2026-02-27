# Meeting Notes — Lighthouse CMS — 2024-04-06

**Date:** 2024-04-06
**Attendees:** Clara Johansson, Isabelle Dupont, Gina Torres, Sofia Andersen
**Project:** Lighthouse CMS

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Clara Johansson raised concerns about memory leak in the worker pool.
Gina Torres explained that this was related to the recent changes in notification-service.

We discussed migrating to Helm for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and media-uploader was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Benchmark the new storage backend — **Clara Johansson** — Due 2026-02-01
- [ ] Add rate limiting to the public api — **Gina Torres** — Due 2026-02-11
- [ ] Migrate the legacy monolith to microservices — **Gina Torres** — Due 2026-01-31
- [ ] Write runbooks for the on-call team — **Isabelle Dupont** — Due 2026-02-21

## Notes

Stack: Helm, PostgreSQL, Kubernetes
Services involved: notification-service, audit-logger, media-uploader
