mod vectors;

/**
 * Main
 */
fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;

    const IMAGE_WIDTH: u16 = 1280;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vectors::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vectors::Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = vectors::Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vectors::Vector3::new(0.0, 0.0, focal_length);

    // Renderer
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\rScanlines remaining: {} ({}%)",
            y,
            ((1.0 - (y as f32 / IMAGE_HEIGHT as f32)) * 100.0) as u16
        );
        for x in 0..IMAGE_WIDTH {
            let u = x as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v = y as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r = vectors::Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color: vectors::Color = vectors::ray_color(r);
            vectors::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
