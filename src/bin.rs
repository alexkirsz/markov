use markov::{kmeans, Vec3};
fn main() {
    let coords = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(0.0, -3.0, 0.0),
    ];

    let points = kmeans(&coords, 2);
    dbg!(points);
}
