#![no_std]
#![no_main]

use wasi_async_runtime::{block_on, Reactor};

// Import the types from the bindings that we use.
use crate::bindings::exports::sketch::embedded::async_run::Guest;
use crate::bindings::sketch::embedded::async_delay::Delay;
use crate::bindings::sketch::embedded::digital::OutputPin;
use wasi::io::poll::Pollable;
use futures_util::join;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

mod bindings;

struct Component;

/// This is the main implementation code.
impl Guest for Component {
    fn run(pin0: OutputPin, pin1: OutputPin, delay: Delay) {
        block_on(|reactor| async move {
            let a = blink(1_000_000_000, pin0, &delay, &reactor);
            let b = blink(300_000_00, pin1, &delay, &reactor);
            join!(a, b);
        })
    }
}

async fn blink(ns: u32, pin: OutputPin, delay: &Delay, reactor: &Reactor) {
    loop {
        pin.set_high().ok();

        let pollable = delay.subscribe_to_delay_ns(ns);
        let pollable = unsafe { Pollable::from_handle(pollable.into_handle()) };
        reactor.wait_for(pollable).await;

        pin.set_low().ok();

        let pollable = delay.subscribe_to_delay_ns(ns);
        let pollable = unsafe { Pollable::from_handle(pollable.into_handle()) };
        reactor.wait_for(pollable).await;
    }
}

/// Define a global allocator, since we're using `no_std`.
/// SAFETY: We're single-threaded.
#[global_allocator]
static GLOBAL_ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

/// Define a panic handler, since we're using `no_std`. Just infloop for
/// now and hope we don't panic.
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // Don't panic ;-).
    loop {}
}
