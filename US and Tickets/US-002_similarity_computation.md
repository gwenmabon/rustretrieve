# US-001 — Embedding Index

**Epic:** Phase 1 / Stage 1.1 — Vector Similarity Engine
**Status:** [ ] Not started

---

## User Story

As a **retrieval engine**,
I want to **compute a similarity measure between user and item embeddings in memory**,
So that **downstream components (top-K) can access any similarity measures between a user and an item in order to rank items**.

---

## Context

The Vector Similarity Engine serves a backend that needs to display recommendations when a user loads a webpage. In US-001, embeddings have been made accessible by the backend. Now that the index lives in the memory -- the next step is to compute simialrity between user's embedding and all the items's embeddings present in the index. Those similarities will make ranking possible

---

## Acceptance Criteria

- [ ] Return a typed `Result` error if either embedding is empty
- [ ] Return a typed `Result` error if the two vectors have different dimensions
- [ ] Embeddings are assumed pre-normalized (unit norm is a precondition, not enforced here)
- [ ] Output is an `f32` in the range [-1.0, 1.0]
- [ ] Cosine similarity of two identical vectors returns 1.0
- [ ] Cosine similarity of two orthogonal vectors returns 0.0

---

## Failure Modes

| Failure | Expected behavior |
|---------|-------------------|
| Vectors have different dimensions | Fail fast — return a typed error before any computation |
| Either vector is empty | Return a typed error |
| Input vectors are not normalized | Fail fast — return a typed error before any computation |
---

## Design Questions

1. How do you check if a vector is normalized in floating point?
   → `|‖v‖ - 1.0| < 1e-8`

2. Who owns the normalization responsibility?
   → The index (US-001) — embeddings are guaranteed normalized at insertion. US-002 trusts the index.

3. What parameter type for the function signature?
   → `&[f32]` — a slice, more general than `&Vec<f32>`, accepted by Clippy

4. Where does this function live?
   → A standalone math module (e.g. `similarity.rs`), not a method on the index

---

## Interview Signal

Error handling philosophy in Rust (Result, typed errors, no panics).
Fail-fast design: enforce invariants at the boundary (index insertion),
not scattered defensively across the codebase.
Floating point precision: knowing that ‖v‖ == 1.0 is unreliable and
using an epsilon tolerance instead.
Rust ownership: choosing &[f32] over &Vec<f32> or Vec<f32> — understanding
slices, borrowing, and what Clippy enforces.
