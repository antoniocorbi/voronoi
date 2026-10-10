// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// -- Uses: ---------------------------------------------------------------
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn mid_point(&self, p: &Point) -> Self {
        Point::new((self.x + p.x) / 2.0, (self.y + p.y) / 2.0)
    }

    pub fn point_prect(&self, p: &Point, x: f32) -> Self {
        let mp = self.mid_point(p);
        let m2 = self.pslope(p);
        let b = mp.y - m2 * mp.x;
        // dbg!(m2);
        // dbg!(b);
        let y = m2 * x + b;
        // dbg!(y);
        let pp = Point::new(x, y);

        pp
    }

    pub fn slope(&self, p: &Point) -> f32 {
        let x = p.x - self.x;
        let y = p.y - self.y;
        // let m = y / x;

        if x.abs() < f32::EPSILON {
            if y.abs() < f32::EPSILON {
                f32::NAN // Puntos idénticos
            } else {
                f32::INFINITY // Línea vertical pura
            }
        } else {
            y / x
        }
    }

    pub fn pslope(&self, p: &Point) -> f32 {
        let m = self.slope(p);

        if m.is_nan() {
            f32::NAN
        } else if m.abs() < f32::EPSILON {
            f32::INFINITY // Si la pendiente es 0 (horizontal), la perpendicular es vertical (infinito)
        } else if m.is_infinite() {
            0.0 // Si la pendiente es infinito (vertical), la perpendicular es 0 (horizontal)
        } else {
            -1.0 / m // Nota: la pendiente perpendicular es el opuesto recíproco (-1/m)
        }
    }
}

// ==========================================
// SUMA (+ y +=)
// ==========================================
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// ==========================================
// RESTA (- y -=)
// ==========================================
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// ==========================================
// NEGACIÓN UNARIA (-Point)
// ==========================================
impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

// ==========================================
// MULTIPLICACIÓN POR ESCALAR (* y *=)
// ==========================================
// Point * f32
impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

// f32 * Point (Permite la propiedad conmutativa: 2.0 * p)
impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

// ==========================================
// DIVISIÓN POR ESCALAR (/ y /=)
// ==========================================
impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// ┌───────┐
// │ Tests │---------------------------------------------------------------
// └───────┘
#[cfg(test)]
mod tests {
    use super::*;

    // const EPSILON: f32 = 1e-6;

    /// Helper para comparar floats con tolerancia matemática
    fn approx_eq(a: f32, b: f32) -> bool {
        const EPSILON: f32 = 1e-6;
        (a - b).abs() < EPSILON
    }

    #[allow(dead_code)]
    fn point_approx_eq(p1: Point, p2: Point) -> bool {
        approx_eq(p1.x, p2.x) && approx_eq(p1.y, p2.y)
    }

