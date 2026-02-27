# Meeting Notes — Nexus API — 2026-01-27

**Date:** 2026-01-27
**Attendees:** Alice Chen, Priya Patel, Henrik Larsen, Ravi Sharma
**Project:** Nexus API

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Priya Patel raised concerns about goroutine leak in the WebSocket handler.
Priya Patel explained that this was related to the recent changes in payment-processor.

We discussed migrating to GraphQL for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and auth-service was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Henrik Larsen** — Due 2026-02-15
- [ ] Migrate the legacy monolith to microservices — **Henrik Larsen** — Due 2026-02-21
- [ ] Refactor the authentication middleware — **Henrik Larsen** — Due 2026-01-28

## Notes

Stack: GraphQL, gRPC, ArgoCD, Helm
Services involved: payment-processor, auth-service
