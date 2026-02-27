# Meeting Notes — Auth Overhaul — 2023-03-11

**Date:** 2023-03-11
**Attendees:** Alice Chen, Frank Müller, Clara Johansson, Elena Rossi, Mohamed Al-Rashid
**Project:** Auth Overhaul

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Alice Chen raised concerns about race condition during concurrent writes.
Clara Johansson explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Helm for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and analytics-pipeline was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Benchmark the new storage backend — **Elena Rossi** — Due 2026-02-17
- [ ] Implement circuit breakers for downstream calls — **Elena Rossi** — Due 2026-01-19
- [ ] Write runbooks for the on-call team — **Elena Rossi** — Due 2026-02-14

## Notes

Stack: Helm, S3, Kafka
Services involved: analytics-pipeline
