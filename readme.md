# Mnist inference web demo

The [burn mnist example](./burn_mnist) is copied from the [burn examples](https://github.com/tracel-ai/burn/tree/main/examples/mnist-inference-web) and we use ndarray for the backend by default.

# Run it

In order to run it, run

```sh
cargo xtask serve <port>
```

If no port is given, it defaults to 3000.

# Prerequisites

- [Rust installed](https://rust-lang.org/tools/install/)
- [wasm-pack installed]: `cargo install wasm-pack` (after you installed rust)
