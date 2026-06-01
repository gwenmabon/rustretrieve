# 🦀 RustRetrieve - Roadmap

**Learn Rust by building a real ML retrieval system from scratch.**

This project is a hands-on learning path that combines:

1. **Rust fluency** - learn the language through a real system, not toy examples
2. **Data structures & algorithms** - implement them in context, not in isolation
3. **ML systems engineering** - understand how retrieval, ranking, and serving work end-to-end

---

## 🗺️ Phases Overview

```
Phase 1                Phase 2                Phase 3               Phase 4
Core Retrieval    →    Modern Reco       →    Online/Streaming  →   Full Platform
Engine                 Architectures          Systems               
                                                                    
heaps, hashmaps        ANN, two-tower         bandits, delayed      end-to-end
similarity, ranking    embeddings, serving    feedback, drift       ML system
                                                                    
Rust foundations       ML systems depth       real-time systems     platform design
(2-3 months)           (2-3 months)           (2-3 months)          (3+ months)
```

**Philosophy:**
- Never learn a data structure in the abstract. Always: *"what real system problem does this solve?"*
- Code it yourself. Use AI as a reviewer/teacher, not a code generator.

---

## 🧩 What You'll Build & Learn

| You build... | Data structure | Pattern |
|---|---|---|
| Cosine similarity | `Vec<f32>`, dot product | Array math |
| Top-k retrieval | `BinaryHeap` | Heap / Priority Queue |
| Embedding index | `HashMap<u64, Vec<f32>>` | Hash map |
| Feature store | `enum` + nested maps | Design + Hash map |
| Ranking pipeline | Custom sort, comparators | Sorting / Merge |
| Sliding time window | `VecDeque` | Sliding window |
| Event queue | Channels, `VecDeque` | Queue / BFS |
| HNSW graph | Adjacency list, layers | Graph traversal |
| HNSW search | Greedy beam search | Greedy / Search |
| Online updates | `Arc<RwLock<T>>` | Concurrency |
| Candidate generation | Set intersection | Set operations |

---

# Phase 1 - Core Retrieval Engine

**Duration:** 2–3 months · **Focus:** DS&A foundations in Rust

---

### Stage 1.1 - Vector Similarity Engine (2–3 weeks)

**Build:**
- [ ] User and item embeddings in memory (`Vec<f32>`)
- [ ] Cosine similarity between two vectors
- [ ] Top-k search using a min-heap (not full sort)
- [ ] Index: store/retrieve embeddings by ID
- [ ] CLI interface (load, query, display)

**Rust to learn:**
- Ownership, borrowing, lifetimes - face them now
- `Vec`, `HashMap`, `BinaryHeap`
- Modules: `mod`, `pub`, file structure
- `#[derive(Debug, Clone)]`

**Design questions:**
- `BinaryHeap` + `Reverse` vs `.sort().truncate(k)` - complexity trade-off?
- Pre-normalized vectors: what does it save, what does it cost?
- Code structure: `embedding` / `index` / `cli` modules?

