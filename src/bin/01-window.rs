// Make a window

use winit::dpi::LogicalSize;
use winit::{CreationError, Event, EventsLoop, Window, WindowBuilder, WindowEvent};

pub const WINDOW_NAME: &str = "01-Window";

#[derive(Debug)]
pub struct WinitState {
    pub event_loop: EventsLoop,
    pub window: Window,
}

impl WinitState {
    /// Setup a new event loop and window
    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        let event_loop = EventsLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(size)
            .build(&event_loop);

        output.map(|window| Self { event_loop, window })
    }
}

impl Default for WinitState {
    /// Makes an 800x600 window with the `WINDOW_NAME` value as the title.
    /// ## Panics
    /// If a `CreationError` occurs.
    fn default() -> Self {
        Self::new(
            WINDOW_NAME,
            LogicalSize {
                width: 800.0,
                height: 600.0,
            },
        )
        .expect("Could not create a window!")
    }
}

fn main() {
    let mut win_state = WinitState::default();

    // Main loop
    let mut running = true;
    while running {
        win_state.event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close requested");
                running = false;
            }
            _ => (),
        });
    }
}
