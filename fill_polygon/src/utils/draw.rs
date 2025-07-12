use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

/// Fn para dibujar un punto
/// 
/// ---
/// x_coord -> Coordenada X del punto
/// y_coord -> Coordenada Y del punto
/// color -> Color del punto
pub fn dot(
    fb: &mut Framebuffer,
    p: Vector2,
    color: Color
) {
    fb.set_pixel(p.x as i32, p.y as i32, color);
}

/// Fn para dibujar una linea con algoritmo de Bresenham
/// ---
/// RaylibDrawHandle -> Pincel para dibujar
/// x0 -> Coordenada X del punto inicial
/// y0 -> Coordenada Y del punto inicial
/// x1 -> Coordenada X del punto final
/// y1 -> Coordenada Y del punto final
/// color -> Color de la linea
pub fn line(
    fb: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
    color: Color
){
    // Obtener coordenadas de inicio y fin de vectores
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    // Lo que nos desplazaremos
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    loop {
        fb.set_pixel(x0, y0, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = err * 2;

        // Movernos en X
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }

        // Movernos en Y
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Fn para dibujar un poligono
/// ---
/// fb -> Framebuffer donde se dibuja el poligono
/// vertices -> Puntos del poligono
/// color -> Color del poligono
pub fn polygon(
    fb: &mut Framebuffer,
    vertices: &[Vector2],
    color: Color
){
    if vertices.len() < 3 {
        println!("No es un polígono válido");
        return;
    }

    // Recorrer todos los pares de puntos
    for vertice in vertices.windows(2) {
        line(fb, vertice[0], vertice[1], color);
    }

    // Cerrar el poligono, tomamos el ultimo valor, y colocamos como 'punto destino'
    // el primero
    line(fb, *vertices.last().unwrap(), vertices[0], color);
}

/// Fn para pintar un poligono
/// ---
/// fb -> Framebuffer donde se dibuja el poligono
/// vertices -> Puntos del poligono
/// fill_color -> Color para rellenar el poligono
/// border_color -> Color del borde del poligono
pub fn fill_polygon(
    fb: &mut Framebuffer,
    vertices: &[Vector2],
    fill_color: Color,
    border_color: Color,
) {
    // Necesitamos al menos 3 vértices para que sea poligono
    if vertices.len() < 3 {
        return;
    }

    // Encontrar y‑mín y y‑máx - define el 'bounding box'
    let (mut y_min, mut y_max) = (vertices[0].y as i32, vertices[0].y as i32);
    for v in vertices {
        let y = v.y as i32;
        if y < y_min { y_min = y; }
        if y > y_max { y_max = y; }
    }

    // Recorrer cada scanline
    for y in y_min..=y_max {
        let mut x_intersections: Vec<f32> = Vec::new();

        // Calcular intersecciones con cada arista
        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];

            // Descartar horizontales
            if (v1.y - v2.y).abs() < f32::EPSILON {
                continue;
            }

            // Asegurar y1 < y2
            let (x1, y1, x2, y2) = if v1.y < v2.y {
                (v1.x, v1.y, v2.x, v2.y)
            } else {
                (v2.x, v2.y, v1.x, v1.y)
            };

            // La scanline debe estar en [y1, y2)
            if (y as f32) < y1 || (y as f32) >= y2 {
                continue;
            }

            // Intersección precisa
            let t = (y as f32 - y1) / (y2 - y1);
            x_intersections.push(x1 + t * (x2 - x1));
        }

        // Ordenar las intersecciones de menor a mayor
        x_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Rellenar pares usando regla even‑odd
        let mut i = 0;
        while i + 1 < x_intersections.len() {
            let x_left  = x_intersections[i];
            let x_right = x_intersections[i + 1];

            let x_start = if x_left.fract() == 0.0 {
                x_left as i32 + 1
            } else {
                x_left.ceil() as i32
            };

            let x_end = if x_right.fract() == 0.0 {
                x_right as i32 - 1 
            } else {
                x_right.floor() as i32 
            };

            if x_start <= x_end {
                for x in x_start..=x_end {
                    if let Some(c) = fb.get_pixel(x, y) {
                        if c == border_color {
                            continue;
                        }
                    }
                    fb.set_pixel(x, y, fill_color);
                }
            }

            i += 2; // siguiente par
        }
    }
}