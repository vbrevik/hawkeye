# Meeting Notes — Forge CI/CD — 2024-05-18

**Date:** 2024-05-18
**Attendees:** Ravi Sharma, Priya Patel, Tomas Novak, Sofia Andersen, Henrik Larsen
**Project:** Forge CI/CD

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Forge CI/CD. Henrik Larsen raised concerns about flaky tests in the integration suite.
Priya Patel explained that this was related to the recent changes in auth-service.

We discussed migrating to Grafana for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and payment-processor was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Document the deployment process — **Tomas Novak** — Due 2026-01-24
- [ ] Implement circuit breakers for downstream calls — **Priya Patel** — Due 2026-01-19
- [ ] Add rate limiting to the public api — **Sofia Andersen** — Due 2026-01-10
- [ ] Add structured logging with trace ids — **Henrik Larsen** — Due 2026-01-27

## Notes

Stack: Grafana
Services involved: auth-service, payment-processor
