# US-005 — Typed Feature Store with Basic API

**Epic:** Phase 1 / Stage 1.2 — In-Memory Feature Store
**Status:** [ ] Not started

---

## User Story

As a **ranking pipeline**,
I want **a typed feature store that can store, retrieve, and update numeric, categorical, and timestamp features per user or item**,
So that **I can access feature values alongside embeddings at ranking time, with type safety enforced at compile time**.

---

## Context

The embedding index (Stage 1.1) stores one vector per entity. But ranking requires more information than similarity alone: click-through rates, item categories, recency of interactions. These are features of different types (numeric, categorical, timestamp) that must coexist under a single model. This story defines the data types and the basic get/set/update operations so that features can be stored and retrieved by entity ID and feature name. Batch ingestion and time-windowed features are separate stories.

---

## Acceptance Criteria

- [x] A single `FeatureValue` type can represent a numeric value, a categorical value, or a timestamp
- [x] Each entity (user or item) can have multiple named features of different types
- [x] The type system is exhaustive: adding a new variant forces handling in all `match` expressions
- [x] Set a feature: store a named feature value for a given entity
- [x] Get a feature: retrieve a feature by entity ID and feature name
- [x] Update a feature: overwrite an existing feature value for a given entity
- [x] Missing features are represented explicitly (not via sentinel values or silent defaults)
- [x] Feature values can be displayed in human-readable form (for debugging and CLI output)

---

## Failure Modes


| Failure                                                     | Expected behavior                                                                   |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| Entity ID not found                                         | Return a specific, typed error                                                      |
| Feature name not found for an existing entity               | Return an explicit absence, not a panic or default                                  |
| Consumer assumes a feature is numeric but it is categorical | The type system makes this a compile-time or explicit match error, not a silent bug |
| Update called on a feature that does not exist yet          | Decide: treat as a set (upsert), or return an error? (design question)              |
| New feature type is added later (e.g., vector, boolean)     | Adding a variant to the enum causes compiler errors everywhere it is not handled    |


---

## Design Questions

1. `FeatureValue` as an `enum` with data vs `Box<dyn Feature>` trait object: what are the trade-offs?
2. How do you represent missing features: `Option<FeatureValue>` in the map, or absence from the map entirely?
3. What concrete Rust type do you use for timestamps?
4. How do you associate a set of named features with an entity: what nesting of maps or structs?
5. One store for users and items, or separate ones?
6. Update semantics: upsert (set-or-create) or strict update (fail if absent)?

---

## Interview Signal

Enum design and tagged unions. Pattern matching exhaustiveness. Rust's `Option` and `Result` for explicit error handling. `Display` trait implementation. API surface design: clean, minimal, hard to misuse. Data modeling decisions: maps of enums vs structs, flexibility vs type safety.