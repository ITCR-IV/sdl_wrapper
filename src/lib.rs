//! This module provides simple wrapper functions that will be the only interactions the rest of
//! the project has with the [sdl2](sdl2) crate.

mod constants;

use crate::constants::COLOR_DEPTH;
use sdl2::{
    event::EventPollIterator,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};

pub use sdl2::{event::Event, keyboard::Keycode};

/// This struct abstracts away any direct interaction with the SDL module, so that the user may
/// only need to call the provided methods without `use`ing any sdl modules.
pub struct ScreenContextManager {
    canvas: Canvas<Window>,
    framebuffer: Vec<u8>,
    texture_creator: TextureCreator<WindowContext>,
    color: Color,
    event_pump: EventPump,
    height: u32,
    width: u32,
    width_times_color: u32,
}

impl ScreenContextManager {
    /// Creates a new object, with the side-effect of creating a new window with the title given.
    pub fn new(title: &str, width: u32, height: u32) -> ScreenContextManager {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

        let texture_creator = canvas.texture_creator();
        let event_pump = sdl.event_pump().unwrap();

        ScreenContextManager {
            canvas,
            // Create empty framebuffer
            framebuffer: vec![0; (width * COLOR_DEPTH * height) as usize],
            texture_creator,
            event_pump,
            color: Color::RGB(0, 0, 0),
            height,
            width,
            width_times_color: width * COLOR_DEPTH,
        }
    }

    /// Sets the color to be used for drawing operations.
    /// Parameters correspond to RGB colors and must be real numbers in the range [0, 1].
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Color::RGB(
            (r * 255 as f32).round() as u8,
            (g * 255 as f32).round() as u8,
            (b * 255 as f32).round() as u8,
        );
    }

    /// Plots a single pixel on the framebuffer.
    pub fn plot_pixel(&mut self, x: u32, y: u32) {
        let i = (y * self.width_times_color + x * COLOR_DEPTH) as usize;
        //println!("Drawing to {}, {}, {}", i, i + 1, i + 2);
        self.framebuffer[i] = self.color.r;
        self.framebuffer[i + 1] = self.color.g;
        self.framebuffer[i + 2] = self.color.b;
    }

    /// Clears the entire framebuffer with a grey shadow given by a real number in the range [0,
    /// 1].
    pub fn clear(&mut self, shadow: f32) {
        self.framebuffer.fill((shadow * 255 as f32).round() as u8);
    }

    /// Presents the current contents of the framebuffer on the window's canvas
    pub fn present(&mut self) {
        let mut texture = self
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, self.width, self.height)
            .unwrap();

        texture
            .update(None, &self.framebuffer, (self.width_times_color) as usize)
            .unwrap();

        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    /// Returns an iterator that will hold all the current window events. The iterator will
    /// terminate once there are no pending events.
    pub fn get_events(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
