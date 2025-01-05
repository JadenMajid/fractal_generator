// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{f32::consts::PI, ops::RangeInclusive};

use eframe::egui;
use egui::{epaint::PathStroke, Painter, Pos2, Rgba};
const HEIGHT: f32 = 1000.0;
const WIDTH: f32 = 2000.0;

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "Fractal Generator",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Debug, Copy, Clone)]
struct MyApp {
    rotation_speed: f32,
    increment_angle_add: bool,
    spin_fractal: bool,
    arms: i32,
    fractal_params: FractalParams,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fractal_params: FractalParams {
                origin: Pos2::new(WIDTH / 2.0, HEIGHT / 2.0),
                depth: 30,
                angle: 0.0,
                angle_add: PI / 3.0,
                length: 300.0,
                length_factor: 0.9,
                line_thickness: 1.,
                color: [255., 0., 0.],
            },
            spin_fractal: true,
            rotation_speed: 0.001,
            increment_angle_add: true,
            arms: 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct FractalParams {
    origin: Pos2,
    depth: u32,
    angle: f32,
    length: f32,
    angle_add: f32,
    length_factor: f32,
    line_thickness: f32,
    color: [f32; 3],
}

fn recurse(fractal_params: &mut FractalParams, painter: &Painter) {
    if fractal_params.depth == 0 {
        return;
    }
    let vec_to_add = egui::Vec2::angled(fractal_params.angle) * fractal_params.length;
    let line_endpoint = fractal_params.origin + vec_to_add;
    painter.line_segment(
        [fractal_params.origin, line_endpoint],
        PathStroke::new(
            fractal_params.line_thickness,
            Rgba::from_rgb(
                fractal_params.color[0],
                fractal_params.color[1],
                fractal_params.color[2],
            ),
        ),
    );

    fractal_params.origin = line_endpoint;
    fractal_params.length *= fractal_params.length_factor;
    fractal_params.depth -= 1;
    fractal_params.angle += fractal_params.angle_add;
    recurse(&mut fractal_params.clone(), painter);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.spin_fractal {
            self.fractal_params.angle += self.rotation_speed;
        }

        if self.increment_angle_add {
            self.fractal_params.angle_add += self.rotation_speed;
            if self.fractal_params.angle_add >= 2. * PI {
                self.fractal_params.angle_add = 0.0;
            }
        }
        self.fractal_params.origin = ctx.input(|i: &egui::InputState| i.screen_rect()).center();

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            for i in 1..self.arms + 1 {
                let mut state = self.fractal_params.clone();
                state.angle += 2.0 * PI / (self.arms as f32) * (i as f32);
                recurse(&mut state, painter);
            }

            ui.heading("Fractal Generator");
            ui.add(
                egui::Slider::new(
                    &mut self.fractal_params.angle_add,
                    RangeInclusive::new(0.0, 2.0 * PI),
                )
                .text("Angle_Add"),
            );
            ui.add(
                egui::Slider::new(&mut self.rotation_speed, RangeInclusive::new(0.0, 0.01))
                    .text("Arm Rotation Speed"),
            );
            ui.add(egui::Slider::new(&mut self.arms, RangeInclusive::new(1, 10)).text("Arms"));
            ui.add(
                egui::Slider::new(
                    &mut self.fractal_params.length,
                    RangeInclusive::new(0.0, 400.0),
                )
                .text("Arm Length"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.fractal_params.line_thickness,
                    RangeInclusive::new(0.0, 10.0),
                )
                .text("Arm Thickness"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.fractal_params.length_factor,
                    RangeInclusive::new(0.000001, 3.),
                )
                .text("Arm Length Factor"),
            );
            ui.add(
                egui::Slider::new(&mut self.fractal_params.depth, RangeInclusive::new(1, 100))
                    .text("depth"),
            );
            ui.add(egui::Label::new("Fractal Color"));
            egui::color_picker::color_edit_button_rgb(ui, &mut self.fractal_params.color);
            // ui.add(egui::Label::new("Background Color")); // Need to figure out later
            // egui::color_picker::color_edit_button_rgb(ui, &mut self.fractal_params.color);

            ui.add(egui::Checkbox::new(&mut self.spin_fractal, "Spin Fractal"));
            ui.add(egui::Checkbox::new(
                &mut self.increment_angle_add,
                "Advance Fractal",
            ));

            ui.ctx().request_repaint();
        });
    }
}
