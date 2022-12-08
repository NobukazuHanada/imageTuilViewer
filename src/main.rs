use image::GenericImageView;

fn main() {

    let img = image::open("Thumbsup1.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // getting pixels
    let pixels = img.pixels();
    let step = 25;

    for (x,y,rgba) in pixels {
        if x % step  == 0 && y % step == 0 {
            if x == 0 {
                println!("");
            }
            let [r,g,b,_] = rgba.0;
            let gray = (r as f64) * 0.3 + (g as f64) * 0.59 + (b as f64) * 0.11;
            let threshold = 120.0;
            if gray > threshold {
                print!("#");
            }else{
                print!(" ");
            }
        }
    }
    println!("");
}