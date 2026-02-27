# Meeting Notes — Atlas Platform — 2023-05-18

**Date:** 2023-05-18
**Attendees:** Oscar Lindberg, Bob Martins
**Project:** Atlas Platform

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Bob Martins raised concerns about disk I/O bottleneck during bulk import.
Bob Martins explained that this was related to the recent changes in notification-service.

We discussed migrating to React for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and notification-service was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Add structured logging with trace ids — **Oscar Lindberg** — Due 2026-02-12
- [ ] Implement circuit breakers for downstream calls — **Oscar Lindberg** — Due 2026-01-27
- [ ] Benchmark the new storage backend — **Bob Martins** — Due 2026-01-18

## Notes

Stack: React, FastAPI, Helm
Services involved: notification-service
