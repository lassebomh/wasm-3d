#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub mod matrix;
pub mod matrix_3d;
use core::{f32, panic};

use matrix::Matrix;

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::ImageData;

use crate::matrix_3d::{
    Model, Point2D, Triangle, cube, from_screen, perspective, quad, rotate_x, rotate_y, scale,
    screen, translate,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct Bitmap {
    width: u32,
    height: u32,
    rows: Vec<Vec<Color>>,
}

impl Bitmap {
    fn new(width: u32, height: u32) -> Bitmap {
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

    fn to_image_data(&self) -> ImageData {
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

fn cross_product(a: Point2D, b: Point2D, c: Point2D) -> f32 {
    (b.x() - a.x()) * (c.y() - a.y()) - (b.y() - a.y()) * (c.x() - a.x())
}

fn inside_triangle(screen_0: Point2D, screen_1: Point2D, screen_2: Point2D, p: Point2D) -> bool {
    let ab = cross_product(screen_1, screen_0, p) >= 0.;
    if !ab {
        return false;
    };
    let ac = cross_product(screen_0, screen_2, p) >= 0.;
    if !ac {
        return false;
    };
    let bc = cross_product(screen_2, screen_1, p) >= 0.;
    if !bc {
        return false;
    };
    return true;
}

#[wasm_bindgen]
pub fn render(
    ctx: web_sys::CanvasRenderingContext2d,
    width: f32,
    height: f32,
    t: f32,
) -> Result<(), JsValue> {
    let mut bmp = Bitmap::new(width as u32, height as u32);

    let projection = perspective(f32::consts::PI / 2., width / height, 0., 100.);

    let camera = translate(0., 0., -5.);
    let view = camera.inv();
    let view_projection = view(projection);

    let model = Model {
        material: 0,
        mesh: cube().apply(rotate_y(t / 1000.)(rotate_x(t / 2000.))),
    };

    for trig in model.mesh.0.iter() {
        bmp.render_trig(
            *trig,
            view_projection,
            Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        );
    }

    // let x = width / 2.;
    // let y = height / 2.;

    // let point = from_screen(Matrix([[x, y]]), width, height)(view_projection.inv());

    // let mut point = Matrix([[0.5, 0.5]])(view_projection);
    // point.0[0][3] = 1.;

    ctx.put_image_data(&bmp.to_image_data(), 0., 0.)?;

    Ok(())
}
