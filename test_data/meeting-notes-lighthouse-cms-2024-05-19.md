# Meeting Notes — Lighthouse CMS — 2024-05-19

**Date:** 2024-05-19
**Attendees:** Henrik Larsen, Priya Patel, Gina Torres
**Project:** Lighthouse CMS

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Gina Torres raised concerns about SSL certificate not renewing automatically.
Priya Patel explained that this was related to the recent changes in api-gateway.

We discussed migrating to Vault for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and webhook-handler was identified as a risk.
Gina Torres will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Add structured logging with trace ids — **Henrik Larsen** — Due 2026-01-19
- [ ] Add rate limiting to the public api — **Priya Patel** — Due 2026-01-09

## Notes

Stack: Vault, Helm, Celery
Services involved: api-gateway, media-uploader, webhook-handler
