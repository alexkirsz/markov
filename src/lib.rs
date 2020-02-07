mod gibbs;
mod kmeans;
mod vec3;

use palette::{Srgb,Lab};

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

pub fn process_img(
    rgb: &image::RgbImage,
    n: usize,
    clique_type: CliqueType,
    beta: f32,
    t_init: f32,
) -> image::RgbImage {
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

    // let (points, barycenters) = kmeans(&values, &random_barycenters(&values, 8), 0.001);
    let (points, barycenters) = kmeans(&values, &ref_colors, 0.001);


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

    let mk_res = mk.simulated_annealing(n, clique_type, beta, t_init);
    println!("MK done");


    /* translate the ref colors and barycenters to lab */
    let ref_colors_lab: Vec<_> = ref_colors.iter().map(|c| Vec3::from(Lab::from(Srgb::new(c.x, c.y, c.z)))).collect();
    let barycenters_lab: Vec<_> = barycenters.iter().map(|c| Vec3::from(Lab::from(Srgb::new(c.x, c.y, c.z)))).collect();

    /* find the best permutation of ref colors, considering there are 8 or less barycenters and
    ** 8 reference colors. we then use the reference color to barycenter mapping to get the opposite relation.
    */
    let color_map = {
        let mut best_permutation: Option<Vec<usize>> = None;
        let mut best_err = 0.0f32;
        /* build the vector to create permutations of */
        let mut ref_colors_map: Vec<_> = (0..ref_colors.len()).collect();
        heap_recursive(&mut ref_colors_map, |ref_permutation| {
            let err = ref_permutation.iter()
                .map(|i| ref_colors_lab[*i])
                .zip(barycenters_lab.iter())
                .map(|(a, b)| (a - b).norm())
                .sum();
            if best_permutation.is_none() || err < best_err {
                println!("found new err: {}", err);
                best_permutation = Some(ref_permutation.to_vec());
                best_err = err;
            }
        });
        let best_permutation = best_permutation.unwrap();
        dbg!(&best_permutation);
        best_permutation
    };

    let out = image::RgbImage::from_fn(width, height, |x, y| {
        let cluster = mk_res[(y * width + x) as usize];
        let clr = ref_colors[color_map[cluster]];
        image::Rgb([
            (clr.x * 255.0).round() as u8,
            (clr.y * 255.0).round() as u8,
            (clr.z * 255.0).round() as u8,
        ])
    });

    out
}

#[cfg(feature = "web")]
#[wasm_bindgen]
pub struct WasmMK {
    img: image::DynamicImage,
}

#[cfg(feature = "web")]
#[wasm_bindgen]
impl WasmMK {
    #[wasm_bindgen]
    pub fn new(img: &[u8]) -> Result<WasmMK, JsValue> {
        match image::load_from_memory(img) {
            Ok(img) => Ok(WasmMK { img }),
            Err(e) => Err(js_sys::Error::new(&e.to_string()).into()),
        }
    }

    #[wasm_bindgen]
    pub fn process(&self, n: u32, beta: f32, t_init: f32, c8: bool) -> Result<Vec<u8>, JsValue> {
        let res = process_img(
            &self.img.to_rgb(),
            n as usize,
            if c8 {
                CliqueType::Conn8
            } else {
                CliqueType::Conn4
            },
            beta,
            t_init,
        );
        let mut out = Vec::new();
        let encoder = image::png::PNGEncoder::new(&mut out);
        match encoder.encode(&res, res.width(), res.height(), image::ColorType::RGB(8)) {
            Err(e) => Err(js_sys::Error::new(&e.to_string()).into()),
            Ok(_) => Ok(out),
        }
    }
}
