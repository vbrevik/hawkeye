# Meeting Notes — Pulse Monitoring — 2025-02-21

**Date:** 2025-02-21
**Attendees:** Tomas Novak, Ravi Sharma, Gina Torres, Oscar Lindberg
**Project:** Pulse Monitoring

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Oscar Lindberg raised concerns about cache invalidation not propagating across regions.
Oscar Lindberg explained that this was related to the recent changes in payment-processor.

We discussed migrating to Grafana for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and report-generator was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Ravi Sharma** — Due 2026-02-06
- [ ] Review and rotate all secrets in vault — **Oscar Lindberg** — Due 2026-02-16
- [ ] Migrate the legacy monolith to microservices — **Ravi Sharma** — Due 2026-02-11

## Notes

Stack: Grafana, Rust, Nginx, Redis
Services involved: payment-processor, report-generator
