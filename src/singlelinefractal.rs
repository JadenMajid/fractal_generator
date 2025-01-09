use std::{f32::consts::PI, ops::RangeInclusive};
use egui::{epaint::PathStroke, Painter, Pos2, Rgba, Ui};

use crate::DrawableFractal;


#[derive(Debug, Copy, Clone)]
pub(crate) struct SingleLineRotatedFractal {
    pub origin: Pos2,
    pub depth: u32,
    pub angle: f32,
    pub length: f32,
    pub angle_add: f32,
    pub length_factor: f32,
    pub line_thickness: f32,
    pub color: [f32; 3],
    pub arm_rotation_speed: f32,
    pub whole_rotation_speed: f32,
    pub increment_angle_add: bool,
    pub spin_fractal: bool,
    pub arms: i32
}

impl DrawableFractal for SingleLineRotatedFractal {
    fn update(&mut self, ctx: &egui::Context) {
        let dt = ctx.input(|i|{i.stable_dt});
        if self.spin_fractal {
            self.angle += self.whole_rotation_speed*dt;
            self.angle %= 2. * PI;
            if self.angle >= 2. * PI{
                self.angle = 0.0;
            } else if self.angle <= 0. {
                self.angle = 2.*PI;
            }
        }

        if self.increment_angle_add {
            self.angle_add += self.arm_rotation_speed*dt;
            self.angle_add %= 2. * PI;
        }
        self.origin = ctx.input(|i: &egui::InputState| i.screen_rect()).center();
    }

    fn draw(&mut self, painter: &Painter){
        
        for i in 1..self.arms + 1 {
            let mut state = self.clone();
            state.angle += 2.0 * PI / (self.arms as f32) * (i as f32);
            state.recurse(painter);
        }
    }

    fn recurse(&mut self, painter: &Painter) {
        if self.depth == 0 {
            return;
        }
        let vec_to_add = egui::Vec2::angled(self.angle) * self.length;
        let line_endpoint = self.origin + vec_to_add;
        painter.line_segment(
            [self.origin, line_endpoint],
            PathStroke::new(
                self.line_thickness,
                Rgba::from_rgb(
                    self.color[0],
                    self.color[1],
                    self.color[2],
                ),
            ),
        );

        self.origin = line_endpoint;
        self.length *= self.length_factor;
        self.depth -= 1;
        self.angle += self.angle_add;
        self.recurse(painter);
    }

    fn draw_controls(&mut self, ui: &mut Ui) {
            // This is all ui stuff, feel free to ignore
            ui.add(
                egui::Slider::new(
                    &mut self.angle,
                    RangeInclusive::new(0.0, 2.0 * PI),
                )
                .text("Fractal Angle"),
            );
            ui.add(
                egui::Slider::new(&mut self.whole_rotation_speed, RangeInclusive::new(-5., 5.))
                    .text("Fractal Rotation Speed"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.angle_add,
                    RangeInclusive::new(0.0, 2.0 * PI),
                )
                .text("Arm Angle"),
            );
            ui.add(
                egui::Slider::new(&mut self.arm_rotation_speed, RangeInclusive::new(0., 5.))
                    .text("Arm Rotation Speed"),
            );

            ui.add(egui::Slider::new(&mut self.arms, RangeInclusive::new(1, 20)).text("Arms"));
            ui.add(
                egui::Slider::new(
                    &mut self.length,
                    RangeInclusive::new(0.0, 800.0),
                )
                .text("Arm Length"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.line_thickness,
                    RangeInclusive::new(0.0, 10.0),
                )
                .text("Arm Thickness"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.length_factor,
                    RangeInclusive::new(0.000001, 3.),
                )
                .text("Arm Length Factor"),
            );
            ui.add(
                egui::Slider::new(&mut self.depth, RangeInclusive::new(1, 200))
                    .text("depth"),
            );
            ui.add(egui::Label::new("Fractal Color"));
            egui::color_picker::color_edit_button_rgb(ui, &mut self.color);
    }
}
