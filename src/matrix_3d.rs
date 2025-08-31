use core::f32;

use crate::matrix::Matrix;

pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix<4, 4> {
    let f = f32::tan(f32::consts::PI * 0.5 - 0.5 * fov);
    let range_inv = 1.0 / (near - far);

    return Matrix([
        [f / aspect, 0., 0., 0.],
        [0., f, 0., 0.],
        [0., 0., (near + far) * range_inv, -1.],
        [0., 0., near * far * range_inv * 2., 0.],
    ]);
}

pub fn rotate_x(radians: f32) -> Matrix<4, 4> {
    let mut out = Matrix::<4, 4>::identity();
    out[1][1] = f32::cos(radians);
    out[1][2] = -f32::sin(radians);
    out[2][1] = f32::sin(radians);
    out[2][2] = f32::cos(radians);
    return out;
}

pub fn rotate_y(radians: f32) -> Matrix<4, 4> {
    let mut out = Matrix::<4, 4>::identity();
    out[0][0] = f32::cos(radians);
    out[0][2] = f32::sin(radians);
    out[2][0] = -f32::sin(radians);
    out[2][2] = f32::cos(radians);
    return out;
}

pub fn rotate_z(radians: f32) -> Matrix<4, 4> {
    let mut out = Matrix::<4, 4>::identity();
    out[0][0] = f32::cos(radians);
    out[0][1] = -f32::sin(radians);
    out[1][0] = f32::sin(radians);
    out[1][1] = f32::cos(radians);
    return out;
}

pub fn translate(x: f32, y: f32, z: f32) -> Matrix<4, 4> {
    return Matrix([
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [x, y, z, 1.],
    ]);
}

pub fn scale(x: f32, y: f32, z: f32) -> Matrix<4, 4> {
    return Matrix([
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.],
    ]);
}

pub fn screen(pos: Matrix<1, 4>, screen_width: f32, screen_height: f32) -> Matrix<1, 2> {
    return Matrix([[
        ((pos.x() / pos.z() + 1.) * (screen_width)) / 2.,
        ((1. - pos.y() / pos.z()) * (screen_height)) / 2.,
    ]]);
}
pub fn from_screen(
    screen_pos: Matrix<1, 2>,
    screen_width: f32,
    screen_height: f32,
) -> Matrix<1, 4> {
    let screen_x = screen_pos.x();
    let screen_y = screen_pos.y();
    let width = screen_width;
    let height = screen_height;

    let x = (screen_x * 2.0 / width) - 1.0;
    let y = 1.0 - (screen_y * 2.0 / height);
    let z = 0.0;
    let w = 1.0;

    Matrix([[x, y, z, w]])
}

pub type Point = Matrix<1, 4>;
pub type Point2D = Matrix<1, 2>;

#[derive(Clone, Copy)]
pub struct Triangle(pub Point, pub Point, pub Point);

pub struct Mesh(pub Vec<Triangle>);

impl Mesh {
    pub fn apply(&self, mat: Matrix<4, 4>) -> Self {
        let mut array: Vec<Triangle> = Vec::new();

        for trig in self.0.iter() {
            let mut new_trig = trig.clone();
            new_trig.0 = new_trig.0(mat);
            new_trig.1 = new_trig.1(mat);
            new_trig.2 = new_trig.2(mat);
            array.push(new_trig);
        }

        Mesh(array)
    }

    pub fn join(&mut self, other: Mesh) {
        self.0.append(&mut other.0.clone());
    }
}

pub struct Model {
    pub mesh: Mesh,
    pub color: Matrix<1, 4>,
    pub reflect: f32,
}

pub fn quad() -> Mesh {
    Mesh(vec![
        Triangle(
            Matrix([[0., 0., 0., 1.]]),
            Matrix([[1., 0., 0., 1.]]),
            Matrix([[0., 1., 0., 1.]]),
        ),
        Triangle(
            Matrix([[1., 1., 0., 1.]]),
            Matrix([[0., 1., 0., 1.]]),
            Matrix([[1., 0., 0., 1.]]),
        ),
    ])
}

