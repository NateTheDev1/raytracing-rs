mod lib;

use std::fs::{self, File, OpenOptions};
use std::io::Write;

use lib::{Color, Point3, Ray, Vec3};

fn write_image_to_file(data: &String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("image.ppm")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2.0 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn startup() {
    fs::remove_file("image.ppm").unwrap();

    File::create("image.ppm").unwrap();
}

fn ray_color(r: Ray) -> Color {
    if (hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r)) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::new(r.direction.x, r.direction.y, r.direction.z).unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    startup();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width as f64 / aspect_ratio;

    // Ca,era

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width as f32, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height as f32, 0.0);

    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let str = format!("P3\n{} {} \n255", image_width, image_height);
    write_image_to_file(&str);

    let mut cursor = image_height - 1.0;

    while cursor >= 1.0 {
        let mut sub_cursor = 0;

        print!("\x1B[2J\x1B[1;1H");
        println!("Scanlines remaining: {}", cursor);

        while sub_cursor < image_width {
            // let r = sub_cursor as f64 / (image_width - 1) as f64;
            // let g = cursor as f64 / (image_height - 1) as f64;
            // let b = 0.25;

            // let ir: f64 = 255.999 * r;
            // let ig: f64 = 255.999 * g;
            // let ib: f64 = 255.999 * b;

            // let ln = format!("{} {} {} ", ir as i32, ig as i32, ib as i32);

            // write_image_to_file(&ln);

            let u = sub_cursor as f32 / (image_width - 1) as f32;
            let v = cursor as f32 / (image_height - 1.0) as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(r);

            let ln = format!(
                "{} {} {} ",
                255.999 * pixel_color.x,
                255.999 * pixel_color.y,
                255.999 * pixel_color.z
            );

            write_image_to_file(&ln);

            sub_cursor += 1;
        }

        cursor -= 1 as f64;
    }

    println!("Done!");
}
