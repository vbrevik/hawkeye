# Meeting Notes — Beacon Analytics — 2025-12-14

**Date:** 2025-12-14
**Attendees:** Isabelle Dupont, Mohamed Al-Rashid, Clara Johansson, Laura Bianchi
**Project:** Beacon Analytics

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Isabelle Dupont raised concerns about cache invalidation not propagating across regions.
Isabelle Dupont explained that this was related to the recent changes in scheduler.

We discussed migrating to Elasticsearch for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and api-gateway was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Clara Johansson** — Due 2026-02-06
- [ ] Write runbooks for the on-call team — **Isabelle Dupont** — Due 2026-02-27
- [ ] Add rate limiting to the public api — **Isabelle Dupont** — Due 2026-01-15
- [ ] Set up alerting for p99 latency — **Mohamed Al-Rashid** — Due 2026-02-27

## Notes

Stack: Elasticsearch, ArgoCD, Nginx, PostgreSQL
Services involved: scheduler, auth-service, api-gateway
