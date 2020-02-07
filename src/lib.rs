mod gibbs;
mod kmeans;
mod vec3;

pub use gibbs::{CliqueType, MK};
pub use kmeans::{kmeans, random_barycenters};
pub use vec3::Vec3;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "web")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn process_img(rgb: &image::RgbImage) -> image::RgbImage {
    use permutohedron::heap_recursive;

    let (width, height) = rgb.dimensions();

    let values: Vec<_> = rgb
        .pixels()
        .map(|v| {
            Vec3::new(
                v[0] as f32 / 255.0,
                v[1] as f32 / 255.0,
                v[2] as f32 / 255.0,
            )
        })
        .collect();

    println!("K-Means");

    let ref_colors: &[Vec3] = &[
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., 1.),
        Vec3::new(0., 1., 0.),
        Vec3::new(0., 1., 1.),
        Vec3::new(1., 0., 0.),
        Vec3::new(1., 0., 1.),
        Vec3::new(1., 1., 0.),
        Vec3::new(1., 1., 1.),
    ];

    let (points, barycenters) = kmeans(&values, &random_barycenters(&values, 8), 0.001);
    // let (points, barycenters) = kmeans(&values, &ref_colors, 0.001);

    println!("K-Means done, kept {} clusters", barycenters.len());

    let mk = MK {
        width: width as usize,
        height: height as usize,
        num_classes: barycenters.len(),

        dist: points
            .iter()
            .map(|p| p.coords)
            .map(|p| barycenters.iter().map(|b| (b - p).norm()).collect())
            .collect(),
        x: points.iter().map(|p| p.cluster).collect(),
    };

    let mk_res = mk.simulated_annealing(10, CliqueType::Conn8, 1.0, 1.0);
    println!("MK done");

    let mut barycenters_map: Vec<_> = (0..barycenters.len()).collect();

    // let ref_colors_lab: Vec<_> = ref_colors.iter().map(|c| Lab::from(Srgb::new(c.x, c.y, c.z))).collect();
    // let barycenters_lab: Vec<_> = barycenters.iter().map(|c| Lab::from(Srgb::new(c.x, c.y, c.z))).collect();
    let color_map = {
        let mut best_permutation: Option<Vec<usize>> = None;
        let mut best_err = 0.0f32;

        heap_recursive(&mut barycenters_map, |permutation| {
            let err = permutation
                .iter()
                .enumerate()
                .map(|(i, mi)| (ref_colors[i] - barycenters[*mi]).norm())
                .sum();
            if best_permutation.is_none() || err < best_err {
                println!("found new err: {}", err);
                best_permutation = Some(permutation.to_vec());
                best_err = err;
            }
        });
        best_permutation.unwrap()
    };

    let out = image::RgbImage::from_fn(width, height, |x, y| {
        // let vec = &points[(y * width + x) as usize];
        // let clr = barycenters[vec.cluster];
        let cluster = mk_res[(y * width + x) as usize];
        let clr = ref_colors[color_map[cluster]];
        // let clr = barycenters[cluster];
        image::Rgb([
            (clr.x * 255.0).round() as u8,
            (clr.y * 255.0).round() as u8,
            (clr.z * 255.0).round() as u8,
        ])
    });

    out
}