**References:**
- 📖 [Rust Book - Ch. 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) - Read and re-read
- 📖 [Rust Book - Ch. 8: Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
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

### Stage 1.2 - In-Memory Feature Store (1–2 weeks)

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
- `FeatureValue` as `enum` vs `Box<dyn Feature>` - trade-offs?
- Missing features: `Option<T>` everywhere, or default values?

**References:**
- 📖 [Rust Book - Ch. 6: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- 📖 [Rust Book - Ch. 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- 📖 [Rust Book - Ch. 10: Generics, Traits, Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- 📚 [std::collections::VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- 📝 [Real-time ML - Chip Huyen](https://huyenchip.com/2023/09/17/real-time-ml.html)

---

### Stage 1.3 - Ranking Pipeline (2–4 weeks)

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
- N_retrieval / N_final ratio - quality vs speed?
- Trait vs closure for `Scorer` - when which?

**References:**
- 📖 [Rust Book - Ch. 10: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- 📖 [Rust Book - Ch. 13: Iterators & Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- 📚 [serde.rs](https://serde.rs/)
- 📝 [Google ML Recommendation Course](https://developers.google.com/machine-learning/recommendation/overview) - Free, excellent
- 📝 [System Design for Discovery - Eugene Yan](https://eugeneyan.com/writing/system-design-for-discovery/)

**Milestone ✅:**
```
$ rustretrieve rank --user 42 --k 10 --config pipeline.toml
  1. item_7823  (hybrid: 0.872) [sim=0.98, ctr=0.045, fresh=0.91]
Latency: retrieval=2.3ms, reranking=0.4ms, total=2.7ms
```

---

# Phase 2 - Modern Reco Architectures

**Duration:** 2–3 months · **Focus:** ANN, embeddings, vector search - the deepest learning phase

---

### Stage 2.1 - ANN: Approximate Nearest Neighbors (3–6 weeks)

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
- HNSW vs IVF - graph vs partitioning, fundamental trade-offs?
- recall@k - why is it THE metric?
- Cache locality - why does memory layout matter for similarity search?
- M parameter - build time vs query time trade-off?

**References:**
- 📄 [HNSW - Malkov & Yashunin, 2018](https://arxiv.org/abs/1603.09320) - The original paper
- 📄 [Product Quantization - Jégou et al., 2011](https://lear.inrialpes.fr/pubs/2011/JDS11/jegou_searching_with_quantization.pdf)
- 📝 [HNSW explained - Pinecone](https://www.pinecone.io/learn/series/faiss/hnsw/) - Best visual explainer
- 📝 [ann-benchmarks.com](http://ann-benchmarks.com/)
- 📖 [Rust Book - Ch. 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- 📚 [criterion.rs](https://docs.rs/criterion/latest/criterion/)
- 🔍 [instant-distance (Rust)](https://github.com/InstantDomain/instant-distance) - Read AFTER coding yours

**Milestone ✅:**
```
$ rustretrieve bench --k 10
  Brute-force:  avg=45.2ms   recall@10=1.000
  HNSW (ef=50): avg=0.31ms   recall@10=0.953
  Speedup: ~145x
```

---

### Stage 2.2 - Two-Tower Architecture (2–4 weeks)

> You don't train a neural net in Rust. You understand the architecture
> and wire it into your engine.

**Build:**
- [ ] Separate user/item encoder interfaces (traits)
- [ ] Load pre-computed embeddings (from Python training)
- [ ] Full pipeline: encode → ANN search → rerank → serve
- [ ] Batched query support
- [ ] A/B comparison: random vs retrieval vs retrieval+reranking

**Key papers to read:**
- 📄 [YouTube DNN Recommendations - Covington et al., 2016](https://research.google/pubs/pub45530/) - Started it all
- 📄 [DLRM - Naumov et al., 2019](https://arxiv.org/abs/1906.00091) - Facebook's approach
- 📄 [Wide & Deep - Cheng et al., 2016](https://arxiv.org/abs/1606.07792) - Google's hybrid

**Explainers:**
- 📝 [Two-Tower Models - Google](https://developers.google.com/machine-learning/recommendation/dnn/retrieval)
- 📝 [Embedding-based Retrieval at Facebook](https://arxiv.org/abs/2006.11632)
- 📝 [Candidate Generation - Eugene Yan](https://eugeneyan.com/writing/real-time-recommendations/)

---

# Phase 3 - Online / Streaming Systems

**Duration:** 2–3 months · **Focus:** Real-time ML systems

---

### Stage 3.1 - Streaming Updates (2–3 weeks)

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
- 📖 [Rust Book - Ch. 16: Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- 📚 [tokio tutorial](https://tokio.rs/tokio/tutorial)
- 📚 [crossbeam](https://docs.rs/crossbeam/latest/crossbeam/)

---

### Stage 3.2 - Contextual Bandit / Online Ranker (3–6 weeks)

**Build:**
- [ ] Epsilon-greedy exploration
- [ ] Thompson Sampling
- [ ] UCB (Upper Confidence Bound)
- [ ] Delayed feedback handling (click arrives after impression)
- [ ] Online metrics: CTR, regret, exploration rate
- [ ] Drift detection

**Design questions:**
- Why not always pick the highest-scoring item? (exploitation trap)
- How does delayed feedback break naive online learning?
- Thompson Sampling vs UCB - when does each win?
- How to detect drift? Sliding window comparison?

**References:**
- 📖 [Bandit Algorithms - Lattimore & Szepesvári](https://tor-lattimore.com/downloads/book/book.pdf) - Free textbook
- 📄 [Delayed Feedback in Ads - Chapelle, 2014](https://arxiv.org/abs/1407.7073)
- 📝 [Contextual Bandits - Microsoft Research](https://www.microsoft.com/en-us/research/project/explore-exploit/)

---

# Phase 4 - Full Platform Integration

**Duration:** 3+ months · **Focus:** Tie everything together

> Combine all components into a cohesive ML platform.
> Practice thinking about systems holistically:
> requirements → API → architecture → deep dive → trade-offs → scaling.

---

### Stage 4.1 - Mini Real-Time ML Platform

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

**Key design skills:**
- Service decomposition and API boundaries
- Data flow and consistency guarantees
- Latency budgets across the pipeline
- Graceful degradation (ANN stale? feature store down?)
- Capacity planning and scaling reasoning

**References:**
- 📝 [System Design Primer](https://github.com/donnemartin/system-design-primer)
- 📝 [Netflix Tech Blog](https://netflixtechblog.com/)
- 📝 [Uber Engineering Blog](https://www.uber.com/en-FR/blog/engineering/)
- 📝 [Designing ML Systems - Chip Huyen](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/)

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
| [Jon Gjengset - YouTube](https://www.youtube.com/@jonhoo) | Advanced live-coding |

### DS&A Practice
| Resource | Why |
|---|---|
| [NeetCode 150](https://neetcode.io/) | Curated problems - solve after building each feature |
| [AlgoMonster](https://algo.monster/) | Pattern-based learning |

### ML Systems
| Resource | Why |
|---|---|
| [System Design Primer](https://github.com/donnemartin/system-design-primer) | Classic system design patterns |
| [Designing ML Systems - Chip Huyen](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/) | End-to-end ML systems |

### Papers
| Paper | Relevant to |
|---|---|
| [YouTube DNN Recommendations (2016)](https://research.google/pubs/pub45530/) | Two-tower, candidate generation |
| [Wide & Deep (2016)](https://arxiv.org/abs/1606.07792) | Hybrid ranking |
| [DLRM (2019)](https://arxiv.org/abs/1906.00091) | Embedding tables |
| [HNSW (2018)](https://arxiv.org/abs/1603.09320) | ANN index |
| [Delayed Feedback - Chapelle (2014)](https://arxiv.org/abs/1407.7073) | Online learning |

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
4. **`cargo clippy`** often - teaches idiomatic Rust.
5. **First correct, then fast.** Resist premature optimization.
6. **Stuck > 30 min on borrow checker?** Draw ownership on paper.

---

*Last updated: May 2026*