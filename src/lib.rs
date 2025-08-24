#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub mod matrix;
pub mod matrix_3d;
use core::f32;

use matrix::Matrix;

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::ImageData;

use crate::matrix_3d::{Model, Point2D, Triangle, perspective, quad, rotate_y, screen, translate};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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
    width: u32,
    height: u32,
    t: f32,
) -> Result<(), JsValue> {
    // let ctx = canvas
    //     .get_context("2d")?
    //     .unwrap()
    //     .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let mut bmp = Bitmap::new(width, height);

    let projection = perspective(
        f32::consts::PI / 2.,
        width as f32 / height as f32,
        1.,
        2000.,
    );

    let camera = translate(0., 0., 0.);

    let view_projection = camera.inv()(projection);

    let model = Model {
        material: 0,
        mesh: quad(),
    };

    for trig in model.mesh.0 {
        let p0 = trig.0(view_projection);
        let p1 = trig.1(view_projection);
        let p2 = trig.2(view_projection);

        let s0 = screen(p0, width, height);
        let s1 = screen(p1, width, height);
        let s2 = screen(p2, width, height);

        let min_x = s0.x().min(s1.x()).min(s2.x()) as usize;
        let max_x = s0.x().max(s1.x()).max(s2.x()) as usize;

        let min_y = s0.y().min(s1.y()).min(s2.y()) as usize;
        let max_y = s0.y().max(s1.y()).max(s2.y()) as usize;

        for x in min_x..max_x {
            for y in min_y..max_y {
                let p = Matrix([[x as f32 + 0.5, y as f32 + 0.5]]);
                if inside_triangle(s0, s1, s2, p) {
                    bmp.rows[y][x].r = 255;
                    bmp.rows[y][x].a = 255;
                }
            }
        }
    }

    ctx.put_image_data(&bmp.to_image_data(), 0., 0.)?;

    Ok(())
}
