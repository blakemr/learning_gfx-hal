//! Learning gfx-hal
//! attempt 2
//! Step 1: Fully understand how to make a window
//! this part shouldn't require gfx-hal at all
use winit::{Event, EventsLoop, WindowBuilder, WindowEvent};

/// Window struct. Comprised of a window and an event loop
///
/// # args
/// * **events_loop** *EventsLoop* - event loop object. Controls events. duh
/// * **window** *WindowBuilder* - window that opens (not the device). Controls how the window looks.
#[derive(Debug)]
pub struct EventWindow {
    pub events_loop: EventsLoop,
    pub window: WindowBuilder,
}

/// Main function. Main loop goes here
fn main() {
    // Winit needs an events loop, so lets give it one.
    // Note: poll_events requires this to be mutable,
    // but the docs don't really cover that. FYI
    let mut events_loop = EventsLoop::new();

    // Init the window with a builder for later customization
    let builder = WindowBuilder::new()
        .with_title("Init a window")
        .build(&events_loop)
        .expect("Could not build window.");

    // Main loop
    let mut exit = false;
    while !exit {
        // Every loop, poll events for input
        events_loop.poll_events(|event| match event {
            // Check for an exit, if so break loop and end
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => exit = true,

            // Default behavior (for events we don't care about)
            _ => (),
        });
    }
}
