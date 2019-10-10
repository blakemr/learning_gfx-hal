//! Learning gfx-hal
//! attempt 2

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

use gfx_hal::pso::PipelineStage;
use winit::dpi::LogicalSize;
use winit::{CreationError, Event, EventsLoop, Window, WindowBuilder, WindowEvent};
use Err;

#[macro_use]
extern crate log;

use log::Level;

/// CONSTANTS
// Window name
pub const WINDOW_TITLE: &str = "I'm Learning";
// Window size
pub const WINDOW_SIZE: LogicalSize = LogicalSize {
    width: 500.0,
    height: 500.0,
};

/// Main function. Main loop goes here
fn main() {
    env_logger::init();

    // INIT MANAGERS
    let mut event_window = EventWindow::default();
    let mut gfx_state = GfxState::new(&event_window.window);
    let mut local_state = LocalState::default();

    // MAIN LOOP
    loop {
        let inputs = UserInput::poll_events(&mut event_window.events_loop);
        if inputs.end_requested {
            break;
        }
        local_state.update_from_input(inputs);

        // Render screen
        if let Err(e) = render_screen(&mut gfx_state, &local_state) {
            error!("Rendering Error: {:?}", e);
            break;
        }
    }

    // CLEANUP
    // TODO
}

/// Window with an event loop
///
/// ## args
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
    /// ## args
    /// * **title** *String or reference* - Window title
    /// * **size** *LogicalSize* - Window size (x, y).
    ///
    /// ## returns
    /// * Self or CreationError
    ///
    /// ## errors
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
    /// ## Errors
    /// * CreationError - Error when building window
    fn default() -> Self {
        Self::new(WINDOW_TITLE, WINDOW_SIZE).expect("Window creation failed: EventWindow.default()")
    }
}

/// Graphics manager
pub struct GfxState {}
impl GfxState {
    /// Make new GfxState
    ///
    /// ## args
    /// * **window** *&Window* - window to use when rendering
    ///
    /// ## returns
    /// Result
    ///
    /// ## errors
    /// TODO
    pub fn new(window: &Window) -> Result<(), &'static str> {
        unimplemented!()
    }
    /// Clear the screen to a specified color
    ///
    /// ## args
    /// * **gfx** *GfxState* - object handling the graphics operations
    /// * **locals** *LocalState* - object handling the inputs/state changes/etc...
    ///
    /// ## Returns
    /// Returns nothing unless there's an error
    ///
    /// ## Errors
    /// Returns an error to be handled by the user if something goes wrong
    pub fn clear_screen(&mut self, color: [f32; 4]) -> Result<(), &'static str> {
        // Frame setup
        // TODO

        // Record commands
        // TODO

        // Submission
        let command_buffers = vec![the_command_buffer];
        let wait_semaphores = vec![(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)];
        let signal_semaphores = vec![render_finished];
        let present_wait_semaphores = vec![render_finished];
        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores,
        };
        unsafe {
            the_command_queue.submit(submission, Some(flight_fence));
            the_swapchain
                .present(&mut the_command_queue, i_u32, present_wait_semaphores)
                .map_err(|_| "Failed to present into the swapchain!")
        }
    }
}

/// Clear the screen to a specified color
///
/// ## args
/// * **gfx** *GfxState* - object handling the graphics operations
/// * **locals** *LocalState* - object handling the inputs/state changes/etc...
///
/// ## Returns
/// Returns nothing unless there's an error
///
/// ## Errors
/// Returns an error to be handled by the user if something goes wrong
pub fn clear_screen(gfx: &mut GfxState, locals: &LocalState) -> Result<(), &str> {
    gfx.clear_screen(locals.color());
}
