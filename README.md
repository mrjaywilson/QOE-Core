# QoE Core

A Rust simulation engine for evaluating Quality of Experience (QoE) in adaptive bitrate (ABR) video streaming.

This crate provides:

- Modular ABR strategies (Fixed, Buffer-Based, Throughput-Based)
- A simulation engine for playback behavior
- QoE scoring based on average bitrate, stall count, switch frequency, and buffer usage
- JSON and CSV export for analysis
- FFI-ready functions for use with Python, Unity, CLI, etc.

---

## Features

- `SimulationConfig` to customize ABR behavior
- `run_simulation()` to simulate playback across strategies
- `evaluate_qoe()` to generate a final QoE score
- FFI-compatible exports for use with C/C#/Python
- JSON & CSV serialization of frame-level metrics

---

## Project Structure

```
src/
├── lib.rs              # Public interface / FFI entry points
├── abr/                # ABR strategy implementations
│   ├── mod.rs
│   ├── fixed.rs
│   ├── buffer_based.rs
│   └── throughput_based.rs
├── playback/
│   └── engine.rs       # The simulation engine
├── metrics/
│   ├── model.rs        # Per-frame metrics struct
│   ├── logger.rs       # CSV export
│   └── qoe.rs          # QoE scoring algorithm
├── models.rs           # SimulationConfig, ABRType
└── ffi.rs              # C-compatible bindings (optional)
```

---

## Usage

Add to your Rust project:

```toml
[dependencies]
qoe_core = { path = "../qoe-core" }
```

Then use:

```rust
use qoe_core::{
    models::{SimulationConfig, ABRType},
    playback::engine::run_simulation,
    metrics::qoe::evaluate_qoe,
};

let config = SimulationConfig {
    abr_type: ABRType::ThroughputBased { window_size: 3 },
    ..Default::default()
};

let metrics = run_simulation(&config);
let score = evaluate_qoe(&metrics);
```

---

## Testing

Run unit tests:

```bash
cargo test
```

Test coverage includes:
- Fixed, Buffer, Throughput ABR logic
- Simulation engine stability
- QoE scoring
- CSV/JSON output generation

---

## FFI Usage

Export the simulation for use in C/C#/Python:

```rust
#[no_mangle]
pub extern "C" fn simulate_and_get_json(config: SimConfig) -> *mut c_char
```

Then load it in:
- Unity: `DllImport("qoe_core")`
- Python: `ctypes.CDLL("qoe_core.dll")`

---

## QoE Scoring Formula

QoE is based on:
- Average bitrate (higher is better)
- Stall count (penalized heavily)
- Bitrate switch frequency
- Buffer underruns

This produces a score from 0–100.

---

## License

MIT

---

## Credits

Created by [@mrjaywilson](https://github.com/mrjaywilson)  
Designed to support real-world ABR prototyping, analysis, and visualization.
