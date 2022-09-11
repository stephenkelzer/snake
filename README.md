# snake
Welcome to my first app written in Rust!

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
- Setup [Yew](https://yew.rs/)
   - [Follow the instructions on this page](https://yew.rs/docs/getting-started/introduction)

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
