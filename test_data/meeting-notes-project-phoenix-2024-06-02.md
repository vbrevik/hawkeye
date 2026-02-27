# Meeting Notes — Project Phoenix — 2024-06-02

**Date:** 2024-06-02
**Attendees:** Priya Patel, Tomas Novak, Ravi Sharma, Alice Chen, Sofia Andersen
**Project:** Project Phoenix

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Sofia Andersen raised concerns about token expiry edge case when clock skew > 30s.
Sofia Andersen explained that this was related to the recent changes in search-service.

We discussed migrating to React for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and search-service was identified as a risk.
Tomas Novak will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Add structured logging with trace ids — **Priya Patel** — Due 2026-01-12
- [ ] Document the deployment process — **Tomas Novak** — Due 2026-01-04
- [ ] Review and rotate all secrets in vault — **Ravi Sharma** — Due 2026-01-03
- [ ] Implement circuit breakers for downstream calls — **Tomas Novak** — Due 2026-02-24

## Notes

Stack: React
Services involved: search-service
