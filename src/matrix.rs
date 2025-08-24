use core::fmt;
use std::ops::{self, Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix<const H: usize, const W: usize>(pub [[f32; W]; H]);

impl<const H: usize, const W: usize> Index<usize> for Matrix<H, W> {
    type Output = [f32; W];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const H: usize, const W: usize> IndexMut<usize> for Matrix<H, W> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl<const H: usize, const W: usize> Default for Matrix<H, W> {
    fn default() -> Self {
        Matrix { 0: [[0.; W]; H] }
    }
}

macro_rules! impl_ops {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<const H: usize, const W: usize> std::ops::$trait<f32> for Matrix<H, W> {
            type Output = Matrix<H, W>;

            fn $method(self, rhs: f32) -> Self::Output {
                let mut output = Matrix::default();
                for y in 0..H {
                    for x in 0..W {
                        output[y][x] = self[y][x] $op rhs;
                    }
                }
                output
            }
        }

        impl<const H: usize, const W: usize> std::ops::$trait<Matrix<H, W>> for f32 {
            type Output = Matrix<H, W>;

            fn $method(self, rhs: Matrix<H, W>) -> Self::Output {
                let mut output = Matrix::default();
                for y in 0..H {
                    for x in 0..W {
                        output[y][x] = self $op rhs[y][x];
                    }
                }
                output
            }
        }


        impl<const H: usize, const W: usize> ops::$trait for Matrix<H, W> {
            type Output = Matrix<H, W>;
            fn $method(self, rhs: Self) -> Self::Output {
                let mut output: Matrix<H, W> = Matrix::default();

                for y in 0..H {
                    for x in 0..W {
                        output[y][x] = self[y][x] $op rhs[y][x];
                    }
                }

                output
            }
        }
    };
}
impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops!(Mul, mul, *);
impl_ops!(Div, div, /);

impl<const H: usize, const W: usize> ops::Neg for Matrix<H, W> {
    type Output = Matrix<H, W>;
    fn neg(self) -> Self::Output {
        let mut output: Matrix<H, W> = Matrix::default();

        for y in 0..H {
            for x in 0..W {
                output[y][x] = -self[y][x];
            }
        }

        output
    }
}

impl<const H: usize, const W: usize> Matrix<H, W> {
    pub fn transpose(self) -> Matrix<W, H> {
        let mut output: Matrix<W, H> = Matrix::default();

        for y in 0..H {
            for x in 0..W {
                output[x][y] = self[y][x];
            }
        }

        output
    }

    pub fn dot<const T: usize>(self, other: Matrix<W, T>) -> Matrix<H, T> {
        let mut output: Matrix<H, T> = Matrix::default();

        for y in 0..H {
            for x in 0..T {
                for i in 0..W {
                    output[y][x] += other[i][x] * self[y][i];
                }
            }
        }

        output
    }

    pub fn round(&self, digits: u32) -> Self {
        let mut copy = self.clone();
        let mult = (10 as u32).pow(digits) as f32;
        for y in 0..H {
            for x in 0..W {
                copy[y][x] = (self[y][x] * mult).round() / mult;
            }
        }
        return copy;
    }
}

impl<const H: usize, const W: usize, const T: usize> FnOnce<(Matrix<W, T>,)> for Matrix<H, W> {
    type Output = Matrix<H, T>;

    extern "rust-call" fn call_once(self, args: (Matrix<W, T>,)) -> Self::Output {
        self.dot(args.0)
    }
}

impl<const T: usize> Matrix<T, T> {
    pub fn identity() -> Self {
        let mut output: Self = Matrix::default();

        for i in 0..T {
            output[i][i] = 1.0;
        }

        output
    }
}

impl Matrix<1, 1> {
    pub fn inv(self) -> Self {
        todo!();
    }

    pub fn det(self) -> f32 {
        self[0][0]
    }
}

impl Matrix<2, 2> {
    pub fn inv(self) -> Self {
        todo!();
    }

    pub fn det(self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix<3, 3> {
    pub fn inv(self) -> Self {
        let mut determinant: f32 = 0.;

        let mut minor_dets: Matrix<3, 3> = Matrix::default();

        for y in 0..3 {
            for x in 0..3 {
                let minor = self.minor(y, x);

                let det = minor.det();

                if y == 0 {
                    if x & 1 == 0 {
                        determinant += self[y][x] * det;
                    } else {
                        determinant -= self[y][x] * det;
                    }
                }

                if (x + y) & 1 == 0 {
                    minor_dets[y][x] = det;
                } else {
                    minor_dets[y][x] = -det;
                }
            }
        }

        minor_dets.transpose() / determinant
    }

    pub fn minor(&self, target_y: usize, target_x: usize) -> Matrix<2, 2> {
        let mut output: Matrix<2, 2> = Matrix::default();

        for y in 0..3 {
            if y == target_y {
                continue;
            }
            for x in 0..3 {
                if x == target_x {
                    continue;
                }

                let mut output_y = y;
                if y > target_y {
                    output_y -= 1;
                }
                let mut output_x = x;
                if x > target_x {
                    output_x -= 1;
                }

                output[output_y][output_x] = self[y][x];
            }
        }

        output
    }

    pub fn det(self) -> f32 {
        let a = self.minor(0, 0).det() * self[0][0];
        let b = self.minor(0, 1).det() * self[0][1];
        let c = self.minor(0, 2).det() * self[0][2];

        a - b + c
    }
}

impl Matrix<4, 4> {
    pub fn inv(self) -> Self {
        let mut determinant: f32 = 0.;

        let mut minor_dets: Matrix<4, 4> = Matrix::default();

        for y in 0..4 {
            for x in 0..4 {
                let minor = self.minor(y, x);

                let det = minor.det();

                if y == 0 {
                    if x & 1 == 0 {
                        determinant += self[y][x] * det;
                    } else {
                        determinant -= self[y][x] * det;
                    }
                }

                if (x + y) & 1 == 0 {
                    minor_dets[y][x] = det;
                } else {
                    minor_dets[y][x] = -det;
                }
            }
        }

        minor_dets.transpose() / determinant
    }
    pub fn minor(&self, target_y: usize, target_x: usize) -> Matrix<3, 3> {
        let mut output: Matrix<3, 3> = Matrix::default();

        for y in 0..4 {
            if y == target_y {
                continue;
            }
            for x in 0..4 {
                if x == target_x {
                    continue;
                }

                let mut output_y = y;
                if y > target_y {
                    output_y -= 1;
                }
                let mut output_x = x;
                if x > target_x {
                    output_x -= 1;
                }

                output[output_y][output_x] = self[y][x];
            }
        }

        output
    }

    pub fn det(self) -> f32 {
        let a = self.minor(0, 0).det() * self[0][0];
        let b = self.minor(0, 1).det() * self[0][1];
        let c = self.minor(0, 2).det() * self[0][2];
        let d = self.minor(0, 3).det() * self[0][3];

        a - b + c - d
    }
}

impl<const H: usize, const W: usize> fmt::Display for Matrix<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();
        string += "[";
        for y in 0..H {
            if y != 0 {
                string += " ";
            }
            string += "[";
            for x in 0..W {
                string += format!("{:>5.1}", self[y][x]).as_str();
                if x != W - 1 {
                    string += ", ";
                }
            }
            string += "]";
            if y != H - 1 {
                string += "\n";
            }
        }
        string += "]";

        write!(f, "{}", string)
    }
}

impl<const H: usize, const W: usize> Matrix<H, W> {
    pub fn x(&self) -> f32 {
        self.0[0][0]
    }
    pub fn y(&self) -> f32 {
        self.0[0][1]
    }
    pub fn z(&self) -> f32 {
        self.0[0][2]
    }
    pub fn w(&self) -> f32 {
        self.0[0][3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let x = Matrix([[1., 2.], [0., 1.], [2., 3.]]);
        let y = Matrix([[2., 5., 1., 1.], [6., 7., 1., 1.]]);
        let z = Matrix([
            [14.0, 19.0, 3.0, 3.0],
            [6.0, 7.0, 1.0, 1.0],
            [22.0, 31.0, 5.0, 5.0],
        ]);

        assert_eq!(x(y), z);

        assert_eq!(x(y).transpose(), y.transpose()(x.transpose())); // (AB)^T == B^T * A^T
    }

    #[test]
    fn test_smoke_math() {
        let x = Matrix([[1., 2.], [1., 1.], [2., 3.]]);
        let y = Matrix([[3., 1.], [0., 2.], [5., 9.]]);
        let z = Matrix([[0., 2.], [3., 1.], [5., 9.]]);
        let one = Matrix([[1., 1.], [1., 1.], [1., 1.]]);
        let zero = Matrix([[0., 0.], [0., 0.], [0., 0.]]);

        assert_eq!(x, x);
        assert_eq!(x + zero, x);
        assert_eq!(x - x, zero);
        assert_eq!(x / x, one);
        assert_eq!(x + zero, x); // additive identity
        assert_eq!(zero + x, x); // commutativity of addition with zero
        assert_eq!(x - x, zero); // subtraction to zero
        assert_eq!(x * one, x); // multiplicative identity
        assert_eq!(one * x, x); // left multiplicative identity
        assert_eq!(x + y, y + x); // addition is commutative
        assert_eq!(x * y, y * x); // only if you know it should be true
        assert_eq!((x + y) + one, x + (y + one)); // addition is associative
        assert_eq!((x * y) * z, x * (y * z)); // again, only valid for some matrices
        assert_eq!(x + (-x), zero); // additive inverse
        assert_eq!(-(-x), x); // double negation
        assert_eq!(x.transpose().transpose(), x); // transpose is involutive
        assert_eq!(x * (y + one), x * y + x * one); // left distributive
        assert_eq!((y + one) * x, y * x + one * x); // right distributive
    }

    #[test]
    fn test_identity() {
        let x = Matrix([[1., 2.], [1., 1.]]);
        let identity: Matrix<2, 2> = Matrix::identity();
        assert_eq!(x(identity), x);
    }
    #[test]
    fn test_minor() {
        let x = Matrix([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);

        assert_eq!(x.minor(0, 0), Matrix([[5., 6.], [8., 9.]]));
        assert_eq!(x.minor(1, 1), Matrix([[1., 3.], [7., 9.]]));
    }

    #[test]
    fn test_determinant() {
        let x = Matrix([[3., 8.], [4., 6.]]);
        assert_eq!(x.det(), -14.);

        let y = Matrix([[6., 1., 1.], [4., -2., 5.], [2., 8., 7.]]);
        assert_eq!(y.det(), -306.);

        let z = Matrix([
            [1., 3., 5., 9.],
            [1., 3., 1., 7.],
            [4., 3., 9., 7.],
            [5., 2., 0., 9.],
        ]);
        assert_eq!(z.det(), -376.);
    }
    #[test]
    fn test_inverse() {
        let x = Matrix([[3., 0., 2.], [2., 0., -2.], [0., 1., 1.]]);

        assert_eq!(
            x.inv(),
            Matrix([[0.2, 0.2, 0.], [-0.2, 0.3, 1.], [0.2, -0.3, 0.]])
        );

        let y = Matrix([
            [1., 0., 4., -6.],
            [2., 5., 0., 3.],
            [-1., 2., 3., 5.],
            [2., 1., -2., 3.],
        ]);

        assert_eq!(((y.inv()(y)).round(6)), Matrix::<4, 4>::identity());
    }
}
