# US-001 — Embedding Index

**Epic:** Phase 1 / Stage 1.1 — Vector Similarity Engine
**Status:** [ ] Not started

---

## User Story

As a **retrieval engine**,
I want **to store user and item embeddings in memory and retrieve them by ID**,
So that **downstream components (similarity, top-K) can access any embedding in constant time without hitting a database**.

---

## Context

The Vector Similarity Engine serves a backend that needs to display recommendations when a user loads a webpage. Before any similarity can be computed, embeddings must be accessible. This index is the foundation layer: a HashMap mapping entity IDs to their embedding vectors (128 floats). All embeddings are pre-computed and loaded into memory at startup. Nothing else in the engine works without this.

---

## Acceptance Criteria

- [ ] Store a user or item embedding by ID
- [ ] Retrieve an embedding by ID and get back the full vector (128 f32)
- [ ] Return a clear, typed error when an ID is not found
- [ ] Support 1M item embeddings in memory without exceeding reasonable memory bounds
- [ ] Lookup by ID completes in O(1) average time

---

## Failure Modes

| Failure | Expected behavior |
|---------|-------------------|
| User ID does not exist in the index | Return a specific "not found" error, not a panic |
| Item ID does not exist in the index | Same: specific error |
| Embedding is stale / too old | Out of scope for this story, but the data model should not prevent adding staleness checks later |

---

## Design Questions

1. One HashMap for both users and items, or two separate ones?
2. What is the key type — u64, String, a custom ID type?
3. How do you load 1M embeddings at startup — what does the data source look like?
4. How much memory does 1M x 128 x f32 actually consume?

---

## Interview Signal

Data structure choice and justification. Memory estimation (can you do the math on the spot?). Error handling philosophy in Rust (Result vs Option vs panic). Clean API surface design.
