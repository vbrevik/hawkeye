# Meeting Notes — Atlas Platform — 2024-07-15

**Date:** 2024-07-15
**Attendees:** Isabelle Dupont, Clara Johansson, Laura Bianchi, Frank Müller
**Project:** Atlas Platform

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Clara Johansson raised concerns about flaky tests in the integration suite.
Isabelle Dupont explained that this was related to the recent changes in report-generator.

We discussed migrating to Kubernetes for better performance. Clara Johansson had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Write runbooks for the on-call team — **Frank Müller** — Due 2026-01-01
- [ ] Implement circuit breakers for downstream calls — **Isabelle Dupont** — Due 2026-02-14
- [ ] Review and rotate all secrets in vault — **Laura Bianchi** — Due 2026-02-22
- [ ] Set up alerting for p99 latency — **Isabelle Dupont** — Due 2026-02-25

## Notes

Stack: Kubernetes
Services involved: report-generator
