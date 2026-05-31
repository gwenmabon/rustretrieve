# US-003 - Top K search

**Epic:** Phase 1 / Stage 1.1 — Vector Similarity Engine
**Status:** [ ] Not started

---

## User Story

As a retrieval engine, 
I want to be able to access the K top items relevant for a certain id in a N log K time complexity, 
So that the IDs of the K items can be selected.

---

## Goal

Given a user ID (or batch of user IDs), retrieve the K most similar items from the index using a min-heap, in O(N log K) rather than O(N log N) full sort.

---

## Acceptance Criteria

- Results in decreasing score order
- Scores included alongside item IDs
- At most K items per user
- O(N log K) complexity per user
- Uses BinaryHeap from stdlib

---

## Failure Modes


| Failure                            | Expected behavior                    |
| ---------------------------------- | ------------------------------------ |
| User ID not found in index         | Propagate typed error from US-001    |
| K = 0                              | Return a typed error                 |
| K > N (fewer items than requested) | Return all available items, no error |


---

## Design Questions

1. BinaryHeap requires Ord on its elements. f32 does not implement Ord. How do you deal with that?
2. You need a min-heap of size K, but Rust's BinaryHeap is a max-heap. How do you flip it?
3. What is the return type? A Vec<(u64, f32)>? A custom struct? What are the trade-offs?
4. Where does this function live? A method on the index, or a standalone function that takes the index as a parameter?

---

## Interview Signal

Data structure choice and complexity justification (why heap over sort). 
Rust trait system: navigating the Ord/PartialOrd gap for floats. 
Understanding of min-heap vs max-heap and how Reverse works. 
API design: choosing return types that serve downstream consumers. 