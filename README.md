# altius

[![CI](https://github.com/maekawatoshiki/altius/workflows/CI/badge.svg)](https://github.com/maekawatoshiki/altius/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/maekawatoshiki/altius/branch/main/graph/badge.svg)](https://codecov.io/gh/maekawatoshiki/altius)

Small ONNX inference runtime written in Rust

Feel free to create [issues](https://github.com/maekawatoshiki/altius/issues) and [discussions](https://github.com/maekawatoshiki/altius/discussions)

# Requirements

- Python 3.x (Used in some tests; You can disable them by just ignoring tests in `./altius-py`)

# Run

```sh
# Download large models.
(cd models && ./download.sh)

# Run examples.
# {mnist, mobilenet, vit} are available.
./run.sh mnist
./run.sh mobilenet
./run.sh vit # You can specify the number of threads in ./session/examples/vit.rs

# On macOS, you can use 'accelerate' library.
# Replace `blis` default feature in `session/Cargo.toml` with `accelerate`.
cargo run --release --example mobielnet
```

# Run from WebAssembly

Currently, mobilenet v3 runs on web browsers.

```sh
cd wasm
cargo install wasm-pack
wasm-pack build --target web
yarn
yarn serve
```

# Run from Python

```sh
cd altius-py
python -m venv .env
source .env/bin/activate
pip install -r requirements.txt
RUSTFLAGS="-C target-cpu=native" maturin develop -r --features blis
python mobilenet.py
```
