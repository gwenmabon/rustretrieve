# US-006 — Batch Ingestion for Feature Store

**Epic:** Phase 1 / Stage 1.2 — In-Memory Feature Store
**Status:** [ ] Not started

---

## User Story

As a **data pipeline operator**,
I want **to ingest multiple entities' features into the feature store in a single call**,
So that **scheduled batch jobs (e.g., hourly data dumps) can load features efficiently without calling upsert entity by entity**.

---

## Context

The feature store (US-005) supports get and upsert for individual entities. In production, feature data arrives in bulk: a data job runs periodically, produces a batch of user or item features, and the feature store must ingest all of it. A batch always contains a single entity type (users or items), since user pipelines and item pipelines are separate data sources. The batch method uses upsert semantics: new entities are inserted, existing entities have their features merged/overwritten. The Rust type system guarantees that all `FeatureValue`s in the batch are valid at compile time, so failure handling at this layer is minimal.

---

## Acceptance Criteria

- [ ] Batch upsert of N new entities makes all N accessible via `get` with correct feature values
- [ ] Batch upsert of entities that already exist in the store updates their features (overwrite per feature key, preserving features not in the batch)
- [ ] A single batch containing both new and existing entity IDs handles insert and update correctly in the same call
- [ ] An empty batch is a no-op: no panic, no side effects
- [ ] The method returns information about the operation (e.g., count of entities processed)

---

## Failure Modes

| Failure                          | Expected behavior                                                        |
|----------------------------------|--------------------------------------------------------------------------|
| Empty batch                      | No-op, returns a result indicating zero entities processed               |
| Duplicate entity IDs in batch    | Last occurrence wins (or define explicit merge behavior), no panic       |
| Very large batch                 | Completes without panic; performance degrades linearly, not explosively  |

---

## Design Questions

1. What input type represents the batch? A `Vec` of tuples, a `HashMap`, or a custom struct?
2. Should the method return a `Result`, a summary struct, or nothing (`()`)?
3. Should `batch_upsert` call `upsert` internally, or operate directly on the store's maps?
4. If the batch contains duplicate entity IDs, what is the merge strategy?

---

## Interview Signal

API design for bulk operations. Trade-offs between convenience and performance (iterating vs direct map operations). Choosing the right collection type for the input. Understanding upsert semantics in data systems. Reasoning about edge cases in batch processing.
