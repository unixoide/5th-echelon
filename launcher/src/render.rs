//! Handles window creation, OpenGL context setup, and the main rendering loop
//! using `winit`, `glutin`, and `imgui`.
//!
//! This module abstracts the boilerplate required for setting up a graphical
//! user interface and provides a simple `render` function to run the application.

use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use glow::Context;
use glow::HasContext;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextAttributesBuilder;
use glutin::context::NotCurrentGlContext;
use glutin::context::PossiblyCurrentContext;
use glutin::display::GetGlDisplay;
use glutin::display::GlDisplay;
use glutin::surface::GlSurface;
use glutin::surface::Surface;
use glutin::surface::SurfaceAttributesBuilder;
use glutin::surface::WindowSurface;
use imgui_glow_renderer::TextureMap;
use imgui_winit_support::winit::event_loop::EventLoop;
#[allow(deprecated)]
use imgui_winit_support::winit::raw_window_handle::HasRawWindowHandle;
use imgui_winit_support::winit::window::Window;
use imgui_winit_support::winit::{self};
use imgui_winit_support::HiDpiMode;
use imgui_winit_support::WinitPlatform;

// Include the logo image data, which is processed by the build script.
static LOGO_PIXELS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.dat"));
pub const LOGO_WIDTH: i32 = 256;
pub const LOGO_HEIGTH: i32 = 256;

static OLD_LOGO_PIXELS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/old_logo.dat"));
pub const OLD_LOGO_WIDTH: i32 = 440;
pub const OLD_LOGO_HEIGTH: i32 = 419;

/// The main rendering function for the application.
///
/// This function creates a window, sets up the rendering context, and runs the
/// main event loop, calling the `do_render` closure on each frame.
pub fn render(initial_size: winit::dpi::LogicalSize<u32>, mut imgui_context: imgui::Context, mut do_render: impl FnMut(&mut imgui::Ui, f32, f32, imgui::TextureId)) {
    let (event_loop, window, surface, context) = create_window(initial_size);
    let mut winit_platform = imgui_init(&window, &mut imgui_context);

    // Create a glow OpenGL context.
    let gl = glow_context(&context);

    // Create and upload the logo texture to the GPU.
    let logo_texture = unsafe {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA8 as i32,
            LOGO_WIDTH,
            LOGO_HEIGTH,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(LOGO_PIXELS),
        );
        texture
    };

    // Create and upload the old logo texture to the GPU.
    let old_logo_texture = unsafe {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA8 as i32,
            OLD_LOGO_WIDTH,
            OLD_LOGO_HEIGTH,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(OLD_LOGO_PIXELS),
        );
        texture
    };

    // Initialize the imgui glow renderer.
    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::new(gl, &mut imgui_context).expect("failed to create renderer");
    let _logo_texture = ig_renderer.texture_map_mut().register(logo_texture).unwrap();
    let old_logo_texture = ig_renderer.texture_map_mut().register(old_logo_texture).unwrap();

    let mut last_frame = Instant::now();
    let mut last_redraw = Instant::now();
    const TARGET_FPS: u32 = 60;
    const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

    // Start the winit event loop.
    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                winit::event::Event::NewEvents(_) => {
                    // Update the delta time for imgui.
                    let now = Instant::now();
                    imgui_context.io_mut().update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                winit::event::Event::AboutToWait => {
                    // Limit FPS to avoid excessive CPU usage
                    let now = Instant::now();
                    let time_since_last_redraw = now.duration_since(last_redraw);
                    
                    if time_since_last_redraw >= FRAME_DURATION {
                        // Prepare for the next frame.
                        winit_platform.prepare_frame(imgui_context.io_mut(), &window).unwrap();
                        window.request_redraw();
                        last_redraw = now;
                    }
                    
                    // Use Wait to avoid busy-looping
                    window_target.set_control_flow(winit::event_loop::ControlFlow::Wait);
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Clear the screen.
                    unsafe { ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

                    // Create a new imgui frame.
                    let ui = imgui_context.frame();

                    // Call the user-provided render function.
                    let size = window.inner_size();
                    let size = size.to_logical(window.scale_factor());
                    let h = size.height;
                    let w = size.width;
                    do_render(ui, w, h, old_logo_texture);

                    // Prepare and render the imgui draw data.
                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();
                    ig_renderer.render(draw_data).expect("error rendering imgui");

                    // Swap the buffers to display the new frame.
                    surface.swap_buffers(&context).expect("Failed to swap buffers");
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    // Exit the application when the window is closed.
                    window_target.exit();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    // Handle window resizing.
                    if new_size.width > 0 && new_size.height > 0 {
                        surface.resize(&context, NonZeroU32::new(new_size.width).unwrap(), NonZeroU32::new(new_size.height).unwrap());
                    }
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
                event => {
                    // Pass other events to the winit platform.
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("main event loop");
}

/// Creates the main window and OpenGL context.
fn create_window(initial_size: winit::dpi::LogicalSize<u32>) -> (EventLoop<()>, Window, Surface<WindowSurface>, PossiblyCurrentContext) {
    let event_loop = EventLoop::new().expect("event loop");

    // Set up the window attributes.
    let attr = Window::default_attributes()
        .with_title("5th Echelon - Launcher")
        .with_inner_size(initial_size)
        .with_window_icon(winit::window::Icon::from_rgba(LOGO_PIXELS.to_vec(), LOGO_WIDTH as u32, LOGO_HEIGTH as u32).ok());

    // Build the window and OpenGL context.
    let (window, cfg) = glutin_winit::DisplayBuilder::new()
        .with_window_attributes(Some(attr))
        .build(&event_loop, ConfigTemplateBuilder::new(), |mut configs| configs.next().unwrap())
        .expect("Failed to create OpenGL window");

    let window = window.unwrap();

    // Create the OpenGL context.
    #[allow(deprecated)]
    let raw_handle = window.raw_window_handle().expect("raw window handle");
    let context_attribs = ContextAttributesBuilder::new().build(Some(raw_handle));
    let context = unsafe { cfg.display().create_context(&cfg, &context_attribs).expect("Failed to create OpenGL context") };

    // Create the window surface.
    let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new().with_srgb(Some(true)).build(
        raw_handle,
        NonZeroU32::new(initial_size.width).unwrap(),
        NonZeroU32::new(initial_size.height).unwrap(),
    );
    let surface = unsafe { cfg.display().create_window_surface(&cfg, &surface_attribs).expect("Failed to create OpenGL surface") };

    // Make the context current.
    let context = context.make_current(&surface).expect("Failed to make OpenGL context current");

    (event_loop, window, surface, context)
}

/// Creates a glow OpenGL context from a glutin context.
fn glow_context(context: &PossiblyCurrentContext) -> Context {
    unsafe { Context::from_loader_function_cstr(|s| context.display().get_proc_address(s).cast()) }
}

/// Initializes imgui for the given window.
fn imgui_init(window: &Window, imgui: &mut imgui::Context) -> WinitPlatform {
    let mut winit_platform = WinitPlatform::new(imgui);
    {
        // Set the HiDPI mode, allowing for a forced DPI factor for debugging.
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_FORCE_DPI_FACTOR") {
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        winit_platform.attach_window(imgui.io_mut(), window, dpi_mode);
    }

    imgui.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    // Set the clipboard backend.
    imgui.set_clipboard_backend(crate::sys::clipboard_backend(window));

    winit_platform
}
