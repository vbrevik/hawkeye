# Meeting Notes — Nexus API — 2025-09-03

**Date:** 2025-09-03
**Attendees:** Laura Bianchi, Frank Müller, Mohamed Al-Rashid, Priya Patel, Quinn Murphy
**Project:** Nexus API

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Frank Müller raised concerns about slow query on the user lookup table (missing index).
Laura Bianchi explained that this was related to the recent changes in scheduler.

We discussed migrating to gRPC for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and auth-service was identified as a risk.
Priya Patel will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Quinn Murphy** — Due 2026-02-01
- [ ] Refactor the authentication middleware — **Mohamed Al-Rashid** — Due 2026-02-25

## Notes

Stack: gRPC, React, FastAPI
Services involved: scheduler, auth-service
