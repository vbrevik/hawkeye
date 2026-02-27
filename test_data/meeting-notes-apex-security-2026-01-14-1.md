# Meeting Notes — Apex Security — 2026-01-14

**Date:** 2026-01-14
**Attendees:** David Park, Sofia Andersen, Mohamed Al-Rashid
**Project:** Apex Security

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Mohamed Al-Rashid raised concerns about disk I/O bottleneck during bulk import.
Sofia Andersen explained that this was related to the recent changes in user-service.

We discussed migrating to Kafka for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and event-bus was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Set up alerting for p99 latency — **Sofia Andersen** — Due 2026-01-06
- [ ] Review and rotate all secrets in vault — **David Park** — Due 2026-01-27
- [ ] Refactor the authentication middleware — **Mohamed Al-Rashid** — Due 2026-01-18

## Notes

Stack: Kafka, Terraform
Services involved: user-service, event-bus
