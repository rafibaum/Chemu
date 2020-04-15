use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const OFF_COLOUR: Color = Color::RGB(0, 0, 0);
const ON_COLOUR: Color = Color::RGB(255, 255, 255);

pub struct Display {
    width: u32,
    height: u32,
    pixels: [[bool; WIDTH]; HEIGHT],
    canvas: WindowCanvas,
}

impl Display {
    pub fn new(sdl_context: Sdl, width: u32, height: u32) -> Display {
        let video = sdl_context.video().unwrap();
        let window = video
            .window("Chemu", width, height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Display {
            width,
            height,
            pixels: [[false; WIDTH]; HEIGHT],
            canvas,
        }
    }

    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = false;
            }
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) {
        for (i, &row) in sprite.iter().enumerate() {
            let mut mask: u8 = 0x80;
            for j in 0..8 {
                let pixel = mask & row;
                if pixel != 0 {
                    // Flip pixel
                    self.pixels[y + i][x + j] = !self.pixels[y + i][x + j];
                    self.update_pixel(x + j, y + i);
                }
                mask >>= 1;
            }
        }

        self.canvas.present();
    }

    fn update_pixel(&mut self, x: usize, y: usize) {
        let height_scale = self.height / HEIGHT as u32;
        let width_scale = self.width / WIDTH as u32;

        let x_scaled = x * width_scale as usize;
        let y_scaled = y * height_scale as usize;

        let rect = Rect::new(x_scaled as i32, y_scaled as i32, width_scale, height_scale);
        let colour = if self.pixels[y][x] {
            ON_COLOUR
        } else {
            OFF_COLOUR
        };

        self.canvas.set_draw_color(colour);
        self.canvas.draw_rect(rect).unwrap();
    }
}
