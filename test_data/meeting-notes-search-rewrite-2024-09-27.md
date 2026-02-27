# Meeting Notes — Search Rewrite — 2024-09-27

**Date:** 2024-09-27
**Attendees:** Ravi Sharma, Tomas Novak, Frank Müller, Mohamed Al-Rashid
**Project:** Search Rewrite

## Agenda

- Status update on notification-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Tomas Novak raised concerns about goroutine leak in the WebSocket handler.
Ravi Sharma explained that this was related to the recent changes in notification-service.

We discussed migrating to ArgoCD for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between notification-service and report-generator was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Agreed to sunset the legacy Python service by end of Q2.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Document the deployment process — **Frank Müller** — Due 2026-01-15
- [ ] Review and rotate all secrets in vault — **Mohamed Al-Rashid** — Due 2026-02-22

## Notes

Stack: ArgoCD, Grafana, Elasticsearch
Services involved: notification-service, report-generator
