mod framebuffer;
mod utils;

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use utils::draw::{polygon, fill_polygon};

fn main() {
    let mut fb = Framebuffer::new(800, 600, Color::DARKBLUE);

    // Cuadrado
    let poly2 = vec![
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];
    polygon(&mut fb, &poly2, Color::WHITE);
    fill_polygon(&mut fb, &poly2, Color::BLUE, Color::WHITE);

    // Exporatr imagen PNG
    fb.export_to_file("out.png");
    // Exporatr imagen BMP
    fb.export_to_file("out.bmp");
}