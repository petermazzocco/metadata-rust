use std::env;
use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;

fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if the image path is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_image>", args[0]);
        std::process::exit(1);
    }

    // Get the image path from the command line arguments
    let image_path = std::path::Path::new(&args[1]);
    let mut metadata = Metadata::new();

    // Set the EXIF tag and write it back to the image
    metadata.set_tag(ExifTag::Copyright("Hello World!!".to_string()));
    metadata.write_to_file(&image_path).expect("Failed to write metadata");
    println!("Metadata updated!")
}
