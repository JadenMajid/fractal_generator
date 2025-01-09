// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{f32::consts::PI, ops::RangeInclusive};

use eframe::egui;
use egui::{epaint::PathStroke, Button, Painter, Pos2, Rgba, Ui};
use singlelinefractal::SingleLineRotatedFractal;
use treefractal::TreeFractal;
const HEIGHT: f32 = 1000.0;
const WIDTH: f32 = 2000.0;

mod singlelinefractal;
mod treefractal;

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "Fractal Generator",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Debug)]
struct MyApp {
    fractal_params:  Box<dyn DrawableFractal>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fractal_params: Box::new(STARTING_CONDS_TREE.clone()),
        }
    }
}

const STARTING_CONDS_SINGLE_LINE: SingleLineRotatedFractal = SingleLineRotatedFractal {
    origin: Pos2::new(WIDTH / 2.0, HEIGHT / 2.0),
    depth: 35,
    angle: 0.0,
    angle_add: PI / 3.0,
    length: 220.0,
    length_factor: 1.01,
    line_thickness: 0.5,
    color: [255., 0., 0.],
    spin_fractal: true,
    arm_rotation_speed: 0.05,
    whole_rotation_speed: 0.25,
    increment_angle_add: true,
    arms: 3,
};

const STARTING_CONDS_TREE: TreeFractal = TreeFractal {
    origin: Pos2::new(WIDTH / 2.0, HEIGHT / 2.0),
    depth: 8,
    angle: PI*3./2.,
    sweep_angle: PI / 3.0,
    length: 190.0,
    length_factor: 0.6,
    line_thickness: 0.5,
    color: [255., 255., 255.],
    arms: 3,
    centered: true,
    sweep: true,
    sweep_speed: 0.15
};

pub trait DrawableFractal: std::fmt::Debug{
    fn update(&mut self, ctx: &egui::Context);
    fn draw(&mut self, painter: &Painter);
    fn recurse(&mut self, painter: &Painter);
    fn draw_controls(&mut self, painter: &mut Ui);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            // Change Mode
            ui.heading("Fractal Generator");
            if ui.button("Single Line Fractal").clicked {
                self.fractal_params = Box::new(STARTING_CONDS_SINGLE_LINE.clone());
            }
            if ui.button("Tree Fractal").clicked {
                self.fractal_params = Box::new(STARTING_CONDS_TREE.clone());
            }


            let painter = ui.painter();
            self.fractal_params.draw(painter);
            self.fractal_params.draw_controls(ui);
            self.fractal_params.update(ctx);

            // ui.add(egui::Label::new("Background Color")); // Need to figure out later
            // egui::color_picker::color_edit_button_rgb(ui, &mut self.fractal_params.color);

            // ui.add(egui::Checkbox::new(&mut self.spin_fractal, "Spin Fractal"));
            // ui.add(egui::Checkbox::new(
            //     &mut self.increment_angle_add,
            //     "Advance Fractal",
            // ));

            // force egui to render new frames even if no new input is detected
            ui.ctx().request_repaint();
        });
    }
    
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
    
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    
        // _visuals.window_fill() would also be a natural choice
    }
    
    fn persist_egui_memory(&self) -> bool {
        true
    }
    
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}
