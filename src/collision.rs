// check for the collision between a circle of radius r centered at z
// and the line which contains points p and q
pub fn check_segment_collision(z: &(f32, f32), p: &(f32, f32), q: &(f32, f32), r: f32) -> bool {
    let d = (q.0 - p.0, q.1 - p.1);

    // If we parameterize the line with t as
    // (p_x + t(q_x - p_x), p_y + t*(q_y - p_y))
    // then plugging these in for circle equation we get these coefficients:
    let a = d.0 * d.0 + d.1 * d.1;
    let b = 2f32 * ((p.0 - z.0) * d.0 + (p.1 - z.1) * d.1);
    let c = (p.0 - z.0).powi(2) + (p.1 - z.1).powi(2) - r * r;

    // line and circle intersect iff discrim = b^2 - 4ac >= 0
    let discrim = b * b - 4f32 * a * c;
    if discrim >= 0f32 {
        let t1 = (-b + discrim.sqrt()) / (2f32 * a);
        let t2 = (-b - discrim.sqrt()) / (2f32 * a);

        (t1 >= 0f32 && t1 <= 1f32) || (t2 >= 0f32 && t2 <= 1f32)
    } else {
        false
    }
}

pub fn check_point_collision(z: &(f32, f32), p: &(f32, f32), r: f32) -> bool {
    let (dx, dy) = (z.0 - p.0, z.1 - p.1);
    dx * dx + dy * dy <= r * r
}
