use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
}

trait RandomColor {
    fn random() -> Self;
}

trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

impl RandomColor for Color {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
        )
    }
}

impl Displayable for raster::Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            // Utilisez le résultat renvoyé par set_pixel et gérez les erreurs correctement
            if let Err(e) = self.set_pixel(x as i32, y as i32, color) {
                eprintln!("Erreur lors du réglage du pixel : {}", e);
            }
        }
    }
}
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(0, width);
        let y: i32 = rng.gen_range(0, height);

        Point::new(x, y)
    }
}

fn draw_point(point: &Point, image: &mut Image) {
    let _ = image.set_pixel(point.x, point.y, Color::random());
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        draw_point(self, image);
    }
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    // Define a constructor for Line
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            start: start.clone(),
            end: end.clone(),
        }
    }
    pub fn random(width: i32, height: i32) -> Self {
        // Générez deux points aléatoires pour les extrémités de la ligne

        let start = Point::random(width, height);
        let end = Point::random(width, height);

        // Retournez une nouvelle instance de Line avec les points aléatoires
        Line::new(&start, &end)
    }
}

fn drawline(line: &Line, image: &mut Image, color: &Color) {
    let dx = (line.end.x - line.start.x).abs();
    let dy = (line.end.y - line.start.y).abs();
    let sx = if line.start.x < line.end.x { 1 } else { -1 };
    let sy = if line.start.y < line.end.y { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = line.start.x;
    let mut y = line.start.y;

    while x != line.end.x || y != line.end.y {
        let _ = image.set_pixel(x, y, color.clone());
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn extract_line<'a>(start: &Point, end: &Point) -> (Point, Point) {
    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let min_y = start.y.min(end.y);
    let max_y = start.y.max(end.y);

    (
        Point {
            x: min_x.clone(),
            y: min_y.clone(),
        },
        Point {
            x: max_x.clone(),
            y: max_y.clone(),
        },
    )
}

fn extract_min_point<'a>(start: &Point, end: &Point) -> Point {
    let (min_point, _) = extract_line(start, end);
    min_point
}

fn extract_max_point<'a>(start: &Point, end: &Point) -> Point {
    let (_, max_point) = extract_line(start, end);
    max_point
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        drawline(&self, image, &Color::random());
    }
}

pub struct Rectangle {
    pub start: Point,
    pub end: Point,
}

impl Rectangle {
    pub fn new(start: &Point, end: &Point) -> Self {
        Rectangle {
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        // Générez deux points aléatoires pour les coins opposés du rectangle
        let start = Point::random(width, height);
        let end = Point::random(width, height);

        // Créez une nouvelle instance de Rectangle avec les coins opposés aléatoires
        Rectangle::new(
            &extract_min_point(&start, &end),
            &extract_max_point(&start, &end),
        )
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let ab = Line::new(
            &Point::new(self.start.x, self.start.y),
            &Point::new(self.start.x, self.end.y),
        );
        let bc = Line::new(
            &Point::new(self.start.x, self.start.y),
            &Point::new(self.end.x, self.start.y),
        );
        let cd = Line::new(
            &Point::new(self.end.x, self.start.y),
            &Point::new(self.end.x, self.end.y),
        );
        let ad = Line::new(
            &Point::new(self.start.x, self.end.y),
            &Point::new(self.end.x, self.end.y),
        );

        let color = Color::random();

        drawline(&ab, image, &color);
        drawline(&bc, image, &color);
        drawline(&cd, image, &color);
        drawline(&ad, image, &color);
    }
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        // Générez deux points aléatoires pour les coins opposés du rectangle
        let a = Point::random(width, height);
        let b = Point::random(width, height);
        let c = Point::random(width, height);

        // Déterminez les coordonnées des coins opposés du rectangle

        Triangle::new(&a, &b, &c)
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let ab = Line::new(&self.a, &self.b);

        let ac = Line::new(&self.a, &self.c);

        let bc = Line::new(&self.b, &self.c);

        let color = Color::random();

        drawline(&ab, image, &color);
        drawline(&ac, image, &color);
        drawline(&bc, image, &color);
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        // Générez deux points aléatoires pour les coins opposés du rectangle
        let mut rng = rand::thread_rng();
        let center = Point::random(width, height);
        let radius = rng.gen_range(10, width / 2);

        // Déterminez les coordonnées des coins opposés du rectangle

        Circle::new(&center, radius)
    }
}

fn draw_circle(circle: &Circle, image: &mut Image, color: &Color) {
    let mut x = 0;
    let mut y = circle.radius;
    let mut d = 3 - 2 * circle.radius;

    while x <= y {
        plot_circle_points(&circle.center, x, y, image, color);
        x += 1;
        if d < 0 {
            d = d + 4 * x + 6;
        } else {
            d = d + 4 * (x - y) + 10;
            y -= 1;
        }
    }
}

fn plot_circle_points(center: &Point, x: i32, y: i32, image: &mut Image, color: &Color) {
    draw_pixel(center.x + x, center.y + y, image, color);
    draw_pixel(center.x - x, center.y + y, image, color);
    draw_pixel(center.x + x, center.y - y, image, color);
    draw_pixel(center.x - x, center.y - y, image, color);
    draw_pixel(center.x + y, center.y + x, image, color);
    draw_pixel(center.x - y, center.y + x, image, color);
    draw_pixel(center.x + y, center.y - x, image, color);
    draw_pixel(center.x - y, center.y - x, image, color);
}

fn draw_pixel(x: i32, y: i32, image: &mut Image, color: &Color) {
    if x >= 0 && x < image.width && y >= 0 && y < image.height {
        let _ = image.set_pixel(x, y, color.clone());
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = Color::random();
        draw_circle(&self, image, &color);
    }
}
