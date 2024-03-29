//! Learning gfx-hal
//! attempt 2
//! Step 1: Fully understand how to make a window
//! this part shouldn't require gfx-hal at all
use winit::dpi::LogicalSize;
use winit::{CreationError, Event, EventsLoop, Window, WindowBuilder, WindowEvent};

/// CONSTANTS
// Window name
pub const WINDOW_TITLE: &str = "I'm Learning";
// Window size
pub const WINDOW_SIZE: LogicalSize = LogicalSize {
    width: 500.0,
    height: 500.0,
};

/// Window with an event loop
///
/// # args
/// * **events_loop** *EventsLoop* - event loop object. Controls events. duh
/// * **window** *WindowBuilder* - window that opens (not the device). Controls how the window looks.
#[derive(Debug)]
pub struct EventWindow {
    pub events_loop: EventsLoop,
    pub window: Window,
}

impl EventWindow {
    /// Init new EventWindow with events_loop and window
    ///
    /// # args
    /// * **title** *String or reference* - Window title
    /// * **size** *LogicalSize* - Window size (x, y).
    ///
    /// # returns
    /// * Self or CreationError
    ///
    /// # errors
    /// * **CreationError** - winit error that can happen when trying to make a window
    ///
    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        // Init events loop and window
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(size)
            .build(&events_loop);

        // window is currently a result type, so that needs to be mapped to Self
        // iff it's not an error.
        window.map(|window| Self {
            events_loop,
            window,
        })
    }
}

impl Default for EventWindow {
    /// Default window. Uses the WINDOW_TITLE and WINDOW_SIZE consts
    ///
    /// # Errors
    /// * CreationError - Error when building window
    fn default() -> Self {
        Self::new(WINDOW_TITLE, WINDOW_SIZE).expect("Window creation failed: EventWindow.default()")
    }
}

/// Main function. Main loop goes here
fn main() {
    // Create a new EventWindow
    let mut event_window = EventWindow::default();

    // Main loop
    let mut exit = false;
    while !exit {
        // Every loop, poll events for input
        event_window.events_loop.poll_events(|event| match event {
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
