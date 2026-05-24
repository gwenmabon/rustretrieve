# 🦀 RustRetrieve — Roadmap

**Staff MLE interview prep + Rust fluency + portfolio piece.**

You're a Staff MLE at Adikteev with deep Python expertise and production RTB experience.
This project has three goals:

1. **Sharpen DS&A** for coding rounds — through a real system, not abstract grinding
2. **Build Rust fluency** as a systems language alongside Python
3. **Create a portfolio piece** that shows ML systems depth at FAANG interview bar

---

## 🧭 Why This Project

You already know how recommendation and ad-serving systems work in production.
The gap for top-tier interviews isn't knowledge — it's:

- **Coding rounds:** Articulating solutions fast, in clean code, under time pressure. You need the patterns drilled, not explained.
- **System design rounds:** You have the intuition. You need to practice structuring it in the format interviewers expect (requirements → high-level → deep dive → trade-offs).
- **Rust:** Many top ML infra teams (Meta, Google, Cloudflare, Databricks) value systems language experience. Rust on your profile signals depth.

The project forces all three simultaneously: every feature is a Rust exercise, a DS&A pattern, and a system design building block.

---

## 🗺️ Phases Overview

```
Phase 1                Phase 2                Phase 3               Phase 4
Core Retrieval    →    Modern Reco       →    Online/Streaming  →   Full Platform
Engine                 Architectures          Systems               
                                                                    
heaps, hashmaps        ANN, two-tower         bandits, delayed      end-to-end
similarity, ranking    embeddings, serving    feedback, drift       ML system
                                                                    
DS&A drill             ML systems depth       your RTB edge         system design prep
(2-3 months)           (2-3 months)           (2-3 months)          (3+ months)
```

**Philosophy:**
- Never learn a data structure "for the interview". Always: *"what real system problem does this solve?"*
- Code it yourself. Use AI as a reviewer/teacher, not a code generator.
- After each feature: solve 2-3 related LeetCode problems. They'll feel easy because you already built the structure.

---

## 🎯 Interview Pattern Map

Reference this as you build. Every feature trains a specific interview skill.

### Coding Rounds (DS&A)

| You build... | Data structure | Interview pattern | Practice problems |
|---|---|---|---|
| Cosine similarity | `Vec<f32>`, dot product | Array math | Dot Product of Sparse Vectors (LC 1570) |
| Top-k retrieval | `BinaryHeap` | **Heap / Priority Queue** | Kth Largest (LC 215), Top K Frequent (LC 347) |
| Embedding index | `HashMap<u64, Vec<f32>>` | **Hash map** | Two Sum (LC 1), LRU Cache (LC 146) |
| Feature store | `enum` + nested maps | **Design + Hash map** | Time-Based KV Store (LC 981) |
| Ranking pipeline | Custom sort, comparators | **Sorting / Merge** | Merge K Sorted Lists (LC 23) |
| Sliding time window | `VecDeque` | **Sliding window** | Sliding Window Max (LC 239) |
| Event queue | Channels, `VecDeque` | **Queue / BFS** | Task Scheduler (LC 621) |
| HNSW graph | Adjacency list, layers | **Graph traversal** | Network Delay Time (LC 743) |
| HNSW search | Greedy beam search | **Greedy / Search** | Word Ladder (LC 127) |
| Online updates | `Arc<RwLock<T>>` | **Concurrency** | Producer-consumer patterns |
| Candidate generation | Set intersection | **Set operations** | Intersection of Two Arrays (LC 349) |

### System Design Rounds

| You build... | Maps to interview question |
|---|---|
| Retrieval → Reranking pipeline | "Design a recommendation system" |
| Feature store | "Design a real-time feature serving system" |
| ANN index | "Design a similarity search service" |
| Streaming updates | "Design a real-time analytics pipeline" |
| Bandit-based ranking | "Design an ad serving system with exploration" |
| Full platform | "Design an end-to-end ML platform" (the Staff-level question) |

---

# Phase 1 — Core Retrieval Engine

**Duration:** 2–3 months · **Focus:** DS&A foundations in Rust

---

### Stage 1.1 — Vector Similarity Engine (2–3 weeks)

**Build:**
- [ ] User and item embeddings in memory (`Vec<f32>`)
- [ ] Cosine similarity between two vectors
- [ ] Top-k search using a min-heap (not full sort)
- [ ] Index: store/retrieve embeddings by ID
- [ ] CLI interface (load, query, display)

**Rust to learn:**
- Ownership, borrowing, lifetimes — face them now
- `Vec`, `HashMap`, `BinaryHeap`
- Modules: `mod`, `pub`, file structure
- `#[derive(Debug, Clone)]`

