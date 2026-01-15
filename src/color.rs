use std::io::{self, Write};

use crate::{interval::Interval, vec3};

pub type Color = vec3::Vec3;

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color<T: Write>(out: &mut T, pixel_color: &Color) -> io::Result<()> {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    // translate [0, 1] rgb components to byte range [0, 255]
    const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };

    let rbyte = (256.0 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as i32;

    out.write_fmt(format_args!("{} {} {}\n", rbyte, gbyte, bbyte))
}
