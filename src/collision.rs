// check for the collision between a circle of radius r centered at z
// and the line which contains points p and q
pub fn check_segment_collision(z: &(f32, f32), p: &(f32, f32), q: &(f32, f32), r: f32) -> bool {
    // solve for values alpha, beta that parameterize line between segments via
    // y + alpha * x + beta = 0
    let alpha = (q.1 - p.1) / (p.0 - q.0);
    let beta = (p.1 * q.0 - p.0 * q.1) / (p.0 - q.0);

    // now get coefficients of quadratic equation ax^2 + bx + c = 0
    // by plugging y = -alpha * x - b into circle equation (x-z.0)^2 + (y-z.1)^2 = r^2
    let a = 1f32 + alpha * alpha;
    let b = 2f32 * alpha * (beta + z.1) - 2f32 * z.0;
    let c = z.0 * z.0 + (beta + z.1) * (beta + z.1) - r * r;

    // line and circle intersect iff discrim = b^2 - 4ac >= 0
    let discriminant = b * b - 4f32 * a * c;
    if discriminant >= 0f32 {
        // get x values for solutions
        let x1 = (-b + discriminant.sqrt()) / (2f32 * a);
        let x2 = (-b - discriminant.sqrt()) / (2f32 * a);

        // check if it lies between segments
        let x_min = p.0.min(q.0);
        let x_max = p.0.max(q.0);

        if ((x1 >= x_min) && (x1 <= x_max)) {
            println!("x1={} is between x_min={} and x_max={}", x1, x_min, x_max);
            return true;
        }

        if ((x2 >= x_min) && (x2 <= x_max)) {
            println!("x2={} is between x_min={} and x_max={}", x2, x_min, x_max);
            return true;
        }

        false
    } else {
        false
    }
}

pub fn check_point_collision(z: &(f32, f32), p: &(f32, f32), r: f32) -> bool {
    let (dx, dy) = (z.0 - p.0, z.1 - p.1);
    dx * dx + dy * dy <= r * r
}
