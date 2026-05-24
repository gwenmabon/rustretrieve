# RustRetrieve

A real-time retrieval and ranking engine for recommendation systems, written from scratch in Rust.

## Overview

RustRetrieve is a self-contained recommendation engine that covers the full retrieval-to-ranking pipeline:

- **Vector similarity search** with cosine similarity and top-k retrieval
- **In-memory feature store** supporting numeric, categorical, and temporal features
- **Multi-stage ranking pipeline** with configurable hybrid scoring
- **Approximate Nearest Neighbors (ANN)** index for sub-millisecond retrieval at scale
- **Streaming updates** with event ingestion and incremental model updates

The system is designed around the same architectural patterns used in production recommendation systems at scale: candidate generation via embedding similarity, followed by a feature-enriched reranking stage.

## Architecture

```
rustretrieve/
├── src/
│   ├── main.rs              # Entry point and CLI
│   ├── lib.rs               # Public API
│   ├── embedding.rs         # Dense vectors, cosine similarity, normalization
│   ├── index/
│   │   ├── brute_force.rs   # Exact nearest neighbor search
│   │   └── hnsw.rs          # HNSW approximate nearest neighbor index
│   ├── feature_store.rs     # In-memory typed feature storage
│   ├── ranking/
│   │   ├── hybrid.rs        # Hybrid scoring (similarity + CTR + freshness)
│   │   └── config.rs        # Pipeline configuration
│   └── streaming/
│       └── event_queue.rs   # Event ingestion and online updates
└── benches/
    └── ann_bench.rs         # Latency and recall benchmarks
```

## Key Design Decisions

- **No external vector database.** The ANN index (HNSW) is implemented from scratch to understand the trade-offs between build time, query latency, and recall.
- **Two-stage pipeline.** Retrieval narrows candidates via ANN search, then reranking applies a hybrid score combining embedding similarity, CTR, and freshness.
- **Trait-based abstractions.** Index backends and scoring functions are defined as traits, making it straightforward to swap implementations or add new strategies.
- **Zero-copy where possible.** Borrowing and slices are preferred over cloning to minimize allocations on the hot path.

## Usage

```bash
# Build
cargo build --release

# Load embeddings and query
rustretrieve load --file items.csv
rustretrieve query --user 42 --k 10

# Run the full ranking pipeline
rustretrieve rank --user 42 --k 10 --config pipeline.toml

# Build an ANN index and benchmark
rustretrieve index build --algorithm hnsw --file items.csv --m 16 --ef-construction 200
rustretrieve bench --index index.hnsw --queries queries.csv --k 10
```

## Dependencies

Minimal by design. External crates are added only when the standard library does not provide a reasonable alternative.

| Crate | Purpose |
|---|---|
| `clap` | CLI argument parsing |
| `serde` + `toml` | Configuration and serialization |
| `rand` | Probabilistic graph construction (HNSW) |
| `criterion` | Statistical benchmarking |

## License

MIT
