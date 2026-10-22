use libvoronoi::{point::Point, pointset::PointSet};
use std::env;

fn main() {
    let mut ps = PointSet::new(20);

    println!(
        "\nVoronoi cli. Using libvoronoi version: {}",
        libvoronoi::version::VERSION
    );

    println!("Pointset (20):\n--------------");
    println!("{}", ps);

    ps.compute_perp_lines();

    println!(
        "Pointset has #{} points and #{} bisect lines.",
        ps.npoints(),
        ps.nbisect_lines()
    );

    println!("Check op[12]: ps[12] = {}", ps[12]);
}
