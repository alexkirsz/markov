use markov::{kmeans, random_barycenters, CliqueType, Vec3, MK};

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

    println!("K-Means");

    let (points, barycenters) = kmeans(&values, &random_barycenters(&values, 8), 0.001);

    println!("K-Means done");

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

    let mk_res = mk.recuit_simule(10, CliqueType::Conn4, 1.0, 1.0);
    println!("MK done");

    let out = image::RgbImage::from_fn(width, height, |x, y| {
        // let vec = &points[(y * width + x) as usize];
        // let clr = barycenters[vec.cluster];
        let cluster = mk_res[(y * width + x) as usize];
        let clr = barycenters[cluster];
        image::Rgb([
            (clr.x * 255.0).round() as u8,
            (clr.y * 255.0).round() as u8,
            (clr.z * 255.0).round() as u8,
        ])
    });

    out.save(output).expect("Failed to save output image");
}
