# pdf-to-whatever

export pages of a pdf to images, all using WASM in browser, no server.

## TODO

- use suspense or somehting to show loading animation (https://yew.rs/docs/concepts/suspense)
- use web workers to processs in background. maybe multithreading?

## developing locally

1. install _trunk-rs_.

2. have target `wasm32-unknown-unknown` installed:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```
