use crate::sketch::embedded::{delay, digital};
use anyhow::Context;
use gpio_cdev::{Chip, EventRequestFlags, LineRequestFlags};
use linux_embedded_hal::CdevPin;
use std::fs;
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
        "sketch:embedded/digital/input-pin": InputPin,
        "sketch:embedded/digital/output-pin": OutputPin,
    },
});

pub struct Delay;
pub struct InputPin(CdevPin);
pub struct OutputPin(CdevPin);

struct HostComponent {
    table: ResourceTable,
}

impl digital::Host for HostComponent {}
impl delay::Host for HostComponent {}

impl digital::HostInputPin for HostComponent {
    fn is_low(
        &mut self,
        self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        match embedded_hal::digital::InputPin::is_low(&mut self_.0) {
            Ok(value) => Ok(Ok(value)),
            Err(_) => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn is_high(
        &mut self,
        self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        match embedded_hal::digital::InputPin::is_high(&mut self_.0) {
            Ok(value) => Ok(Ok(value)),
            Err(_) => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn wait_for_high(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!("InputPin::wait_for_high")
    }

    fn wait_for_low(
        &mut self,
        _self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!("InputPin::wait_for_low")
    }

    fn wait_for_rising_edge(
        &mut self,
        self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let mut events = self_.0.line().events(
            LineRequestFlags::INPUT,
            EventRequestFlags::RISING_EDGE,
            "hello-embedded",
        )?;
        match events.next() {
            Some(Ok(_)) => Ok(Ok(())),
            _ => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn wait_for_falling_edge(
        &mut self,
        self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let mut events = self_.0.line().events(
            LineRequestFlags::INPUT,
            EventRequestFlags::FALLING_EDGE,
            "hello-embedded",
        )?;
        match events.next() {
            Some(Ok(_)) => Ok(Ok(())),
            _ => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn wait_for_any_edge(
        &mut self,
        self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let mut events = self_.0.line().events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            "hello-embedded",
        )?;
        match events.next() {
            Some(Ok(_)) => Ok(Ok(())),
            _ => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<digital::InputPin>,
    ) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}

impl digital::HostOutputPin for HostComponent {
    fn set_low(
        &mut self,
        self_: wasmtime::component::Resource<digital::OutputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        match embedded_hal::digital::OutputPin::set_low(&mut self_.0) {
            Ok(()) => Ok(Ok(())),
            Err(_) => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn set_high(
        &mut self,
        self_: wasmtime::component::Resource<digital::OutputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        match embedded_hal::digital::OutputPin::set_high(&mut self_.0) {
            Ok(()) => Ok(Ok(())),
            Err(_) => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn set_state(
        &mut self,
        self_: wasmtime::component::Resource<digital::OutputPin>,
        state: digital::PinState,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;

        let state = match state {
            digital::PinState::Low => embedded_hal::digital::PinState::Low,
            digital::PinState::High => embedded_hal::digital::PinState::High,
        };

        match embedded_hal::digital::OutputPin::set_state(&mut self_.0, state) {
            Ok(()) => Ok(Ok(())),
            Err(_) => Ok(Err(digital::ErrorCode::Other)),
        }
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<digital::OutputPin>,
    ) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}

impl digital::HostStatefulOutputPin for HostComponent {
    fn is_set_high(
        &mut self,
        _self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        todo!("StatefulOutputLin::is_set_high")
    }

    fn is_set_low(
        &mut self,
        _self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<Result<bool, digital::ErrorCode>> {
        todo!("StatefulOutputLin::is_set_low")
    }

    fn toggle(
        &mut self,
        _self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<Result<(), digital::ErrorCode>> {
        todo!("StatefulOutputLin::toggle")
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<digital::StatefulOutputPin>,
    ) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}

impl delay::HostDelay for HostComponent {
    fn delay_ns(
        &mut self,
        self_: wasmtime::component::Resource<delay::Delay>,
        ns: u32,
    ) -> wasmtime::Result<()> {
        let _self_ = self.table.get_mut(&self_)?;
        std::thread::sleep(std::time::Duration::from_nanos(ns.into()));
        Ok(())
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<delay::Delay>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
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

    // Open the GPIO device.
    let mut chip = Chip::new("/dev/gpiochip0")
        .context("opening gpio device /dev/gpiochip0")?;

    // Request pin 0 as output.
    let output = CdevPin::new(chip.get_line(0)?.request(
        LineRequestFlags::OUTPUT,
        0,
        "write-output",
    )?)?;

    // Create the resources we'll pass into the `run` function.
    let led = my_state.host.table.push(OutputPin(output))?;
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
