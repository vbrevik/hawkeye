# Meeting Notes — Atlas Platform — 2023-04-02

**Date:** 2023-04-02
**Attendees:** David Park, Quinn Murphy, Nadia Kovač, Elena Rossi
**Project:** Atlas Platform

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. David Park raised concerns about flaky tests in the integration suite.
Quinn Murphy explained that this was related to the recent changes in notification-service.

We discussed migrating to React for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and analytics-pipeline was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **David Park** — Due 2026-01-19
- [ ] Review and rotate all secrets in vault — **Nadia Kovač** — Due 2026-01-05

## Notes

Stack: React, RabbitMQ, Elasticsearch
Services involved: notification-service, analytics-pipeline
