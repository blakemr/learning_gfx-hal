/// Print a triangle to the screen
#[cfg(windows)]
extern crate gfx_backend_dx12 as backend;
#[cfg(target_os = "macos")]
extern crate gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan as backend;

use winit::dpi::LogicalSize;
use winit::{CreationError, Event, EventsLoop, Window, WindowBuilder, WindowEvent};

use arrayvec::ArrayVec;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub const WINDOW_NAME: &str = "01-Window";

/// Struct for a window setup. Has an event_loop and window.
#[derive(Debug)]
pub struct WinitState {
    pub event_loop: EventsLoop,
    pub window: Window,
}

#[derive(Debug)]
pub struct GfxState {

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

impl GfxState {
    /// Draw a frame that's just cleared to the color specified.
    pub fn draw_clear_frame(&mut self, color: [f32; 4]) -> Result<(), &str> {
        // SETUP FOR THIS FRAME
        let image_available = &self.image_available_semaphores[self.current_frame];
        let render_finished = &self.render_finished_semaphores[self.current_frame];
        // Advance the frame _before_ we start using the `?` operator
        self.current_frame = (self.current_frame + 1) % self.frames_in_flight;

        let (i_u32, i_usize) = unsafe {
        let image_index = self
            .swapchain
            .acquire_image(core::u64::MAX, FrameSync::Semaphore(image_available))
            .map_err(|_| "Couldn't acquire an image from the swapchain!")?;
        (image_index, image_index as usize)
        };

        let flight_fence = &self.in_flight_fences[i_usize];
        unsafe {
        self
            .device
            .wait_for_fence(flight_fence, core::u64::MAX)
            .map_err(|_| "Failed to wait on the fence!")?;
        self
            .device
            .reset_fence(flight_fence)
            .map_err(|_| "Couldn't reset the fence!")?;
        }

        // RECORD COMMANDS
        unsafe {
        let buffer = &mut self.command_buffers[i_usize];
        let clear_values = [ClearValue::Color(ClearColor::Float(color))];
        buffer.begin(false);
        buffer.begin_render_pass_inline(
            &self.render_pass,
            &self.framebuffers[i_usize],
            self.render_area,
            clear_values.iter(),
        );
        buffer.finish();
        }

        // SUBMISSION AND PRESENT
        let command_buffers = &self.command_buffers[i_usize..=i_usize];
        let wait_semaphores: ArrayVec<[_; 1]> = [(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
        let signal_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        // yes, you have to write it twice like this. yes, it's silly.
        let present_wait_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        let submission = Submission {
        command_buffers,
        wait_semaphores,
        signal_semaphores,
        };
        let the_command_queue = &mut self.queue_group.queues[0];
        unsafe {
        the_command_queue.submit(submission, Some(flight_fence));
        self
            .swapchain
            .present(the_command_queue, i_u32, present_wait_semaphores)
            .map_err(|_| "Failed to present into the swapchain!")
        }
    }
}

impl Default for GfxState {
    fn default(win: &Window) -> Self {
        unimplemented!()
    }
}

fn main() {
    // Init logging
    env_logger::init();

    // Init states
    let win_state = WinitState::default();
    let gfx_state = GfxState::default(&win_state.window);
    let mut local_state = LocalState::default();

    // Main loop
    loop {
        // Get input
        let inputs = UserInput::poll_events_loop(&mut win_state.event_loop);

        // Input response
        if inputs.end_loop {break;}
        local_state.update_from_input(inputs);

        // Update screen
        if let Err(e) = render_update(&mut gfx_state, &local_state) {
            error!("Error updating the render: {:?}", e);
            break;
        }
    }
}

pub fn render_update(gfx: &mut GfxState, locals: &LocalState) -> Result<(), &str> {
    gfx.draw_clear_frame(locals.color());
}