**Design questions:**
- `BinaryHeap` + `Reverse` vs `.sort().truncate(k)` — complexity trade-off?
- Pre-normalized vectors: what does it save, what does it cost?
- Code structure: `embedding` / `index` / `cli` modules?

**References:**
- 📖 [Rust Book — Ch. 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) — Read and re-read
- 📖 [Rust Book — Ch. 8: Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- 📚 [std::collections::BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
- 📚 [std::cmp::Reverse](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)
- 🛠️ [clap crate](https://docs.rs/clap/latest/clap/)

**Milestone ✅:**
```
$ rustretrieve query --user 42 --k 10
Top 10 similar items for user 42:
  1. item_7823  (score: 0.9834)
  2. item_1291  (score: 0.9712)
```

---

### Stage 1.2 — In-Memory Feature Store (1–2 weeks)

**Build:**
- [ ] Typed features per entity (user/item): numeric, categorical, timestamp
- [ ] Get/set/update API
- [ ] Batch ingestion + unit updates
- [ ] Time window: keep last N events (`VecDeque`)

**Rust to learn:**
- `enum` as tagged unions (this is where Rust's type system clicks)
- Pattern matching with `match`
- Error handling: `Result<T, E>`, `?` operator
- Implement `Display` trait

**Design questions:**
- `FeatureValue` as `enum` vs `Box<dyn Feature>` — trade-offs?
- Missing features: `Option<T>` everywhere, or default values?

**References:**
- 📖 [Rust Book — Ch. 6: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- 📖 [Rust Book — Ch. 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- 📖 [Rust Book — Ch. 10: Generics, Traits, Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- 📚 [std::collections::VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- 📝 [Real-time ML — Chip Huyen](https://huyenchip.com/2023/09/17/real-time-ml.html)

---

### Stage 1.3 — Ranking Pipeline (2–4 weeks)

**Build:**
- [ ] Two-stage: retrieval (N candidates) → reranking (top K)
- [ ] Hybrid score: `score(u,a) = α⟨eᵤ, eₐ⟩ + β·CTR(a) + γ·freshness(a)`
- [ ] Configurable weights via TOML config
- [ ] Latency benchmarking

**Rust to learn:**
- Traits as abstractions: `trait Scorer`
- Generics: `fn rank<S: Scorer>(...)`
- Explicit lifetimes
- Closures & iterators: `.iter().map().filter().collect()`
- `serde` + `toml`

**Design questions:**
- Why two-stage? (do the latency math: N_catalog × scoring_cost)
- N_retrieval / N_final ratio — quality vs speed?
- Trait vs closure for `Scorer` — when which?

**References:**
- 📖 [Rust Book — Ch. 10: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- 📖 [Rust Book — Ch. 13: Iterators & Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- 📚 [serde.rs](https://serde.rs/)
- 📝 [Google ML Recommendation Course](https://developers.google.com/machine-learning/recommendation/overview) — Free, excellent
- 📝 [System Design for Discovery — Eugene Yan](https://eugeneyan.com/writing/system-design-for-discovery/)

**Milestone ✅:**
```
$ rustretrieve rank --user 42 --k 10 --config pipeline.toml
  1. item_7823  (hybrid: 0.872) [sim=0.98, ctr=0.045, fresh=0.91]
Latency: retrieval=2.3ms, reranking=0.4ms, total=2.7ms
```

---

# Phase 2 — Modern Reco Architectures

**Duration:** 2–3 months · **Focus:** ANN, embeddings, vector search — the deepest learning phase

---

### Stage 2.1 — ANN: Approximate Nearest Neighbors (3–6 weeks)

> **The most formative stage.** Non-trivial algorithm, complex data structures,
> concrete precision/speed trade-offs you can measure.

**Build:**
- [ ] Implement HNSW or IVF from scratch
- [ ] Index construction + approximate top-k search
- [ ] Benchmarks vs brute-force: latency, recall@k, memory
- [ ] Tunable parameters (M, ef_construction, ef_search)
- [ ] Save/load index to disk

**Rust to learn:**
- Smart pointers: `Box`, `Rc`, `Arc`
- Unsafe Rust (optional): understand why it exists
- Memory layout, cache locality
- `criterion` for statistical benchmarking
- `rand` for probabilistic graph construction

**Design questions:**
- HNSW vs IVF — graph vs partitioning, fundamental trade-offs?
- recall@k — why is it THE metric?
- Cache locality — why does memory layout matter for similarity search?
- M parameter — build time vs query time trade-off?

**References:**
- 📄 [HNSW — Malkov & Yashunin, 2018](https://arxiv.org/abs/1603.09320) — The original paper
- 📄 [Product Quantization — Jégou et al., 2011](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf)
- 📝 [HNSW explained — Pinecone](https://www.pinecone.io/learn/series/faiss/hnsw/) — Best visual explainer
- 📝 [ann-benchmarks.com](http://ann-benchmarks.com/)
- 📖 [Rust Book — Ch. 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- 📚 [criterion.rs](https://docs.rs/criterion/latest/criterion/)
- 🔍 [instant-distance (Rust)](https://github.com/InstantDomain/instant-distance) — Read AFTER coding yours

**Milestone ✅:**
```
$ rustretrieve bench --k 10
  Brute-force:  avg=45.2ms   recall@10=1.000
  HNSW (ef=50): avg=0.31ms   recall@10=0.953
  Speedup: ~145x
```

---

### Stage 2.2 — Two-Tower Architecture (2–4 weeks)

> You don't train a neural net in Rust. You understand the architecture
> and wire it into your engine.

**Build:**
- [ ] Separate user/item encoder interfaces (traits)
- [ ] Load pre-computed embeddings (from Python training)
- [ ] Full pipeline: encode → ANN search → rerank → serve
- [ ] Batched query support
- [ ] A/B comparison: random vs retrieval vs retrieval+reranking

**Key papers to read:**
- 📄 [YouTube DNN Recommendations — Covington et al., 2016](https://research.google/pubs/pub45530/) — Started it all
- 📄 [DLRM — Naumov et al., 2019](https://arxiv.org/abs/1906.00091) — Facebook's approach
- 📄 [Wide & Deep — Cheng et al., 2016](https://arxiv.org/abs/1606.07792) — Google's hybrid

**Explainers:**
- 📝 [Two-Tower Models — Google](https://developers.google.com/machine-learning/recommendation/dnn/retrieval)
- 📝 [Embedding-based Retrieval at Facebook](https://arxiv.org/abs/2006.11632)
- 📝 [Candidate Generation — Eugene Yan](https://eugeneyan.com/writing/real-time-recommendations/)

---

# Phase 3 — Online / Streaming Systems

**Duration:** 2–3 months · **Focus:** Your competitive advantage

> Your DSP background is a superpower here. You already have intuitions about
> latency budgets, real-time constraints, and event-driven systems.
> Now formalize them and make them interviewable.

---

### Stage 3.1 — Streaming Updates (2–3 weeks)

**Build:**
- [ ] Event queue (producer/consumer)
- [ ] Ingest user events (clicks, views, purchases)
- [ ] Incremental embedding updates (exponential moving average)
- [ ] Sliding windows for features
- [ ] Server mode: listen + respond

**Rust to learn:**
- `std::thread`, channels (`mpsc`), `Arc<Mutex<T>>`
- Async: `tokio`, `async/await` (optional but valuable)
- `Mutex` vs `RwLock` vs channels

**References:**
- 📖 [Rust Book — Ch. 16: Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- 📚 [tokio tutorial](https://tokio.rs/tokio/tutorial)
- 📚 [crossbeam](https://docs.rs/crossbeam/latest/crossbeam/)

---

### Stage 3.2 — Contextual Bandit / Online Ranker (3–6 weeks)

> This is where your RTB experience directly applies.

**Build:**
- [ ] Epsilon-greedy exploration
- [ ] Thompson Sampling
- [ ] UCB (Upper Confidence Bound)
- [ ] Delayed feedback handling (click arrives after impression)
- [ ] Online metrics: CTR, regret, exploration rate
- [ ] Drift detection

**Design questions:**
- Why not always pick the highest-scoring item? (exploitation trap)
- How does delayed feedback break naive online learning? (you've seen this at Adikteev)
- Thompson Sampling vs UCB — when does each win?
- How to detect drift? Sliding window comparison?

**References:**
- 📖 [Bandit Algorithms — Lattimore & Szepesvári](https://tor-lattimore.com/downloads/book/book.pdf) — Free textbook
- 📄 [Delayed Feedback in Ads — Chapelle, 2014](https://arxiv.org/abs/1407.7073) — Directly your domain
- 📝 [Contextual Bandits — Microsoft Research](https://www.microsoft.com/en-us/research/project/explore-exploit/)

---

# Phase 4 — Staff-Level System Design

**Duration:** 3+ months · **Focus:** Tie everything together

> This phase is specifically about practicing the **Staff system design interview format**:
> requirements → API → high-level → deep dive on 1-2 components → trade-offs → scaling.
> You're building the system that IS the interview answer.

---

### Stage 4.1 — Mini Real-Time ML Platform

**Build:**
- [ ] Feature store (Phase 1)
- [ ] Embedding service (Phase 2)
- [ ] ANN index with online updates (Phase 2+3)
- [ ] Training pipeline: batch retrain from collected events
- [ ] Online inference: query → features → retrieve → rank → serve
- [ ] Monitoring: latency p50/p95/p99, drift, data quality

**Architecture:**
```
                     ┌─────────────┐
   User event ──────→│ Event Queue │
                     └──────┬──────┘
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
     ┌────────────┐  ┌───────────┐  ┌───────────┐
     │  Feature   │  │ Embedding │  │  Training  │
     │   Store    │  │  Updates  │  │  Pipeline  │
     └─────┬──────┘  └─────┬─────┘  └───────────┘
           │               │
           ▼               ▼
     ┌──────────────────────────┐
     │    Serving Pipeline      │
     │  retrieve → rank → serve │
     └────────────┬─────────────┘
                  │
                  ▼
     ┌──────────────────────┐
     │    Monitoring        │
     │ latency · drift · QA │
     └──────────────────────┘
```

**Staff-level design skills:**
- Service decomposition and API boundaries
- Data flow and consistency guarantees
- Latency budgets across the pipeline
- Graceful degradation (ANN stale? feature store down?)
- Capacity planning and scaling reasoning

**References:**
- 📝 [System Design Primer](https://github.com/donnemartin/system-design-primer)
- 📝 [Netflix Tech Blog](https://netflixtechblog.com/)
- 📝 [Uber Engineering Blog](https://www.uber.com/en-FR/blog/engineering/)
- 📝 [Designing ML Systems — Chip Huyen](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/)

---

## 📚 Resources

### Rust
| Resource | Why |
|---|---|
| [The Rust Programming Language](https://doc.rust-lang.org/book/) | THE reference. Read chapters as needed. |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | Practical companion |
| [Rustlings](https://github.com/rust-lang/rustlings) | Drill when stuck |
| [Programming Rust, 2nd Ed.](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) | Deep reference |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | Unsafe, memory layouts |
| [Rust Performance Book](https://nnethercote.github.io/perf-book/) | Optimization |
| [Jon Gjengset — YouTube](https://www.youtube.com/@jonhoo) | Advanced live-coding |

### Interview Prep (targeted)
| Resource | Why |
|---|---|
| [NeetCode 150](https://neetcode.io/) | Curated problems — solve after building each feature |
| [AlgoMonster](https://algo.monster/) | Pattern-based, not grind-based |
| [System Design Primer](https://github.com/donnemartin/system-design-primer) | Classic system design |
| [ML Design Interviews](https://www.mldesigninterview.com/) | ML-specific system design |

### Papers
| Paper | Relevant to |
|---|---|
| [YouTube DNN Recommendations (2016)](https://research.google/pubs/pub45530/) | Two-tower, candidate generation |
| [Wide & Deep (2016)](https://arxiv.org/abs/1606.07792) | Hybrid ranking |
| [DLRM (2019)](https://arxiv.org/abs/1906.00091) | Embedding tables |
| [HNSW (2018)](https://arxiv.org/abs/1603.09320) | ANN index |
| [Delayed Feedback — Chapelle (2014)](https://arxiv.org/abs/1407.7073) | Online learning, your domain |

---

## 🧰 Cargo Dependencies (add incrementally)

```toml
# Phase 1
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"

# Phase 2
rand = "0.8"
criterion = "0.5"

# Phase 3
tokio = { version = "1", features = ["full"] }
crossbeam = "0.8"
```

---

## 📐 Architecture (let it emerge)

```
rustretrieve/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── embedding.rs
│   ├── index/
│   │   ├── mod.rs
│   │   ├── brute_force.rs
│   │   └── hnsw.rs
│   ├── feature_store.rs
│   ├── ranking/
│   │   ├── mod.rs
│   │   ├── hybrid.rs
│   │   └── config.rs
│   ├── bandit/
│   │   ├── mod.rs
│   │   ├── epsilon_greedy.rs
│   │   └── thompson.rs
│   └── streaming/
│       ├── mod.rs
│       └── event_queue.rs
├── benches/
│   └── ann_bench.rs
└── data/
```

> ⚠️ Don't scaffold this upfront. Start with `main.rs`. Extract modules when files get big.

---

## 🧭 Practical Tips

1. **Start SMALL.** 100 hardcoded items is fine for week 1.
2. **The Rust compiler is your best teacher.** Listen to it.
3. **Write tests early.** `#[cfg(test)] mod tests { ... }`
4. **`cargo clippy`** often — teaches idiomatic Rust.
5. **First correct, then fast.** Resist premature optimization.
6. **Stuck > 30 min on borrow checker?** Draw ownership on paper.
7. **After each feature:** find 2-3 related LeetCode problems. Notice how much easier they are.
8. **For system design prep:** practice talking through your architecture out loud, as if in an interview. "I would start with requirements... the key trade-off here is..."

---

*Last updated: May 2026*
