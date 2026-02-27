# Meeting Notes — Pulse Monitoring — 2026-01-28

**Date:** 2026-01-28
**Attendees:** Oscar Lindberg, Elena Rossi, Priya Patel
**Project:** Pulse Monitoring

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Priya Patel raised concerns about goroutine leak in the WebSocket handler.
Oscar Lindberg explained that this was related to the recent changes in user-service.

We discussed migrating to ArgoCD for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and user-service was identified as a risk.
Priya Patel will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Document the deployment process — **Oscar Lindberg** — Due 2026-02-13
- [ ] Add rate limiting to the public api — **Elena Rossi** — Due 2026-01-10

## Notes

Stack: ArgoCD
Services involved: user-service
