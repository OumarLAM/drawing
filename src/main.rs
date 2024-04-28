extern crate raster;

mod geometrical_shapes;
use geometrical_shapes as gs;

use gs::Drawable;
use raster::Image;

fn main() {
    // Create a new blank image with dimensions 1000x1000 pixels
    let _image = Image::blank(1000, 1000);

    //  Save the image to a file
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);
    gs::Rectangle::random(image.width, image.height).draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png");
}