    // --- SUMA ---
    #[test]
    fn test_add() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1 + p2, Point::new(4.0, 6.0));
    }

    #[test]
    fn test_add_assign() {
        let mut p1 = Point::new(1.0, 2.0);
        p1 += Point::new(3.0, 4.0);
        assert_eq!(p1, Point::new(4.0, 6.0));
    }

    // --- RESTA ---
    #[test]
    fn test_sub() {
        let p1 = Point::new(5.0, 7.0);
        let p2 = Point::new(2.0, 3.0);
        assert_eq!(p1 - p2, Point::new(3.0, 4.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut p1 = Point::new(5.0, 7.0);
        p1 -= Point::new(2.0, 3.0);
        assert_eq!(p1, Point::new(3.0, 4.0));
    }

    // --- NEGACIÓN ---
    #[test]
    fn test_neg() {
        let p = Point::new(3.0, -4.5);
        assert_eq!(-p, Point::new(-3.0, 4.5));
        assert_eq!(-(-p), p);
    }

    // --- MULTIPLICACIÓN POR ESCALAR ---
    #[test]
    fn test_mul_scalar() {
        let p = Point::new(2.0, -3.0);
        assert_eq!(p * 2.5, Point::new(5.0, -7.5));
        assert_eq!(2.5 * p, Point::new(5.0, -7.5)); // Test conmutatividad
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut p = Point::new(2.0, -3.0);
        p *= 2.0;
        assert_eq!(p, Point::new(4.0, -6.0));
    }

    #[test]
    fn test_mul_by_zero() {
        let p = Point::new(10.0, -20.0);
        assert_eq!(p * 0.0, Point::new(0.0, 0.0));
    }

    // --- DIVISIÓN POR ESCALAR ---
    #[test]
    fn test_div_scalar() {
        let p = Point::new(10.0, -20.0);
        assert_eq!(p / 2.0, Point::new(5.0, -10.0));
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut p = Point::new(10.0, -20.0);
        p /= 2.0;
        assert_eq!(p, Point::new(5.0, -10.0));
    }

    #[test]
    fn test_div_by_zero() {
        let p = Point::new(5.0, -5.0);
        let res = p / 0.0;

        assert_eq!(res.x, f32::INFINITY);
        assert_eq!(res.y, f32::NEG_INFINITY);
    }

    #[test]
    fn test_div_zero_by_zero() {
        let p = Point::new(0.0, 0.0);
        let res = p / 0.0;

        assert!(res.x.is_nan());
        assert!(res.y.is_nan());
    }

    #[test]
    fn test_new() {
        let p = Point::new(3.0, -4.5);
        assert!(approx_eq(p.x, 3.0));
        assert!(approx_eq(p.y, -4.5));
    }

    #[test]
    fn test_clone_and_copy() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = p1; // Probamos que implementa Copy/Clone correctamente
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_mid_point_standard() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(10.0, 20.0);
        let mid = p1.mid_point(&p2);

        assert!(approx_eq(mid.x, 5.0));
        assert!(approx_eq(mid.y, 10.0));
    }

    #[test]
    fn test_mid_point_negative_coordinates() {
        let p1 = Point::new(-5.0, -10.0);
        let p2 = Point::new(5.0, 10.0);
        let mid = p1.mid_point(&p2);

        assert!(approx_eq(mid.x, 0.0));
        assert!(approx_eq(mid.y, 0.0));
    }

    #[test]
    fn test_mid_point_same_point() {
        let p = Point::new(3.14, -2.71);
        let mid = p.mid_point(&p);

        assert!(approx_eq(mid.x, p.x));
        assert!(approx_eq(mid.y, p.y));
    }

    #[test]
    fn test_mid_point_extreme_values() {
        // Puntos muy grandes (verificar que no haya overflow inesperado en rango normal)
        let p1 = Point::new(f32::MAX / 4.0, f32::MIN / 4.0);
        let p2 = Point::new(f32::MAX / 4.0, f32::MIN / 4.0);
        let mid = p1.mid_point(&p2);

        assert!(approx_eq(mid.x, f32::MAX / 4.0));
        assert!(approx_eq(mid.y, f32::MIN / 4.0));
    }

    #[test]
    fn test_mid_point_handling_nan() {
        let p1 = Point::new(f32::NAN, 1.0);
        let p2 = Point::new(2.0, 2.0);
        let mid = p1.mid_point(&p2);

        // NaN operado con cualquier float resulta en NaN
        assert!(mid.x.is_nan());
        assert!(approx_eq(mid.y, 1.5));
    }

    #[test]
    fn test_mid_point_handling_infinity() {
        let p1 = Point::new(f32::INFINITY, f32::NEG_INFINITY);
        let p2 = Point::new(10.0, 10.0);
        let mid = p1.mid_point(&p2);

        assert_eq!(mid.x, f32::INFINITY);
        assert_eq!(mid.y, f32::NEG_INFINITY);
    }
}

