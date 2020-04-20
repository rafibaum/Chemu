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
    update_pending: bool,
}

impl Display {
    pub fn new(sdl_context: Sdl, width: u32, height: u32) -> Display {
        let video = sdl_context.video().unwrap();
        let window = video
            .window("Chemu", width, height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(OFF_COLOUR);
        canvas.clear();
        canvas.present();

        Display {
            width,
            height,
            pixels: [[false; WIDTH]; HEIGHT],
            canvas,
            update_pending: false,
        }
    }

    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = false;
            }
        }

        self.update_pending = true;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) {
        for (i, &row) in sprite.iter().enumerate() {
            let mut mask: u8 = 0x80;
            for j in 0..8 {
                let pixel = mask & row;
                if pixel != 0 {
                    // Flip pixel
                    self.pixels[(y + i) % HEIGHT][(x + j) % WIDTH] =
                        !self.pixels[(y + i) % HEIGHT][(x + j) % WIDTH];
                }
                mask >>= 1;
            }
        }

        self.update_pending = true;
    }

    pub fn update(&mut self) {
        if !self.update_pending {
            return;
        }

        let height_scale = self.height / HEIGHT as u32;
        let width_scale = self.width / WIDTH as u32;

        self.canvas.set_draw_color(OFF_COLOUR);
        self.canvas.clear();
        self.canvas.set_draw_color(ON_COLOUR);

        for (j, row) in self.pixels.iter().enumerate() {
            let y_scaled = j * height_scale as usize;
            for (i, pixel) in row.iter().enumerate() {
                if *pixel {
                    let x_scaled = i * width_scale as usize;
                    let rect =
                        Rect::new(x_scaled as i32, y_scaled as i32, width_scale, height_scale);
                    self.canvas.draw_rect(rect).unwrap();
                    self.canvas.fill_rect(rect).unwrap();
                }
            }
        }

        self.canvas.present();
        self.update_pending = false;
    }
}
