#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub mod bitmap;
pub mod matrix;
pub mod matrix_3d;
use core::{f32, panic};

use matrix::Matrix;

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::ImageData;

use crate::{
    bitmap::Bitmap,
    matrix_3d::{
        Model, Point2D, RaycastHit, Triangle, cube, from_screen, perspective, quad,
        ray_intersects_triangle, rotate_x, rotate_y, scale, screen, translate,
    },
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
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

    let aspect_ratio = width / height;
    let fov = f32::consts::PI / 2.;

    let camera = translate(0., 0., -5.);

    let background_color = Matrix([[0., 0., 0., 1.]]);
    let light_position = Matrix([[0., -5., 0., 1.]]);
    let light_direction = Matrix([[0., -1., -0.1, 0.]]).normalize();

    let models: Vec<Model> = vec![
        Model {
            color: Matrix([[1., 0., 0., 1.]]),
            reflect: 0.5,
            mesh: cube().apply(translate(3., 0., 0.)),
        },
        Model {
            color: Matrix([[0., 1., 0., 1.]]),
            reflect: 0.5,
            mesh: cube().apply(rotate_y(t / 10000.)),
        },
        Model {
            color: Matrix([[0., 0., 1., 1.]]),
            reflect: 0.5,
            mesh: cube().apply(translate(-3., 0., 0.)),
        },
    ];

    for screen_x in 0..(width as usize) {
        let x = screen_x as f32;
        for screen_y in 0..(height as usize) {
            let y = screen_y as f32;
            let forward = Matrix([[0., 0., 1., 0.]]);

            let pitch = ((y / height) - 0.5) * fov;
            let yaw = ((x / width) - 0.5) * fov * aspect_ratio;

            let origin = Matrix([[0., 0., 0., 1.]])(camera);
            let direction = forward(rotate_y(yaw)(rotate_x(pitch)));

            fn raycast_color(
                origin: Matrix<1, 4>,
                direction: Matrix<1, 4>,
                background_color: Matrix<1, 4>,
                models: &Vec<Model>,
                depth: u32,
            ) -> Matrix<1, 4> {
                let mut hits: Vec<(RaycastHit, &Model)> = Vec::new();

                for model in models.iter() {
                    for trig in model.mesh.0.iter() {
                        match ray_intersects_triangle(origin, direction, *trig) {
                            Some(hit) => {
                                hits.push((hit, model));
                            }
                            None => {}
                        }
                    }
                }

                let nearest = hits
                    .iter()
                    .min_by(|a, b| a.0.t.partial_cmp(&b.0.t).unwrap());

                match nearest {
                    Some((hit, model)) => {
                        let mut out = model.color * model.reflect;

                        if out.w() < 1. && depth < 2 {
                            let dot = direction.dot(hit.normal.transpose()).x();
                            let direction_reflected = direction - hit.normal * (2.0 * dot);

                            let origin_reflected = origin + direction * hit.t - hit.normal / 1000.0;

                            let other = raycast_color(
                                origin_reflected,
                                direction_reflected,
                                background_color,
                                models,
                                depth + 1,
                            );

                            out = out + other;

                            // console_log!("{}", out);
                            // console_log!("{}", other);

                            // panic!();

                            out
                        } else {
                            out
                        }
                    }
                    None => background_color,
                }
            }

            bmp.rows[screen_y][screen_x] =
                raycast_color(origin, direction, background_color, &models, 0).to_color();
        }
    }

    ctx.put_image_data(&bmp.to_image_data(), 0., 0.)?;

    Ok(())
}

// #[wasm_bindgen]
// pub fn render(
//     ctx: web_sys::CanvasRenderingContext2d,
//     width: f32,
//     height: f32,
//     t: f32,
// ) -> Result<(), JsValue> {
//     let mut bmp = Bitmap::new(width as u32, height as u32);

//     let projection = perspective(f32::consts::PI / 2., width / height, 0., 100.);

//     let camera = translate(0., 0., -5.);
//     let view = camera.inv();
//     let view_projection = view(projection);

//     let model = Model {
//         material: 0,
//         mesh: cube().apply(rotate_y(t / 1000.)(rotate_x(t / 2000.))),
//     };

//     for trig in model.mesh.0.iter() {
//         bmp.render_trig(
//             *trig,
//             view_projection,
//             Color {
//                 r: 255,
//                 g: 0,
//                 b: 0,
//                 a: 255,
//             },
//         );
//     }

//     // let x = width / 2.;
//     // let y = height / 2.;

//     // let point = from_screen(Matrix([[x, y]]), width, height)(view_projection.inv());

//     // let mut point = Matrix([[0.5, 0.5]])(view_projection);
//     // point.0[0][3] = 1.;

//     ctx.put_image_data(&bmp.to_image_data(), 0., 0.)?;

//     Ok(())
// }
