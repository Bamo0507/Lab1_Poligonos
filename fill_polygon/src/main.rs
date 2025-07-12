mod framebuffer;
mod utils;

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use utils::draw::{polygon, fill_polygon};

fn main() {
    let mut fb = Framebuffer::new(800, 600, Color::DARKBLUE);

    //Triangulo
    let poly3 = vec![
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];
    polygon(&mut fb, &poly3, Color::WHITE);
    fill_polygon(&mut fb, &poly3, Color::RED, Color::WHITE);

    // Exporatr imagen PNG
    fb.export_to_file("out.png");
    // Exporatr imagen BMP
    fb.export_to_file("out.bmp");
}