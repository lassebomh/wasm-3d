use wasm_bindgen::Clamped;
use web_sys::ImageData;

use crate::{
    inside_triangle,
    matrix::Matrix,
    matrix_3d::{Triangle, screen},
};

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub rows: Vec<Vec<Color>>,
}

impl Bitmap {
    pub fn new(width: u32, height: u32) -> Bitmap {
        let mut rows: Vec<Vec<Color>> = Vec::new();

        for _ in 0..height {
            let mut row: Vec<Color> = Vec::new();
            for _ in 0..width {
                row.push(Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                })
            }
            rows.push(row);
        }

        Bitmap {
            width,
            height,
            rows,
        }
    }

    pub fn render_trig(&mut self, trig: Triangle, view_projection: Matrix<4, 4>, color: Color) {
        let p0 = trig.0(view_projection);
        let p1 = trig.1(view_projection);
        let p2 = trig.2(view_projection);

        let s0 = screen(p0, self.width as f32, self.height as f32);
        let s1 = screen(p1, self.width as f32, self.height as f32);
        let s2 = screen(p2, self.width as f32, self.height as f32);

        let min_x = s0.x().min(s1.x()).min(s2.x()) as usize;
        let max_x = s0.x().max(s1.x()).max(s2.x()) as usize;
        let min_y = s0.y().min(s1.y()).min(s2.y()) as usize;
        let max_y = s0.y().max(s1.y()).max(s2.y()) as usize;

        for x in min_x..max_x {
            for y in min_y..max_y {
                let p = Matrix([[x as f32 + 0.5, y as f32 + 0.5]]);
                if inside_triangle(s0, s1, s2, p) {
                    self.rows[y][x] = color;
                }
            }
        }
    }

    pub fn to_image_data(&self) -> ImageData {
        let mut buf: Vec<u8> = Vec::new();

        for col in self.rows.iter() {
            for color in col.iter() {
                buf.push(color.r);
                buf.push(color.g);
                buf.push(color.b);
                buf.push(color.a);
            }
        }

        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buf), self.width, self.height)
            .expect("failed to create image data")
    }
}
