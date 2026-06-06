# US-004 — CLI Interface

**Epic:** Phase 1 / Stage 1.1 — Vector Similarity Engine
**Status:** [ ] Not started

---

## User Story

As a **developer using the retrieval engine**,
I want **a CLI that loads embeddings from CSV files and queries the top-K most similar items for a given user**,
So that **I can run the full retrieval pipeline end-to-end from the terminal without writing code**.

---

## Context

US-001, US-002, and US-003 built the core components: embedding index, cosine similarity, and top-K search. But right now they only work through tests and hardcoded data. This story wires them into a usable pipeline — load real data from CSV, run a query, and display results — completing Stage 1.1.

---

## Acceptance Criteria

- [ ] Load user and item embeddings from CSV files into the index
- [ ] Accept a user ID and a value K as command-line arguments
- [ ] Return the K most similar items with their IDs and scores, in decreasing score order
- [ ] Display results in a formatted output:
  ```
  Top K similar items for user <id>:
    1. <item_id>  (score: <score>)
    2. <item_id>  (score: <score>)
  ```
- [ ] Fail with a clear error message if the file path does not exist
- [ ] Fail with a clear error message if the CSV is malformatted

---

## Failure Modes

| Failure                          | Expected behavior                                      |
| -------------------------------- | ------------------------------------------------------ |
| File path does not exist         | Clear error message, no panic                          |
| CSV is malformatted              | Clear error message indicating the issue, no panic     |
| User ID not found in index       | Propagate typed error from US-001                      |
| K = 0                            | Propagate typed error from US-003                      |
| K > number of items              | Return all available items (from US-003)               |

---

## Design Questions

1. What CSV format do you use? One file for users and one for items, or a single file with a type column?
2. What crate do you use for argument parsing — and why?
3. Where does the pipeline wiring live — `main.rs`, `lib.rs`, or a dedicated module?
4. How do you handle the CSV parsing — `csv` crate, or manual line splitting?

---

## Interview Signal

End-to-end system wiring: taking isolated components and composing them into a working pipeline.
Error handling across layers: surfacing the right error from the right component.
CLI design: clean argument parsing, helpful error messages, formatted output.
