# snake
Welcome to my first app written in Rust!

## Prerequisites:
- Install [`Rust`](https://www.rust-lang.org/tools/install)
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
- Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) (WebAssembly packager)
   ```
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```
- Install [`cargo-cmd`](https://crates.io/crates/cargo-cmd) (cargo command runner)
   ```
   cargo install cargo-cmd
   ```
- Install [`sfz`](https://crates.io/crates/sfz) (static file server)
   ```
   cargo install sfz
   ```

## To run locally:
```bash
cargo cmd run
```