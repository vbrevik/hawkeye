# Meeting Notes — Migration to K8s — 2025-08-10

**Date:** 2025-08-10
**Attendees:** Oscar Lindberg, Mohamed Al-Rashid, Ravi Sharma
**Project:** Migration to K8s

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Mohamed Al-Rashid raised concerns about memory leak in the worker pool.
Oscar Lindberg explained that this was related to the recent changes in user-service.

We discussed migrating to Docker for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and payment-processor was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Set up alerting for p99 latency — **Ravi Sharma** — Due 2026-01-31
- [ ] Add rate limiting to the public api — **Mohamed Al-Rashid** — Due 2026-01-10
- [ ] Write runbooks for the on-call team — **Mohamed Al-Rashid** — Due 2026-02-23

## Notes

Stack: Docker, Go, Helm, Rust
Services involved: user-service, payment-processor