#[cfg(test)]
mod slope_tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        const EPSILON: f32 = 1e-6;
        (a - b).abs() < EPSILON || (a.is_nan() && b.is_nan())
    }

    #[test]
    fn test_slope_video_yt() {
        let p1 = Point::new(2.0, 20.0);
        let p2 = Point::new(14.0, 24.0);
        let m1 = p1.slope(&p2);
        let m2 = -1.0 / m1;
        assert!(approx_eq(m1, 0.3333333));
        assert!(approx_eq(m2, -3.0));
    }

    #[test]
    fn test_slope_standard_positive() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(2.0, 4.0);
        let m = p1.slope(&p2);
        assert!(approx_eq(m, 2.0));
    }

    #[test]
    fn test_slope_standard_negative() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(3.0, -3.0);
        // y = -3 - 1 = -4, x = 3 - 1 = 2 -> -4 / 2 = -2.0
        let m = p1.slope(&p2);
        assert!(approx_eq(m, -2.0));
    }

    #[test]
    fn test_slope_horizontal_line() {
        let p1 = Point::new(1.0, 5.0);
        let p2 = Point::new(6.0, 5.0);
        // y = 0, x = 5 -> 0.0 / 5.0 = 0.0
        let m = p1.slope(&p2);
        assert!(approx_eq(m, 0.0));
    }

    #[test]
    fn test_slope_vertical_line() {
        let p1 = Point::new(3.0, 2.0);
        let p2 = Point::new(3.0, 10.0);
        // y = 8, x = 0 -> 8 / 0 = Infinity
        let m = p1.slope(&p2);
        assert_eq!(m, f32::INFINITY);
    }

    #[test]
    fn test_slope_same_point() {
        let p1 = Point::new(4.0, 4.0);
        let p2 = Point::new(4.0, 4.0);
        // y = 0, x = 0 -> 0 / 0 = NaN
        let m = p1.slope(&p2);
        assert!(m.is_nan());
    }

    #[test]
    fn test_pslope_standard() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(2.0, 4.0);
        // slope = 2.0 -> pslope = 1.0 / 2.0 = 0.5
        let ps = p1.pslope(&p2);
        dbg!(ps);
        assert!(approx_eq(ps, -0.5));
    }

    #[test]
    fn test_pslope_horizontal_line() {
        // Línea horizontal tiene slope = 0.0 -> pslope = 1.0 / 0.0 = Infinity
        let p1 = Point::new(0.0, 3.0);
        let p2 = Point::new(5.0, 3.0);
        let ps = p1.pslope(&p2);
        assert_eq!(ps, f32::INFINITY);
    }

    #[test]
    fn test_pslope_vertical_line() {
        // Línea vertical tiene slope = Infinity -> pslope = 1.0 / Inf = 0.0
        let p1 = Point::new(2.0, 0.0);
        let p2 = Point::new(2.0, 10.0);
        let ps = p1.pslope(&p2);
        assert!(approx_eq(ps, 0.0));
    }

    #[test]
    fn test_pslope_same_point() {
        // Misma posición da slope = NaN -> pslope = 1.0 / NaN = NaN
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(1.0, 1.0);
        let ps = p1.pslope(&p2);
        assert!(ps.is_nan());
    }
}

#[cfg(test)]
mod point_prect_tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        const EPSILON: f32 = 1e-6;
        (a - b).abs() < EPSILON || (a.is_nan() && b.is_nan())
    }

    #[test]
    fn test_point_prect_standard() {
        // Segmento de (0,0) a (4,4)
        // Punto medio = (2,2)
        // Pendiente del segmento = 1.0 -> pslope = -1.0
        // Ecuación de la mediatriz: y - 2 = -1 * (x - 2) => y = -x + 4
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(4.0, 4.0);

        // Evaluamos en x = 1.0 -> y = -1.0 + 4.0 = 3.0
        let res = p1.point_prect(&p2, 1.0);
        assert!(approx_eq(res.x, 1.0));
        assert!(approx_eq(res.y, 3.0));
    }

    #[test]
    fn test_point_prect_at_midpoint_x() {
        // Evaluar en la misma coordenada x del punto medio debe devolver la altura del punto medio
        let p1 = Point::new(2.0, 1.0);
        let p2 = Point::new(6.0, 5.0);
        // Punto medio = (4.0, 3.0)
        let mp_x = 4.0;

        let res = p1.point_prect(&p2, mp_x);
        assert!(approx_eq(res.x, 4.0));
        assert!(approx_eq(res.y, 3.0));
    }

    #[test]
    fn test_point_prect_horizontal_segment() {
        // Segmento horizontal: p1(0, 2) y p2(4, 2)
        // Punto medio = (2, 2). Slope = 0.0 -> pslope = Infinity.
        let p1 = Point::new(0.0, 2.0);
        let p2 = Point::new(4.0, 2.0);

        let res = p1.point_prect(&p2, 2.0);

        // dbg!(res);

        // Al involucrar infinitos en f32, verificamos el comportamiento esperado de la aritmética flotante
        assert!(approx_eq(res.x, 2.0));
        assert!(res.y.is_nan());
    }

    #[test]
    fn test_point_prect_vertical_segment() {
        // Segmento vertical: p1(3, 0) y p2(3, 4)
        // Punto medio = (3, 2). Slope = Inf -> pslope = 0.0.
        // La mediatriz es horizontal: y = 2.0
        let p1 = Point::new(3.0, 0.0);
        let p2 = Point::new(3.0, 4.0);

        // Evaluamos en cualquier x, por ejemplo x = 10.0 -> y debe ser 2.0
        let res = p1.point_prect(&p2, 10.0);
        assert!(approx_eq(res.x, 10.0));
        assert!(approx_eq(res.y, 2.0));
    }

    #[test]
    fn test_point_prect_same_point() {
        let p1 = Point::new(5.0, 5.0);
        let p2 = Point::new(5.0, 5.0);

        let res = p1.point_prect(&p2, 1.0);
        assert!(res.x.is_nan() || res.y.is_nan());
    }
}
