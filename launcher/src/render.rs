use std::num::NonZeroU32;
use std::time::Instant;

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
use imgui_winit_support::winit::dpi::LogicalSize;
use imgui_winit_support::winit::event_loop::EventLoop;
use imgui_winit_support::winit::window::Window;
use imgui_winit_support::winit::window::WindowBuilder;
use imgui_winit_support::winit::{self};
use imgui_winit_support::HiDpiMode;
use imgui_winit_support::WinitPlatform;
use raw_window_handle::HasRawWindowHandle;

static LOGO_PIXELS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.dat"));
pub const LOGO_WIDTH: i32 = 440;
pub const LOGO_HEIGTH: i32 = 419;

pub fn render(
    mut imgui_context: imgui::Context,
    mut do_render: impl FnMut(&mut imgui::Ui, f32, f32, imgui::TextureId),
) {
    let (event_loop, window, surface, context) = create_window();
    let mut winit_platform = imgui_init(&window, &mut imgui_context);

    // OpenGL context from glow
    let gl = glow_context(&context);

    let texture = unsafe {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
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

    // OpenGL renderer from this crate
    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("failed to create renderer");
    let logo_texture = ig_renderer.texture_map_mut().register(texture).unwrap();

    let mut last_frame = Instant::now();

    // Standard winit event loop
    event_loop
        .run(move |event, window_target| {
            match event {
                winit::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui_context
                        .io_mut()
                        .update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                winit::event::Event::AboutToWait => {
                    winit_platform
                        .prepare_frame(imgui_context.io_mut(), &window)
                        .unwrap();
                    window.request_redraw();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::RedrawRequested,
                    ..
                } => {
                    // The renderer assumes you'll be clearing the buffer yourself
                    unsafe { ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

                    let ui = imgui_context.frame();

                    let size = window.inner_size();
                    let size = size.to_logical(window.scale_factor());
                    let h = size.height;
                    let w = size.width;
                    do_render(ui, w, h, logo_texture);

                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();

                    // This is the only extra render step to add
                    ig_renderer
                        .render(draw_data)
                        .expect("error rendering imgui");

                    surface
                        .swap_buffers(&context)
                        .expect("Failed to swap buffers");
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    window_target.exit();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface.resize(
                            &context,
                            NonZeroU32::new(new_size.width).unwrap(),
                            NonZeroU32::new(new_size.height).unwrap(),
                        );
                    }
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("main event loop");
}

fn create_window() -> (
    EventLoop<()>,
    Window,
    Surface<WindowSurface>,
    PossiblyCurrentContext,
) {
    let event_loop = EventLoop::new().expect("event loop");

    let window_builder = WindowBuilder::new()
        .with_title("5th Echelon - Launcher")
        .with_inner_size(LogicalSize::new(1024, 768))
        .with_window_icon(
            winit::window::Icon::from_rgba(
                LOGO_PIXELS.to_vec(),
                LOGO_WIDTH as u32,
                LOGO_HEIGTH as u32,
            )
            .ok(),
        );
    let (window, cfg) = glutin_winit::DisplayBuilder::new()
        .with_window_builder(Some(window_builder))
        .build(&event_loop, ConfigTemplateBuilder::new(), |mut configs| {
            configs.next().unwrap()
        })
        .expect("Failed to create OpenGL window");

    let window = window.unwrap();

    let context_attribs = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
    let context = unsafe {
        cfg.display()
            .create_context(&cfg, &context_attribs)
            .expect("Failed to create OpenGL context")
    };

    let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new()
        .with_srgb(Some(true))
        .build(
            window.raw_window_handle(),
            NonZeroU32::new(1024).unwrap(),
            NonZeroU32::new(768).unwrap(),
        );
    let surface = unsafe {
        cfg.display()
            .create_window_surface(&cfg, &surface_attribs)
            .expect("Failed to create OpenGL surface")
    };

    let context = context
        .make_current(&surface)
        .expect("Failed to make OpenGL context current");

    (event_loop, window, surface, context)
}

fn glow_context(context: &PossiblyCurrentContext) -> Context {
    unsafe { Context::from_loader_function_cstr(|s| context.display().get_proc_address(s).cast()) }
}

fn imgui_init(window: &Window, imgui: &mut imgui::Context) -> WinitPlatform {
    let mut winit_platform = WinitPlatform::init(imgui);
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
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

    winit_platform
}
