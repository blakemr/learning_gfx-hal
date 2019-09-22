// Crates
// Change depending on desired backend
extern crate gfx_backend_vulkan as backend;

extern crate gfx_hal;
extern crate winit;

use gfx_hal::{
    command::{ClearColor, ClearValue},
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{Access, Layout, SubresourceRange, ViewKind},
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    },
    pool::CommandPoolCreateFlags,
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    },
    queue::Submission,
    Backbuffer, Device, FrameSync, Graphics, Instance, Primitive, Surface, SwapImageIndex,
    Swapchain, SwapchainConfig,
};
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent};

fn main() {
    // Create window with winit.
    // This is not specific to gfx, but you need some windowing crate
    // to put pictures on.
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().build(&events_loop).unwrap();

    // Instance is the graphics API entry point.
    // The name and version are not important. (why? LEARN_MORE)
    // This also gives access to the surface we're going to draw to.
    let instance = backend::Instance::create("00 Triangle", 1);
    let mut surface = instance.create_surface(&window);

    // Adapter represents a physical device (ex: graphics card)/
    // This example picks the first one, but you can choose one here
    // LEARN_MORE
    let mut adapter = instance.enumerate_adapters().remove(0);

    // LEARN_MORE. I have no idea what this bit means.
    // "...The device here is a logical device rather than a physical one.
    // It’s an abstraction responsible for allocating and freeing resources,
    // which we’ll see later."
    //
    // The device is a logical device allowing you to perform GPU operations.
    // The queue group contains a set of command queues which we can later submit
    // drawing commands to.
    //
    // Here we're requesting 1 queue, with the `Graphics` capability so we can do
    // rendering. We also pass a closure to choose the first queue family that our
    // surface supports to allocate queues from. More on queue families in a later
    // tutorial.
    let num_queues = 1;
    let (device, mut queue_group) = adapter
        .open_with::<_, Graphics>(num_queues, |family| surface.supports_queue_family(family))
        .unwrap();

    // A command pool is used to get command buffers
    // Command buffers are used to send drawing instructions to the GPU
    let max_buffers = 16;
    let mut command_pool = device.create_command_pool_typed(
        &queue_group,
        CommandPoolCreateFlags::empty(),
        max_buffers,
    );

    // Everything above is standard for almost all gfx projects.
    // Next...Setting up the rendering pipeline.

    let physical_device = &adapter.physical_device;

    // We want to get the capabilities (`caps`) of the surface, which tells us what
    // parameters we can use for our swapchain later. We also get a list of supported
    // image formats for our surface.
    let (caps, formats, _) = surface.compatibility(physical_device);

    let surface_color_format = {
        // We must pick a color format from the list of supported formats. If there
        // is no list, we default to Rgba8Srgb.
        match formats {
            Some(choices) => choices
                .into_iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .unwrap(),
            None => Format::Rgba8Srgb,
        }
    };

    // A render pass defines which attachments (images) are to be used for what
    // purposes. Right now, we only have a color attachment for the final output,
    // but eventually we might have depth/stencil attachments, or even other color
    // attachments for other purposes.
    let render_pass = {
        let color_attachment = Attachment {
            format: Some(surface_color_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::Present,
        };

        // A render pass should have multiple subpasses, but only one is needed for now.
        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        // This is the dependancies between subpasses. Only one is needed for now.
        let dependency = SubpassDependency {
            passes: SubpassRef::External..SubpassRef::Pass(0),
            stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
            accesses: Access::empty()
                ..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
        };

        device.create_render_pass(&[color_attachment], &[subpass], &[dependency])
    };

    // Shader modules are needed to create a pipeline definition.
    // The shader is loaded from SPIR-V binary files.
    let vertex_shader_module = {
        let spirv = include_bytes!("../../assets/shaders/part00.vert");
        device.create_shader_module(spirv).unwrap()
    };
    let fragment_shader_module = {
        let spirv = include_bytes!("../../assets/shaders/part00.frag");
        device.create_shader_module(spirv).unwrap()
    };

    let pipeline_layout = device.create_pipeline_layout(&[], &[]);

    // A pipeline object encodes almost all the state you need in order to draw
    // geometry on screen. For now that's really only which shaders to use, what
    // kind of blending to do, and what kind of primitives to draw.
    let pipeline = {
        let vs_entry = EntryPoint::<backend::Backend> {
            entry: "main",
            module: &vertex_shader_module,
            specialization: Default::default(),
        };

        let fs_entry = EntryPoint::<backend::Backend> {
            entry: "main",
            module: &fragment_shader_module,
            specialization: Default::default(),
        };

        let shader_entries = GraphicsShaderSet {
            vertex: vs_entry,
            fragment: Some(fs_entry),
            hull: None,
            domain: None,
            geometry: None,
        };

        let subpass = Subpass {
            index: 0,
            main_pass: &render_pass,
        };

        let mut pipeline_desc = GraphicsPipelineDesc::new(
            shader_entries,
            Primitive::TriangleList,
            Rasterizer::FILL,
            &pipeline_layout,
            subpass,
        );

        pipeline_desc
            .blender
            .targets
            .push(ColorBlendDesc(ColorMask::ALL, BlendState::ALPHA));

        device
            .create_graphics_pipeline(&pipeline_desc, None)
            .unwrap()
    };

    // Init the swapchain
    let swap_config = SwapchainConfig::from_caps(&caps, surface_color_format);

    let extent = swap_config.extent.to_extent();

    let (mut swapchain, backbuffer) = device.create_swapchain(&mut surface, swap_config, None);
}
