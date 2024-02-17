# Hello Embedded! 👋

This repository contains a sketch of an embedded API described in Wit,
and a simple example application that builds with it that blinks an LED.

The API is in the wit directory, and is based off of the
[embedded-hal] API.

Building the example application currently requires a cargo component with
[this patch] applied. Build with `cargo component build`.

```sh
$ cargo component build
```

That produces a component in `target/wasm32-wasi/debug/hello_embedded.wasm`.
We can examine it with `wasm-tools`:

```sh
$ wasm-tools component wit target/wasm32-wasi/debug/hello_embedded.wasm
package root:component;

world root {
  import sketch:embedded/delay@0.0.0;
  import sketch:embedded/digital@0.0.0;

  export sketch:embedded/run@0.0.0;
}
```

Here we can see it's exporting the `run` interface, which has the `run`
entrypoint function, and importing the `digital` and `delay` and interfaces,
which it uses to set the led and control its speed, respectively.

There aren't currently any host implementations, so the application can't
currently be run anywhere yet. This is an early sketch.

[embedded-hal]: https://docs.rs/embedded-hal/latest/embedded_hal/
[this patch]: https://github.com/bytecodealliance/cargo-component/pull/231
