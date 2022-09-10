# snake
Welcome to my first app written in Rust!

![image](https://user-images.githubusercontent.com/19741798/189489002-1454a5fe-91c5-497a-bf97-5c9979c4aa60.png)

[GitHub Page Demo](https://stephenkelzer.github.io/snake)

---

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

---

## To run locally:
```bash
cargo cmd run
```

---

## Deployment Process:
1) Create a pull request pointing at the `main` branch
1) Merge the pull request
1) The 'Release Drafter' GitHub Action will create a draft release
1) Edit the draft release and set it to a published state
1) The 'Deploy to GitHub Pages' GitHub Action will deploy the app to GitHub Pages
1) View the deployed app at [https://stephenkelzer.github.io/snake](https://stephenkelzer.github.io/snake)
