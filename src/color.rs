use std::io::Write;

use crate::vec3;

pub type Color = vec3::Vec3;

pub fn write_color<T: Write>(out: &mut T, pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    out.write_fmt(format_args!("{} {} {}\n", rbyte, gbyte, bbyte));
}
