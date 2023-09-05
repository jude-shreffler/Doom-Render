use std::f32::consts::PI;

use minifb::{Key, Window, WindowOptions};

const RESOLUTION: u8 = 1; // 0=160x120, 1=360x240, 4=640x480
const SCREEN_WIDTH: u32 = (160 * RESOLUTION) as u32;
const SCREEN_HEIGHT: u32 = (120 * RESOLUTION) as u32;
const PIXEL_SCALE: u8 = 4 / RESOLUTION;
pub const BUFFER_WIDTH: u32 = SCREEN_WIDTH * (PIXEL_SCALE as u32);
pub const BUFFER_HEIGHT: u32 = SCREEN_HEIGHT * (PIXEL_SCALE as u32);

pub struct Doom {
    buffer: Vec<u32>,
    window: Window,
    trig: Trig,
    player_position: Point3,
    player_yaw: i32,
    player_tilt: i32,
}

impl Doom {
    pub fn new() -> Doom {
        let mut buffer: Vec<u32> = vec![0; (BUFFER_WIDTH * BUFFER_HEIGHT) as usize];

        let mut window = Window::new(
            "Test - ESC to exit",
            BUFFER_WIDTH as usize,
            BUFFER_HEIGHT as usize,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        
        let trig = Trig::init();
        let player_position = Point3 { x: 70.0, y: -110.0, z: 20.0 };
        let player_yaw = 0;
        let player_tilt = 0;

        Doom {buffer, window, trig, player_position, player_yaw, player_tilt}
    }
    
    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.draw_frame();
        }
    }

    fn draw_frame(&mut self) {
        // debug printing
        self.debug();
        
        self.clear_background(0x0);
        self.draw3D();
        self.check_inputs();



        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        self.window
            .update_with_buffer(&self.buffer, BUFFER_WIDTH as usize, BUFFER_HEIGHT as usize)
            .unwrap();
    }

    fn debug(&self) {
        println!("Player position: {} {} {}", self.player_position.x, self.player_position.y, self.player_position.z);
        println!("Player yaw: {}", self.player_yaw)
    }

    fn pixel(&mut self, x: u32, y: u32, color: u32) {
        self.buffer[screen_to_buffer(x, y)] = color;
    }

    fn clear_background(&mut self, color: u32) {
        for i in self.buffer.iter_mut() {
            *i = color;
        }
    }

    fn draw3D(&mut self) {

    }

    fn check_inputs(&mut self) {
        for key in self.window.get_keys() {
            match key {
                Key::A => {
                    self.player_yaw -= 4;
                    if self.player_yaw < 0 {
                        self.player_yaw += 360;
                    }
                },
                Key::D => {
                    self.player_yaw += 4;
                    if self.player_yaw > 359 {
                        self.player_yaw -= 360;
                    }
                },
                Key::W => {
                    let delta_x = self.trig.sin[self.player_yaw as usize] * 10.0;
                    let delta_y = self.trig.cos[self.player_yaw as usize] * 10.0;
                    self.player_position.x += delta_x;
                    self.player_position.y += delta_y;
                },
                Key::S => {
                    let delta_x = self.trig.sin[self.player_yaw as usize] * -5.0;
                    let delta_y = self.trig.cos[self.player_yaw as usize] * -5.0;
                    self.player_position.x += delta_x;
                    self.player_position.y += delta_y;
                },

                _ => (),
            }
        }
    }
}

struct Point3 {
    x: f32,
    y: f32,
    z: f32
}

struct Trig {
    sin: [f32; 360],
    cos: [f32; 360],
}

impl Trig {
    fn init() -> Trig {
        let mut sin: [f32; 360] = [0.0; 360];
        let mut cos: [f32; 360] = [0.0; 360];

        for i in 0..360 {
            sin[i] = (i as f32 / 180.0*PI).sin();
            cos[i] = (i as f32 / 180.0*PI).cos();
        }

        Trig {sin, cos}
    }
}

// takes a screen position starting from the top left and converts it to an index for our buffer
fn screen_to_buffer(x: u32, y: u32) -> usize {
    (y * BUFFER_WIDTH + x) as usize
}