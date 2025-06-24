use eframe::egui::{self, ColorImage};
use eframe::{egui::TextureHandle, App, Frame, NativeOptions};
use egui::Color32;
use rayon::prelude::*;

struct MandelbrotApp {
    width: usize,
    height: usize,
    max_iter: usize,
    zoom: f64,
    center: (f64, f64),
    image: Option<ColorImage>,
    texture: Option<TextureHandle>,
}

impl Default for MandelbrotApp {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            max_iter: 500,
            zoom: 4.0,
            center: (-0.5, 0.0),
            image: None,
            texture: None,
        }
    }
}

impl MandelbrotApp {
    fn regenerate(&mut self, ctx: &egui::Context) {
        let img = generate_mandelbrot(
            self.width,
            self.height,
            self.max_iter,
            self.zoom,
            self.center,
        );
        self.texture = Some(ctx.load_texture("fractal", img.clone(), egui::TextureOptions::default()));
        self.image = Some(img);
    }
}

impl App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.image.is_none() {
            self.regenerate(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Regenerate").clicked() {
                    self.regenerate(ctx);
                }
                ui.add(egui::Slider::new(&mut self.max_iter, 10..=2000).text("Max iterations"));
                ui.add(egui::Slider::new(&mut self.zoom, 0.1..=10.0).text("Zoom"));
            });

            if let Some(texture) = &self.texture {
                ui.image(texture, egui::vec2(self.width as f32, self.height as f32));
            }
        });
    }
}

fn generate_mandelbrot(
    width: usize,
    height: usize,
    max_iter: usize,
    zoom: f64,
    center: (f64, f64),
) -> ColorImage {
    let mut data = vec![Color32::BLACK; width * height];
    let scale = zoom / width as f64;
    data.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
        let x = idx % width;
        let y = idx / width;
        let cx = center.0 + (x as f64 - width as f64 / 2.0) * scale;
        let cy = center.1 + (y as f64 - height as f64 / 2.0) * scale;
        let mut zx = 0.0;
        let mut zy = 0.0;
        let mut iter = 0;
        while zx * zx + zy * zy <= 4.0 && iter < max_iter {
            let temp = zx * zx - zy * zy + cx;
            zy = 2.0 * zx * zy + cy;
            zx = temp;
            iter += 1;
        }
        let t = iter as f32 / max_iter as f32;
        *pixel = if iter == max_iter {
            Color32::BLACK
        } else {
            Color32::from_rgb(
                (9.0 * (1.0 - t) * t * t * t * 255.0) as u8,
                (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8,
                (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8,
            )
        };
    });
    ColorImage { size: [width, height], pixels: data }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Rust Fractal",
        options,
        Box::new(|_cc| Box::<MandelbrotApp>::default()),
    )
}

