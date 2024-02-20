use crate::sketch::embedded::{delay, digital};
use anyhow::Context;
use std::fs;
use std::io::Write;
use wasmtime::{
    component::{bindgen, Component, Linker, ResourceTable},
    Config, Engine, Result, Store,
};

// Generate bindings of the guest and host components.
bindgen!({
    world: "blink",
    path: "../wit",
    with: {
        "sketch:embedded/delay/delay": Delay,
        "sketch:embedded/digital/output-pin": OutputPin,
    },
});

pub struct Delay;
pub struct OutputPin;

struct HostComponent {
    table: ResourceTable,
}

impl digital::Host for HostComponent {}
impl delay::Host for HostComponent {}

impl digital::HostInputPin for HostComponent {
    fn is_low(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        todo!()
    }
    fn is_high(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        todo!()
    }
    fn wait_for_high(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!()
    }
    fn wait_for_low(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!()
    }
    fn wait_for_rising_edge(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!()
    }
    fn wait_for_falling_edge(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!()
    }
    fn wait_for_any_edge(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!()
    }

    fn drop(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl digital::HostOutputPin for HostComponent {
    fn set_low(
        &mut self,
        _self_: wasmtime::component::Resource<digital::OutputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        print!("     \r");
        std::io::stdout().flush().unwrap();
        Ok(Ok(()))
    }

    fn set_high(
        &mut self,
        _self_: wasmtime::component::Resource<digital::OutputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        print!("    💡\r");
        std::io::stdout().flush().unwrap();
        Ok(Ok(()))
    }

    fn set_state(
        &mut self,
        _self_: wasmtime::component::Resource<digital::OutputPin>,
        _state: digital::PinState,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!()
    }

    fn drop(
        &mut self,
        _self_: wasmtime::component::Resource<digital::OutputPin>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl digital::HostStatefulOutputPin for HostComponent {
    fn is_set_high(
        &mut self,
        _self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        todo!()
    }

    fn is_set_low(
        &mut self,
        _self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        todo!()
    }

    fn toggle(
        &mut self,
        _self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!()
    }

    fn drop(
        &mut self,
        _self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl delay::HostDelay for HostComponent {
    fn delay_ns(
        &mut self,
        _self_: wasmtime::component::Resource<delay::Delay>,
        ns: u32,
    ) -> wasmtime::Result<()> {
        std::thread::sleep(std::time::Duration::from_nanos(ns.into()));
        Ok(())
    }

    fn drop(
        &mut self,
        _self_: wasmtime::component::Resource<delay::Delay>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

struct MyState {
    host: HostComponent,
}

fn main() -> Result<()> {
    // Create the engine and the linker.
    let engine = Engine::new(Config::new().wasm_component_model(true))?;
    let mut linker = Linker::new(&engine);
    Blink::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;

    // Read the guest component file.
    let path = "../guest/target/wasm32-wasi/release/hello_embedded.wasm";
    let component_bytes = fs::read(path).context("failed to read input file")?;
    let component = Component::from_binary(&engine, &component_bytes)?;

    // Create the state that will be stored in the store, and link it in.
    let mut my_state = MyState {
        host: HostComponent {
            table: ResourceTable::new(),
        },
    };

    // Create the resources we'll pass into the `run` function.
    let led = my_state.host.table.push(OutputPin)?;
    let delay = my_state.host.table.push(Delay)?;

    // Create the store and instantiate the component.
    let mut store = Store::new(&engine, my_state);
    let (blink, _instance) = Blink::instantiate(&mut store, &component, &linker)?;

    // Run!
    blink
        .sketch_embedded_run()
        .call_run(&mut store, led, delay)?;

    Ok(())
}
