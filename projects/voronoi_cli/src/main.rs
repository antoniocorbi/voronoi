use libvoronoi::{point::Point, pointset::PointSet};
use std::env;

fn main() {
    let ps = PointSet::new(20);

    println!(
        "\nVoronoi cli. Using libvoronoi version: {}",
        libvoronoi::version::VERSION
    );

    println!("Pointset (20):\n--------------");
    println!("{}", ps);

    ps.compute_perp_lines();
}
