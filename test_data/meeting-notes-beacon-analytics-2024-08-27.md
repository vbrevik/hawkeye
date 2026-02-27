# Meeting Notes — Beacon Analytics — 2024-08-27

**Date:** 2024-08-27
**Attendees:** Isabelle Dupont, Clara Johansson, Priya Patel, Laura Bianchi, Quinn Murphy
**Project:** Beacon Analytics

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Priya Patel raised concerns about slow query on the user lookup table (missing index).
Priya Patel explained that this was related to the recent changes in search-service.

We discussed migrating to Grafana for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and report-generator was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Priya Patel** — Due 2026-01-06
- [ ] Refactor the authentication middleware — **Quinn Murphy** — Due 2026-02-19
- [ ] Review and rotate all secrets in vault — **Clara Johansson** — Due 2026-01-17
- [ ] Add rate limiting to the public api — **Clara Johansson** — Due 2026-01-30

## Notes

Stack: Grafana, DynamoDB, SQLite, RabbitMQ
Services involved: search-service, payment-processor, report-generator
