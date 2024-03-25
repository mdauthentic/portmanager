# Port Manager
A cross-platofrm desktop app to manage your ports made with [Dioxus](https://dioxuslabs.com/) and `Rust`.

<img src="screenshot.png" alt="portmanager">

> This app has been tested only on `macOS`. Test on other platform will be done later.

## Setup

```sh
git clone repo
cd repo
# run to start the tailwind css compile
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
dx serve --hot-reload --platform desktop
# alternatively, you could run
cargo run
```