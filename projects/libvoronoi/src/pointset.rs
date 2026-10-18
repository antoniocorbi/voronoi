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
use crate::point::Point;
use std::fmt;

// -- Data Types: ---------------------------------------------------------
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PointSet {
    points: Vec<Point>,
}

// -- Implementations: ----------------------------------------------------
impl fmt::Display for PointSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.points {
            let _ = writeln!(f, "{}", p);
        }

        Ok(())
    }
}

impl PointSet {
    pub fn new(n: usize) -> Self {
        // let points: Vec<_> = vec![Point::random(); n];
        let points: Vec<_> = (0..n).map(|_| Point::random_01()).collect();

        // dbg!(points.clone());

        PointSet { points }
    }

    pub fn compute_perp_lines(&self) {
        if self.points.len() > 0 {
            let npoints = self.points.len() - 1;
            for i in 0..npoints {
                let p1 = &self.points[i];
                let next_i = i + 1;
                for j in next_i..self.points.len() {
                    let p2 = &self.points[j];
                    let bline = p1.bisector_line(p2);

                    println!("p1[{}] -- {} -- p2[{}]", p1, bline, p2);
                }
            }
        }
    }
}

// -- Tests: --------------------------------------------------------------
#[cfg(test)]
mod point_set_tests {
    use super::*;

    #[test]
    fn test_point_set_new_empty() {
        let ps = PointSet::new(0);
        assert_eq!(ps.points.len(), 0);
        assert!(ps.points.is_empty());
    }

    #[test]
    fn test_point_set_new_standard_size() {
        let n = 50;
        let ps = PointSet::new(n);

        // Verifica que la cantidad de puntos coincida exactamente con n
        assert_eq!(ps.points.len(), n);
    }

    #[test]
    fn test_point_set_random_coordinates_bounds() {
        // Generamos un conjunto grande para validar los límites de la aleatoriedad
        let n = 100;
        let ps = PointSet::new(n);

        for point in &ps.points {
            // Asumiendo que Point::random() genera valores entre 0.0 y 1.0 (inclusivo)
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

            // Verificamos que no se generen valores NaN por error en el generador
            assert!(!point.x.is_nan());
            assert!(!point.y.is_nan());
        }
    }

    #[test]
    fn test_point_set_distinct_elements() {
        // Con un número razonable de puntos aleatorios, es altamente improbable
        // que todos sean exactamente idénticos (salvo colisiones de punto flotante extremas).
        let ps = PointSet::new(10);

        if ps.points.len() > 1 {
            let first = ps.points[0];
            let all_same = ps.points.iter().all(|p| p.x == first.x && p.y == first.y);
            assert!(!all_same, "Todos los puntos aleatorios generados son idénticos, posible fallo en la aleatoriedad.");
        }
    }
}
