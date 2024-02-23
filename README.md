# Hello Embedded! 👋

This repository contains a sketch of an embedded API described in [Wit],
and a simple example Wasm application that builds with it that blinks
an LED, and a Wasmtime-based simulator that can run it.

[Wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

## The Wits

The API is described in Wit interfaces in the wit directory, and is based off
of the [embedded-hal] API.

## The guest

The example application is in the guest directory. Building it currently
requires [cargo component 0.8]. Build with `cargo component build`.

[cargo component 0.8]: https://github.com/bytecodealliance/cargo-component/releases/tag/v0.8.0

```sh
$ cd guest
$ cargo component build --release
[...]
$ cd ..
```

That produces a component in `guest/target/wasm32-wasi/release/hello_embedded.wasm`.
We can examine it with `wasm-tools`:

```sh
$ wasm-tools component wit guest/target/wasm32-wasi/release/hello_embedded.wasm
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

## The host simulator

Once the guest is built, it can be run in the host simulator:
```sh
$ cd host
$ cargo run
[...]
```

```
   💡
```
```
    
```
```
   💡
```
```
    
```
```
   💡
```
...

## The Linux host

⚠ The following is *entirely untested* at this time!!! ⚠

If you can run this on a Linux board with a GPIO pin 0 wired up to
an LED, and if luck smiles on us, this should run the guest which
should make that LED blink:

```sh
$ cd linux-host
$ cargo run
[...]
```

## Bonus points

This example also demonstrates [Typed Main]. The `run` function takes
two handle arguments, providing the component with exactly what it
needs.

[Typed Main]: https://sunfishcode.github.io/typed-main-wasi-presentation/chapter_1.html

[embedded-hal]: https://docs.rs/embedded-hal/latest/embedded_hal/
