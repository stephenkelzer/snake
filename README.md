# snake
Welcome to my first app written in Rust!

[![Build and Test](https://github.com/stephenkelzer/snake/actions/workflows/build_and_test.yml/badge.svg?branch=main)](https://github.com/stephenkelzer/snake/actions/workflows/build_and_test.yml) [![GitHub Pages](https://img.shields.io/website-up-down-green-red/https/stephenkelzer.github.io/snake.svg)](https://stephenkelzer.github.io/snake)

<a href="https://stephenkelzer.github.io/snake" target="_blank">
   <p align="center">
      <img src="https://user-images.githubusercontent.com/19741798/189489002-1454a5fe-91c5-497a-bf97-5c9979c4aa60.png">
   </p>
   <p align="center">
      Play the latest version by clicking here!
   </p>
</a>

---

## Prerequisites:
- Install [`Rust`](https://www.rust-lang.org/tools/install)
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
- Install WebAssembly Target
   ```
   rustup target add wasm32-unknown-unknown
   ```
- Install [Trunk](https://trunkrs.dev/)
   ```
   cargo install trunk
   ```

---

## To run locally:
```bash
trunk serve
```

---

## Deployment Process:
1) Create a pull request pointing at the `main` branch
1) Merge the pull request
1) The 'Release Drafter' GitHub Action will create a draft release
1) Edit the draft release and set it to a published state
1) The 'Deploy to GitHub Pages' GitHub Action will deploy the app to GitHub Pages
1) View the deployed app at [https://stephenkelzer.github.io/snake](https://stephenkelzer.github.io/snake)
