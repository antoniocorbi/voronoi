// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>

// -- Uses: ---------------------------------------------------------------
use crate::point::{Line, Point};
use std::fmt;
use std::ops::Index;

// -- Data Types: ---------------------------------------------------------
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Voronoi {
    pub points: Vec<Point>,
    pub bisects: Vec<Line>,
}

// -- Implementations: ----------------------------------------------------
impl fmt::Display for Voronoi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.points {
            let _ = writeln!(f, "{}", p);
        }

        Ok(())
    }
}

impl Index<usize> for Voronoi {
    type Output = Point;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.points[idx]
    }
}

impl Voronoi {
    pub fn new(n: usize) -> Self {
        // let points: Vec<_> = vec![Point::random(); n];
        let points: Vec<_> = (0..n).map(|_| Point::random_01()).collect();

        // dbg!(points.clone());

        Voronoi {
            points,
            bisects: vec![],
        }
    }

    pub fn resize(&mut self, n: usize) {
        let points: Vec<_> = (0..n).map(|_| Point::random_01()).collect();

        self.points = points;
        self.bisects.clear();
        self.bisects.shrink_to_fit();
    }

    pub fn compute_perp_lines(&mut self) {
        let mut blines: Vec<Line> = vec![];
        let mut lc = 0;

        if self.points.len() > 0 {
            let npoints = self.points.len() - 1;
            for i in 0..npoints {
                let p1 = &self.points[i];
                let next_i = i + 1;
                for j in next_i..self.points.len() {
                    let p2 = &self.points[j];
                    let bline = p1.bisector_line(p2);

                    blines.push(bline);
                    lc += 1;

                    // println!("{lc:03}: p{i:02}[{}] -- {} -- p{j:02}[{}]", p1, bline, p2);
                    // for px in -1..=1 {
                    //     let x: f32 = px as f32;
                    //     println!("\t f({x:.2}) = {}", bline.f(x));
                    // }
                }
            }
            self.bisects = blines;
        }
    }

    pub fn npoints(&self) -> usize {
        self.points.len()
    }

    pub fn nbisect_lines(&self) -> usize {
        self.bisects.len()
    }
}

// -- Tests: --------------------------------------------------------------
#[cfg(test)]
mod voronoi_tests {
    use super::*;

    #[test]
    fn test_voronoi_distinct_elements() {
        // Con un número razonable de puntos aleatorios, es altamente improbable
        // que todos sean exactamente idénticos (salvo colisiones de punto flotante extremas).
        let ps = Voronoi::new(10);

        if ps.points.len() > 1 {
            let first = ps.points[0];
            let all_same = ps.points.iter().all(|p| p.x == first.x && p.y == first.y);
            assert!(!all_same, "Todos los puntos aleatorios generados son idénticos, posible fallo en la aleatoriedad.");
        }
    }

    // --- Tests para el constructor new() y propiedades de Voronoi ---

    fn approx_eq(a: f32, b: f32) -> bool {
        const EPSILON: f32 = 1e-6;
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_voronoi_new_empty() {
        let ps = Voronoi::new(0);
        assert_eq!(ps.points.len(), 0);
        assert!(ps.points.is_empty());
        assert!(ps.bisects.is_empty());
    }

    #[test]
    fn test_voronoi_new_standard_size() {
        let n = 50;
        let ps = Voronoi::new(n);

        // Verifica que la cantidad de puntos coincida exactamente con n
        assert_eq!(ps.points.len(), n);
        assert!(ps.bisects.is_empty());
    }

    #[test]
    fn test_voronoi_random_coordinates_bounds() {
        let n = 100;
        let ps = Voronoi::new(n);

        for point in &ps.points {
            // Verifica que Point::random_01() genere valores dentro de [0.0, 1.0]
            assert!(
                point.x >= 0.0 && point.x <= 1.0,
                "Coordenada x fuera de rango: {}",
                point.x
            );
            assert!(
                point.y >= 0.0 && point.y <= 1.0,
                "Coordenada y fuera de rango: {}",
                point.y
            );

            assert!(!point.x.is_nan());
            assert!(!point.y.is_nan());
        }
    }

    // --- Tests para la implementación de Index<usize> ---

    #[test]
    fn test_voronoi_index_valid() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let p3 = Point::new(5.0, 6.0);

        // Usamos Default para rellenar automáticamente bisects como vec![]
        let ps = Voronoi {
            points: vec![p1, p2, p3],
            ..Default::default()
        };

        assert_eq!(ps[0], p1);
        assert_eq!(ps[1], p2);
        assert_eq!(ps[2], p3);
    }

    #[test]
    fn test_voronoi_index_reference_type() {
        let p = Point::new(10.0, 20.0);
        let ps = Voronoi {
            points: vec![p],
            bisects: vec![],
        };

        let pt_ref: &Point = &ps[0];
        assert_eq!(pt_ref.x, 10.0);
        assert_eq!(pt_ref.y, 20.0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_voronoi_index_out_of_bounds() {
        let ps = Voronoi {
            points: vec![Point::new(0.0, 0.0)],
            ..Default::default()
        };

        // Debe disparar un panic controlado por el slice interno
        let _ = ps[5];
    }
}
