use markov::{process_img, CliqueType};

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
    let out = process_img(&rgb, 10, CliqueType::Conn8, 1.0, 1.0);
    out.save(output).expect("Failed to save output image");
}
