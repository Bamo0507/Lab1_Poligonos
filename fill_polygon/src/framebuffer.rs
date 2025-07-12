use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    width: i32,
    height: i32,
}

// Implementación de Framebuffer, se parece al Object de Kotlin
impl Framebuffer {
    // Crea un framebuffer (matriz) de tamaño width x height
    pub fn new(width: i32, height: i32, bg_color: Color) -> Self {
        let image = Image::gen_image_color(width, height, bg_color);
        Framebuffer {
            image, width, height
        }
    }

    // Limpiar el FB con color dado
    pub fn clear(&mut self, color: Color){
        // Sobreescribir la image con el color dado
        self.image = Image::gen_image_color(self.width, self.height, color);
    }

    // Dibujar un pixel en el FB
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x>= 0 && x < self.width && y >= 0 && y < self.height {
            // Dibujar pixel en la imagen
            self.image.draw_pixel(x, y, color);
        } else {
            println!("Error: Pixel fuera de los límites del framebuffer");
        }
    }

    /// Devuelve el color almacenado en (x, y) o `None` si está fuera de rango
    pub fn get_pixel(&mut self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(self.image.get_color(x, y))
    }

    // Exportar el framebuffer a un archivo de imagen png o bitmap
    pub fn export_to_file(&self, file_path: &str) {
        self.image.export_image(file_path);
    }

    // Draw el framebuffer en la pantalla
    pub fn draw_on_screen(
        &self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        dst_x: i32,
        dst_y: i32,
    ) {
        // Carga la textura → Result<…>
        let tex = d
            .load_texture_from_image(thread, &self.image)
            .expect("No se pudo crear la textura");
        // Ahora sí, pinta la textura
        d.draw_texture(&tex, dst_x, dst_y, Color::WHITE);
    }

    // Devuelve la imagen del framebuffer
    pub fn image(&self) -> &Image {
        &self.image
    }

}