pub fn cube() -> Mesh {
    let mut quad = quad().apply(translate(-0.5, -0.5, -0.5));
    quad.join(quad.apply(rotate_y(f32::consts::PI)));

    let mut mesh = Mesh(Vec::new());

    mesh.join(quad.apply(rotate_y(0.)));
    mesh.join(quad.apply(rotate_y(f32::consts::PI / 2.)));
    mesh.join(quad.apply(rotate_x(f32::consts::PI / 2.)));

    // mesh.join(quad.apply(rotate_y(0.)));
    // mesh.join(quad.apply(rotate_y(f32::consts::PI / 2.)));
    // mesh.join(quad.apply(rotate_y(f32::consts::PI)));
    // mesh.join(quad.apply(rotate_y(f32::consts::PI / 2. * 3.)));

    mesh
}

pub struct RaycastHit {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub normal: Matrix<1, 4>,
}

pub fn ray_intersects_triangle(
    origin: Matrix<1, 4>,
    direction: Matrix<1, 4>,
    trig: Triangle,
) -> Option<RaycastHit> {
    let epsilon = 1e-8;
    let origin = Matrix([[origin.x(), origin.y(), origin.z()]]);
    let direction = Matrix([[direction.x(), direction.y(), direction.z()]]);

    let p0 = Matrix([[trig.0.x(), trig.0.y(), trig.0.z()]]);
    let p1 = Matrix([[trig.1.x(), trig.1.y(), trig.1.z()]]);
    let p2 = Matrix([[trig.2.x(), trig.2.y(), trig.2.z()]]);

    let edge1 = p1 - p0;
    let edge2 = p2 - p0;

    let h = direction.cross(edge2);
    let a = edge1.dot(h.transpose()).x();

    if a.abs() < epsilon {
        return None;
    }

    let f = 1.0 / a;
    let s = origin - p0;
    let u = f * s.dot(h.transpose()).x();

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = s.cross(edge1);

    let v = f * direction.dot(q.transpose()).x();

    if v < 0.0 || v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(q.transpose()).x();

    if t > epsilon {
        let normal = edge1.cross(edge2).normalize();

        return Some(RaycastHit {
            t: t,
            u: u,
            v: v,
            normal: Matrix([[normal.x(), normal.y(), normal.z(), 0.]]),
        });
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap::Bitmap;

    use super::*;

    #[test]
    fn test_raycast() {
        let ray_origin = Matrix([[0., 0., 0., 1.]]);
        let ray_direction = Matrix([[0., 0.2, 1., 0.]]);

        let triangle = Triangle(
            Matrix([[-1., -1., 5., 1.]]),
            Matrix([[1., -1.0, 5., 1.]]),
            Matrix([[0.0, 1., 5., 1.]]),
        );
    }

    #[test]
    fn test_render() {
        let width: f32 = 100.;
        let height: f32 = 100.;

        let mut bmp = Bitmap::new(width as u32, height as u32);

        let aspect_ratio = width / height;
        let fov = f32::consts::PI / 2.;

        let camera = translate(0., 0., -5.);

        let background_color = Matrix([[0., 0., 0., 1.]]);
        let light_position = Matrix([[0., -5., 0., 1.]]);
        let light_direction = Matrix([[0., -1., -0.1, 0.]]).normalize();

        let t = 0.;

        let models: Vec<Model> = vec![
            Model {
                color: Matrix([[1., 0., 0., 1.]]),
                reflect: 0.3,
                mesh: cube().apply(translate(3., 0., 0.)),
            },
            Model {
                color: Matrix([[0., 1., 0., 1.]]),
                reflect: 0.3,
                mesh: cube().apply(rotate_y(t / 1000.)(rotate_x(t / 2000.))),
            },
            Model {
                color: Matrix([[0., 0., 1., 1.]]),
                reflect: 0.3,
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
                            let out = model.color * model.reflect;

                            if out.w() < 1. && depth < 2 {
                                let dot = direction.dot(hit.normal.transpose()).x();
                                let direction_reflected = direction - hit.normal * (2.0 * dot);
                                let origin_reflected = origin + direction * hit.t;

                                out + (1. - out.w())
                                    * raycast_color(
                                        origin_reflected,
                                        direction_reflected,
                                        background_color,
                                        models,
                                        depth + 1,
                                    )
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
    }
}
