// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{f32::consts::PI, ops::RangeInclusive};

use eframe::egui;
use egui::{epaint::PathStroke, Button, Color32, Painter, Pos2, Rgba, Stroke, Ui};
use singlelinefractal::{SingleLineRotatedFractal, SingleLineRotatedFractalNode};
use treefractal::TreeFractal;
const HEIGHT: f32 = 1000.0;
const WIDTH: f32 = 2000.0;

mod singlelinefractal;
mod treefractal;
mod mandelbrot;
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
    fractal_params:  Box<dyn Drawable>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fractal_params: Box::new(STARTING_CONDS_TREE.clone()),
        }
    }
}

const STARTING_CONDS_SINGLE_LINE: SingleLineRotatedFractal = SingleLineRotatedFractal {
    node: SingleLineRotatedFractalNode {
        origin: Pos2::new(WIDTH / 2.0, HEIGHT / 2.0),
        depth: 35,
        angle: 0.0,
        length: 220.0,
    },
    angle_add: PI / 3.0,
    length_factor: 1.01,
    spin_fractal: true,
    arm_rotation_speed: 0.05,
    whole_rotation_speed: 0.25,
    increment_angle_add: true,
    arms: 3,
    stroke: Stroke { width: 0.5, color: Color32::RED }
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

pub trait Drawable: std::fmt::Debug{
    fn update(&mut self, ctx: &egui::Context);
    fn draw(&mut self, painter: &Painter);
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
        });
    }
}
