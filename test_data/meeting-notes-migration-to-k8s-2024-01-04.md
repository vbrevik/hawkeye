# Meeting Notes — Migration to K8s — 2024-01-04

**Date:** 2024-01-04
**Attendees:** Nadia Kovač, Frank Müller, Laura Bianchi, Henrik Larsen
**Project:** Migration to K8s

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Frank Müller raised concerns about flaky tests in the integration suite.
Laura Bianchi explained that this was related to the recent changes in api-gateway.

We discussed migrating to Celery for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and auth-service was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Set up alerting for p99 latency — **Henrik Larsen** — Due 2026-01-19
- [ ] Document the deployment process — **Frank Müller** — Due 2026-02-03
- [ ] Migrate the legacy monolith to microservices — **Frank Müller** — Due 2026-01-07
- [ ] Review and rotate all secrets in vault — **Laura Bianchi** — Due 2026-02-15

## Notes

Stack: Celery, Kubernetes, Prometheus
Services involved: api-gateway, auth-service
