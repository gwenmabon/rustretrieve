# US-007 Time Window: Keep Last N Events

**Epic:** Phase 1 / Stage 1.2 — In-Memory Feature Store
**Status:** [ ] Not started

---

## User Story

As a **ranking pipeline**,
I want **features that retain a bounded history of their last N values instead of only the latest one**,
So that **I can compute time-aware signals (e.g., average CTR over last 10 interactions, recent click categories) without unbounded memory growth**.

---

## Context

The feature store (US-005, US-006) stores one value per feature per entity. When `upsert` is called, the previous value is overwritten. This is correct for point-in-time features (current price, segment label), but insufficient for behavioral features that require recent history: click counts, purchase amounts, interaction categories over a sliding window.

A windowed feature keeps the last N values in a bounded buffer. When a new value arrives and the buffer is full, the oldest value is evicted. This is a classic sliding window pattern, and `VecDeque` from Rust's standard library provides the right semantics: O(1) push and pop at both ends, with a contiguous-ish memory layout.

The window size N is fixed at creation time and is part of the feature definition, not a query parameter. A windowed feature is a new variant of `FeatureValue`, not a separate data structure outside the store.

---



## Acceptance Criteria

- [x] A new `FeatureValue` variant can hold a bounded history of values with a configurable max capacity N
- [x] Pushing a value into a window that has not reached capacity appends it (window grows)
- [x] Pushing a value into a full window evicts the oldest value and appends the new one (FIFO)
- [ ] The current contents of a window can be retrieved in chronological order (oldest first)
- [x] `upsert` detects when the target feature is a window variant and pushes instead of replacing
- [x] `upsert` on a non-window feature continues to overwrite as before (no regression)
- [x] `Display` is implemented for the new variant so windowed features can be printed for debugging
- [ ] An empty window (capacity N, zero values pushed) is a valid state

---



## Failure Modes


| Failure                                                | Expected behavior                                                           |
| ------------------------------------------------------ | --------------------------------------------------------------------------- |
| Push onto a window with capacity 0                     | Decide: reject at creation time, or silently drop? (design question)        |
| `upsert` sends a single value for a window field       | The value is pushed into the existing window, not replaced                  |
| `upsert` sends a window variant for a non-window field | Decide: replace the feature entirely, or return an error? (design question) |
| Window capacity differs between old and new            | Decide: keep old capacity, adopt new capacity, or error? (design question)  |
| `get` on a window feature                              | Returns the window variant; caller must handle it via pattern matching      |


---



## Design Questions

1. What does the new `FeatureValue` variant look like? What types does it hold?
2. Where does the push-and-evict logic live: inside `upsert`, in a method on `FeatureValue`, or on a dedicated struct?
3. Should the values inside the window be `FeatureValue` themselves (recursive enum), or a simpler inner type?
4. How does `get` expose windowed features: return the whole `VecDeque`, or provide a separate query method?
5. Should there be a convenience method to create an empty window with a given capacity?

---



## Interview Signal

Understanding of bounded buffers and FIFO eviction. Choosing the right collection (`VecDeque` vs `Vec` with manual index management). Enum design for mixed single-value and multi-value features. Ownership and borrowing when pushing into a nested collection inside a `HashMap`. Handling the interaction between `upsert` semantics and windowed vs non-windowed features.