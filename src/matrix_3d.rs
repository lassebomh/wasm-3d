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

pub fn screen(pos: Matrix<1, 4>, screen_width: u32, screen_height: u32) -> Matrix<1, 2> {
    return Matrix([[
        ((pos.x() / pos.z() + 1.) * (screen_width as f32)) / 2.,
        ((1. - pos.y() / pos.z()) * (screen_height as f32)) / 2.,
    ]]);
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
    pub material: u32,
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
