mod framebuffer;
mod utils;

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use utils::draw::{polygon, fill_polygon};

fn main() {
    let mut fb = Framebuffer::new(800, 600, Color::DARKBLUE);

    // Estrella
    let poly1 = vec![
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];
    polygon(&mut fb, &poly1, Color::WHITE);
    fill_polygon(&mut fb, &poly1, Color::YELLOW, Color::WHITE);

    // Exporatr imagen PNG
    fb.export_to_file("out.png");
    // Exporatr imagen BMP
    fb.export_to_file("out.bmp");
}