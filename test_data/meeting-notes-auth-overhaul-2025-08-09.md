# Meeting Notes — Auth Overhaul — 2025-08-09

**Date:** 2025-08-09
**Attendees:** Frank Müller, Alice Chen, Bob Martins, Priya Patel
**Project:** Auth Overhaul

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Frank Müller raised concerns about goroutine leak in the WebSocket handler.
Frank Müller explained that this was related to the recent changes in user-service.

We discussed migrating to Docker for better performance. Alice Chen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and notification-service was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Review and rotate all secrets in vault — **Priya Patel** — Due 2026-01-14
- [ ] Set up alerting for p99 latency — **Bob Martins** — Due 2026-02-26
- [ ] Implement circuit breakers for downstream calls — **Bob Martins** — Due 2026-02-10
- [ ] Write runbooks for the on-call team — **Bob Martins** — Due 2026-02-18

## Notes

Stack: Docker
Services involved: user-service, auth-service, notification-service
