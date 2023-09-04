use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use winit_input_helper::WinitInputHelper;

const RESOLUTION: u8 = 1; // 0=160x120, 1=360x240, 4=640x480
const SCREEN_WIDTH: u32 = (160 * RESOLUTION) as u32;
const SCREEN_HEIGHT: u32 = (120 * RESOLUTION) as u32;
const PIXEL_SCALE: u8 = 4 / RESOLUTION;
pub const BUFFER_WIDTH: u32 = SCREEN_WIDTH * (PIXEL_SCALE as u32);
pub const BUFFER_HEIGHT: u32 = SCREEN_HEIGHT * (PIXEL_SCALE as u32);

struct Doom {
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
    render_window: Window,
    pixel_buffer: Pixels,

}

impl Doom {
    fn new() -> Option<Doom> {
        env_logger::init();
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let render_window = {
            let size = LogicalSize::new(BUFFER_WIDTH as f64, BUFFER_HEIGHT as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixel_buffer = {
            let window_size = render_window.inner_size();
            let surface_texture: SurfaceTexture<'_, winit::window::Window> = SurfaceTexture::new(window_size.width, window_size.height, &render_window);
            Pixels::new(BUFFER_WIDTH, BUFFER_HEIGHT, surface_texture).unwrap()
        };

        Some(Doom {event_loop, input, render_window, pixel_buffer})
    }

    fn run() -> Option<()> {
        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            
    
            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
    
                // Resize the window
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
    
                // Update internal state and request a redraw
                
                window.request_redraw();
            }
        });

        Some(())
    }
}

struct Keys {
    w: bool, // up, left, down, right
    a: bool,
    s: bool,
    d: bool,
    sl: bool, // strafe left and right
    sr: bool, 
}