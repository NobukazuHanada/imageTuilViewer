use image::GenericImageView;

fn main() {

    let img = image::open("Thumbsup1.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
}