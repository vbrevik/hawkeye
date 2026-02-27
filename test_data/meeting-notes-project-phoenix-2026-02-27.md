# Meeting Notes — Project Phoenix — 2026-02-27

**Date:** 2026-02-27
**Attendees:** Oscar Lindberg, Laura Bianchi, Gina Torres
**Project:** Project Phoenix

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Gina Torres raised concerns about goroutine leak in the WebSocket handler.
Laura Bianchi explained that this was related to the recent changes in scheduler.

We discussed migrating to Helm for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Team agreed on a 2-week sprint cadence going forward.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Add rate limiting to the public api — **Oscar Lindberg** — Due 2026-01-16
- [ ] Document the deployment process — **Laura Bianchi** — Due 2026-02-11

## Notes

Stack: Helm
Services involved: scheduler
