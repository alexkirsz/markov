use markov::{kmeans, random_barycenters, Vec3};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: {} <input> [output]", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let output = args.get(2).map(String::as_ref).unwrap_or("output.png");

    let img = image::open(input).expect("Failed to load input image");

    let rgb = img.to_rgb();
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

    let (points, barycenters) = kmeans(&values, &random_barycenters(&values, 8), 0.001);

    let out = image::RgbImage::from_fn(width, height, |x, y| {
        let vec = &points[(y * width + x) as usize];
        let clr = barycenters[vec.cluster];
        image::Rgb([
            (clr.x * 255.0).round() as u8,
            (clr.y * 255.0).round() as u8,
            (clr.z * 255.0).round() as u8,
        ])
    });

    out.save(output).expect("Failed to save output image");
}
