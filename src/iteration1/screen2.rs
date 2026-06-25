use web_sys::CanvasRenderingContext2d as context;
use super::common::*;

pub fn draw_frame(ctx: &context, input1: f64, input2: f64) -> Result<(), wasm_bindgen::JsValue> {

    let sides = 3.0 + 3.0 * input1;
    let sides_whole = sides.floor();
    let percentage = sides - sides_whole;

    // calculate the two expanding points
    let angle_end = PI / (sides_whole + 1.0);
    let magnitude_start = (PI / sides_whole).cos();

    let angle = angle_end * percentage;
    let magnitude = magnitude_start + (1.0 - magnitude_start) * (((percentage-0.5)*PI).sin()/2.0+0.5);

    // create the relative sizes of the polygon angles used to create the points
    // the first and last size changes from 0.5 to 1.0 as the percentage increases untill it resets.
    // if the number of sides is even (meaning an odd number of relative sizes), the middle side shrinks from 1.0 to 0.0.
    // if the number of sides is odd (even #of relative sizes), the middle two shrink from 1.0 to 0.5.
    let mut relative_sizes: Vec<f64> = vec![1.0; sides_whole as usize + 1];
    relative_sizes[0] = 0.5 + 0.5 * percentage;
    relative_sizes[sides_whole as usize] = relative_sizes[0];
    if relative_sizes.len() % 2 == 0 {
        let size = 1.0 - 0.5 * percentage;
        let i = relative_sizes.len() / 2 - 1;
        relative_sizes[i] = size;
        relative_sizes[i + 1] = size;
    } else {
        let size = 1.0 - percentage;
        let i = (relative_sizes.len() - 1) / 2;
        relative_sizes[i] = size;
    }
    let total_relative_size = relative_sizes.iter().sum::<f64>();

    // create the points of the polygon
    let mut points: Vec<(f64, f64)> = Vec::new();
    let mut sum = 0.0;
    for i in 0..sides_whole as usize {
        sum += relative_sizes[i];
        let percent = sum / total_relative_size;
        let length_to_cover = 2.0 * (PI - angle);
        let offset = angle;
        let angle = length_to_cover * percent + offset;
        points.push((angle, 1.0));
    }

    // push the two expanding points
    points.push((angle, magnitude));
    points.push((-angle, magnitude));

    // if the number of sides is odd, adjust the magnitude of the middle point
    if sides_whole as usize % 2 == 1 {
        let i = (sides_whole as usize - 1) / 2;
        let end_magnitude = (PI / (sides_whole + 1.0)).cos();
        let magnitude = 1.0 - (1.0 - end_magnitude) * percentage;
        points[i].1 = magnitude;
    }

    // Order the points in a clockwise direction
    points.sort_by(|a, b| a.0.total_cmp(&b.0));

    // Convert the points into (x, y) coordinates
    let radius = 90.0;
    let mut coords = Vec::new();
    for point in points {
        let x = 100.0 + radius * point.0.sin() * point.1;
        let y = 100.0 - radius * point.0.cos() * point.1;
        coords.push((x, y));
    }

    // determine the bevel
    let bevel_radius = radius*0.3*input2;

    // Draw BG
    white_bg(ctx);

    // Draw the rounded polygon
    ctx.set_fill_style_str("black");
    ctx.begin_path();
    rounded_polygon(ctx, coords, bevel_radius)?;
    ctx.fill();

    Ok(())
}

struct Vector2D {
    //x: f64,
    //y: f64,
    len: f64,
    nx: f64,
    ny: f64,
    ang: f64,
}

impl Vector2D {
    // convert 2 points into vector form, polar form, and normalised 
    pub fn new((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> Self {
        let x = x2 - x1;
        let y = y2 - y1;
        let len = (x*x + y*y).sqrt();
        let nx = x / len;
        let ny = y / len;
        
        Self {
            //x,
            //y,
            len,
            nx: x / len,
            ny: y / len,
            ang: ny.atan2(nx),
        }
    }
}

/// create a rounded closed polygon. 
/// radius is the max rounding radius. 
/// 
/// one must call between:
/// ```
/// ctx.beginPath();
/// rounded_polygon(ctx, points, radius);
/// ctx.stroke();
/// ctx.fill();
/// ```
/// source: https://stackoverflow.com/questions/12012073/how-to-draw-a-rounded-polygon-in-canvas
fn rounded_polygon(ctx: &context, points: Vec<(f64, f64)>, radius: f64) -> Result<(), wasm_bindgen::JsValue> {

    let len = points.len();

    let mut p1 = points[len-1];
    for i in 0..len {
        let p2 = points[i % len];
        let p3 = points[(i + 1) % len];

        // Part 1
        let v1 = Vector2D::new(p2, p1);
        let v2 = Vector2D::new(p2, p3);
        let sin_a = v1.nx * v2.ny - v1.ny * v2.nx;
        let sin_a90 = v1.nx * v2.nx - v1.ny * -v2.ny;
        let mut angle = sin_a.clamp(-1.0, 1.0).asin();

        let mut rad_direction = 1.0;
        let mut draw_direction = false;
        if sin_a90 < 0.0 {
            if angle < 0.0 {
                angle = PI + angle;
            } else {
                angle = PI - angle;
                rad_direction = -1.0;
                draw_direction = true;
            }
        } else {
            if angle > 0.0 {
                rad_direction = -1.0;
                draw_direction = true;
            }
        }

        // Part 2
        let half_angle = angle / 2.0;

        // Part 3
        let mut len_out = (half_angle.cos() * radius / half_angle.sin()).abs();

        // Special Part A
        let c_radius;
        if len_out > (v1.len / 2.0).min(v2.len / 2.0) {
            len_out = (v1.len / 2.0).min(v2.len / 2.0);
            c_radius = (len_out * half_angle.sin() / half_angle.cos()).abs();
        } else {
            c_radius = radius;
        }

        // Part 4
        let mut x = p2.0 + v2.nx * len_out;
        let mut y = p2.1 + v2.ny * len_out;

        // Part 5
        x += -v2.ny * c_radius * rad_direction;
        y += v2.nx * c_radius * rad_direction;

        // Part 6
        ctx.arc_with_anticlockwise(x, y, c_radius, v1.ang + PI / 2.0 * rad_direction, v2.ang - PI / 2.0 * rad_direction, draw_direction)?;

        p1 = p2;
    }

    ctx.close_path();

    Ok(())
}