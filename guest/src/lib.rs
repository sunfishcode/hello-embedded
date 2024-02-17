#![no_std]
#![no_main]

// Import the types from the bindings that we use.
use crate::bindings::exports::sketch::embedded::run::Guest;
use crate::bindings::sketch::embedded::delay::Delay;
use crate::bindings::sketch::embedded::digital::OutputPin;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

mod bindings;

struct Component;

/// This is the main implementation code.
impl Guest for Component {
    fn run(led: OutputPin, delay: Delay) {
        // Turn the led on. Wait. Turn it off. Wait. Repeat!
        loop {
            led.set_high().ok();
            delay.delay_ns(1_000_000_000);
            led.set_low().ok();
            delay.delay_ns(1_000_000_000);
        }
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
