# Teaching Artifact Rules

Rules for generating course documents for each stage of the RustRetrieve project.

---

## Two Versions

Every stage course should be produced in two versions when requested:

### 1. Practical version (`stage_X_Y_practical.md`)

Target audience: an engineer who wants to understand the concepts well enough to implement them correctly.

Style:
- Plain-text math expressions (no LaTeX). Use code blocks for formulas when needed.
- Explain concepts through intuition, analogies, and concrete examples.
- Worked numerical examples with step-by-step computation.
- Focus on "what does this do" and "why do we use this" over formal derivations.
- Complexity analysis in plain language ("roughly 5x fewer operations").
- Reference CS terminology: arrays, hash maps, cache lines, FLOPS.
- Tone: clear, direct, like a good senior engineer explaining to a peer.

### 2. Maths version (`stage_X_Y_maths.md`)

Target audience: a reader with a PhD in mathematics (probability/statistics school) who wants the concepts grounded in proper mathematical framework.

Style:
- All math in LaTeX (`$...$` inline, `$$...$$` display).
- Ground concepts in the appropriate mathematical structures: inner product spaces, Hilbert spaces, metric spaces, manifolds (e.g., $S^{d-1}$), measure theory, order statistics, etc.
- State key results formally (Cauchy-Schwarz, polarization identity, equivalences on the unit sphere).
- Derivations and proofs where they illuminate the "why" (not for their own sake).
- Connect CS constructs to their mathematical counterparts (e.g., hash map = finite map, normalization = projection onto $S^{d-1}$, top-k = order statistics).
- Worked numerical examples with the same data as the practical version, but enriched with mathematical interpretation (angles, geodesic distances, verification of equivalences).
- The reader is NOT a computer scientist. Do not assume familiarity with CS jargon (cache lines, SIMD, etc.) without explanation. Do assume fluency in linear algebra, analysis, probability, and measure theory.
- Tone: precise but not dry. Like a well-written graduate textbook, not a paper.

---

## Shared Structure

Both versions of a given stage must follow the same section outline so they can be read side by side. The typical structure is:

1. **Problem statement**: What are we trying to solve and why.
2. **Core concepts**: The mathematical or conceptual objects involved (embeddings, similarity, etc.).
3. **Key results and relationships**: How the concepts relate to each other (equivalences, trade-offs).
4. **Algorithm**: How to solve the problem computationally (top-k, indexing, etc.).
5. **Complexity analysis**: Time, space, and what dominates.
6. **Data structure choices**: What to use and why, with trade-off discussion.
7. **Connection to real systems**: How this stage fits into a production recommendation system.
8. **Worked example**: A small-scale numerical example to verify an implementation against.
9. **Summary checklist**: What the reader should understand before coding.

Sections may be added, split, or reordered if the stage demands it, but the overall arc (theory, algorithm, analysis, example, checklist) should remain.

---

## Formatting Rules

### Both versions
- No emoticons.
- No em dashes. Use commas, colons, semicolons, or periods instead.
- Professional tone throughout.
- No code (Rust or otherwise). These are concept documents, not implementation guides.
- **No pseudocode. No step-by-step algorithms.** The whole point of this project is for the reader to figure out the implementation themselves.
- Use tables for comparisons and trade-offs.
- Use blockquotes (`>`) for key insights or takeaways.

### Data structures: explain from scratch, with diagrams

The reader has a PhD in mathematics but **no CS background in data structures**. Do NOT assume they know what a heap, a trie, a B-tree, or a graph traversal is. Every data structure introduced in a course must be explained from scratch:

1. **What it is** (the structure, the invariant, why it exists).
2. **How it is stored** (array, pointers, adjacency list, etc.).
3. **Visual diagrams** showing the structure. This is mandatory. Use:
   - **Mermaid diagrams** (`mermaid` code blocks) for tree structures, graphs, and flowcharts.
   - **ASCII art** for simple array-to-tree correspondences when mermaid is overkill.
4. **Step-by-step visual mutations**: show the state of the structure before and after key operations (insert, delete, etc.) with a diagram at each step. The reader needs to SEE what changes.
5. **Complexity proof**: why each operation costs what it costs (e.g., "the tree has height log n because each level doubles the number of nodes").

The reader learns by visualization. Text-only explanations of tree structures are not sufficient.

### Teaching principle: concept, not solution

The course documents must follow this rule strictly:

1. **Name** the data structure or algorithm family (e.g., "binary heap", "HNSW graph").
2. **Explain from scratch** what it is, what properties it satisfies, why it works. Include diagrams.
3. **Show visually** how the structure changes during key operations (insert, delete, query). Use before/after diagrams.
4. **Give hints** about the key insight (e.g., "a complete binary tree has a natural bijection with array indices").
5. **State** the complexity targets the implementation must achieve (e.g., "insert in O(log k), peek-min in O(1)").
6. **Prove the complexity** in terms the reader understands (tree height, path length, level doubling).
7. **Never** give the implementation: no pseudocode, no step-by-step algorithm, no "here is how sift-down works."

The reader's job is to go from the concept, the visuals, and the properties to a working implementation. That is where the learning happens.

Worked examples (Section 8) are the exception: they show full numerical computation so the reader can verify their implementation produces correct results.

### Stdlib-first philosophy (Phase 1)

In Phase 1, the course tells the reader to **use Rust's standard library** data structures (`BinaryHeap`, `HashMap`, `VecDeque`, etc.). The course must explain deeply HOW the data structure works internally and WHY it is the right choice for the problem, but the reader uses the stdlib implementation, not their own.

The learning value in Phase 1 comes from:
- Understanding the data structure's behavior, invariants, and complexity.
- Using it correctly in a real system context.
- Implementing the necessary Rust traits (`Ord`, `PartialOrd`, `Eq`, `Hash`, etc.) to integrate custom types with stdlib containers.

Reimplementation of data structures (e.g., writing your own heap) is a separate, focused exercise for Phase 2 or later. It should not block system-level progress in Phase 1.

### Maths version only
- Use `$...$` for all inline math: variables, sets, functions, norms, etc.
- Use `$$...$$` for all display math: definitions, equations, identities, derivations.
- Use proper symbols: $\langle \cdot, \cdot \rangle$ for inner product, $\|\cdot\|$ for norm, $\mathbb{R}^d$, $S^{d-1}$, $\varphi$ for embedding maps, $\forall$, $\exists$, $\in$, $\subseteq$, etc.
- Number or name key results for cross-reference (e.g., "the equivalence from Section 2.4").

---

## Naming Convention

Files are named by stage number and version:

```
stage_1_1_practical.md      # Stage 1.1, practical version
stage_1_1_maths.md          # Stage 1.1, maths version
stage_1_2_practical.md      # Stage 1.2, practical version
stage_1_2_maths.md          # Stage 1.2, maths version
...
```

When only one version is requested, use the appropriate suffix. When both are requested, produce both files.

---

## When the User Asks

- "Write me the course for stage X.Y" or "both versions": produce both files.
- "Write me the practical version": produce only `stage_X_Y_practical.md`.
- "Write me the maths version": produce only `stage_X_Y_maths.md`.
- If unspecified, ask which version(s) the user wants.
